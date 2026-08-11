use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GridPos {
    pub x: u16,
    pub z: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DirtyRegion {
    pub min: GridPos,
    pub max: GridPos,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NavGrid {
    width: u16,
    height: u16,
    blocked: Vec<bool>,
    height_centimetres: Vec<i16>,
    #[serde(skip)]
    dirty_regions: Vec<DirtyRegion>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum NavigationError {
    #[error("grid dimensions must be non-zero")]
    EmptyGrid,
    #[error("grid cell buffers do not match the declared dimensions")]
    BufferSize,
    #[error("start or goal is outside the grid")]
    OutsideGrid,
    #[error("start or goal is blocked")]
    BlockedEndpoint,
    #[error("no route exists")]
    NoRoute,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OpenNode {
    position: GridPos,
    estimated_total: u32,
    cost: u32,
}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_total
            .cmp(&self.estimated_total)
            .then_with(|| other.cost.cmp(&self.cost))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl NavGrid {
    pub fn new(
        width: u16,
        height: u16,
        blocked: Vec<bool>,
        height_centimetres: Vec<i16>,
    ) -> Result<Self, NavigationError> {
        if width == 0 || height == 0 {
            return Err(NavigationError::EmptyGrid);
        }
        let expected = usize::from(width) * usize::from(height);
        if blocked.len() != expected || height_centimetres.len() != expected {
            return Err(NavigationError::BufferSize);
        }
        Ok(Self {
            width,
            height,
            blocked,
            height_centimetres,
            dirty_regions: Vec::new(),
        })
    }

    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }

    #[must_use]
    pub fn contains(&self, position: GridPos) -> bool {
        position.x < self.width && position.z < self.height
    }

    #[must_use]
    pub fn is_walkable(&self, position: GridPos) -> bool {
        self.index(position)
            .is_some_and(|index| !self.blocked[index])
    }

    #[must_use]
    pub fn height_at(&self, position: GridPos) -> Option<i16> {
        self.index(position)
            .map(|index| self.height_centimetres[index])
    }

    pub fn set_blocked(
        &mut self,
        region: DirtyRegion,
        blocked: bool,
    ) -> Result<(), NavigationError> {
        if !self.contains(region.min) || !self.contains(region.max) {
            return Err(NavigationError::OutsideGrid);
        }
        for z in region.min.z..=region.max.z {
            for x in region.min.x..=region.max.x {
                let index = self
                    .index(GridPos { x, z })
                    .expect("validated grid position");
                self.blocked[index] = blocked;
            }
        }
        self.dirty_regions.push(region);
        Ok(())
    }

    pub fn take_dirty_regions(&mut self) -> Vec<DirtyRegion> {
        std::mem::take(&mut self.dirty_regions)
    }

    pub fn find_path(
        &self,
        start: GridPos,
        goal: GridPos,
    ) -> Result<Vec<GridPos>, NavigationError> {
        if !self.contains(start) || !self.contains(goal) {
            return Err(NavigationError::OutsideGrid);
        }
        if !self.is_walkable(start) || !self.is_walkable(goal) {
            return Err(NavigationError::BlockedEndpoint);
        }
        if start == goal {
            return Ok(vec![start]);
        }

        let mut open = BinaryHeap::new();
        let mut came_from = HashMap::<GridPos, GridPos>::new();
        let mut costs = HashMap::<GridPos, u32>::from([(start, 0)]);
        open.push(OpenNode {
            position: start,
            estimated_total: manhattan(start, goal) * 10,
            cost: 0,
        });

        while let Some(current) = open.pop() {
            if current.position == goal {
                return Ok(reconstruct_path(&came_from, start, goal));
            }
            if current.cost > costs[&current.position] {
                continue;
            }
            for neighbour in self.neighbours(current.position) {
                let height_delta =
                    (i32::from(self.height_at(neighbour).expect("neighbour is in bounds"))
                        - i32::from(
                            self.height_at(current.position)
                                .expect("current is in bounds"),
                        ))
                    .unsigned_abs();
                let next_cost = current.cost + 10 + height_delta / 100;
                if next_cost < *costs.get(&neighbour).unwrap_or(&u32::MAX) {
                    costs.insert(neighbour, next_cost);
                    came_from.insert(neighbour, current.position);
                    open.push(OpenNode {
                        position: neighbour,
                        estimated_total: next_cost + manhattan(neighbour, goal) * 10,
                        cost: next_cost,
                    });
                }
            }
        }
        Err(NavigationError::NoRoute)
    }

    fn index(&self, position: GridPos) -> Option<usize> {
        self.contains(position)
            .then(|| usize::from(position.z) * usize::from(self.width) + usize::from(position.x))
    }

    fn neighbours(&self, position: GridPos) -> impl Iterator<Item = GridPos> + '_ {
        let candidates = [
            position
                .x
                .checked_sub(1)
                .map(|x| GridPos { x, z: position.z }),
            position
                .x
                .checked_add(1)
                .map(|x| GridPos { x, z: position.z }),
            position
                .z
                .checked_sub(1)
                .map(|z| GridPos { x: position.x, z }),
            position
                .z
                .checked_add(1)
                .map(|z| GridPos { x: position.x, z }),
        ];
        candidates
            .into_iter()
            .flatten()
            .filter(|position| self.is_walkable(*position))
    }
}

fn manhattan(left: GridPos, right: GridPos) -> u32 {
    u32::from(left.x.abs_diff(right.x)) + u32::from(left.z.abs_diff(right.z))
}

fn reconstruct_path(
    came_from: &HashMap<GridPos, GridPos>,
    start: GridPos,
    goal: GridPos,
) -> Vec<GridPos> {
    let mut current = goal;
    let mut path = vec![goal];
    while current != start {
        current = came_from[&current];
        path.push(current);
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> NavGrid {
        NavGrid::new(8, 8, vec![false; 64], vec![0; 64]).unwrap()
    }

    #[test]
    fn path_routes_around_dynamic_building() {
        let mut grid = grid();
        grid.set_blocked(
            DirtyRegion {
                min: GridPos { x: 3, z: 0 },
                max: GridPos { x: 3, z: 6 },
            },
            true,
        )
        .unwrap();
        let path = grid
            .find_path(GridPos { x: 0, z: 0 }, GridPos { x: 7, z: 0 })
            .unwrap();
        assert!(path.iter().any(|position| position.z == 7));
        assert_eq!(grid.take_dirty_regions().len(), 1);
    }

    #[test]
    fn can_plan_for_three_hundred_agents() {
        let grid = grid();
        for agent in 0..300_u16 {
            let start = GridPos {
                x: agent % 8,
                z: (agent / 8) % 8,
            };
            let goal = GridPos {
                x: 7 - start.x,
                z: 7 - start.z,
            };
            assert!(grid.find_path(start, goal).is_ok());
        }
    }
}
