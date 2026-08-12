use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

pub const CURRENT_CONTENT_SCHEMA: u32 = 9;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ContentCatalog {
    pub schema_version: u32,
    #[serde(default)]
    pub archetypes: BTreeMap<StableId, ArchetypeDef>,
    pub buildings: BTreeMap<StableId, BuildingDef>,
    pub roles: BTreeMap<StableId, RoleDef>,
    pub technology: TechTree,
    #[serde(default)]
    pub source_records: BTreeMap<StableId, AuthoredRecord>,
}

/// A Unity prefab reduced to the stable data Bevy needs to spawn it.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ArchetypeDef {
    pub display_name: String,
    pub kind: ArchetypeKind,
    pub source_guid: String,
    pub source_path: String,
    pub bounds: ArchetypeBounds,
    pub footprint: [u16; 2],
    pub scenes: Vec<ArchetypeScene>,
    pub component_types: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeKind {
    Building,
    Player,
    Enemy,
    Resource,
    Environment,
    Vfx,
    Ui,
    #[default]
    Other,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ArchetypeBounds {
    pub center: [f32; 3],
    pub size: [f32; 3],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ArchetypeScene {
    pub source_model: String,
    /// Asset-server-relative path below `bevy-port/assets`.
    pub asset_path: String,
    pub age: Option<u8>,
    pub is_default: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BuildingDef {
    pub display_name: String,
    pub archetype: StableId,
    pub footprint: [u16; 2],
    pub cost: BTreeMap<StableId, u32>,
    pub placeable: bool,
    pub can_level: bool,
    pub level_cost: BTreeMap<StableId, u32>,
    pub level_cost_multiplier_per_thousand: u32,
    #[serde(default)]
    pub storage: Vec<StorageContribution>,
    #[serde(default)]
    pub station: Option<StationDef>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StorageContribution {
    pub resource: StableId,
    pub base_amount: u32,
    pub increment_amount: u32,
    pub level_multiplier_per_thousand: u32,
}

/// An authored Unity station reduced to deterministic, engine-independent data.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StationDef {
    pub accepts_all_roles: bool,
    #[serde(default)]
    pub accepted_role_kinds: BTreeSet<StableId>,
    pub targets_all: bool,
    #[serde(default)]
    pub target_kinds: BTreeSet<StableId>,
    pub max_targets: u16,
    pub update_milliseconds: u32,
    /// Unity ranges converted through its authored two-unit building grid.
    pub search_range_milli_cells: u32,
}

/// Renderer-node bindings from Unity's `CharacterModelHandler`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RoleEquipmentDef {
    pub body_nodes: [String; 3],
    pub left_hand_node: Option<String>,
    pub right_hand_node: Option<String>,
    pub helmet_node: Option<String>,
    pub carry_animation: Option<String>,
    pub left_hand_permanent: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RoleDef {
    pub display_name: String,
    pub movement_speed_multiplier_per_thousand: u16,
    pub experience_multiplier_per_thousand: u32,
    pub base_action_amount: u32,
    pub action_amount_per_level_milli: u32,
    pub base_action_milliseconds: u32,
    pub action_milliseconds_reduction_per_level: u32,
    pub base_action_range_milli_cells: u32,
    pub action_range_milli_cells_per_level: u32,
    pub base_health: u32,
    pub health_per_level_milli: u32,
    pub base_health_regen_per_second: i32,
    pub health_regen_milli_per_second_per_level: u32,
    pub base_damage_reduction_percent: i32,
    pub damage_reduction_milli_percent_per_level: u32,
    pub base_movement_speed_milli_cells_per_second: u32,
    pub movement_speed_milli_cells_per_second_per_level: u32,
    pub base_carry_capacity: u32,
    pub carry_capacity_per_level_milli: u32,
    #[serde(default)]
    pub resource: Option<StableId>,
    #[serde(default)]
    pub station_kinds: BTreeSet<StableId>,
    #[serde(default)]
    pub targets_all: bool,
    #[serde(default)]
    pub target_kinds: BTreeSet<StableId>,
    #[serde(default)]
    pub equipment: Option<RoleEquipmentDef>,
    pub granted_abilities: Vec<StableId>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TechTree {
    pub nodes: BTreeMap<StableId, TechNode>,
    #[serde(default)]
    pub groups: BTreeMap<StableId, TechGroup>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TechNode {
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    pub prerequisites: Vec<StableId>,
    pub unlocks: Vec<StableId>,
    /// Authored `Upgrade Building` effects, keyed by the affected building.
    #[serde(default)]
    pub building_level_caps: BTreeMap<StableId, u16>,
    /// Authored `Unlock Building` effects applied when this technology is unlocked.
    #[serde(default)]
    pub unlocked_buildings: BTreeSet<StableId>,
    /// Authored percentage reductions keyed by building type.
    #[serde(default)]
    pub building_cost_reduction_percent: BTreeMap<StableId, i32>,
    /// Unity supports a `BuildingType.Count` target for a global reduction.
    #[serde(default)]
    pub global_building_cost_reduction_percent: i32,
    /// Authored percentage storage boosts keyed by resource type.
    #[serde(default)]
    pub storage_boost_percent: BTreeMap<StableId, i32>,
    /// Authored global percentage stat boosts keyed by stat type.
    #[serde(default)]
    pub global_stat_boost_percent: BTreeMap<StableId, i32>,
    /// Authored percentage stat boosts keyed first by role, then by stat type.
    #[serde(default)]
    pub role_stat_boost_percent: BTreeMap<StableId, BTreeMap<StableId, i32>>,
    /// Authored `Age Up Building` effects. An unlocked effect selects the age-two scene.
    #[serde(default)]
    pub aged_buildings: BTreeSet<StableId>,
    pub objectives: Vec<StableId>,
    #[serde(default)]
    pub group: Option<StableId>,
    #[serde(default)]
    pub age: String,
    #[serde(default)]
    pub tier: i32,
    #[serde(default)]
    pub initially_unlocked: bool,
    #[serde(default)]
    pub unavailable: bool,
    #[serde(default)]
    pub icon_path: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TechGroup {
    pub display_name: String,
    pub nodes: Vec<StableId>,
}

/// Lossless, engine-independent provenance retained beside semantic content.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthoredRecord {
    pub source_guid: String,
    pub source_path: String,
    pub source_type: String,
    pub properties: BTreeMap<String, AuthoredValue>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AuthoredValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    Sequence(Vec<AuthoredValue>),
    Object(BTreeMap<String, AuthoredValue>),
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ContentError {
    #[error("unsupported content schema version {0}")]
    Schema(u32),
    #[error("technology {node} references missing prerequisite {prerequisite}")]
    MissingPrerequisite {
        node: StableId,
        prerequisite: StableId,
    },
    #[error("technology graph contains a cycle involving {0}")]
    TechnologyCycle(StableId),
    #[error("building {0} has an empty footprint")]
    EmptyFootprint(StableId),
    #[error("building {building} references missing archetype {archetype}")]
    MissingArchetype {
        building: StableId,
        archetype: StableId,
    },
    #[error("building {0} has an invalid zero level-cost multiplier")]
    InvalidLevelCostMultiplier(StableId),
    #[error("building {building} has an invalid zero storage multiplier for {resource}")]
    InvalidStorageMultiplier {
        building: StableId,
        resource: StableId,
    },
    #[error("building {0} has invalid station timing, range, or target capacity")]
    InvalidStation(StableId),
    #[error("role {role} equipment contains an empty renderer node for {slot}")]
    EmptyEquipmentNode { role: StableId, slot: &'static str },
    #[error("archetype {0} has an empty footprint")]
    EmptyArchetypeFootprint(StableId),
    #[error("archetype {archetype} has invalid scene asset path {path}")]
    InvalidScenePath { archetype: StableId, path: String },
    #[error("archetype {archetype} has {defaults} default scenes; expected exactly one")]
    DefaultSceneCount {
        archetype: StableId,
        defaults: usize,
    },
    #[error("technology group {group} references missing node {node}")]
    MissingGroupNode { group: StableId, node: StableId },
    #[error("technology {technology} references missing building {building}")]
    MissingTechnologyBuilding {
        technology: StableId,
        building: StableId,
    },
    #[error("technology {technology} references missing role {role}")]
    MissingTechnologyRole {
        technology: StableId,
        role: StableId,
    },
}

impl ContentCatalog {
    pub fn validate(&self) -> Result<(), ContentError> {
        if self.schema_version != CURRENT_CONTENT_SCHEMA {
            return Err(ContentError::Schema(self.schema_version));
        }
        for (id, archetype) in &self.archetypes {
            if archetype.footprint[0] == 0 || archetype.footprint[1] == 0 {
                return Err(ContentError::EmptyArchetypeFootprint(id.clone()));
            }
            for scene in &archetype.scenes {
                if !scene.asset_path.starts_with("migrated/models/")
                    || !Path::new(&scene.asset_path)
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
                    || scene.asset_path.contains("..")
                    || scene.asset_path.contains('\\')
                {
                    return Err(ContentError::InvalidScenePath {
                        archetype: id.clone(),
                        path: scene.asset_path.clone(),
                    });
                }
            }
            let defaults = archetype
                .scenes
                .iter()
                .filter(|scene| scene.is_default)
                .count();
            if !archetype.scenes.is_empty() && defaults != 1 {
                return Err(ContentError::DefaultSceneCount {
                    archetype: id.clone(),
                    defaults,
                });
            }
        }
        for (id, building) in &self.buildings {
            if building.footprint[0] == 0 || building.footprint[1] == 0 {
                return Err(ContentError::EmptyFootprint(id.clone()));
            }
            if !self.archetypes.contains_key(&building.archetype) {
                return Err(ContentError::MissingArchetype {
                    building: id.clone(),
                    archetype: building.archetype.clone(),
                });
            }
            if building.level_cost_multiplier_per_thousand == 0 {
                return Err(ContentError::InvalidLevelCostMultiplier(id.clone()));
            }
            if let Some(storage) = building
                .storage
                .iter()
                .find(|storage| storage.level_multiplier_per_thousand == 0)
            {
                return Err(ContentError::InvalidStorageMultiplier {
                    building: id.clone(),
                    resource: storage.resource.clone(),
                });
            }
            if building.station.as_ref().is_some_and(|station| {
                station.max_targets == 0
                    || station.update_milliseconds == 0
                    || station.search_range_milli_cells == 0
            }) {
                return Err(ContentError::InvalidStation(id.clone()));
            }
        }
        for (id, role) in &self.roles {
            if let Some(equipment) = &role.equipment {
                for (slot, node) in [
                    ("slim body", equipment.body_nodes[0].as_str()),
                    ("bulk body", equipment.body_nodes[1].as_str()),
                    ("feminine body", equipment.body_nodes[2].as_str()),
                ] {
                    if node.trim().is_empty() {
                        return Err(ContentError::EmptyEquipmentNode {
                            role: id.clone(),
                            slot,
                        });
                    }
                }
            }
        }
        for (technology_id, technology) in &self.technology.nodes {
            for building in technology
                .building_level_caps
                .keys()
                .chain(&technology.unlocked_buildings)
                .chain(technology.building_cost_reduction_percent.keys())
                .chain(&technology.aged_buildings)
            {
                if !self.buildings.contains_key(building) {
                    return Err(ContentError::MissingTechnologyBuilding {
                        technology: technology_id.clone(),
                        building: building.clone(),
                    });
                }
            }
            for role in technology.role_stat_boost_percent.keys() {
                if !self.roles.contains_key(role) {
                    return Err(ContentError::MissingTechnologyRole {
                        technology: technology_id.clone(),
                        role: role.clone(),
                    });
                }
            }
        }
        self.technology.validate()
    }
}

impl TechTree {
    pub fn validate(&self) -> Result<(), ContentError> {
        for (group_id, group) in &self.groups {
            for node in &group.nodes {
                if !self.nodes.contains_key(node) {
                    return Err(ContentError::MissingGroupNode {
                        group: group_id.clone(),
                        node: node.clone(),
                    });
                }
            }
        }
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
                        ..TechNode::default()
                    },
                ),
                (
                    second,
                    TechNode {
                        display_name: "Second".into(),
                        prerequisites: vec![first.clone()],
                        unlocks: vec![],
                        objectives: vec![],
                        ..TechNode::default()
                    },
                ),
            ]),
            groups: BTreeMap::new(),
        };
        assert_eq!(tree.validate(), Err(ContentError::TechnologyCycle(first)));
    }
}
