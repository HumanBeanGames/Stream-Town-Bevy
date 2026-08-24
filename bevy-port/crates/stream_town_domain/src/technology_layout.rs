//! Persistent, engine-independent authoring layout for the technology graph.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{StableId, TechTree};

pub const CURRENT_TECHNOLOGY_LAYOUT_SCHEMA: u32 = 1;

const NODE_WIDTH: f32 = 220.0;
const NODE_HEIGHT: f32 = 76.0;
const COLUMN_GAP: f32 = 70.0;
const ROW_GAP: f32 = 34.0;
const GROUP_PADDING: f32 = 54.0;
const GROUP_HEADER: f32 = 42.0;
const GROUP_GAP: f32 = 110.0;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GraphPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct GraphSize {
    pub width: f32,
    pub height: f32,
}

impl Default for GraphSize {
    fn default() -> Self {
        Self {
            width: NODE_WIDTH + GROUP_PADDING * 2.0,
            height: NODE_HEIGHT + GROUP_PADDING * 2.0 + GROUP_HEADER,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TechnologyNodeLayout {
    pub position: GraphPoint,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TechnologyGroupLayout {
    pub position: GraphPoint,
    pub size: GraphSize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TechnologyGraphLayout {
    pub schema_version: u32,
    pub nodes: BTreeMap<StableId, TechnologyNodeLayout>,
    pub groups: BTreeMap<StableId, TechnologyGroupLayout>,
}

impl Default for TechnologyGraphLayout {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_TECHNOLOGY_LAYOUT_SCHEMA,
            nodes: BTreeMap::new(),
            groups: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum TechnologyLayoutError {
    #[error("unsupported technology-layout schema version {0}")]
    Schema(u32),
    #[error("technology-layout node {0} is not present in the content catalog")]
    UnknownNode(StableId),
    #[error("content-catalog node {0} has no technology-layout entry")]
    MissingNode(StableId),
    #[error("technology-layout group {0} is not present in the content catalog")]
    UnknownGroup(StableId),
    #[error("content-catalog group {0} has no technology-layout entry")]
    MissingGroup(StableId),
    #[error("technology-layout {kind} {id} contains a non-finite coordinate")]
    NonFinite { kind: &'static str, id: StableId },
    #[error("technology-layout group {0} has a non-positive size")]
    InvalidGroupSize(StableId),
}

impl TechnologyGraphLayout {
    /// Creates a deterministic, readable fallback for catalogs without an
    /// imported Unity graph. Authored positions can subsequently be moved and
    /// persisted without changing runtime technology data.
    #[must_use]
    pub fn automatic(tree: &TechTree) -> Self {
        let depths = technology_depths(tree);
        let mut layout = Self::default();
        let mut cursor_y = 0.0;

        for (group_id, group) in &tree.groups {
            let mut columns = BTreeMap::<usize, Vec<StableId>>::new();
            for node_id in &group.nodes {
                if tree.nodes.contains_key(node_id) {
                    columns
                        .entry(depths.get(node_id).copied().unwrap_or_default())
                        .or_default()
                        .push(node_id.clone());
                }
            }
            for nodes in columns.values_mut() {
                nodes.sort();
            }

            let max_rows = columns.values().map(Vec::len).max().unwrap_or(1);
            let max_depth = columns.keys().copied().max().unwrap_or_default();
            let max_rows = bounded_layout_index(max_rows);
            let max_depth = bounded_layout_index(max_depth);
            let origin = GraphPoint {
                x: 0.0,
                y: cursor_y,
            };
            let size = GraphSize {
                width: (max_depth + 1.0) * NODE_WIDTH
                    + max_depth * COLUMN_GAP
                    + GROUP_PADDING * 2.0,
                height: max_rows * NODE_HEIGHT
                    + (max_rows - 1.0).max(0.0) * ROW_GAP
                    + GROUP_PADDING * 2.0
                    + GROUP_HEADER,
            };
            layout.groups.insert(
                group_id.clone(),
                TechnologyGroupLayout {
                    position: origin,
                    size,
                },
            );
            for (depth, nodes) in columns {
                for (row, node_id) in nodes.into_iter().enumerate() {
                    layout.nodes.insert(
                        node_id,
                        TechnologyNodeLayout {
                            position: GraphPoint {
                                x: origin.x
                                    + GROUP_PADDING
                                    + bounded_layout_index(depth) * (NODE_WIDTH + COLUMN_GAP),
                                y: origin.y
                                    + GROUP_HEADER
                                    + GROUP_PADDING
                                    + bounded_layout_index(row) * (NODE_HEIGHT + ROW_GAP),
                            },
                        },
                    );
                }
            }
            cursor_y += size.height + GROUP_GAP;
        }

        let mut ungrouped: Vec<_> = tree
            .nodes
            .iter()
            .filter(|(_, node)| node.group.is_none())
            .map(|(id, _)| id.clone())
            .collect();
        ungrouped.sort();
        for (index, node_id) in ungrouped.into_iter().enumerate() {
            layout.nodes.insert(
                node_id,
                TechnologyNodeLayout {
                    position: GraphPoint {
                        x: bounded_layout_index(index) * (NODE_WIDTH + COLUMN_GAP),
                        y: cursor_y,
                    },
                },
            );
        }
        layout
    }

    /// Removes stale entries and deterministically supplies entries for newly
    /// created catalog records while preserving every existing authored move.
    pub fn reconcile(&mut self, tree: &TechTree) {
        self.schema_version = CURRENT_TECHNOLOGY_LAYOUT_SCHEMA;
        self.nodes.retain(|id, _| tree.nodes.contains_key(id));
        self.groups.retain(|id, _| tree.groups.contains_key(id));
        let fallback = Self::automatic(tree);
        for id in tree.nodes.keys() {
            if let Some(value) = fallback.nodes.get(id) {
                self.nodes.entry(id.clone()).or_insert(*value);
            }
        }
        for id in tree.groups.keys() {
            if let Some(value) = fallback.groups.get(id) {
                self.groups.entry(id.clone()).or_insert(*value);
            }
        }
    }

    pub fn validate(&self, tree: &TechTree) -> Result<(), TechnologyLayoutError> {
        if self.schema_version != CURRENT_TECHNOLOGY_LAYOUT_SCHEMA {
            return Err(TechnologyLayoutError::Schema(self.schema_version));
        }
        for (id, node) in &self.nodes {
            if !tree.nodes.contains_key(id) {
                return Err(TechnologyLayoutError::UnknownNode(id.clone()));
            }
            if !node.position.x.is_finite() || !node.position.y.is_finite() {
                return Err(TechnologyLayoutError::NonFinite {
                    kind: "node",
                    id: id.clone(),
                });
            }
        }
        for id in tree.nodes.keys() {
            if !self.nodes.contains_key(id) {
                return Err(TechnologyLayoutError::MissingNode(id.clone()));
            }
        }
        for (id, group) in &self.groups {
            if !tree.groups.contains_key(id) {
                return Err(TechnologyLayoutError::UnknownGroup(id.clone()));
            }
            if !group.position.x.is_finite()
                || !group.position.y.is_finite()
                || !group.size.width.is_finite()
                || !group.size.height.is_finite()
            {
                return Err(TechnologyLayoutError::NonFinite {
                    kind: "group",
                    id: id.clone(),
                });
            }
            if group.size.width <= 0.0 || group.size.height <= 0.0 {
                return Err(TechnologyLayoutError::InvalidGroupSize(id.clone()));
            }
        }
        for id in tree.groups.keys() {
            if !self.groups.contains_key(id) {
                return Err(TechnologyLayoutError::MissingGroup(id.clone()));
            }
        }
        Ok(())
    }
}

fn technology_depths(tree: &TechTree) -> BTreeMap<StableId, usize> {
    fn visit(
        id: &StableId,
        tree: &TechTree,
        visiting: &mut BTreeSet<StableId>,
        depths: &mut BTreeMap<StableId, usize>,
    ) -> usize {
        if let Some(depth) = depths.get(id) {
            return *depth;
        }
        if !visiting.insert(id.clone()) {
            return 0;
        }
        let depth = tree.nodes.get(id).map_or(0, |node| {
            node.prerequisites
                .iter()
                .map(|parent| visit(parent, tree, visiting, depths).saturating_add(1))
                .max()
                .unwrap_or_default()
        });
        visiting.remove(id);
        depths.insert(id.clone(), depth);
        depth
    }

    let mut depths = BTreeMap::new();
    for id in tree.nodes.keys() {
        visit(id, tree, &mut BTreeSet::new(), &mut depths);
    }
    depths
}

fn bounded_layout_index(value: usize) -> f32 {
    f32::from(u16::try_from(value).unwrap_or(u16::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TechGroup, TechNode};

    fn id(value: &str) -> StableId {
        StableId::new(value).unwrap()
    }

    fn tree() -> TechTree {
        let root = id("tech:root");
        let child = id("tech:child");
        let group = id("tech_group:town");
        TechTree {
            nodes: BTreeMap::from([
                (
                    root.clone(),
                    TechNode {
                        display_name: "Root".to_owned(),
                        group: Some(group.clone()),
                        ..TechNode::default()
                    },
                ),
                (
                    child.clone(),
                    TechNode {
                        display_name: "Child".to_owned(),
                        prerequisites: vec![root.clone()],
                        group: Some(group.clone()),
                        ..TechNode::default()
                    },
                ),
            ]),
            groups: BTreeMap::from([(
                group,
                TechGroup {
                    display_name: "Town".to_owned(),
                    nodes: vec![root, child],
                },
            )]),
        }
    }

    #[test]
    fn automatic_layout_is_complete_deterministic_and_valid() {
        let tree = tree();
        let first = TechnologyGraphLayout::automatic(&tree);
        let second = TechnologyGraphLayout::automatic(&tree);
        assert_eq!(first, second);
        assert_eq!(first.nodes.len(), 2);
        assert_eq!(first.groups.len(), 1);
        first.validate(&tree).unwrap();
        assert!(
            first.nodes[&id("tech:child")].position.x > first.nodes[&id("tech:root")].position.x
        );
    }

    #[test]
    fn reconcile_preserves_moves_and_repairs_catalog_coverage() {
        let mut tree = tree();
        let mut layout = TechnologyGraphLayout::automatic(&tree);
        layout.nodes.get_mut(&id("tech:root")).unwrap().position.x = 1234.0;
        layout.nodes.remove(&id("tech:child"));
        tree.nodes.insert(
            id("tech:new"),
            TechNode {
                display_name: "New".to_owned(),
                ..TechNode::default()
            },
        );
        layout.reconcile(&tree);
        layout.validate(&tree).unwrap();
        assert!((layout.nodes[&id("tech:root")].position.x - 1234.0).abs() < f32::EPSILON);
        assert!(layout.nodes.contains_key(&id("tech:child")));
        assert!(layout.nodes.contains_key(&id("tech:new")));
    }
}
