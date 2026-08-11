use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ContentCatalog {
    pub schema_version: u32,
    pub buildings: BTreeMap<StableId, BuildingDef>,
    pub roles: BTreeMap<StableId, RoleDef>,
    pub technology: TechTree,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BuildingDef {
    pub display_name: String,
    pub archetype: StableId,
    pub footprint: [u16; 2],
    pub cost: BTreeMap<StableId, u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RoleDef {
    pub display_name: String,
    pub movement_speed_multiplier_per_thousand: u16,
    pub granted_abilities: Vec<StableId>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TechTree {
    pub nodes: BTreeMap<StableId, TechNode>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TechNode {
    pub display_name: String,
    pub prerequisites: Vec<StableId>,
    pub unlocks: Vec<StableId>,
    pub objectives: Vec<StableId>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ContentError {
    #[error("technology {node} references missing prerequisite {prerequisite}")]
    MissingPrerequisite {
        node: StableId,
        prerequisite: StableId,
    },
    #[error("technology graph contains a cycle involving {0}")]
    TechnologyCycle(StableId),
    #[error("building {0} has an empty footprint")]
    EmptyFootprint(StableId),
}

impl ContentCatalog {
    pub fn validate(&self) -> Result<(), ContentError> {
        for (id, building) in &self.buildings {
            if building.footprint[0] == 0 || building.footprint[1] == 0 {
                return Err(ContentError::EmptyFootprint(id.clone()));
            }
        }
        self.technology.validate()
    }
}

impl TechTree {
    pub fn validate(&self) -> Result<(), ContentError> {
        for (node_id, node) in &self.nodes {
            for prerequisite in &node.prerequisites {
                if !self.nodes.contains_key(prerequisite) {
                    return Err(ContentError::MissingPrerequisite {
                        node: node_id.clone(),
                        prerequisite: prerequisite.clone(),
                    });
                }
            }
        }
        let mut visiting = BTreeSet::new();
        let mut visited = BTreeSet::new();
        for node in self.nodes.keys() {
            self.visit(node, &mut visiting, &mut visited)?;
        }
        Ok(())
    }

    fn visit(
        &self,
        node: &StableId,
        visiting: &mut BTreeSet<StableId>,
        visited: &mut BTreeSet<StableId>,
    ) -> Result<(), ContentError> {
        if visited.contains(node) {
            return Ok(());
        }
        if !visiting.insert(node.clone()) {
            return Err(ContentError::TechnologyCycle(node.clone()));
        }
        for prerequisite in &self.nodes[node].prerequisites {
            self.visit(prerequisite, visiting, visited)?;
        }
        visiting.remove(node);
        visited.insert(node.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_technology_cycles() {
        let first = StableId::new("tech:first").unwrap();
        let second = StableId::new("tech:second").unwrap();
        let tree = TechTree {
            nodes: BTreeMap::from([
                (
                    first.clone(),
                    TechNode {
                        display_name: "First".into(),
                        prerequisites: vec![second.clone()],
                        unlocks: vec![],
                        objectives: vec![],
                    },
                ),
                (
                    second,
                    TechNode {
                        display_name: "Second".into(),
                        prerequisites: vec![first.clone()],
                        unlocks: vec![],
                        objectives: vec![],
                    },
                ),
            ]),
        };
        assert_eq!(tree.validate(), Err(ContentError::TechnologyCycle(first)));
    }
}
