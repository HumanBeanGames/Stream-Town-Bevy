use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
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
    #[serde(default)]
    topology_signature: u64,
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
        let topology_signature =
            calculate_topology_signature(width, height, &blocked, &height_centimetres);
        Ok(Self {
            width,
            height,
            blocked,
            height_centimetres,
            topology_signature,
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

    /// Constant-time fingerprint of the complete navigation topology.
    ///
    /// This lets runtime navigation caches detect occupancy changes without
    /// rescanning every terrain cell each rendered frame.
    #[must_use]
    pub fn topology_signature(&self) -> u64 {
        if self.topology_signature == 0 {
            calculate_topology_signature(
                self.width,
                self.height,
                &self.blocked,
                &self.height_centimetres,
            )
        } else {
            self.topology_signature
        }
    }

    /// Returns every traversable cardinal and diagonal step with its movement
    /// cost. Diagonals require both adjoining cardinal cells to remain open, so
    /// actors cannot cut through the corner of a building or resource node.
    #[must_use]
    pub fn walkable_neighbours(&self, position: GridPos) -> [Option<(GridPos, u32)>; 8] {
        self.walkable_neighbours_with(position, &|candidate| self.is_walkable(candidate))
    }

    pub fn set_blocked(
        &mut self,
        region: DirtyRegion,
        blocked: bool,
    ) -> Result<(), NavigationError> {
        if !self.contains(region.min) || !self.contains(region.max) {
            return Err(NavigationError::OutsideGrid);
        }
        if self.topology_signature == 0 {
            self.topology_signature = calculate_topology_signature(
                self.width,
                self.height,
                &self.blocked,
                &self.height_centimetres,
            );
        }
        let mut changed = false;
        for z in region.min.z..=region.max.z {
            for x in region.min.x..=region.max.x {
                let index = self
                    .index(GridPos { x, z })
                    .expect("validated grid position");
                if self.blocked[index] == blocked {
                    continue;
                }
                self.topology_signature ^=
                    topology_cell_token(index, self.blocked[index], self.height_centimetres[index]);
                self.blocked[index] = blocked;
                self.topology_signature ^=
                    topology_cell_token(index, self.blocked[index], self.height_centimetres[index]);
                changed = true;
            }
        }
        if changed {
            self.dirty_regions.push(region);
        }
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
        self.find_path_with_exceptions(start, goal, &HashSet::new())
    }

    /// Finds a route while treating selected in-bounds blocked cells as walkable.
    /// This preserves one authoritative occupancy grid while supporting actor-specific
    /// passages such as completed town gates.
    pub fn find_path_with_exceptions(
        &self,
        start: GridPos,
        goal: GridPos,
        walkable_exceptions: &HashSet<GridPos>,
    ) -> Result<Vec<GridPos>, NavigationError> {
        if !self.contains(start) || !self.contains(goal) {
            return Err(NavigationError::OutsideGrid);
        }
        let walkable = |position| {
            self.is_walkable(position)
                || (self.contains(position) && walkable_exceptions.contains(&position))
        };
        if !walkable(start) || !walkable(goal) {
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
            estimated_total: octile_distance(start, goal),
            cost: 0,
        });

        while let Some(current) = open.pop() {
            if current.position == goal {
                return Ok(reconstruct_path(&came_from, start, goal));
            }
            if current.cost > costs[&current.position] {
                continue;
            }
            for (neighbour, step_cost) in self
                .walkable_neighbours_with(current.position, &walkable)
                .into_iter()
                .flatten()
            {
                let next_cost = current.cost + step_cost;
                if next_cost < *costs.get(&neighbour).unwrap_or(&u32::MAX) {
                    costs.insert(neighbour, next_cost);
                    came_from.insert(neighbour, current.position);
                    open.push(OpenNode {
                        position: neighbour,
                        estimated_total: next_cost + octile_distance(neighbour, goal),
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

    fn walkable_neighbours_with(
        &self,
        position: GridPos,
        walkable: &impl Fn(GridPos) -> bool,
    ) -> [Option<(GridPos, u32)>; 8] {
        neighbour_candidates(position).map(|candidate| {
            let (neighbour, base_cost) = candidate?;
            if !walkable(neighbour) {
                return None;
            }
            if base_cost == 14 {
                let horizontal = GridPos {
                    x: neighbour.x,
                    z: position.z,
                };
                let vertical = GridPos {
                    x: position.x,
                    z: neighbour.z,
                };
                if !walkable(horizontal) || !walkable(vertical) {
                    return None;
                }
            }
            let height_delta = (i32::from(self.height_at(neighbour)?)
                - i32::from(self.height_at(position)?))
            .unsigned_abs();
            Some((neighbour, base_cost + height_delta / 100))
        })
    }
}

fn neighbour_candidates(position: GridPos) -> [Option<(GridPos, u32)>; 8] {
    [
        offset(position, -1, 0).map(|position| (position, 10)),
        offset(position, 1, 0).map(|position| (position, 10)),
        offset(position, 0, -1).map(|position| (position, 10)),
        offset(position, 0, 1).map(|position| (position, 10)),
        offset(position, -1, -1).map(|position| (position, 14)),
        offset(position, 1, -1).map(|position| (position, 14)),
        offset(position, -1, 1).map(|position| (position, 14)),
        offset(position, 1, 1).map(|position| (position, 14)),
    ]
}

fn offset(position: GridPos, x: i32, z: i32) -> Option<GridPos> {
    let x = i32::from(position.x).checked_add(x)?;
    let z = i32::from(position.z).checked_add(z)?;
    (x >= 0 && z >= 0 && x <= i32::from(u16::MAX) && z <= i32::from(u16::MAX)).then(|| GridPos {
        x: u16::try_from(x).expect("checked grid x fits u16"),
        z: u16::try_from(z).expect("checked grid z fits u16"),
    })
}

fn octile_distance(left: GridPos, right: GridPos) -> u32 {
    let x = u32::from(left.x.abs_diff(right.x));
    let z = u32::from(left.z.abs_diff(right.z));
    let diagonal = x.min(z);
    diagonal * 14 + (x.max(z) - diagonal) * 10
}

fn topology_cell_token(index: usize, blocked: bool, height_centimetres: i16) -> u64 {
    let height = u64::from(u16::from_le_bytes(height_centimetres.to_le_bytes()));
    let mut value = u64::try_from(index).unwrap_or(u64::MAX)
        ^ (height << 32)
        ^ (u64::from(blocked) << 63)
        ^ 0x9e37_79b9_7f4a_7c15;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn calculate_topology_signature(
    width: u16,
    height: u16,
    blocked: &[bool],
    height_centimetres: &[i16],
) -> u64 {
    blocked
        .iter()
        .copied()
        .zip(height_centimetres.iter().copied())
        .enumerate()
        .fold(
            (u64::from(width) << 48) ^ (u64::from(height) << 32) ^ 0xa076_1d64_78bd_642f,
            |signature, (index, (blocked, height))| {
                signature ^ topology_cell_token(index, blocked, height)
            },
        )
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
    fn actor_specific_exception_opens_only_the_requested_blocked_cell() {
        let mut grid = NavGrid::new(5, 3, vec![false; 15], vec![0; 15]).unwrap();
        grid.set_blocked(
            DirtyRegion {
                min: GridPos { x: 2, z: 0 },
                max: GridPos { x: 2, z: 2 },
            },
            true,
        )
        .unwrap();
        let start = GridPos { x: 0, z: 1 };
        let goal = GridPos { x: 4, z: 1 };
        assert_eq!(grid.find_path(start, goal), Err(NavigationError::NoRoute));
        let gate = GridPos { x: 2, z: 1 };
        let path = grid
            .find_path_with_exceptions(start, goal, &HashSet::from([gate]))
            .unwrap();
        assert!(path.contains(&gate));
        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&goal));
    }

    #[test]
    fn open_ground_paths_use_diagonal_steps() {
        let grid = grid();
        let path = grid
            .find_path(GridPos { x: 1, z: 1 }, GridPos { x: 6, z: 6 })
            .unwrap();
        assert_eq!(path.len(), 6);
        assert!(path.windows(2).all(|step| {
            step[0].x.abs_diff(step[1].x) == 1 && step[0].z.abs_diff(step[1].z) == 1
        }));
    }

    #[test]
    fn diagonal_steps_do_not_cut_blocked_corners() {
        let mut grid = grid();
        for position in [GridPos { x: 2, z: 1 }, GridPos { x: 1, z: 2 }] {
            grid.set_blocked(
                DirtyRegion {
                    min: position,
                    max: position,
                },
                true,
            )
            .unwrap();
        }
        let path = grid
            .find_path(GridPos { x: 1, z: 1 }, GridPos { x: 2, z: 2 })
            .unwrap();
        assert!(
            path.len() > 2,
            "blocked corner must not be crossed directly"
        );
    }

    #[test]
    fn topology_signature_changes_only_with_occupancy() {
        let mut grid = grid();
        let initial = grid.topology_signature();
        let position = GridPos { x: 3, z: 4 };
        grid.set_blocked(
            DirtyRegion {
                min: position,
                max: position,
            },
            false,
        )
        .unwrap();
        assert_eq!(grid.topology_signature(), initial);
        assert!(grid.take_dirty_regions().is_empty());

        grid.set_blocked(
            DirtyRegion {
                min: position,
                max: position,
            },
            true,
        )
        .unwrap();
        assert_ne!(grid.topology_signature(), initial);
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
