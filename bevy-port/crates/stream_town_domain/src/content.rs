use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StableId;

pub const CURRENT_CONTENT_SCHEMA: u32 = 33;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ContentCatalog {
    pub schema_version: u32,
    pub loading_screen: LoadingScreenDef,
    #[serde(default)]
    pub archetypes: BTreeMap<StableId, ArchetypeDef>,
    #[serde(default)]
    pub foliage: Vec<FoliageLayerDef>,
    pub buildings: BTreeMap<StableId, BuildingDef>,
    pub roles: BTreeMap<StableId, RoleDef>,
    /// Unity `TargetSettings` policy controlling whether a station replaces or
    /// incrementally repairs each bounded target list when its timer elapses.
    pub station_target_update_modes: BTreeMap<StableId, StationUpdateMode>,
    #[serde(default)]
    pub objectives: BTreeMap<StableId, ObjectiveDef>,
    pub technology: TechTree,
    #[serde(default)]
    pub source_records: BTreeMap<StableId, AuthoredRecord>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoadingScreenDef {
    pub progress_milli_per_second: u32,
    pub completion_hold_milliseconds: u32,
    pub tooltips: Vec<String>,
}

/// An authored Unity foliage generation layer reduced to portable world-space data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FoliageLayerDef {
    pub id: StableId,
    pub source_path: String,
    pub habitat: FoliageHabitat,
    pub source_size: u16,
    pub level_of_detail: u8,
    pub noise_scale: f32,
    pub octaves: u8,
    pub persistence: f32,
    pub lacunarity: f32,
    pub seed: i32,
    pub offset: [f32; 2],
    pub spawn_threshold: f32,
    pub spacing: u16,
    pub material_source_path: String,
    pub variants: Vec<FoliageVariantDef>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FoliageHabitat {
    Land,
    Underwater,
}

/// One converted FBX mesh choice in a Unity foliage generation layer.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FoliageVariantDef {
    pub source_model: String,
    pub asset_path: String,
    pub base_scale: [f32; 3],
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
    /// Unity `SimpleDisableAfterTime` lifetime for self-disabling pooled prefabs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_after_milliseconds: Option<u32>,
    /// Unity `UnitHealthBar` delay after a damaged unit returns to full health.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_bar_hide_milliseconds: Option<u32>,
    #[serde(default)]
    pub rotating_nodes: Vec<RotatingNodeDef>,
    /// Unity `Targetable.SizeSqr` expressed as its unsquared logical-cell size.
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub target_size_milli_cells: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enemy: Option<EnemyDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enemy_models: Option<EnemyModelSetDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enemy_spawner: Option<EnemySpawnerDef>,
    /// Unity `Pets.Pet` follow settings and the authored child-model transforms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pet: Option<PetDef>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
const fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}

/// Portable form of the shipping pet follower and its selectable model children.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PetDef {
    pub closest_distance: f32,
    pub max_distance: f32,
    pub min_move_speed: f32,
    pub max_move_speed: f32,
    pub rotation_radians_per_second: f32,
    pub models: BTreeMap<StableId, PetModelDef>,
}

/// One `PetModel` child resolved from prefab overrides into Bevy coordinates.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PetModelDef {
    pub source_model: String,
    pub local_position: [f32; 3],
    pub local_rotation: [f32; 4],
    pub local_scale: [f32; 3],
}

/// Authored `HealthHandler` and optional player-revival behavior attached to a prefab.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthDef {
    pub max_health: u32,
    pub health_gain_per_level: u32,
    pub regeneration_milli_per_second: i64,
    pub regeneration_requires_food: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revive_milliseconds: Option<u32>,
}

/// Authored combat behavior attached to a pooled Unity enemy prefab.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnemyDef {
    pub enemy_type: StableId,
    pub pool: StableId,
    pub additional_health_milli_per_player: u32,
    pub action_amount: u32,
    pub action_milliseconds: u32,
    pub action_range_milli_cells: u32,
    /// Unity `TargetSensor._targetSearchRange`, converted through the two-unit grid.
    pub target_search_range_milli_cells: u32,
    /// Whether taking damage replaces the current target with a valid attacker.
    pub attack_attacker: bool,
    pub kill_reward: ResourceReward,
    #[serde(default)]
    pub targets_all: bool,
    #[serde(default)]
    pub target_kinds: BTreeSet<StableId>,
}

/// Unity `EnemyModelHandler` choices and the animation contract tied to each weapon.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnemyModelSetDef {
    #[serde(default)]
    pub base_models: Vec<String>,
    #[serde(default)]
    pub permanent_models: Vec<String>,
    #[serde(default)]
    pub optional_models: Vec<String>,
    #[serde(default)]
    pub weapons: Vec<EnemyWeaponModelDef>,
    pub base_animation_variants: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnemyWeaponModelDef {
    pub main_model: String,
    #[serde(default)]
    pub off_hand_models: Vec<String>,
    pub action_animation: String,
    pub action_animation_variants: u8,
    pub run_animation: EnemyRunAnimation,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnemyRunAnimation {
    #[default]
    Generic,
    TwoHanded,
}

impl EnemyRunAnimation {
    #[must_use]
    pub const fn controller_index(self) -> i32 {
        match self {
            Self::Generic => 0,
            Self::TwoHanded => 1,
        }
    }
}

/// A resource amount granted by an authored gameplay event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceReward {
    pub resource: StableId,
    pub amount: u32,
}

/// One weighted enemy entry in a Unity `ChanceObjectList`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WeightedEnemySpawn {
    pub enemy_archetype: StableId,
    pub weight_milli: u32,
}

/// A Unity enemy-camp spawner expressed in deterministic grid coordinates.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnemySpawnerDef {
    pub min_total_enemies: u16,
    pub max_total_enemies: u16,
    pub spawn_milliseconds: u32,
    pub weighted_enemies: Vec<WeightedEnemySpawn>,
    pub spawn_offsets_milli_cells: Vec<[i32; 2]>,
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
    pub role_slots: Vec<RoleSlotContribution>,
    #[serde(default)]
    pub model_handlers: Vec<BuildingModelDef>,
    #[serde(default)]
    pub storage_models: Vec<StorageModelDef>,
    #[serde(default)]
    pub passive_resources: Vec<PassiveResourceContribution>,
    #[serde(default)]
    pub station: Option<StationDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targeting: Option<TargetingScoreDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projectile_shooter: Option<ProjectileShooterDef>,
}

/// Named glTF nodes controlled by one reachable Unity `BuildingModelHandler`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuildingModelDef {
    pub age: u8,
    pub full_model: String,
    pub construction_stages: [String; 3],
    pub upgrades: Vec<String>,
    pub other_models: Vec<String>,
}

/// Named storage-fill nodes controlled by a Unity `BuildingResourceModelHandler`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StorageModelDef {
    pub age: u8,
    pub resource: StableId,
    pub empty_model: String,
    pub half_full_model: String,
    pub full_model: String,
}

/// A prefab hierarchy node driven by Unity's reachable `SimpleRotateOnAxis` behavior.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RotatingNodeDef {
    pub hierarchy_path: String,
    pub age: Option<u8>,
    pub node: String,
    /// Unity local Euler-axis multiplier, preserved before applying speed.
    pub axis: [f32; 3],
    pub degrees_per_second: f32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PassiveResourceContribution {
    pub resource: StableId,
    pub base_milli_per_second: u32,
    pub increment_milli_per_level: u32,
    pub level_event_repetitions: u16,
}

/// A Unity `ProjectileShooter` reduced to deterministic grid-space values.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectileShooterDef {
    pub projectile_pool: String,
    pub movement_milli_cells_per_second: u32,
    pub damage: u32,
    pub range_milli_cells: u32,
    pub fire_milliseconds: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StorageContribution {
    pub resource: StableId,
    pub base_amount: u32,
    pub increment_amount: u32,
    pub level_multiplier_per_thousand: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RoleSlotContribution {
    pub role: StableId,
    pub base_amount: u16,
    pub increment_amount: u16,
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StationUpdateMode {
    Update,
    Clear,
}

/// Authored `Targetable.CalculateScore` weights converted to logical grid units.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TargetingScoreDef {
    /// Score added for every actor already assigned to this target.
    pub assignment_penalty_milli: u32,
    /// Score per logical grid cell between the actor and target.
    pub distance_penalty_milli_per_cell: u32,
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
    /// Exact parameter name from Unity's `AnimationName`/`Character.controller` contract.
    pub action_animation: String,
    pub action_animation_variants: u8,
    pub has_user_limit: bool,
    pub base_max_users: u16,
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectiveKind {
    Build,
    BuildAny,
    Collect,
    Kill,
    KillAny,
    EarnPerHour,
    Sell,
    SellAny,
    Buy,
    BuyAny,
}

/// An authored technology objective. Optional targets are populated only for
/// objective kinds that use them, avoiding Unity's misleading enum defaults.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ObjectiveDef {
    pub kind: ObjectiveKind,
    pub required_amount: u32,
    pub float_value_milli: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building: Option<StableId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enemy: Option<StableId>,
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
    #[error("loading-screen timing or tooltip data is invalid")]
    InvalidLoadingScreen,
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
    #[error("building {building} grants slots for missing role {role}")]
    MissingBuildingRoleSlot { building: StableId, role: StableId },
    #[error("building {0} has invalid station timing, range, or target capacity")]
    InvalidStation(StableId),
    #[error("building {0} has invalid projectile shooter values")]
    InvalidProjectileShooter(StableId),
    #[error("archetype {0} has invalid health or revival values")]
    InvalidHealth(StableId),
    #[error("archetype {0} has invalid enemy combat values")]
    InvalidEnemy(StableId),
    #[error("archetype {0} has invalid enemy-model data")]
    InvalidEnemyModels(StableId),
    #[error("foliage layer {0} has invalid generation or variant values")]
    InvalidFoliage(StableId),
    #[error("archetype {0} has invalid enemy-spawner values")]
    InvalidEnemySpawner(StableId),
    #[error("archetype {0} has invalid disable-after-time data")]
    InvalidDisableAfterTime(StableId),
    #[error("archetype {0} has invalid unit-health-bar data")]
    InvalidUnitHealthBar(StableId),
    #[error("enemy spawner {spawner} references invalid enemy archetype {enemy}")]
    InvalidSpawnedEnemy { spawner: StableId, enemy: StableId },
    #[error("role {role} equipment contains an empty renderer node for {slot}")]
    EmptyEquipmentNode { role: StableId, slot: &'static str },
    #[error("role {0} has an invalid action-animation contract")]
    InvalidRoleAnimation(StableId),
    #[error("archetype {0} has an empty footprint")]
    EmptyArchetypeFootprint(StableId),
    #[error("targetable archetype {0} has no authored target size")]
    InvalidTargetSize(StableId),
    #[error("archetype {archetype} has invalid scene asset path {path}")]
    InvalidScenePath { archetype: StableId, path: String },
    #[error("archetype {archetype} has {defaults} default scenes; expected exactly one")]
    DefaultSceneCount {
        archetype: StableId,
        defaults: usize,
    },
    #[error("technology group {group} references missing node {node}")]
    MissingGroupNode { group: StableId, node: StableId },
    #[error("technology {node} references missing group {group}")]
    MissingTechnologyGroup { node: StableId, group: StableId },
    #[error("technology {node} is assigned to group {group} but is absent from its node list")]
    TechnologyMissingFromGroup { node: StableId, group: StableId },
    #[error("technology group {group} lists node {node} assigned to a different group")]
    TechnologyGroupMismatch { node: StableId, group: StableId },
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
    #[error("technology {technology} references missing objective {objective}")]
    MissingTechnologyObjective {
        technology: StableId,
        objective: StableId,
    },
    #[error("objective {objective} references missing building {building}")]
    MissingObjectiveBuilding {
        objective: StableId,
        building: StableId,
    },
    #[error("objective {0} has a zero required amount")]
    InvalidObjectiveAmount(StableId),
    #[error("building {0} has invalid passive resource income")]
    InvalidPassiveResource(StableId),
    #[error("building {0} has invalid model-handler data")]
    InvalidBuildingModels(StableId),
    #[error("archetype {0} has invalid rotating-node data")]
    InvalidRotatingNode(StableId),
    #[error("archetype {0} has invalid pet follower data")]
    InvalidPet(StableId),
    #[error("building {0} has invalid target-scoring data")]
    InvalidTargetingScore(StableId),
    #[error("station target update policies do not cover the shipping target catalog")]
    InvalidStationTargetUpdateModes,
}

impl ContentCatalog {
    pub fn validate(&self) -> Result<(), ContentError> {
        if self.schema_version != CURRENT_CONTENT_SCHEMA {
            return Err(ContentError::Schema(self.schema_version));
        }
        if self.loading_screen.progress_milli_per_second == 0
            || self.loading_screen.completion_hold_milliseconds == 0
            || self.loading_screen.tooltips.is_empty()
            || self
                .loading_screen
                .tooltips
                .iter()
                .any(|tooltip| tooltip.trim().is_empty())
        {
            return Err(ContentError::InvalidLoadingScreen);
        }
        let shipping_target_kinds = [
            "target:player",
            "target:tree",
            "target:ore",
            "target:bush",
            "target:farm",
            "target:fish",
            "target:enemy",
            "target:boss",
            "target:building",
            "target:damaged_building",
            "target:construction",
            "target:injured_player",
            "target:dead_player",
        ];
        if shipping_target_kinds.iter().any(|kind| {
            StableId::new(*kind)
                .ok()
                .is_none_or(|kind| !self.station_target_update_modes.contains_key(&kind))
        }) {
            return Err(ContentError::InvalidStationTargetUpdateModes);
        }
        let mut foliage_ids = BTreeSet::new();
        for layer in &self.foliage {
            if !foliage_ids.insert(layer.id.clone())
                || layer.source_size == 0
                || layer.level_of_detail > 6
                || !layer.noise_scale.is_finite()
                || layer.noise_scale <= 0.0
                || layer.octaves == 0
                || layer.octaves > 8
                || !layer.persistence.is_finite()
                || !(0.0..=1.0).contains(&layer.persistence)
                || !layer.lacunarity.is_finite()
                || layer.lacunarity <= 0.0
                || layer.offset.iter().any(|value| !value.is_finite())
                || !layer.spawn_threshold.is_finite()
                || !(0.0..=1.0).contains(&layer.spawn_threshold)
                || layer.spacing == 0
                || layer.material_source_path.trim().is_empty()
                || layer.variants.is_empty()
                || layer.variants.iter().any(|variant| {
                    !valid_asset_path(&variant.asset_path)
                        || variant.source_model.trim().is_empty()
                        || variant
                            .base_scale
                            .iter()
                            .any(|value| !value.is_finite() || *value <= 0.0)
                })
            {
                return Err(ContentError::InvalidFoliage(layer.id.clone()));
            }
        }
        for (id, archetype) in &self.archetypes {
            if archetype.footprint[0] == 0 || archetype.footprint[1] == 0 {
                return Err(ContentError::EmptyArchetypeFootprint(id.clone()));
            }
            if archetype
                .component_types
                .iter()
                .any(|component| component.starts_with("Target.Targetable"))
                && archetype.target_size_milli_cells == 0
            {
                return Err(ContentError::InvalidTargetSize(id.clone()));
            }
            let has_disable_component = archetype
                .component_types
                .iter()
                .any(|component| component == "Utils.SimpleDisableAfterTime");
            if has_disable_component != archetype.disable_after_milliseconds.is_some()
                || archetype.disable_after_milliseconds == Some(0)
            {
                return Err(ContentError::InvalidDisableAfterTime(id.clone()));
            }
            let has_unit_health_bar = archetype
                .component_types
                .iter()
                .any(|component| component == "Units.UnitHealthBar");
            if has_unit_health_bar != archetype.health_bar_hide_milliseconds.is_some()
                || archetype.health_bar_hide_milliseconds == Some(0)
            {
                return Err(ContentError::InvalidUnitHealthBar(id.clone()));
            }
            let has_rotate_component = archetype
                .component_types
                .iter()
                .any(|component| component == "Utils.SimpleRotateOnAxis");
            if has_rotate_component == archetype.rotating_nodes.is_empty()
                || archetype.rotating_nodes.iter().any(|node| {
                    node.hierarchy_path.trim().is_empty()
                        || node.node.trim().is_empty()
                        || node.age.is_some_and(|age| !(1..=2).contains(&age))
                        || (archetype.kind == ArchetypeKind::Building && node.age.is_none())
                        || node.axis.iter().any(|value| !value.is_finite())
                        || node.axis.iter().all(|value| value.abs() <= f32::EPSILON)
                        || !node.degrees_per_second.is_finite()
                        || node.degrees_per_second.abs() <= f32::EPSILON
                })
            {
                return Err(ContentError::InvalidRotatingNode(id.clone()));
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
            if archetype.health.as_ref().is_some_and(|health| {
                health.max_health == 0
                    || health.regeneration_milli_per_second < 0
                    || health.revive_milliseconds == Some(0)
            }) {
                return Err(ContentError::InvalidHealth(id.clone()));
            }
            if archetype.enemy.as_ref().is_some_and(|enemy| {
                enemy.action_amount == 0
                    || enemy.action_milliseconds == 0
                    || enemy.action_range_milli_cells == 0
                    || enemy.target_search_range_milli_cells == 0
                    || enemy.kill_reward.amount == 0
                    || (!enemy.targets_all && enemy.target_kinds.is_empty())
            }) {
                return Err(ContentError::InvalidEnemy(id.clone()));
            }
            let has_enemy_model_handler = archetype
                .component_types
                .iter()
                .any(|component| component == "Enemies.EnemyModelHandler");
            if has_enemy_model_handler != archetype.enemy_models.is_some()
                || archetype.enemy_models.as_ref().is_some_and(|models| {
                    models.base_animation_variants == 0
                        || models
                            .base_models
                            .iter()
                            .chain(&models.permanent_models)
                            .chain(&models.optional_models)
                            .any(|name| name.trim().is_empty())
                        || models.weapons.iter().any(|weapon| {
                            weapon.main_model.trim().is_empty()
                                || weapon.action_animation.trim().is_empty()
                                || weapon.action_animation_variants == 0
                                || weapon
                                    .off_hand_models
                                    .iter()
                                    .any(|name| name.trim().is_empty())
                        })
                })
            {
                return Err(ContentError::InvalidEnemyModels(id.clone()));
            }
            if let Some(spawner) = &archetype.enemy_spawner {
                if spawner.min_total_enemies == 0
                    || spawner.max_total_enemies < spawner.min_total_enemies
                    || spawner.spawn_milliseconds == 0
                    || spawner.weighted_enemies.is_empty()
                    || spawner.spawn_offsets_milli_cells.is_empty()
                    || spawner
                        .weighted_enemies
                        .iter()
                        .any(|enemy| enemy.weight_milli == 0)
                {
                    return Err(ContentError::InvalidEnemySpawner(id.clone()));
                }
                for enemy in &spawner.weighted_enemies {
                    if self
                        .archetypes
                        .get(&enemy.enemy_archetype)
                        .is_none_or(|archetype| archetype.enemy.is_none())
                    {
                        return Err(ContentError::InvalidSpawnedEnemy {
                            spawner: id.clone(),
                            enemy: enemy.enemy_archetype.clone(),
                        });
                    }
                }
            }
            let has_pet_component = archetype
                .component_types
                .iter()
                .any(|component| component == "Pets.Pet");
            if has_pet_component != archetype.pet.is_some()
                || archetype.pet.as_ref().is_some_and(|pet| {
                    !pet.closest_distance.is_finite()
                        || pet.closest_distance < 0.0
                        || !pet.max_distance.is_finite()
                        || pet.max_distance <= pet.closest_distance
                        || !pet.min_move_speed.is_finite()
                        || pet.min_move_speed < 0.0
                        || !pet.max_move_speed.is_finite()
                        || pet.max_move_speed < pet.min_move_speed
                        || !pet.rotation_radians_per_second.is_finite()
                        || pet.rotation_radians_per_second <= 0.0
                        || pet.models.is_empty()
                        || pet.models.values().any(|model| {
                            model.source_model.trim().is_empty()
                                || !archetype
                                    .scenes
                                    .iter()
                                    .any(|scene| scene.source_model == model.source_model)
                                || model.local_position.iter().any(|value| !value.is_finite())
                                || model.local_rotation.iter().any(|value| !value.is_finite())
                                || model
                                    .local_scale
                                    .iter()
                                    .any(|value| !value.is_finite() || *value <= 0.0)
                        })
                })
            {
                return Err(ContentError::InvalidPet(id.clone()));
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
            for contribution in &building.role_slots {
                if !self.roles.contains_key(&contribution.role) {
                    return Err(ContentError::MissingBuildingRoleSlot {
                        building: id.clone(),
                        role: contribution.role.clone(),
                    });
                }
            }
            if building.passive_resources.iter().any(|income| {
                income.base_milli_per_second == 0
                    || (income.increment_milli_per_level > 0 && income.level_event_repetitions == 0)
            }) {
                return Err(ContentError::InvalidPassiveResource(id.clone()));
            }
            if building.model_handlers.iter().any(|model| {
                !(1..=2).contains(&model.age)
                    || model.full_model.trim().is_empty()
                    || model.upgrades.iter().any(|name| name.trim().is_empty())
                    || model.other_models.iter().any(|name| name.trim().is_empty())
                    || model
                        .construction_stages
                        .iter()
                        .any(|name| name.trim().is_empty())
            }) || building.storage_models.iter().any(|model| {
                !(1..=2).contains(&model.age)
                    || model.empty_model.trim().is_empty()
                    || model.half_full_model.trim().is_empty()
                    || model.full_model.trim().is_empty()
                    || !building
                        .storage
                        .iter()
                        .any(|storage| storage.resource == model.resource)
            }) {
                return Err(ContentError::InvalidBuildingModels(id.clone()));
            }
            if building.station.as_ref().is_some_and(|station| {
                station.max_targets == 0
                    || station.update_milliseconds == 0
                    || station.search_range_milli_cells == 0
            }) {
                return Err(ContentError::InvalidStation(id.clone()));
            }
            if building.targeting.as_ref().is_some_and(|targeting| {
                targeting.assignment_penalty_milli == 0
                    && targeting.distance_penalty_milli_per_cell == 0
            }) {
                return Err(ContentError::InvalidTargetingScore(id.clone()));
            }
            if building.projectile_shooter.as_ref().is_some_and(|shooter| {
                shooter.projectile_pool.trim().is_empty()
                    || shooter.movement_milli_cells_per_second == 0
                    || shooter.damage == 0
                    || shooter.range_milli_cells == 0
                    || shooter.fire_milliseconds == 0
            }) {
                return Err(ContentError::InvalidProjectileShooter(id.clone()));
            }
        }
        for (id, role) in &self.roles {
            if role.action_animation.trim().is_empty() || role.action_animation_variants == 0 {
                return Err(ContentError::InvalidRoleAnimation(id.clone()));
            }
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
        for (id, objective) in &self.objectives {
            if objective.required_amount == 0 {
                return Err(ContentError::InvalidObjectiveAmount(id.clone()));
            }
            if let Some(building) = &objective.building
                && !self.buildings.contains_key(building)
            {
                return Err(ContentError::MissingObjectiveBuilding {
                    objective: id.clone(),
                    building: building.clone(),
                });
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
            for objective in &technology.objectives {
                if !self.objectives.contains_key(objective) {
                    return Err(ContentError::MissingTechnologyObjective {
                        technology: technology_id.clone(),
                        objective: objective.clone(),
                    });
                }
            }
        }
        self.technology.validate()
    }
}

fn valid_asset_path(path: &str) -> bool {
    path.starts_with("migrated/models/")
        && Path::new(path)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
        && !path.contains("..")
        && !path.contains('\\')
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
                if self.nodes[node].group.as_ref() != Some(group_id) {
                    return Err(ContentError::TechnologyGroupMismatch {
                        node: node.clone(),
                        group: group_id.clone(),
                    });
                }
            }
        }
        for (node_id, node) in &self.nodes {
            if let Some(group_id) = &node.group {
                let Some(group) = self.groups.get(group_id) else {
                    return Err(ContentError::MissingTechnologyGroup {
                        node: node_id.clone(),
                        group: group_id.clone(),
                    });
                };
                if !group.nodes.contains(node_id) {
                    return Err(ContentError::TechnologyMissingFromGroup {
                        node: node_id.clone(),
                        group: group_id.clone(),
                    });
                }
            }
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

    #[test]
    fn technology_groups_are_bidirectionally_consistent() {
        let node_id = StableId::new("tech:grouped").unwrap();
        let group_id = StableId::new("tech_group:test").unwrap();
        let mut tree = TechTree {
            nodes: BTreeMap::from([(
                node_id.clone(),
                TechNode {
                    group: Some(group_id.clone()),
                    ..TechNode::default()
                },
            )]),
            groups: BTreeMap::from([(
                group_id.clone(),
                TechGroup {
                    display_name: "Test".to_owned(),
                    nodes: Vec::new(),
                },
            )]),
        };
        assert_eq!(
            tree.validate(),
            Err(ContentError::TechnologyMissingFromGroup {
                node: node_id.clone(),
                group: group_id.clone(),
            })
        );
        tree.groups.get_mut(&group_id).unwrap().nodes.push(node_id);
        assert_eq!(tree.validate(), Ok(()));
    }
}
