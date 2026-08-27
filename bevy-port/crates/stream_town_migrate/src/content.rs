use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use stream_town_domain::{
    ArchetypeBounds, ArchetypeDef, ArchetypeKind, ArchetypeScene, AuthoredRecord, AuthoredValue,
    BuildingDef, BuildingModelDef, ContentCatalog, EnemyCampGenerationDef, EnemyDef,
    EnemyModelSetDef, EnemyRunAnimation, EnemySpawnerDef, EnemyWeaponModelDef, FoliageHabitat,
    FoliageLayerDef, FoliageVariantDef, HealthDef, LoadingScreenDef, ObjectiveDef, ObjectiveKind,
    PassiveResourceContribution, PetDef, PetModelDef, ProjectileShooterDef, ResourceReward,
    RoleDef, RoleEquipmentDef, RoleSlotContribution, RotatingNodeDef, StableId, StationDef,
    StationUpdateMode, StorageContribution, StorageModelDef, TargetingScoreDef, TechGroup,
    TechNode, TechTree, WeightedEnemySpawn,
};

const BUILDING_CONTAINER: &str = "Assets/DefaultSettings/D_AllBuildingDataSettings.asset";
const BUILDING_PLACER: &str = "Assets/Prefabs/BuildingPlacer.prefab";
const ROLE_CONTAINER: &str = "Assets/DefaultSettings/D_AllRoleDataSettings.asset";
const TECH_TREE: &str = "Assets/Resources/TechTree/Technologies/TechTreeV2/TechTreeV2.asset";
const BUILDING_TYPE: &str = "ScriptablesProcessorInfrastructure.BuildingDataSettings";
const ROLE_TYPE: &str = "ScriptablesProcessorInfrastructure.RoleDataSettings";
const TECH_NODE_TYPE: &str = "TechTree.ScriptableObjects.Node_SO";
const PLAYER_PREFAB: &str = "Assets/Prefabs/Player_Character.prefab";
const POOL_SETTINGS: &str = "Assets/DefaultSettings/D_ObjectPoolingSettings.asset";
const LAND_FOLIAGE_SETTINGS: &str = "Assets/DefaultSettings/D_FoliageGenSettings.asset";
const WATER_FOLIAGE_SETTINGS: &str = "Assets/DefaultSettings/D_WaterFoliageGenSettings.asset";
const CAMP_SETTINGS: &str = "Assets/DefaultSettings/D_CampGenSettings.asset";
const TARGET_SETTINGS: &str = "Assets/DefaultSettings/D_TargetSettings.asset";
const LOADER_SCENE: &str = "Assets/Scenes/LOADER_INITIAL.unity";
const PET_PREFAB: &str = "Assets/Prefabs/Pets/Pet.prefab";

type ArchetypesById = BTreeMap<StableId, ArchetypeDef>;
type BuildingArchetypesBySlug = BTreeMap<String, (StableId, [u16; 2])>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ContentConversionReport {
    pub schema_version: u32,
    pub source_schema_version: u32,
    pub source_unity_version: String,
    pub source_sha256: String,
    pub source_assets: usize,
    pub source_warnings: usize,
    pub missing_main_objects: usize,
    pub archetypes: usize,
    pub archetype_scenes: usize,
    pub disable_after_time_prefabs: usize,
    pub unit_health_bar_prefabs: usize,
    pub pet_followers: usize,
    pub pet_models: usize,
    pub foliage_layers: usize,
    pub foliage_variants: usize,
    pub enemy_camp_generation_layers: usize,
    pub buildings: usize,
    pub building_prefabs: usize,
    pub building_model_handlers: usize,
    pub storage_model_handlers: usize,
    pub rotating_nodes: usize,
    pub passive_resource_generators: usize,
    pub enemy_resource_rewards: usize,
    pub enemy_model_handlers: usize,
    pub enemy_model_nodes: usize,
    pub roles: usize,
    pub technology_nodes: usize,
    pub technology_groups: usize,
    pub technology_edges: usize,
    pub technology_roots: usize,
    pub objectives: usize,
    pub station_target_update_modes: usize,
    pub warnings: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityExport {
    schema_version: u32,
    unity_version: String,
    assets: Vec<UnityAsset>,
    warnings: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityAsset {
    guid: String,
    path: String,
    kind: String,
    name: String,
    unity_type: Option<String>,
    status: String,
    #[serde(default)]
    serialized_fields: Vec<UnityField>,
    #[serde(default)]
    dependencies: Vec<UnityReference>,
    game_object: Option<UnityGameObject>,
    scene: Option<UnityScene>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityField {
    path: String,
    value: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityReference {
    #[serde(default)]
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityGameObject {
    #[serde(default)]
    components: Vec<UnityComponent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityScene {
    #[serde(default)]
    roots: Vec<UnityGameObject>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UnityComponent {
    #[serde(default)]
    hierarchy_path: String,
    #[serde(default)]
    #[serde(rename = "Type")]
    unity_type: Option<String>,
    #[serde(default)]
    fields: Vec<UnityField>,
}

#[derive(Clone, Debug)]
struct BuildingPlacement {
    prefab_guid: String,
    footprint: [u16; 2],
}

#[derive(Default)]
struct PoolIndex {
    pool_by_prefab_guid: BTreeMap<String, StableId>,
    archetype_by_pool_name: BTreeMap<String, StableId>,
}

pub fn convert(export_path: &Path, out_dir: &Path) -> Result<ContentConversionReport> {
    let bytes = fs::read(export_path)
        .with_context(|| format!("failed to read Unity export {}", export_path.display()))?;
    let source_sha256 = hex::encode(Sha256::digest(&bytes));
    let export: UnityExport = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse Unity export {}", export_path.display()))?;
    let (catalog, mut report) = convert_export(&export, source_sha256)?;
    catalog.validate().context("converted content is invalid")?;

    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create output directory {}", out_dir.display()))?;
    let catalog_path = out_dir.join("catalog.ron");
    let report_path = out_dir.join("conversion-report.ron");
    report.outputs = vec![
        normalized_path(&catalog_path),
        normalized_path(&report_path),
    ];
    write_ron_atomic(&catalog_path, &catalog)?;
    write_ron_atomic(&report_path, &report)?;

    let reloaded_catalog: ContentCatalog = ron::from_str(
        &fs::read_to_string(&catalog_path)
            .with_context(|| format!("failed to reload {}", catalog_path.display()))?,
    )
    .with_context(|| format!("failed to parse generated {}", catalog_path.display()))?;
    if reloaded_catalog != catalog {
        bail!("generated catalog changed during its RON round trip");
    }
    reloaded_catalog
        .validate()
        .context("reloaded content catalog is invalid")?;
    let reloaded_report: ContentConversionReport = ron::from_str(
        &fs::read_to_string(&report_path)
            .with_context(|| format!("failed to reload {}", report_path.display()))?,
    )
    .with_context(|| format!("failed to parse generated {}", report_path.display()))?;
    if reloaded_report != report {
        bail!("generated conversion report changed during its RON round trip");
    }
    Ok(report)
}

fn convert_export(
    export: &UnityExport,
    source_sha256: String,
) -> Result<(ContentCatalog, ContentConversionReport)> {
    if export.schema_version != 1 {
        bail!(
            "unsupported Unity content export schema {}",
            export.schema_version
        );
    }
    let assets_by_path: BTreeMap<_, _> = export
        .assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset))
        .collect();
    let assets_by_guid: BTreeMap<_, _> = export
        .assets
        .iter()
        .map(|asset| (asset.guid.as_str(), asset))
        .collect();

    let placements = building_placements(required_asset(&assets_by_path, BUILDING_PLACER)?)?;
    let pools = pool_index(required_asset(&assets_by_path, POOL_SETTINGS)?)?;
    let (archetypes, building_archetypes) = convert_archetypes(export, &placements, &pools)?;
    let mut foliage = foliage_layers(
        required_asset(&assets_by_path, LAND_FOLIAGE_SETTINGS)?,
        "_foliageGenerationSettings",
        FoliageHabitat::Land,
    )?;
    foliage.extend(foliage_layers(
        required_asset(&assets_by_path, WATER_FOLIAGE_SETTINGS)?,
        "_waterFoliageGenerationSettings",
        FoliageHabitat::Underwater,
    )?);
    let enemy_camp_generation = assets_by_path
        .get(CAMP_SETTINGS)
        .map(|asset| enemy_camp_generation_layers(asset, &pools))
        .transpose()?
        .unwrap_or_default();
    let role_equipment = role_equipment(required_asset(&assets_by_path, PLAYER_PREFAB)?)?;
    let station_target_update_modes =
        station_target_update_modes(required_asset(&assets_by_path, TARGET_SETTINGS)?)?;

    let building_guids = referenced_guids(
        required_asset(&assets_by_path, BUILDING_CONTAINER)?,
        "BuildingData.Array.data[",
        Some(BUILDING_TYPE),
    );
    let role_guids = referenced_guids(
        required_asset(&assets_by_path, ROLE_CONTAINER)?,
        "RoleData.Array.data[",
        Some(ROLE_TYPE),
    );
    let technology_guids = referenced_guids(
        required_asset(&assets_by_path, TECH_TREE)?,
        "<",
        Some(TECH_NODE_TYPE),
    );
    if building_guids.is_empty() || role_guids.is_empty() || technology_guids.is_empty() {
        bail!("active Unity content containers did not resolve their referenced assets");
    }

    let mut source_records = BTreeMap::new();
    let mut buildings = BTreeMap::new();
    for guid in &building_guids {
        let asset = required_guid_asset(&assets_by_guid, guid, BUILDING_TYPE)?;
        let building_name = required_string(asset, "BuildingName")?;
        let building_kind = required_enum(asset, "BuildingType")?;
        // `Count` is the Unity enum sentinel. An obsolete Forester Hut asset points
        // at it, but it is not reachable production content and has no prefab.
        if building_kind == "Count" {
            continue;
        }
        let slug = slug(&building_kind);
        let id = stable_id("building", &slug)?;
        let (archetype, footprint) = building_archetypes.get(&slug).with_context(|| {
            format!(
                "{} ({building_kind}) has no active prefab archetype",
                asset.path
            )
        })?;
        let mut cost = BTreeMap::new();
        let mut level_cost = BTreeMap::new();
        for (name, field) in [
            ("wood", "BuildResourceCost.WoodCost"),
            ("ore", "BuildResourceCost.OreCost"),
            ("food", "BuildResourceCost.FoodCost"),
            ("gold", "BuildResourceCost.GoldCost"),
        ] {
            cost.insert(stable_id("resource", name)?, required_u32(asset, field)?);
        }
        for (name, field) in [
            ("wood", "LevelResourceCost.WoodCost"),
            ("ore", "LevelResourceCost.OreCost"),
            ("food", "LevelResourceCost.FoodCost"),
            ("gold", "LevelResourceCost.GoldCost"),
        ] {
            level_cost.insert(stable_id("resource", name)?, required_u32(asset, field)?);
        }
        let level_multiplier = required_f64(asset, "CostIncreasePerLevelMultiplier")?;
        if !level_multiplier.is_finite() || level_multiplier <= 0.0 {
            bail!(
                "{} has invalid CostIncreasePerLevelMultiplier {level_multiplier}",
                asset.path
            );
        }
        let level_cost_multiplier_per_thousand = (level_multiplier * 1000.0)
            .round()
            .to_string()
            .parse::<u32>()
            .with_context(|| format!("{} level multiplier is out of range", asset.path))?;
        let prefab = required_guid_asset(
            &assets_by_guid,
            &archetypes[archetype].source_guid,
            "UnityEngine.GameObject",
        )?;
        buildings.insert(
            id.clone(),
            BuildingDef {
                display_name: building_name,
                archetype: archetype.clone(),
                footprint: *footprint,
                cost,
                placeable: required_bool(asset, "Placeable")?,
                can_level: required_bool(asset, "CanLevel")?,
                level_cost,
                level_cost_multiplier_per_thousand,
                storage: storage_contributions(prefab)?,
                role_slots: role_slot_contributions(prefab)?,
                model_handlers: building_model_definitions(prefab)?,
                storage_models: storage_model_definitions(prefab)?,
                passive_resources: passive_resource_contributions(prefab)?,
                station: station_definition(prefab)?,
                targeting: targeting_score_definition(prefab)?,
                projectile_shooter: projectile_shooter_definition(prefab)?,
            },
        );
        insert_source_record(&mut source_records, id, asset)?;
    }

    let mut roles = BTreeMap::new();
    for guid in &role_guids {
        let asset = required_guid_asset(&assets_by_guid, guid, ROLE_TYPE)?;
        let display_name = required_enum(asset, "Role")?;
        let role_slug = slug(&display_name);
        let id = stable_id("role", &role_slug)?;
        let base_speed = required_u32(asset, "BaseMovementSpeed")?;
        let movement_speed_multiplier_per_thousand = u16::try_from(base_speed.saturating_mul(1000))
            .with_context(|| format!("{} BaseMovementSpeed is out of range", asset.path))?;
        let resource = optional_enum(asset, "Resource")?
            .filter(|resource| resource != "None")
            .map(|resource| stable_id("resource", &slug(&resource)))
            .transpose()?;
        let (targets_all, target_kinds) = authored_mask(asset, "TargetFlags", "target")?;
        let (_, station_kinds) = authored_mask(asset, "StationFlags", "station")?;
        let mut granted_abilities = Vec::new();
        for (field, prefix) in [
            ("RoleFlags", "role_flag"),
            ("TargetFlags", "target"),
            ("Resource", "resource"),
            ("StationFlags", "station"),
        ] {
            let Some(value) = optional_enum(asset, field)? else {
                continue;
            };
            if value != "None" && value != "Nothing" {
                granted_abilities.push(stable_id(prefix, &slug(&value))?);
            }
        }
        granted_abilities.sort();
        granted_abilities.dedup();
        roles.insert(
            id.clone(),
            RoleDef {
                display_name,
                action_animation: animation_parameter_name(&required_enum(
                    asset,
                    "ActionAnimationName",
                )?),
                action_animation_variants: u8::try_from(required_u32(
                    asset,
                    "ActionAnimationVariants",
                )?)
                .with_context(|| {
                    format!("{} ActionAnimationVariants is out of range", asset.path)
                })?,
                has_user_limit: required_bool(asset, "HasUserLimit")?,
                base_max_users: u16::try_from(required_u32(asset, "BaseMaxUserLimit")?)
                    .with_context(|| format!("{} BaseMaxUserLimit is out of range", asset.path))?,
                movement_speed_multiplier_per_thousand,
                experience_multiplier_per_thousand: required_milli(asset, "ExpModifier")?,
                base_action_amount: required_u32(asset, "BaseActionAmount")?,
                action_amount_per_level_milli: required_milli(asset, "ActionAmountPerLevel")?,
                base_action_milliseconds: required_milli(asset, "BaseActionSpeed")?,
                action_milliseconds_reduction_per_level: required_milli(
                    asset,
                    "ActionSpeedPerLevel",
                )?,
                base_action_range_milli_cells: required_milli_cells(asset, "BaseActionRange")?,
                action_range_milli_cells_per_level: required_milli_cells(
                    asset,
                    "ActionRangePerLevel",
                )?,
                base_health: required_u32(asset, "BaseHealth")?,
                health_per_level_milli: required_milli(asset, "HealthPerLevel")?,
                base_health_regen_per_second: required_i32(asset, "BaseHealthRegen")?,
                health_regen_milli_per_second_per_level: required_milli(
                    asset,
                    "HealthRegenPerLevel",
                )?,
                base_damage_reduction_percent: required_i32(asset, "BaseDamageReduction")?,
                damage_reduction_milli_percent_per_level: required_milli(
                    asset,
                    "DamageReductionPerLevel",
                )?,
                base_movement_speed_milli_cells_per_second: required_milli_cells(
                    asset,
                    "BaseMovementSpeed",
                )?,
                movement_speed_milli_cells_per_second_per_level: required_milli_cells(
                    asset,
                    "MovementSpeedPerLevel",
                )?,
                base_carry_capacity: required_u32(asset, "BaseMaxResource")?,
                carry_capacity_per_level_milli: required_milli(asset, "MaxResourcePerLevel")?,
                resource,
                station_kinds,
                targets_all,
                target_kinds,
                equipment: role_equipment.get(&role_slug).cloned(),
                granted_abilities,
            },
        );
        insert_source_record(&mut source_records, id, asset)?;
    }

    let node_ids: BTreeMap<_, _> = technology_guids
        .iter()
        .map(|guid| Ok((guid.clone(), stable_id("tech", guid)?)))
        .collect::<Result<_>>()?;
    let mut prerequisites = BTreeMap::<String, BTreeSet<StableId>>::new();
    let mut technology_edges = 0;
    for guid in &technology_guids {
        let asset = required_guid_asset(&assets_by_guid, guid, TECH_NODE_TYPE)?;
        for child_guid in child_technology_guids(asset) {
            let Some(parent_id) = node_ids.get(guid) else {
                bail!("technology node {guid} has no stable ID");
            };
            if !node_ids.contains_key(&child_guid) {
                bail!(
                    "{} references technology {child_guid} outside the active TechTreeV2 catalog",
                    asset.path
                );
            }
            prerequisites
                .entry(child_guid)
                .or_default()
                .insert(parent_id.clone());
            technology_edges += 1;
        }
    }

    let mut nodes = BTreeMap::new();
    let mut objectives = BTreeMap::new();
    let mut groups = BTreeMap::<StableId, TechGroup>::new();
    for guid in &technology_guids {
        let asset = required_guid_asset(&assets_by_guid, guid, TECH_NODE_TYPE)?;
        let id = node_ids[guid].clone();
        let (global_building_cost_reduction_percent, building_cost_reduction_percent) =
            building_cost_reductions(asset)?;
        let (global_stat_boost_percent, role_stat_boost_percent) = stat_boosts(asset)?;
        let group_name = technology_group_name(&asset.path);
        let group_id = stable_id("tech_group", &slug(&group_name))?;
        groups
            .entry(group_id.clone())
            .or_insert_with(|| TechGroup {
                display_name: group_name,
                nodes: Vec::new(),
            })
            .nodes
            .push(id.clone());
        let unlocks = generated_record_ids(asset, "<Unlocks>k__BackingField", "unlock")?;
        let node_objectives = objective_definitions(asset)?;
        let objective_ids = node_objectives.keys().cloned().collect();
        objectives.extend(node_objectives);
        nodes.insert(
            id.clone(),
            TechNode {
                display_name: required_string(asset, "<TechName>k__BackingField")?,
                description: required_string(asset, "<Description>k__BackingField")?,
                prerequisites: prerequisites
                    .remove(guid)
                    .unwrap_or_default()
                    .into_iter()
                    .collect(),
                unlocks,
                building_level_caps: building_level_caps(asset)?,
                unlocked_buildings: unlocked_buildings(asset)?,
                building_cost_reduction_percent,
                global_building_cost_reduction_percent,
                storage_boost_percent: storage_boosts(asset)?,
                global_stat_boost_percent,
                role_stat_boost_percent,
                aged_buildings: aged_buildings(asset)?,
                objectives: objective_ids,
                group: Some(group_id),
                age: required_enum(asset, "<Age>k__BackingField")?,
                tier: required_i64(asset, "<Tier>k__BackingField")?
                    .try_into()
                    .with_context(|| format!("{} tier is out of range", asset.path))?,
                initially_unlocked: required_bool(asset, "<IsUnlocked>k__BackingField")?,
                unavailable: required_bool(asset, "<Unavailable>k__BackingField")?,
                icon_path: required_string(asset, "<IconPath>k__BackingField")?,
            },
        );
        insert_source_record(&mut source_records, id, asset)?;
    }
    for group in groups.values_mut() {
        group.nodes.sort();
    }
    let technology_roots = nodes
        .values()
        .filter(|node| node.prerequisites.is_empty())
        .count();
    let catalog = ContentCatalog {
        schema_version: stream_town_domain::CURRENT_CONTENT_SCHEMA,
        loading_screen: loading_screen_definition(required_asset(&assets_by_path, LOADER_SCENE)?)?,
        archetypes,
        foliage,
        enemy_camp_generation,
        buildings,
        roles,
        station_target_update_modes,
        objectives,
        technology: TechTree { nodes, groups },
        source_records,
    };
    catalog.validate().context("converted catalog is invalid")?;

    let report = ContentConversionReport {
        schema_version: 10,
        source_schema_version: export.schema_version,
        source_unity_version: export.unity_version.clone(),
        source_sha256,
        source_assets: export.assets.len(),
        source_warnings: export.warnings.len(),
        missing_main_objects: export
            .assets
            .iter()
            .filter(|asset| asset.status == "missing_main_object")
            .count(),
        archetypes: catalog.archetypes.len(),
        archetype_scenes: catalog
            .archetypes
            .values()
            .map(|archetype| archetype.scenes.len())
            .sum(),
        disable_after_time_prefabs: catalog
            .archetypes
            .values()
            .filter(|archetype| archetype.disable_after_milliseconds.is_some())
            .count(),
        unit_health_bar_prefabs: catalog
            .archetypes
            .values()
            .filter(|archetype| archetype.health_bar_hide_milliseconds.is_some())
            .count(),
        pet_followers: catalog
            .archetypes
            .values()
            .filter(|archetype| archetype.pet.is_some())
            .count(),
        pet_models: catalog
            .archetypes
            .values()
            .filter_map(|archetype| archetype.pet.as_ref())
            .map(|pet| pet.models.len())
            .sum(),
        foliage_layers: catalog.foliage.len(),
        foliage_variants: catalog
            .foliage
            .iter()
            .map(|layer| layer.variants.len())
            .sum(),
        enemy_camp_generation_layers: catalog.enemy_camp_generation.len(),
        buildings: catalog.buildings.len(),
        building_prefabs: building_archetypes.len(),
        building_model_handlers: catalog
            .buildings
            .values()
            .map(|building| building.model_handlers.len())
            .sum(),
        storage_model_handlers: catalog
            .buildings
            .values()
            .map(|building| building.storage_models.len())
            .sum(),
        rotating_nodes: catalog
            .archetypes
            .values()
            .map(|archetype| archetype.rotating_nodes.len())
            .sum(),
        passive_resource_generators: catalog
            .buildings
            .values()
            .map(|building| building.passive_resources.len())
            .sum(),
        enemy_resource_rewards: catalog
            .archetypes
            .values()
            .filter(|archetype| archetype.enemy.is_some())
            .count(),
        enemy_model_handlers: catalog
            .archetypes
            .values()
            .filter(|archetype| archetype.enemy_models.is_some())
            .count(),
        enemy_model_nodes: catalog
            .archetypes
            .values()
            .filter_map(|archetype| archetype.enemy_models.as_ref())
            .map(enemy_model_node_count)
            .sum(),
        roles: catalog.roles.len(),
        technology_nodes: catalog.technology.nodes.len(),
        technology_groups: catalog.technology.groups.len(),
        technology_edges,
        technology_roots,
        objectives: catalog.objectives.len(),
        station_target_update_modes: catalog.station_target_update_modes.len(),
        warnings: vec![
            "building footprints use the authored two-unit BuildingPlacer grid; Torch falls back to prefab bounds"
                .to_owned(),
            "prefab archetypes retain spawn-critical component types and converted GLB scene dependencies"
                .to_owned(),
            "pet follow distances, speeds, visible-forward rotation, model choices, and child transforms are converted from the shipping prefab"
                .to_owned(),
            "enemy camp bounds, spacing, count, spawner timing, weighted enemies, and spawn offsets are converted from the shipping settings and prefab"
                .to_owned(),
            "Unity technology objectives are promoted to typed semantic records; remaining authored fields are retained in source_records"
                .to_owned(),
        ],
        outputs: Vec::new(),
    };
    Ok((catalog, report))
}

fn authored_mask(
    asset: &UnityAsset,
    path: &str,
    prefix: &str,
) -> Result<(bool, BTreeSet<StableId>)> {
    let Some(value) = asset
        .serialized_fields
        .iter()
        .find(|field| field.path == path)
    else {
        return Ok((false, BTreeSet::new()));
    };
    let Some(object) = value.value.as_object() else {
        bail!("{} {path} is not an exported enum", asset.path);
    };
    mask_ids(object, prefix)
}

fn mask_ids(
    object: &serde_json::Map<String, Value>,
    prefix: &str,
) -> Result<(bool, BTreeSet<StableId>)> {
    if let Some(raw) = object.get("RawValue").and_then(Value::as_i64) {
        if raw < 0 {
            return Ok((true, BTreeSet::new()));
        }
        let names: &[(i64, &str)] = match prefix {
            "station" => &[
                (1, "food"),
                (2, "ore"),
                (4, "wood"),
                (8, "fish"),
                (16, "combat"),
                (32, "buildings"),
                (64, "enemy_camp"),
                (128, "raid_station"),
            ],
            "target" => &[
                (1, "player"),
                (2, "tree"),
                (4, "ore"),
                (8, "bush"),
                (16, "farm"),
                (32, "fish"),
                (64, "enemy"),
                (128, "boss"),
                (256, "building"),
                (512, "damaged_building"),
                (1024, "construction"),
                (2048, "injured_player"),
                (4096, "dead_player"),
            ],
            _ => bail!("unsupported authored mask prefix {prefix}"),
        };
        let mut remaining = raw;
        let mut ids = BTreeSet::new();
        for (bit, name) in names {
            if raw & bit != 0 {
                remaining &= !bit;
                ids.insert(stable_id(prefix, name)?);
            }
        }
        if remaining != 0 {
            bail!("{prefix} mask contains unknown bits {remaining:#x}");
        }
        return Ok((false, ids));
    }

    // Schema-1 exports produced before RawValue was added can only preserve a
    // single flag. Retain compatibility while new exports preserve combinations.
    if object
        .get("Index")
        .and_then(Value::as_i64)
        .unwrap_or_default()
        < 0
    {
        return Ok((true, BTreeSet::new()));
    }
    let Some(name) = object.get("Name").and_then(Value::as_str) else {
        return Ok((false, BTreeSet::new()));
    };
    if matches!(name, "None" | "Nothing") {
        return Ok((false, BTreeSet::new()));
    }
    Ok((false, BTreeSet::from([stable_id(prefix, &slug(name))?])))
}

fn station_target_update_modes(
    asset: &UnityAsset,
) -> Result<BTreeMap<StableId, StationUpdateMode>> {
    let count: usize = required_i64(asset, "_targetableData.Array.size")?
        .try_into()
        .with_context(|| format!("{} target-policy count is out of range", asset.path))?;
    let mut modes = BTreeMap::new();
    for index in 0..count {
        let prefix = format!("_targetableData.Array.data[{index}]");
        let target = required_enum(asset, &format!("{prefix}.TargetType"))?;
        let mode = match required_enum(asset, &format!("{prefix}.UpdateType"))?.as_str() {
            "Update" => StationUpdateMode::Update,
            "Clear" => StationUpdateMode::Clear,
            other => bail!("{} has unsupported station update mode {other}", asset.path),
        };
        let target = stable_id("target", &slug(&target))?;
        if modes.insert(target.clone(), mode).is_some() {
            bail!("{} repeats station target policy {target}", asset.path);
        }
    }
    Ok(modes)
}

fn station_definition(asset: &UnityAsset) -> Result<Option<StationDef>> {
    let Some(component) = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .find(|component| component_type(component) == "Buildings.Station")
    else {
        return Ok(None);
    };
    let component_mask = |path: &str, prefix: &str| -> Result<(bool, BTreeSet<StableId>)> {
        let value = component_field_value(component, path)
            .with_context(|| format!("{} station is missing {path}", asset.path))?;
        let object = value
            .as_object()
            .with_context(|| format!("{} station {path} is not an enum", asset.path))?;
        mask_ids(object, prefix)
    };
    let positive_u32 = |path: &str, scale: f64| -> Result<u32> {
        let value = component_field_value(component, path)
            .and_then(Value::as_f64)
            .with_context(|| format!("{} station {path} is invalid", asset.path))?;
        if !value.is_finite() || value <= 0.0 {
            bail!("{} station {path} must be positive", asset.path);
        }
        (value * scale)
            .round()
            .to_string()
            .parse()
            .with_context(|| format!("{} station {path} is out of range", asset.path))
    };
    let max_targets = component_field_value(component, "_maxListSize")
        .and_then(Value::as_u64)
        .and_then(|value| u16::try_from(value).ok())
        .filter(|value| *value > 0)
        .with_context(|| format!("{} station _maxListSize is invalid", asset.path))?;
    let (accepts_all_roles, accepted_role_kinds) = component_mask("_flags", "station")?;
    let (targets_all, target_kinds) = component_mask("_targetMask", "target")?;
    Ok(Some(StationDef {
        accepts_all_roles,
        accepted_role_kinds,
        targets_all,
        target_kinds,
        max_targets,
        update_milliseconds: positive_u32("_updateRate", 1_000.0)?,
        search_range_milli_cells: positive_u32("_targetSearchRange", 500.0)?,
    }))
}

fn targeting_score_definition(asset: &UnityAsset) -> Result<Option<TargetingScoreDef>> {
    let Some(component) = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .find(|component| component_type(component).starts_with("Target.Targetable"))
    else {
        return Ok(None);
    };
    let non_negative_milli = |path: &str, scale: f64| -> Result<u32> {
        let value = component_field_value(component, path)
            .and_then(Value::as_f64)
            .with_context(|| format!("{} targetable {path} is invalid", asset.path))?;
        if !value.is_finite() || value < 0.0 {
            bail!("{} targetable {path} must be non-negative", asset.path);
        }
        (value * scale)
            .round()
            .to_string()
            .parse()
            .with_context(|| format!("{} targetable {path} is out of range", asset.path))
    };
    Ok(Some(TargetingScoreDef {
        assignment_penalty_milli: non_negative_milli("_assignmentPenaltyMod", 1_000.0)?,
        // Unity's shipping terrain grid uses two world units per logical cell.
        distance_penalty_milli_per_cell: non_negative_milli("_distancePenaltyMod", 2_000.0)?,
    }))
}

fn targetable_size_milli_cells(asset: &UnityAsset) -> Result<u32> {
    let components = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components);
    let Some(targetable) = components
        .clone()
        .find(|component| component_type(component).starts_with("Target.Targetable"))
    else {
        return Ok(0);
    };
    let use_custom_size = component_field_value(targetable, "_useCustomSize")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let size = if use_custom_size {
        component_field_value(targetable, "_customSize")
            .and_then(Value::as_f64)
            .with_context(|| format!("{} targetable custom size is invalid", asset.path))?
    } else {
        components
            .filter(|component| {
                component.hierarchy_path.is_empty()
                    && component_type(component) == "UnityEngine.BoxCollider"
            })
            .find_map(|component| component_field_value(component, "size").and_then(vector3))
            .map_or(0.0, |size| size[0].max(size[2]))
    };
    if !size.is_finite() || size < 0.0 {
        bail!("{} targetable size must be non-negative", asset.path);
    }
    (size * 500.0)
        .round()
        .to_string()
        .parse()
        .with_context(|| format!("{} targetable size is out of range", asset.path))
}

fn health_definition(asset: &UnityAsset) -> Result<Option<HealthDef>> {
    let Some(component) = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .find(|component| component_type(component) == "Units.HealthHandler")
    else {
        return Ok(None);
    };
    let integer = |path: &str| {
        component_field_value(component, path)
            .and_then(Value::as_i64)
            .with_context(|| format!("{} health component is missing {path}", asset.path))
    };
    let numeric = |path: &str| {
        component_field_value(component, path)
            .and_then(Value::as_f64)
            .with_context(|| format!("{} health component is missing {path}", asset.path))
    };
    let max_health = u32::try_from(integer("_maxHealth")?)
        .with_context(|| format!("{} health maximum is out of range", asset.path))?;
    if max_health == 0 {
        bail!("{} health maximum must be positive", asset.path);
    }
    let regeneration = numeric("_healthRegen")?;
    if !regeneration.is_finite() || regeneration < 0.0 {
        bail!(
            "{} health regeneration must be finite and non-negative",
            asset.path
        );
    }
    let regeneration_milli_per_second = (regeneration * 1_000.0)
        .round()
        .to_string()
        .parse()
        .with_context(|| format!("{} health regeneration is out of range", asset.path))?;
    let regeneration_requires_food = component_field_value(component, "_regenRequiresFood")
        .and_then(Value::as_bool)
        .with_context(|| {
            format!(
                "{} health component is missing _regenRequiresFood",
                asset.path
            )
        })?;
    let health_gain_per_level = component_field_value(component, "_healthGainOnLevel")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .with_context(|| format!("{} health gain per level is invalid", asset.path))?;
    let revive_milliseconds = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .find(|component| component_type(component) == "Character.PlayerDeathHandler")
        .map(|death| {
            let seconds = component_field_value(death, "_reviveTime")
                .and_then(Value::as_f64)
                .with_context(|| format!("{} death handler is missing _reviveTime", asset.path))?;
            if !seconds.is_finite() || seconds <= 0.0 {
                bail!("{} revive time must be positive", asset.path);
            }
            (seconds * 1_000.0)
                .round()
                .to_string()
                .parse()
                .with_context(|| format!("{} revive time is out of range", asset.path))
        })
        .transpose()?;
    Ok(Some(HealthDef {
        max_health,
        health_gain_per_level,
        regeneration_milli_per_second,
        regeneration_requires_food,
        revive_milliseconds,
    }))
}

fn pet_definition(asset: &UnityAsset) -> Result<Option<PetDef>> {
    if asset.path != PET_PREFAB {
        return Ok(None);
    }
    let components = &asset
        .game_object
        .as_ref()
        .with_context(|| format!("{} has no exported hierarchy", asset.path))?
        .components;
    let pet = components
        .iter()
        .find(|component| component_type(component) == "Pets.Pet")
        .with_context(|| format!("{} has no Pets.Pet component", asset.path))?;
    let number = |component: &UnityComponent, path: &str| -> Result<f32> {
        let value = component_field_value(component, path)
            .and_then(Value::as_f64)
            .filter(|value| value.is_finite())
            .with_context(|| format!("{} pet component has invalid {path}", asset.path))?;
        value
            .to_string()
            .parse()
            .with_context(|| format!("{} pet component {path} is outside f32", asset.path))
    };
    let transform_vector = |component: &UnityComponent, path: &str| -> Result<[f32; 3]> {
        let value = component_field_value(component, path)
            .and_then(vector3)
            .with_context(|| format!("{} pet transform has invalid {path}", asset.path))?;
        let value = |index: usize| {
            value[index]
                .to_string()
                .parse::<f32>()
                .with_context(|| format!("{} pet transform {path} is outside f32", asset.path))
        };
        Ok([value(0)?, value(1)?, value(2)?])
    };
    let transform_quaternion = |component: &UnityComponent| -> Result<[f32; 4]> {
        let value = component_field_value(component, "localRotation")
            .and_then(Value::as_object)
            .with_context(|| format!("{} pet transform has invalid rotation", asset.path))?;
        let component_value = |name: &str| -> Result<f32> {
            value
                .get(name)
                .and_then(Value::as_f64)
                .filter(|value| value.is_finite())
                .with_context(|| format!("{} pet rotation has invalid {name}", asset.path))?
                .to_string()
                .parse()
                .with_context(|| format!("{} pet rotation {name} is outside f32", asset.path))
        };
        // Reflect Unity's left-handed +Z into Bevy's right-handed -Z.
        Ok([
            -component_value("x")?,
            -component_value("y")?,
            component_value("z")?,
            component_value("w")?,
        ])
    };

    let mut models = BTreeMap::new();
    for model in components
        .iter()
        .filter(|component| component_type(component) == "Pets.PetModel")
    {
        let pet_name = component_field_value(model, "_petType")
            .and_then(Value::as_object)
            .and_then(|value| value.get("Name"))
            .and_then(Value::as_str)
            .with_context(|| format!("{} pet model has no type", asset.path))?;
        let id = stable_id("pet", &slug(pet_name))?;
        let hierarchy_path = model.hierarchy_path.as_str();
        let transform = components
            .iter()
            .find(|component| {
                component.hierarchy_path == hierarchy_path
                    && component_type(component) == "UnityEngine.Transform"
            })
            .with_context(|| {
                format!("{} pet model {pet_name} has no root transform", asset.path)
            })?;
        let source_model = components
            .iter()
            .filter(|component| {
                component.hierarchy_path == hierarchy_path
                    || component
                        .hierarchy_path
                        .strip_prefix(hierarchy_path)
                        .is_some_and(|suffix| suffix.starts_with('/'))
            })
            .flat_map(|component| &component.fields)
            .filter_map(|field| reference(&field.value))
            .filter_map(|reference| reference.get("Path").and_then(Value::as_str))
            .find(|path| {
                Path::new(path)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("fbx"))
            })
            .with_context(|| format!("{} pet model {pet_name} has no FBX", asset.path))?
            .to_owned();
        let mut local_position = transform_vector(transform, "localPosition")?;
        local_position[2] = -local_position[2];
        let definition = PetModelDef {
            source_model,
            local_position,
            local_rotation: transform_quaternion(transform)?,
            local_scale: transform_vector(transform, "localScale")?,
        };
        if models.insert(id.clone(), definition).is_some() {
            bail!("{} contains duplicate pet model {id}", asset.path);
        }
    }
    Ok(Some(PetDef {
        closest_distance: number(pet, "_closestDistanceToPlayer")?,
        max_distance: number(pet, "_maxDistanceFromPlayer")?,
        min_move_speed: number(pet, "_minMoveSpeed")?,
        max_move_speed: number(pet, "_maxMoveSpeed")?,
        rotation_radians_per_second: number(pet, "_rotationSpeed")?,
        models,
    }))
}

fn pool_index(asset: &UnityAsset) -> Result<PoolIndex> {
    let size = required_u32(asset, "_objectsToPool.Array.size")?;
    let mut index = PoolIndex::default();
    for pool_index in 0..size {
        let prefix = format!("_objectsToPool.Array.data[{pool_index}]");
        let name = required_string(asset, &format!("{prefix}.Name"))?;
        let prefab_guid = field_value(asset, &format!("{prefix}.Prefab"))
            .and_then(reference)
            .and_then(|reference| reference.get("Guid"))
            .and_then(Value::as_str)
            .filter(|guid| !guid.is_empty())
            .with_context(|| format!("{} {prefix} has no prefab GUID", asset.path))?;
        let pool = stable_id("pool", &slug(&name))?;
        let archetype = stable_id("archetype:prefab", prefab_guid)?;
        index
            .pool_by_prefab_guid
            .insert(prefab_guid.to_owned(), pool);
        index.archetype_by_pool_name.insert(slug(&name), archetype);
    }
    Ok(index)
}

fn enemy_definition(asset: &UnityAsset, pools: &PoolIndex) -> Result<Option<EnemyDef>> {
    let mut components = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components);
    let Some(enemy) = components
        .clone()
        .find(|component| component_type(component) == "Enemies.Enemy")
    else {
        return Ok(None);
    };
    let action = components
        .clone()
        .find(|component| {
            component_type(component) == "STStateMachine.States.STSM_Action_EnemyAttack"
        })
        .with_context(|| format!("{} enemy has no attack action", asset.path))?;
    let number = |component: &UnityComponent, path: &str| {
        component_field_value(component, path)
            .and_then(Value::as_f64)
            .with_context(|| format!("{} enemy field {path} is invalid", asset.path))
    };
    let positive_scaled_milli =
        |component: &UnityComponent, path: &str, scale: f64| -> Result<u32> {
            let value = number(component, path)?;
            if !value.is_finite() || value <= 0.0 {
                bail!("{} enemy field {path} must be positive", asset.path);
            }
            (value * scale)
                .round()
                .to_string()
                .parse()
                .with_context(|| format!("{} enemy field {path} is out of range", asset.path))
        };
    let enemy_type = component_field_value(enemy, "_enemyType")
        .and_then(enum_name)
        .with_context(|| format!("{} enemy has no enemy type", asset.path))?;
    let pool = pools
        .pool_by_prefab_guid
        .get(&asset.guid)
        .cloned()
        .with_context(|| format!("{} enemy is absent from object-pool settings", asset.path))?;
    let action_amount = component_field_value(action, "_actionAmount")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .filter(|value| *value > 0)
        .with_context(|| format!("{} enemy attack amount is invalid", asset.path))?;
    let sensor = components
        .clone()
        .find(|component| component_type(component) == "Sensors.TargetSensor")
        .with_context(|| format!("{} enemy has no target sensor", asset.path))?;
    let target_mask = component_field_value(sensor, "_targetMask")
        .and_then(Value::as_object)
        .with_context(|| format!("{} enemy target mask is invalid", asset.path))?;
    let (targets_all, target_kinds) = mask_ids(target_mask, "target")?;
    let attack_attacker = component_field_value(sensor, "_attackAttacker")
        .and_then(Value::as_bool)
        .with_context(|| format!("{} enemy attack-attacker setting is invalid", asset.path))?;
    let reward = components
        .find(|component| component_type(component) == "GameResources.ActiveResourceIncrementer")
        .with_context(|| format!("{} enemy has no active resource reward", asset.path))?;
    let reward_resource = component_field_value(reward, "_resource")
        .and_then(enum_name)
        .with_context(|| format!("{} enemy reward resource is invalid", asset.path))?;
    let reward_amount = component_field_value(reward, "_amount")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .filter(|value| *value > 0)
        .with_context(|| format!("{} enemy reward amount is invalid", asset.path))?;
    Ok(Some(EnemyDef {
        enemy_type: stable_id("enemy", &slug(enemy_type))?,
        pool,
        additional_health_milli_per_player: positive_scaled_milli(
            enemy,
            "_additionalHealthPerPlayer",
            1_000.0,
        )?,
        action_amount,
        action_milliseconds: positive_scaled_milli(action, "_actionRate", 1_000.0)?,
        action_range_milli_cells: positive_scaled_milli(action, "_actionRange", 500.0)?,
        target_search_range_milli_cells: positive_scaled_milli(
            sensor,
            "_targetSearchRange",
            500.0,
        )?,
        attack_attacker,
        kill_reward: ResourceReward {
            resource: stable_id("resource", &slug(reward_resource))?,
            amount: reward_amount,
        },
        targets_all,
        target_kinds,
    }))
}

fn enemy_model_definition(asset: &UnityAsset) -> Result<Option<EnemyModelSetDef>> {
    let Some(component) = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .find(|component| component_type(component) == "Enemies.EnemyModelHandler")
    else {
        return Ok(None);
    };
    let unsigned = |path: &str| -> Result<u8> {
        component_field_value(component, path)
            .and_then(Value::as_u64)
            .and_then(|value| u8::try_from(value).ok())
            .with_context(|| format!("{} enemy-model field {path} is invalid", asset.path))
    };
    let weapon_count = component_field_value(component, "_linkedWeaponModels.Array.size")
        .and_then(Value::as_u64)
        .with_context(|| {
            format!(
                "{} enemy-model component is missing _linkedWeaponModels.Array.size",
                asset.path
            )
        })?;
    let mut weapons = Vec::with_capacity(usize::try_from(weapon_count).unwrap_or(usize::MAX));
    for index in 0..weapon_count {
        let prefix = format!("_linkedWeaponModels.Array.data[{index}]");
        let animation = component_field_value(component, &format!("{prefix}._weaponAnimationName"))
            .and_then(enum_name)
            .with_context(|| format!("{} {prefix} has no weapon animation", asset.path))?;
        let run_animation =
            match component_field_value(component, &format!("{prefix}._runAnimation"))
                .and_then(enum_name)
            {
                Some("Generic") => EnemyRunAnimation::Generic,
                Some("Two Handed") => EnemyRunAnimation::TwoHanded,
                other => bail!(
                    "{} {prefix} has unsupported run animation {other:?}",
                    asset.path
                ),
            };
        weapons.push(EnemyWeaponModelDef {
            main_model: component_reference_name(
                asset,
                component,
                &format!("{prefix}._mainWeaponModel"),
            )?
            .to_owned(),
            off_hand_models: component_reference_names(
                asset,
                component,
                &format!("{prefix}._offHandModels"),
            )?,
            action_animation: animation_parameter_name(animation),
            action_animation_variants: unsigned(&format!("{prefix}._animationVariants"))?,
            run_animation,
        });
    }
    // EnemyModelHandler.RandomizeModel runs unconditionally from Awake in the
    // Unity source, so the serialized `_randomize` flag is intentionally not
    // carried into the portable contract.
    Ok(Some(EnemyModelSetDef {
        base_models: component_reference_names(asset, component, "_baseModels")?,
        permanent_models: component_reference_names(asset, component, "_permanentModels")?,
        optional_models: component_reference_names(asset, component, "_optionalModels")?,
        weapons,
        base_animation_variants: unsigned("_baseAnimationVariants")?,
    }))
}

fn enemy_model_node_count(models: &EnemyModelSetDef) -> usize {
    models.base_models.len()
        + models.permanent_models.len()
        + models.optional_models.len()
        + models
            .weapons
            .iter()
            .map(|weapon| 1 + weapon.off_hand_models.len())
            .sum::<usize>()
}

fn enemy_spawner_definition(
    asset: &UnityAsset,
    pools: &PoolIndex,
) -> Result<Option<EnemySpawnerDef>> {
    let components = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components);
    let Some(spawner) = components
        .clone()
        .find(|component| component_type(component) == "Enemies.EnemySpawner")
    else {
        return Ok(None);
    };
    let unsigned = |path: &str| -> Result<u16> {
        component_field_value(spawner, path)
            .and_then(Value::as_u64)
            .and_then(|value| u16::try_from(value).ok())
            .with_context(|| format!("{} enemy-spawner field {path} is invalid", asset.path))
    };
    let seconds = component_field_value(spawner, "_timeBetweenSpawns")
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite() && *value > 0.0)
        .with_context(|| format!("{} enemy spawn interval is invalid", asset.path))?;
    let spawn_milliseconds = (seconds * 1_000.0)
        .round()
        .to_string()
        .parse()
        .with_context(|| format!("{} enemy spawn interval is out of range", asset.path))?;
    let weighted_size = unsigned("_enemies._list.Array.size")?;
    let mut weighted_enemies = Vec::with_capacity(usize::from(weighted_size));
    for index in 0..weighted_size {
        let prefix = format!("_enemies._list.Array.data[{index}]");
        let pool_name = component_field_value(spawner, &format!("{prefix}.Object"))
            .and_then(|value| {
                value
                    .as_str()
                    .or_else(|| value.as_object()?.get("Name")?.as_str())
            })
            .with_context(|| format!("{} {prefix} has no pool name", asset.path))?;
        let enemy_archetype = pools
            .archetype_by_pool_name
            .get(&slug(pool_name))
            .cloned()
            .with_context(|| format!("{} references unknown enemy pool {pool_name}", asset.path))?;
        let chance = component_field_value(spawner, &format!("{prefix}.Chance"))
            .and_then(Value::as_f64)
            .filter(|value| value.is_finite() && *value > 0.0)
            .with_context(|| format!("{} {prefix} has an invalid weight", asset.path))?;
        let weight_milli = (chance * 1_000.0)
            .round()
            .to_string()
            .parse()
            .with_context(|| format!("{} {prefix} weight is out of range", asset.path))?;
        weighted_enemies.push(WeightedEnemySpawn {
            enemy_archetype,
            weight_milli,
        });
    }
    let location_size = unsigned("_spawnLocations.Array.size")?;
    let mut spawn_offsets_milli_cells = Vec::with_capacity(usize::from(location_size));
    for index in 0..location_size {
        let path = format!("_spawnLocations.Array.data[{index}]");
        let name = component_field_value(spawner, &path)
            .and_then(reference)
            .and_then(|reference| reference.get("Name"))
            .and_then(Value::as_str)
            .with_context(|| format!("{} {path} has no transform name", asset.path))?;
        let transform = components
            .clone()
            .find(|component| {
                component_type(component) == "UnityEngine.Transform"
                    && component.hierarchy_path == name
            })
            .with_context(|| format!("{} cannot resolve spawn transform {name}", asset.path))?;
        let position = component_field_value(transform, "localPosition")
            .and_then(vector3)
            .with_context(|| format!("{} spawn transform {name} has no position", asset.path))?;
        let to_milli_cells = |value: f64| -> Result<i32> {
            (value * 500.0)
                .round()
                .to_string()
                .parse()
                .with_context(|| format!("{} spawn transform {name} is out of range", asset.path))
        };
        spawn_offsets_milli_cells
            .push([to_milli_cells(position[0])?, to_milli_cells(position[2])?]);
    }
    Ok(Some(EnemySpawnerDef {
        min_total_enemies: unsigned("_minTotalEnemies")?,
        max_total_enemies: unsigned("_maxTotalEnemies")?,
        spawn_milliseconds,
        weighted_enemies,
        spawn_offsets_milli_cells,
    }))
}

fn projectile_shooter_definition(asset: &UnityAsset) -> Result<Option<ProjectileShooterDef>> {
    let components: Vec<_> = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .filter(|component| component_type(component) == "Buildings.ProjectileShooter")
        .collect();
    let Some(component) = components.first().copied() else {
        return Ok(None);
    };
    let numeric = |path: &str| {
        component_field_value(component, path)
            .and_then(Value::as_f64)
            .filter(|value| value.is_finite() && *value > 0.0)
            .with_context(|| format!("{} projectile shooter has invalid {path}", asset.path))
    };
    let integer = |path: &str| {
        component_field_value(component, path)
            .and_then(Value::as_u64)
            .and_then(|value| u32::try_from(value).ok())
            .filter(|value| *value > 0)
            .with_context(|| format!("{} projectile shooter has invalid {path}", asset.path))
    };
    let scaled = |path: &str, scale: f64| -> Result<u32> {
        (numeric(path)? * scale)
            .round()
            .to_string()
            .parse()
            .with_context(|| format!("{} projectile shooter {path} is out of range", asset.path))
    };
    let definition = ProjectileShooterDef {
        projectile_pool: component_field_value(component, "ProjectilePoolName")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .with_context(|| format!("{} projectile pool name is invalid", asset.path))?
            .to_owned(),
        // Authored Unity distances use two world units per Bevy grid cell.
        movement_milli_cells_per_second: scaled("_moveSpeed", 500.0)?,
        damage: integer("_damage")?,
        range_milli_cells: scaled("_range", 500.0)?,
        fire_milliseconds: scaled("_fireRate", 1_000.0)?,
    };
    for candidate in components.iter().skip(1) {
        for path in [
            "ProjectilePoolName",
            "_moveSpeed",
            "_damage",
            "_range",
            "_fireRate",
        ] {
            if component_field_value(candidate, path) != component_field_value(component, path) {
                bail!(
                    "{} contains inconsistent projectile shooter variants",
                    asset.path
                );
            }
        }
    }
    Ok(Some(definition))
}

fn role_equipment(asset: &UnityAsset) -> Result<BTreeMap<String, RoleEquipmentDef>> {
    let component = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .find(|component| component_type(component) == "Character.CharacterModelHandler")
        .with_context(|| format!("{} has no CharacterModelHandler", asset.path))?;
    let size = component_field_value(component, "_equipmentSets.Array.size")
        .and_then(Value::as_u64)
        .with_context(|| format!("{} has no equipment set array", asset.path))?;
    let mut result = BTreeMap::new();
    for index in 0..size {
        let prefix = format!("_equipmentSets.Array.data[{index}]");
        let field = |suffix: &str| {
            component_field_value(component, &format!("{prefix}.{suffix}"))
                .with_context(|| format!("{} {prefix}.{suffix} is missing", asset.path))
        };
        let name = field("RoleName")?
            .as_str()
            .with_context(|| format!("{} {prefix}.RoleName is invalid", asset.path))?;
        let reference_name = |suffix: &str| -> Result<Option<String>> {
            let value = field(suffix)?;
            if value.is_null() {
                return Ok(None);
            }
            Ok(value
                .as_object()
                .and_then(|object| object.get("Name"))
                .and_then(Value::as_str)
                .map(str::to_owned))
        };
        let carry_animation = field("HasCarryAnimation")?
            .as_bool()
            .unwrap_or_default()
            .then(|| {
                field("CarryAnimation")
                    .ok()?
                    .as_object()?
                    .get("Name")?
                    .as_str()
                    .map(str::to_owned)
            })
            .flatten();
        let equipment = RoleEquipmentDef {
            body_nodes: [
                reference_name("BodieSlim")?.context("slim body reference is empty")?,
                reference_name("BodieBulk")?.context("bulk body reference is empty")?,
                reference_name("BodieFeminine")?.context("feminine body reference is empty")?,
            ],
            left_hand_node: reference_name("LeftHand")?,
            right_hand_node: reference_name("RightHand")?,
            helmet_node: reference_name("Helmet")?,
            carry_animation,
            left_hand_permanent: field("LeftHandPermanent")?.as_bool().unwrap_or_default(),
        };
        if result.insert(slug(name), equipment).is_some() {
            bail!("{} contains duplicate equipment for {name}", asset.path);
        }
    }
    Ok(result)
}

fn building_placements(asset: &UnityAsset) -> Result<BTreeMap<String, BuildingPlacement>> {
    let game_object = asset
        .game_object
        .as_ref()
        .with_context(|| format!("{} has no exported GameObject", asset.path))?;
    let component = game_object
        .components
        .iter()
        .find(|component| component_type(component) == "Buildings.BuildingPlacer")
        .with_context(|| format!("{} has no Buildings.BuildingPlacer component", asset.path))?;
    let size = component_field_value(component, "_buildData.Array.size")
        .and_then(Value::as_u64)
        .with_context(|| format!("{} has no building placement array", asset.path))?;
    let mut placements = BTreeMap::new();
    for index in 0..size {
        let prefix = format!("_buildData.Array.data[{index}]");
        let building_type = enum_name(
            component_field_value(component, &format!("{prefix}.BuildingType")).with_context(
                || {
                    format!(
                        "{asset_path}:{prefix} has no BuildingType",
                        asset_path = asset.path
                    )
                },
            )?,
        )
        .with_context(|| format!("{}:{prefix} has an invalid BuildingType", asset.path))?;
        let prefab = component_field_value(component, &format!("{prefix}.Prefab"))
            .and_then(Value::as_object)
            .with_context(|| format!("{}:{prefix} has no prefab reference", asset.path))?;
        let prefab_guid = prefab
            .get("Guid")
            .and_then(Value::as_str)
            .filter(|guid| !guid.is_empty())
            .with_context(|| format!("{}:{prefix} has no prefab GUID", asset.path))?;
        let width = component_field_value(component, &format!("{prefix}.BuildingSize.x"))
            .and_then(Value::as_f64)
            .with_context(|| format!("{}:{prefix} has no BuildingSize.x", asset.path))?;
        let depth = component_field_value(component, &format!("{prefix}.BuildingSize.y"))
            .and_then(Value::as_f64)
            .with_context(|| format!("{}:{prefix} has no BuildingSize.y", asset.path))?;
        let key = slug(building_type);
        let previous = placements.insert(
            key.clone(),
            BuildingPlacement {
                prefab_guid: prefab_guid.to_owned(),
                footprint: footprint_from_unity_size(width, depth),
            },
        );
        if previous.is_some() {
            bail!("{} contains duplicate placement for {key}", asset.path);
        }
    }
    Ok(placements)
}

fn convert_archetypes(
    export: &UnityExport,
    placements: &BTreeMap<String, BuildingPlacement>,
    pools: &PoolIndex,
) -> Result<(ArchetypesById, BuildingArchetypesBySlug)> {
    let assets_by_path: BTreeMap<_, _> = export
        .assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset))
        .collect();
    let mut active_buildings = BTreeMap::<String, (String, Option<[u16; 2]>)>::new();
    for (building, placement) in placements {
        active_buildings.insert(
            placement.prefab_guid.clone(),
            (building.clone(), Some(placement.footprint)),
        );
    }

    // Torch is intentionally absent from BuildingPlacer, but it is active content
    // and has a normal production prefab. Discover it through BuildingBase.
    for asset in &export.assets {
        if asset.kind != "prefab"
            || !asset.path.starts_with("Assets/Prefabs/Buildings/")
            || asset.path.contains("/Archive/")
            || asset.path.contains("/Enemy/")
        {
            continue;
        }
        let Some(building_type) = prefab_building_type(asset) else {
            continue;
        };
        let building = slug(building_type);
        if !placements.contains_key(&building) {
            active_buildings
                .entry(asset.guid.clone())
                .or_insert((building, None));
        }
    }

    let mut archetypes = BTreeMap::new();
    let mut building_archetypes = BTreeMap::new();
    for asset in &export.assets {
        if asset.kind != "prefab" || asset.status != "exported" || asset.game_object.is_none() {
            continue;
        }
        let active_building = active_buildings.get(&asset.guid);
        let id = if let Some((building, _)) = active_building {
            stable_id("archetype:building", building)?
        } else {
            stable_id("archetype:prefab", &asset.guid)?
        };
        let bounds = archetype_bounds(asset);
        let footprint = active_building
            .and_then(|(_, footprint)| *footprint)
            .unwrap_or_else(|| {
                footprint_from_unity_size(f64::from(bounds.size[0]), f64::from(bounds.size[2]))
            });
        let scenes = archetype_scenes(asset, &assets_by_path);
        let mut component_types: Vec<_> = asset
            .game_object
            .as_ref()
            .into_iter()
            .flat_map(|game_object| &game_object.components)
            .map(component_type)
            .filter(|component| !component.is_empty())
            .map(str::to_owned)
            .collect();
        component_types.sort();
        component_types.dedup();
        let kind =
            active_building.map_or_else(|| archetype_kind(asset), |_| ArchetypeKind::Building);
        let archetype = ArchetypeDef {
            display_name: asset.name.clone(),
            kind,
            source_guid: asset.guid.clone(),
            source_path: asset.path.clone(),
            bounds,
            footprint,
            scenes,
            component_types,
            disable_after_milliseconds: disable_after_milliseconds(asset)?,
            health_bar_hide_milliseconds: health_bar_hide_milliseconds(asset)?,
            rotating_nodes: rotating_node_definitions(asset)?,
            target_size_milli_cells: targetable_size_milli_cells(asset)?,
            health: health_definition(asset)?,
            enemy: enemy_definition(asset, pools)?,
            enemy_models: enemy_model_definition(asset)?,
            enemy_spawner: enemy_spawner_definition(asset, pools)?,
            pet: pet_definition(asset)?,
        };
        if let Some((building, _)) = active_building {
            let previous = building_archetypes.insert(building.clone(), (id.clone(), footprint));
            if previous.is_some() {
                bail!("multiple active prefabs resolve building {building}");
            }
        }
        if archetypes.insert(id.clone(), archetype).is_some() {
            bail!("duplicate prefab archetype {id}");
        }
    }
    for building in placements.keys() {
        if !building_archetypes.contains_key(building) {
            bail!("building placement {building} did not resolve an exported prefab");
        }
    }
    Ok((archetypes, building_archetypes))
}

fn prefab_building_type(asset: &UnityAsset) -> Option<&str> {
    asset
        .game_object
        .as_ref()?
        .components
        .iter()
        .find(|component| component_type(component) == "Buildings.BuildingBase")
        .and_then(|component| component_field_value(component, "_buildingType"))
        .and_then(enum_name)
}

fn disable_after_milliseconds(asset: &UnityAsset) -> Result<Option<u32>> {
    let components = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .filter(|component| component_type(component) == "Utils.SimpleDisableAfterTime")
        .collect::<Vec<_>>();
    let Some(component) = components.first() else {
        return Ok(None);
    };
    if components.len() != 1 {
        bail!("{} has multiple disable-after-time components", asset.path);
    }
    let seconds = component_field_value(component, "_lifeTime")
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite() && *value > 0.0)
        .with_context(|| format!("{} has invalid disable-after lifetime", asset.path))?;
    let milliseconds = (seconds * 1_000.0)
        .round()
        .to_string()
        .parse::<u32>()
        .with_context(|| format!("{} disable-after lifetime is out of range", asset.path))?;
    Ok(Some(milliseconds))
}

fn health_bar_hide_milliseconds(asset: &UnityAsset) -> Result<Option<u32>> {
    let components = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .filter(|component| component_type(component) == "Units.UnitHealthBar")
        .collect::<Vec<_>>();
    let Some(component) = components.first() else {
        return Ok(None);
    };
    if components.len() != 1 {
        bail!("{} has multiple unit-health-bar components", asset.path);
    }
    let seconds = component_field_value(component, "_hideDelay")
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite() && *value > 0.0)
        .with_context(|| format!("{} has invalid unit-health-bar hide delay", asset.path))?;
    let milliseconds = (seconds * 1_000.0)
        .round()
        .to_string()
        .parse::<u32>()
        .with_context(|| format!("{} unit-health-bar hide delay is out of range", asset.path))?;
    Ok(Some(milliseconds))
}

fn storage_contributions(asset: &UnityAsset) -> Result<Vec<StorageContribution>> {
    let mut storage = Vec::new();
    for component in asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .filter(|component| component_type(component) == "Buildings.ResourceStorageModifier")
    {
        let required = |path: &str| {
            component_field_value(component, path)
                .with_context(|| format!("{} storage component is missing {path}", asset.path))
        };
        let resource = required("_resource")?
            .as_object()
            .and_then(|value| value.get("Name"))
            .and_then(Value::as_str)
            .with_context(|| format!("{} storage component has invalid _resource", asset.path))?;
        let u32_field = |path: &str| {
            required(path)?
                .as_u64()
                .and_then(|value| u32::try_from(value).ok())
                .with_context(|| format!("{} storage field {path} is out of range", asset.path))
        };
        let multiplier = required("_incrementMultiPerLevel")?
            .as_f64()
            .with_context(|| format!("{} storage multiplier is invalid", asset.path))?;
        if !multiplier.is_finite() || multiplier < 0.0 {
            bail!("{} storage multiplier must be non-negative", asset.path);
        }
        let level_multiplier_per_thousand = (multiplier * 1000.0)
            .round()
            .to_string()
            .parse()
            .with_context(|| format!("{} storage multiplier is out of range", asset.path))?;
        storage.push(StorageContribution {
            resource: stable_id("resource", &slug(resource))?,
            base_amount: u32_field("_baseAmount")?,
            increment_amount: u32_field("_incrementAmount")?,
            level_multiplier_per_thousand,
        });
    }
    storage.sort_by(|left, right| left.resource.cmp(&right.resource));
    Ok(storage)
}

fn role_slot_contributions(asset: &UnityAsset) -> Result<Vec<RoleSlotContribution>> {
    let mut slots = Vec::new();
    for component in asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .filter(|component| component_type(component) == "Buildings.RoleSlotModifier")
    {
        let field = |path: &str| {
            component_field_value(component, path)
                .with_context(|| format!("{} role-slot component is missing {path}", asset.path))
        };
        let role = field("_role")?
            .as_object()
            .and_then(|value| value.get("Name"))
            .and_then(Value::as_str)
            .with_context(|| format!("{} role-slot component has invalid _role", asset.path))?;
        let amount = |path: &str| {
            field(path)?
                .as_i64()
                .and_then(|value| u16::try_from(value).ok())
                .with_context(|| format!("{} role-slot field {path} is out of range", asset.path))
        };
        slots.push(RoleSlotContribution {
            role: stable_id("role", &slug(role))?,
            base_amount: amount("_baseAmount")?,
            increment_amount: amount("_incrementAmount")?,
        });
    }
    slots.sort_by(|left, right| left.role.cmp(&right.role));
    Ok(slots)
}

fn passive_resource_contributions(asset: &UnityAsset) -> Result<Vec<PassiveResourceContribution>> {
    let components = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components);
    // Unity serializes persistent UnityEvent calls on sibling components. The shipping
    // Marketplace prefab invokes OnLevelUp twice, and both calls affect this component.
    let repetitions = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .flat_map(|component| &component.fields)
        .filter(|field| {
            field.path.ends_with(".m_MethodName") && field.value.as_str() == Some("OnLevelUp")
        })
        .count();
    let mut income = Vec::new();
    for component in components
        .filter(|component| component_type(component) == "GameResources.PassiveResourceIncrementer")
    {
        let field = |path: &str| {
            component_field_value(component, path).with_context(|| {
                format!(
                    "{} passive resource component is missing {path}",
                    asset.path
                )
            })
        };
        let resource = enum_name(field("_resource")?)
            .with_context(|| format!("{} passive resource type is invalid", asset.path))?;
        let milli = |path: &str| -> Result<u32> {
            let value = field(path)?
                .as_f64()
                .with_context(|| format!("{} passive resource {path} is invalid", asset.path))?;
            if !value.is_finite() || value < 0.0 {
                bail!("{} passive resource {path} is invalid", asset.path);
            }
            let scaled = (value * 1_000.0).round();
            if scaled > f64::from(u32::MAX) {
                bail!("{} passive resource {path} is out of range", asset.path);
            }
            format!("{scaled:.0}")
                .parse::<u32>()
                .with_context(|| format!("{} passive resource {path} is out of range", asset.path))
        };
        income.push(PassiveResourceContribution {
            resource: stable_id("resource", &slug(resource))?,
            base_milli_per_second: milli("_amountPerSecond")?,
            increment_milli_per_level: milli("_amountPerLevel")?,
            level_event_repetitions: u16::try_from(repetitions)
                .context("passive level event count is out of range")?,
        });
    }
    income.sort_by(|left, right| left.resource.cmp(&right.resource));
    Ok(income)
}

fn component_reference_name<'a>(
    asset: &UnityAsset,
    component: &'a UnityComponent,
    path: &str,
) -> Result<&'a str> {
    component_field_value(component, path)
        .and_then(Value::as_object)
        .and_then(|value| value.get("Name"))
        .and_then(Value::as_str)
        .filter(|name| !name.trim().is_empty())
        .with_context(|| {
            format!(
                "{} {} component is missing referenced object {path}",
                asset.path,
                component_type(component)
            )
        })
}

fn component_reference_names(
    asset: &UnityAsset,
    component: &UnityComponent,
    path: &str,
) -> Result<Vec<String>> {
    let size = component_field_value(component, &format!("{path}.Array.size"))
        .and_then(Value::as_u64)
        .with_context(|| {
            format!(
                "{} {} component is missing {path}.Array.size",
                asset.path,
                component_type(component)
            )
        })?;
    (0..size)
        .map(|index| {
            component_reference_name(asset, component, &format!("{path}.Array.data[{index}]"))
                .map(str::to_owned)
        })
        .collect()
}

fn building_node_age(asset: &UnityAsset, name: &str) -> Result<u8> {
    if name.contains("Age01") {
        Ok(1)
    } else if name.contains("Age02") {
        Ok(2)
    } else {
        bail!(
            "{} model node {name:?} has no supported age marker",
            asset.path
        )
    }
}

fn building_model_definitions(asset: &UnityAsset) -> Result<Vec<BuildingModelDef>> {
    let mut models = Vec::new();
    for component in asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .filter(|component| component_type(component) == "Buildings.BuildingModelHandler")
    {
        let full_model = component_reference_name(asset, component, "FullModel")?.to_owned();
        models.push(BuildingModelDef {
            age: building_node_age(asset, &full_model)?,
            full_model,
            construction_stages: [
                component_reference_name(asset, component, "Stage1")?.to_owned(),
                component_reference_name(asset, component, "Stage2")?.to_owned(),
                component_reference_name(asset, component, "Stage3")?.to_owned(),
            ],
            upgrades: component_reference_names(asset, component, "Upgrades")?,
            other_models: component_reference_names(asset, component, "OtherModels")?,
        });
    }
    // Unity's handler index is meaningful for tiled walls, so retain hierarchy
    // order within each age while grouping the two age sets.
    models.sort_by_key(|model| model.age);
    Ok(models)
}

fn storage_model_definitions(asset: &UnityAsset) -> Result<Vec<StorageModelDef>> {
    let components = asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .collect::<Vec<_>>();
    let mut storage_resources = components
        .iter()
        .copied()
        .filter(|component| component_type(component) == "Buildings.ResourceStorageModifier")
        .map(|component| {
            let resource = component_field_value(component, "_resource")
                .and_then(enum_name)
                .with_context(|| {
                    format!("{} storage modifier has invalid _resource", asset.path)
                })?;
            stable_id("resource", &slug(resource))
        });
    let resource = storage_resources.next().transpose()?;
    if storage_resources.next().is_some() {
        bail!(
            "{} has multiple storage resources and ambiguous model handlers",
            asset.path
        );
    }

    let mut models = Vec::new();
    for component in components
        .into_iter()
        .filter(|component| component_type(component) == "Buildings.BuildingResourceModelHandler")
    {
        let resource = resource.clone().with_context(|| {
            format!(
                "{} has a resource model handler without a storage modifier",
                asset.path
            )
        })?;
        let empty_model = component_reference_name(asset, component, "EmptyModel")?.to_owned();
        models.push(StorageModelDef {
            age: building_node_age(asset, &empty_model)?,
            resource,
            empty_model,
            half_full_model: component_reference_name(asset, component, "HalfFullModel")?
                .to_owned(),
            full_model: component_reference_name(asset, component, "FullModel")?.to_owned(),
        });
    }
    models.sort_by_key(|model| model.age);
    Ok(models)
}

fn rotating_node_definitions(asset: &UnityAsset) -> Result<Vec<RotatingNodeDef>> {
    let mut nodes = Vec::new();
    for component in asset
        .game_object
        .as_ref()
        .into_iter()
        .flat_map(|game_object| &game_object.components)
        .filter(|component| component_type(component) == "Utils.SimpleRotateOnAxis")
    {
        let node = component
            .hierarchy_path
            .rsplit('/')
            .next()
            .filter(|name| !name.trim().is_empty())
            .with_context(|| format!("{} rotating component has no hierarchy node", asset.path))?
            .to_owned();
        let axis = component_field_value(component, "_axis")
            .and_then(vector3)
            .with_context(|| format!("{} rotating node {node} has no axis", asset.path))?;
        let axis = converted_rotating_axis(&node, axis);
        let degrees_per_second = component_field_value(component, "_speed")
            .and_then(Value::as_f64)
            .filter(|value| value.is_finite() && value.abs() > f64::EPSILON)
            .with_context(|| format!("{} rotating node {node} has invalid speed", asset.path))?;
        #[allow(clippy::cast_possible_truncation)]
        nodes.push(RotatingNodeDef {
            hierarchy_path: component.hierarchy_path.clone(),
            age: if node.contains("Age01") {
                Some(1)
            } else if node.contains("Age02") {
                Some(2)
            } else {
                None
            },
            node,
            axis,
            degrees_per_second: degrees_per_second as f32,
        });
    }
    nodes.sort_by(|left, right| (left.age, &left.node).cmp(&(right.age, &right.node)));
    Ok(nodes)
}

fn converted_rotating_axis(node: &str, unity_axis: [f64; 3]) -> [f32; 3] {
    #[allow(clippy::cast_possible_truncation)]
    let mut axis = [
        unity_axis[0] as f32,
        unity_axis[1] as f32,
        unity_axis[2] as f32,
    ];
    if node.ends_with("_Windmill_Blades") {
        // Blender bakes the two windmill FBXs' up-axis conversion into their
        // mesh basis: the Age 1 blade plane is XZ and the Age 2 plane is XY in
        // the emitted GLBs. Swap Unity local Y/Z so rotation follows each
        // converted plane normal rather than making the blades tumble.
        axis.swap(1, 2);
    }
    axis
}

fn component_type(component: &UnityComponent) -> &str {
    component
        .unity_type
        .as_deref()
        .unwrap_or_default()
        .split(',')
        .next()
        .unwrap_or_default()
}

fn component_field_value<'a>(component: &'a UnityComponent, path: &str) -> Option<&'a Value> {
    component
        .fields
        .iter()
        .find(|field| field.path == path)
        .map(|field| &field.value)
}

fn loading_screen_definition(asset: &UnityAsset) -> Result<LoadingScreenDef> {
    let component = asset
        .game_object
        .as_ref()
        .into_iter()
        .chain(
            asset
                .scene
                .as_ref()
                .into_iter()
                .flat_map(|scene| &scene.roots),
        )
        .flat_map(|game_object| &game_object.components)
        .find(|component| component_type(component) == "UserInterface.MainMenu.LoadingManager")
        .with_context(|| format!("{} has no loading manager", asset.path))?;
    let scaled_milliseconds = |path: &str| -> Result<u32> {
        let seconds = component_field_value(component, path)
            .and_then(Value::as_f64)
            .filter(|value| value.is_finite() && *value > 0.0)
            .with_context(|| format!("{} has invalid {path}", asset.path))?;
        (seconds * 1_000.0)
            .round()
            .to_string()
            .parse::<u32>()
            .with_context(|| format!("{} {path} is out of range", asset.path))
    };
    let tooltip_count = component_field_value(component, "_toolTips.Array.size")
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .filter(|value| *value > 0)
        .with_context(|| format!("{} has no loading tooltips", asset.path))?;
    let tooltips = (0..tooltip_count)
        .map(|index| {
            component_field_value(component, &format!("_toolTips.Array.data[{index}]"))
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .map(str::to_owned)
                .with_context(|| format!("{} tooltip {index} is invalid", asset.path))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(LoadingScreenDef {
        progress_milli_per_second: scaled_milliseconds("_loadingSpeed")?,
        completion_hold_milliseconds: scaled_milliseconds("_waitTime")?,
        tooltips,
    })
}

fn enum_name(value: &Value) -> Option<&str> {
    value
        .as_object()
        .and_then(|value| value.get("Name"))
        .and_then(Value::as_str)
}

fn archetype_bounds(asset: &UnityAsset) -> ArchetypeBounds {
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    let mut found = false;
    if let Some(game_object) = &asset.game_object {
        for component in &game_object.components {
            let Some(bounds) =
                component_field_value(component, "bounds").and_then(Value::as_object)
            else {
                continue;
            };
            let Some(center) = bounds.get("Center").and_then(vector3) else {
                continue;
            };
            let Some(size) = bounds.get("Size").and_then(vector3) else {
                continue;
            };
            if size.iter().all(|value| *value <= f64::EPSILON) {
                continue;
            }
            for axis in 0..3 {
                minimum[axis] = minimum[axis].min(center[axis] - size[axis] * 0.5);
                maximum[axis] = maximum[axis].max(center[axis] + size[axis] * 0.5);
            }
            found = true;
        }
    }
    if !found {
        return ArchetypeBounds {
            center: [0.0, 0.5, 0.0],
            size: [1.0; 3],
        };
    }
    #[allow(clippy::cast_possible_truncation)]
    ArchetypeBounds {
        center: [
            ((minimum[0] + maximum[0]) * 0.5) as f32,
            ((minimum[1] + maximum[1]) * 0.5) as f32,
            ((minimum[2] + maximum[2]) * 0.5) as f32,
        ],
        size: [
            (maximum[0] - minimum[0]) as f32,
            (maximum[1] - minimum[1]) as f32,
            (maximum[2] - minimum[2]) as f32,
        ],
    }
}

fn vector3(value: &Value) -> Option<[f64; 3]> {
    let value = value.as_object()?;
    Some([
        value.get("x")?.as_f64()?,
        value.get("y")?.as_f64()?,
        value.get("z")?.as_f64()?,
    ])
}

fn footprint_from_unity_size(width: f64, depth: f64) -> [u16; 2] {
    fn cells(value: f64) -> u16 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let cells = (value.max(0.0) / 2.0).ceil() as u16;
        cells.max(1)
    }
    [cells(width), cells(depth)]
}

fn archetype_scenes(
    asset: &UnityAsset,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
) -> Vec<ArchetypeScene> {
    let mut models = BTreeSet::new();
    collect_model_dependencies(asset, assets_by_path, &mut BTreeSet::new(), &mut models);
    let models: Vec<_> = models.into_iter().collect();
    let animator_model = asset.game_object.as_ref().and_then(|game_object| {
        game_object
            .components
            .iter()
            .find(|component| component_type(component) == "UnityEngine.Animator")
            .and_then(|component| component.hierarchy_path.rsplit('/').next())
            .filter(|name| !name.is_empty())
    });
    let default_index = animator_model
        .and_then(|animator_model| {
            models.iter().position(|path| {
                Path::new(path)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .is_some_and(|stem| stem.eq_ignore_ascii_case(animator_model))
            })
        })
        .or_else(|| {
            models.iter().position(|path| {
                let path = path.to_ascii_lowercase();
                path.contains("age01") && path.contains("straight")
            })
        })
        .or_else(|| {
            models
                .iter()
                .position(|path| path.to_ascii_lowercase().contains("age01"))
        })
        .unwrap_or(0);
    models
        .into_iter()
        .enumerate()
        .map(|(index, source_model)| ArchetypeScene {
            asset_path: glb_asset_path(&source_model),
            age: model_age(&source_model),
            source_model,
            is_default: index == default_index,
        })
        .collect()
}

fn collect_model_dependencies(
    asset: &UnityAsset,
    assets_by_path: &BTreeMap<&str, &UnityAsset>,
    visited: &mut BTreeSet<String>,
    models: &mut BTreeSet<String>,
) {
    if !visited.insert(asset.path.clone()) {
        return;
    }
    for dependency in &asset.dependencies {
        let Some(path) = dependency.path.as_deref() else {
            continue;
        };
        if path.to_ascii_lowercase().ends_with(".fbx") {
            models.insert(path.to_owned());
        } else if let Some(dependency_asset) = assets_by_path.get(path)
            && dependency_asset.kind == "prefab"
        {
            collect_model_dependencies(dependency_asset, assets_by_path, visited, models);
        }
    }
}

fn glb_asset_path(source_model: &str) -> String {
    let relative = source_model.strip_prefix("Assets/").unwrap_or(source_model);
    let stem = relative
        .rsplit_once('.')
        .map_or(relative, |(stem, _extension)| stem);
    format!("migrated/models/{stem}.glb")
}

fn enemy_camp_generation_layers(
    asset: &UnityAsset,
    pools: &PoolIndex,
) -> Result<Vec<EnemyCampGenerationDef>> {
    let list_path = "_campGenerationSettings";
    let count = required_u32(asset, &format!("{list_path}.Array.size"))?;
    (0..count)
        .map(|index| {
            let prefix = format!("{list_path}.Array.data[{index}]");
            let pool_name = required_string(asset, &format!("{prefix}.PoolName"))?;
            let camp_archetype = pools
                .archetype_by_pool_name
                .get(&slug(&pool_name))
                .cloned()
                .with_context(|| {
                    format!(
                        "{} references unknown enemy camp pool {pool_name}",
                        asset.path
                    )
                })?;
            let maximum_camps = required_u32(asset, &format!("{prefix}.MaxAmount"))?
                .try_into()
                .with_context(|| format!("{} camp count is out of range", asset.path))?;
            Ok(EnemyCampGenerationDef {
                id: StableId::new(format!(
                    "enemy_camp_generation:{}:{index}",
                    slug(&pool_name)
                ))?,
                source_path: asset.path.clone(),
                camp_archetype,
                minimum_absolute_offset_milli_cells: [
                    required_milli_cells(asset, &format!("{prefix}.MinBounds.x"))?,
                    required_milli_cells(asset, &format!("{prefix}.MinBounds.y"))?,
                ],
                maximum_absolute_offset_milli_cells: [
                    required_milli_cells(asset, &format!("{prefix}.MaxBounds.x"))?,
                    required_milli_cells(asset, &format!("{prefix}.MaxBounds.y"))?,
                ],
                maximum_camps,
                minimum_distance_from_centre_milli_cells: required_milli_cells(
                    asset,
                    &format!("{prefix}.MinDistanceFromCenter"),
                )?,
                minimum_distance_between_camps_milli_cells: required_milli_cells(
                    asset,
                    &format!("{prefix}.MinDistanceFromOther"),
                )?,
                camp_size_milli_cells: required_milli_cells(asset, &format!("{prefix}.CampSize"))?,
            })
        })
        .collect()
}

fn foliage_layers(
    asset: &UnityAsset,
    list_path: &str,
    habitat: FoliageHabitat,
) -> Result<Vec<FoliageLayerDef>> {
    let count = required_u32(asset, &format!("{list_path}.Array.size"))?;
    (0..count)
        .map(|index| {
            let prefix = format!("{list_path}.Array.data[{index}]");
            let mesh_count = required_u32(asset, &format!("{prefix}.MeshSettings.Array.size"))?;
            let mut variants = Vec::new();
            for mesh_index in 0..mesh_count {
                let mesh_prefix = format!("{prefix}.MeshSettings.Array.data[{mesh_index}]");
                let source_model = field_value(asset, &format!("{mesh_prefix}.Mesh"))
                    .and_then(reference)
                    .and_then(|reference| reference.get("Path"))
                    .and_then(Value::as_str)
                    .with_context(|| {
                        format!(
                            "{} foliage mesh {mesh_index} has no source path",
                            asset.path
                        )
                    })?
                    .to_owned();
                variants.push(FoliageVariantDef {
                    asset_path: glb_asset_path(&source_model),
                    source_model,
                    base_scale: [
                        required_f32(asset, &format!("{mesh_prefix}.BaseScale.x"))?,
                        required_f32(asset, &format!("{mesh_prefix}.BaseScale.y"))?,
                        required_f32(asset, &format!("{mesh_prefix}.BaseScale.z"))?,
                    ],
                });
            }
            // Unity 6 no longer exposes the obsolete water `Meshes` list through
            // SerializedObject. The neutral export still retains those FBX
            // dependencies, so recover the two authored sets by their stable names.
            if variants.is_empty() && habitat == FoliageHabitat::Underwater {
                let source_models = asset.dependencies.iter().filter_map(|dependency| {
                    dependency.path.as_deref().filter(|path| {
                        path.to_ascii_lowercase().ends_with(".fbx")
                            && (path.contains("Seaweed") == (index == 0))
                    })
                });
                variants.extend(source_models.map(|source_model| FoliageVariantDef {
                    source_model: source_model.to_owned(),
                    asset_path: glb_asset_path(source_model),
                    base_scale: [1.0; 3],
                }));
            }
            let material_source_path = field_value(asset, &format!("{prefix}.Material"))
                .and_then(reference)
                .and_then(|reference| reference.get("Path"))
                .and_then(Value::as_str)
                .with_context(|| format!("{} foliage layer {index} has no material", asset.path))?
                .to_owned();
            let habitat_slug = match habitat {
                FoliageHabitat::Land => "land",
                FoliageHabitat::Underwater => "underwater",
            };
            Ok(FoliageLayerDef {
                id: StableId::new(format!("foliage:{habitat_slug}:{index}"))?,
                source_path: asset.path.clone(),
                habitat,
                source_size: required_u32(asset, &format!("{prefix}.Size"))?
                    .try_into()
                    .with_context(|| format!("{} foliage size is out of range", asset.path))?,
                level_of_detail: required_u32(asset, &format!("{prefix}.LevelOfDetail"))?
                    .try_into()
                    .with_context(|| format!("{} foliage LOD is out of range", asset.path))?,
                noise_scale: required_f32(asset, &format!("{prefix}.NoiseScale"))?,
                octaves: required_u32(asset, &format!("{prefix}.Octaves"))?
                    .try_into()
                    .with_context(|| format!("{} foliage octaves are out of range", asset.path))?,
                persistence: required_f32(asset, &format!("{prefix}.Persistance"))?,
                lacunarity: required_f32(asset, &format!("{prefix}.Lacunarity"))?,
                seed: required_i32(asset, &format!("{prefix}.Seed"))?,
                offset: [
                    required_f32(asset, &format!("{prefix}.Offset.x"))?,
                    required_f32(asset, &format!("{prefix}.Offset.y"))?,
                ],
                spawn_threshold: required_f32(asset, &format!("{prefix}.SpawnThreshold"))?,
                spacing: required_u32(asset, &format!("{prefix}.Spacing"))?
                    .max(1)
                    .try_into()
                    .with_context(|| format!("{} foliage spacing is out of range", asset.path))?,
                material_source_path,
                variants,
            })
        })
        .collect()
}

fn model_age(path: &str) -> Option<u8> {
    let lower = path.to_ascii_lowercase();
    if lower.contains("age01") {
        Some(1)
    } else if lower.contains("age02") {
        Some(2)
    } else {
        None
    }
}

fn archetype_kind(asset: &UnityAsset) -> ArchetypeKind {
    let path = asset.path.to_ascii_lowercase();
    if path.contains("/userinterface/") || path.contains("/ui_") {
        ArchetypeKind::Ui
    } else if path.contains("/vfx/") {
        ArchetypeKind::Vfx
    } else if path.contains("/enemies/") || path.contains("/enemy/") {
        ArchetypeKind::Enemy
    } else if path.contains("/resources/") || path.contains("resource_") {
        ArchetypeKind::Resource
    } else if path.contains("player") || path.contains("character") {
        ArchetypeKind::Player
    } else if path.contains("environment") || path.contains("foliage") || path.contains("world") {
        ArchetypeKind::Environment
    } else {
        ArchetypeKind::Other
    }
}

fn required_asset<'a>(
    assets: &'a BTreeMap<&str, &UnityAsset>,
    path: &str,
) -> Result<&'a UnityAsset> {
    assets
        .get(path)
        .copied()
        .with_context(|| format!("Unity export is missing active content container {path}"))
}

fn required_guid_asset<'a>(
    assets: &'a BTreeMap<&str, &UnityAsset>,
    guid: &str,
    expected_type: &str,
) -> Result<&'a UnityAsset> {
    let asset = assets
        .get(guid)
        .copied()
        .with_context(|| format!("Unity export is missing referenced GUID {guid}"))?;
    if !asset
        .unity_type
        .as_deref()
        .is_some_and(|value| value.starts_with(expected_type))
    {
        bail!(
            "{} has type {:?}, expected {expected_type}",
            asset.path,
            asset.unity_type
        );
    }
    Ok(asset)
}

fn referenced_guids(
    asset: &UnityAsset,
    path_prefix: &str,
    required_type: Option<&str>,
) -> BTreeSet<String> {
    asset
        .serialized_fields
        .iter()
        .filter(|field| field.path.starts_with(path_prefix))
        .filter_map(|field| reference(&field.value))
        .filter(|reference| {
            required_type.is_none_or(|expected| {
                reference
                    .get("Type")
                    .and_then(Value::as_str)
                    .is_some_and(|value| value.starts_with(expected))
            })
        })
        .filter_map(|reference| reference.get("Guid").and_then(Value::as_str))
        .map(str::to_owned)
        .collect()
}

fn child_technology_guids(asset: &UnityAsset) -> BTreeSet<String> {
    asset
        .serialized_fields
        .iter()
        .filter(|field| {
            field
                .path
                .starts_with("<Children>k__BackingField.Array.data[")
                && field.path.ends_with("].<NextTech>k__BackingField")
        })
        .filter_map(|field| reference(&field.value))
        .filter_map(|reference| reference.get("Guid").and_then(Value::as_str))
        .map(str::to_owned)
        .collect()
}

fn reference(value: &Value) -> Option<&serde_json::Map<String, Value>> {
    value
        .as_object()
        .filter(|object| object.contains_key("Guid"))
}

fn field_value<'a>(asset: &'a UnityAsset, path: &str) -> Option<&'a Value> {
    asset
        .serialized_fields
        .iter()
        .find(|field| field.path == path)
        .map(|field| &field.value)
}

fn required_string(asset: &UnityAsset, path: &str) -> Result<String> {
    field_value(asset, path)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .with_context(|| format!("{} is missing string field {path}", asset.path))
}

fn required_enum(asset: &UnityAsset, path: &str) -> Result<String> {
    field_value(asset, path)
        .and_then(Value::as_object)
        .and_then(|value| value.get("Name"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .with_context(|| format!("{} is missing enum field {path}", asset.path))
}

fn optional_enum(asset: &UnityAsset, path: &str) -> Result<Option<String>> {
    let Some(value) = field_value(asset, path) else {
        return Ok(None);
    };
    let object = value
        .as_object()
        .with_context(|| format!("{} field {path} is not an enum object", asset.path))?;
    Ok(object
        .get("Name")
        .and_then(Value::as_str)
        .map(str::to_owned))
}

fn required_i64(asset: &UnityAsset, path: &str) -> Result<i64> {
    field_value(asset, path)
        .and_then(Value::as_i64)
        .with_context(|| format!("{} is missing integer field {path}", asset.path))
}

fn required_i32(asset: &UnityAsset, path: &str) -> Result<i32> {
    required_i64(asset, path)?
        .try_into()
        .with_context(|| format!("{} field {path} is outside the i32 range", asset.path))
}

fn required_u32(asset: &UnityAsset, path: &str) -> Result<u32> {
    required_i64(asset, path)?
        .try_into()
        .with_context(|| format!("{} field {path} is outside the u32 range", asset.path))
}

fn required_f64(asset: &UnityAsset, path: &str) -> Result<f64> {
    field_value(asset, path)
        .and_then(Value::as_f64)
        .with_context(|| format!("{} is missing numeric field {path}", asset.path))
}

fn required_f32(asset: &UnityAsset, path: &str) -> Result<f32> {
    let value = required_f64(asset, path)?;
    value
        .to_string()
        .parse()
        .with_context(|| format!("{} field {path} is outside the f32 range", asset.path))
}

fn required_milli(asset: &UnityAsset, path: &str) -> Result<u32> {
    required_scaled_milli(asset, path, 1_000.0)
}

fn required_milli_cells(asset: &UnityAsset, path: &str) -> Result<u32> {
    // The shipping Unity terrain grid uses two world units per logical cell.
    // Role ranges and movement speeds are authored in those world units.
    required_scaled_milli(asset, path, 500.0)
}

fn required_scaled_milli(asset: &UnityAsset, path: &str, scale: f64) -> Result<u32> {
    let value = required_f64(asset, path)?;
    if !value.is_finite() || value < 0.0 {
        bail!(
            "{} field {path} must be finite and non-negative",
            asset.path
        );
    }
    (value * scale)
        .round()
        .to_string()
        .parse()
        .with_context(|| {
            format!(
                "{} field {path} is outside the milli-unit range",
                asset.path
            )
        })
}

fn required_bool(asset: &UnityAsset, path: &str) -> Result<bool> {
    field_value(asset, path)
        .and_then(Value::as_bool)
        .with_context(|| format!("{} is missing Boolean field {path}", asset.path))
}

fn generated_record_ids(asset: &UnityAsset, prefix: &str, kind: &str) -> Result<Vec<StableId>> {
    let size_path = format!("{prefix}.Array.size");
    let size = required_u32(asset, &size_path)?;
    (0..size)
        .map(|index| StableId::new(format!("{kind}:{}:{index}", asset.guid)).map_err(Into::into))
        .collect()
}

fn objective_definitions(asset: &UnityAsset) -> Result<BTreeMap<StableId, ObjectiveDef>> {
    let prefix = "<Objectives>k__BackingField";
    let size = required_u32(asset, &format!("{prefix}.Array.size"))?;
    let mut objectives = BTreeMap::new();
    for index in 0..size {
        let field = format!("{prefix}.Array.data[{index}]");
        let kind_name = required_enum(asset, &format!("{field}.<ObjectiveType>k__BackingField"))?;
        let kind = match kind_name.as_str() {
            "Build" => ObjectiveKind::Build,
            "Build Any" => ObjectiveKind::BuildAny,
            "Collect" => ObjectiveKind::Collect,
            "Kill" => ObjectiveKind::Kill,
            "Kill Any" => ObjectiveKind::KillAny,
            "Earn Per Hour" => ObjectiveKind::EarnPerHour,
            "Sell" => ObjectiveKind::Sell,
            "Sell Any" => ObjectiveKind::SellAny,
            "Buy" => ObjectiveKind::Buy,
            "Buy Any" => ObjectiveKind::BuyAny,
            unknown => bail!("{} has unsupported objective type {unknown}", asset.path),
        };
        let required_amount = required_u32(asset, &format!("{field}.<IntValue>k__BackingField"))?;
        let float_value = required_f64(asset, &format!("{field}.<FloatValue>k__BackingField"))?;
        let float_value_milli = (float_value * 1_000.0)
            .round()
            .to_string()
            .parse()
            .with_context(|| format!("{} objective float value is out of range", asset.path))?;
        let resource = matches!(
            kind,
            ObjectiveKind::Collect
                | ObjectiveKind::EarnPerHour
                | ObjectiveKind::Sell
                | ObjectiveKind::Buy
        )
        .then(|| {
            required_enum(asset, &format!("{field}.<ResourceType>k__BackingField"))
                .and_then(|value| stable_id("resource", &slug(&value)))
        })
        .transpose()?;
        let building = matches!(kind, ObjectiveKind::Build)
            .then(|| {
                required_enum(asset, &format!("{field}.<BuildingType>k__BackingField"))
                    .and_then(|value| stable_id("building", &slug(&value)))
            })
            .transpose()?;
        let enemy = matches!(kind, ObjectiveKind::Kill)
            .then(|| {
                required_enum(asset, &format!("{field}.<EnemyType>k__BackingField"))
                    .and_then(|value| stable_id("enemy", &slug(&value)))
            })
            .transpose()?;
        let id = StableId::new(format!("objective:{}:{index}", asset.guid))?;
        objectives.insert(
            id,
            ObjectiveDef {
                kind,
                required_amount,
                float_value_milli,
                resource,
                building,
                enemy,
            },
        );
    }
    Ok(objectives)
}

fn building_level_caps(asset: &UnityAsset) -> Result<BTreeMap<StableId, u16>> {
    let size = required_u32(asset, "<Unlocks>k__BackingField.Array.size")?;
    let mut caps = BTreeMap::new();
    for index in 0..size {
        let prefix = format!("<Unlocks>k__BackingField.Array.data[{index}]");
        if required_enum(asset, &format!("{prefix}.<TechType>k__BackingField"))?
            != "Upgrade Building"
        {
            continue;
        }
        let building = required_enum(asset, &format!("{prefix}.<BuildingType>k__BackingField"))?;
        let cap = u16::try_from(required_u32(
            asset,
            &format!("{prefix}.<IntValue>k__BackingField"),
        )?)
        .with_context(|| format!("{} building level cap is out of range", asset.path))?;
        caps.insert(stable_id("building", &slug(&building))?, cap.max(1));
    }
    Ok(caps)
}

fn unlocked_buildings(asset: &UnityAsset) -> Result<BTreeSet<StableId>> {
    let size = required_u32(asset, "<Unlocks>k__BackingField.Array.size")?;
    let mut buildings = BTreeSet::new();
    for index in 0..size {
        let prefix = format!("<Unlocks>k__BackingField.Array.data[{index}]");
        if required_enum(asset, &format!("{prefix}.<TechType>k__BackingField"))?
            != "Unlock Building"
        {
            continue;
        }
        let building = required_enum(asset, &format!("{prefix}.<BuildingType>k__BackingField"))?;
        buildings.insert(stable_id("building", &slug(&building))?);
    }
    Ok(buildings)
}

fn building_cost_reductions(asset: &UnityAsset) -> Result<(i32, BTreeMap<StableId, i32>)> {
    let size = required_u32(asset, "<Unlocks>k__BackingField.Array.size")?;
    let mut global = 0_i32;
    let mut buildings = BTreeMap::<StableId, i32>::new();
    for index in 0..size {
        let prefix = format!("<Unlocks>k__BackingField.Array.data[{index}]");
        if required_enum(asset, &format!("{prefix}.<TechType>k__BackingField"))?
            != "Building Cost Reduction"
        {
            continue;
        }
        let amount = technology_i32(asset, &prefix)?;
        let building = required_enum(asset, &format!("{prefix}.<BuildingType>k__BackingField"))?;
        if building == "Count" {
            global = global.saturating_add(amount);
        } else {
            let current = buildings
                .entry(stable_id("building", &slug(&building))?)
                .or_default();
            *current = current.saturating_add(amount);
        }
    }
    Ok((global, buildings))
}

fn storage_boosts(asset: &UnityAsset) -> Result<BTreeMap<StableId, i32>> {
    let size = required_u32(asset, "<Unlocks>k__BackingField.Array.size")?;
    let mut resources = BTreeMap::<StableId, i32>::new();
    for index in 0..size {
        let prefix = format!("<Unlocks>k__BackingField.Array.data[{index}]");
        if required_enum(asset, &format!("{prefix}.<TechType>k__BackingField"))? != "Storage Boost"
        {
            continue;
        }
        let resource = required_enum(asset, &format!("{prefix}.<ResourceType>k__BackingField"))?;
        let current = resources
            .entry(stable_id("resource", &slug(&resource))?)
            .or_default();
        *current = current.saturating_add(technology_i32(asset, &prefix)?);
    }
    Ok(resources)
}

type StatBoosts = (
    BTreeMap<StableId, i32>,
    BTreeMap<StableId, BTreeMap<StableId, i32>>,
);

fn stat_boosts(asset: &UnityAsset) -> Result<StatBoosts> {
    let size = required_u32(asset, "<Unlocks>k__BackingField.Array.size")?;
    let mut global = BTreeMap::<StableId, i32>::new();
    let mut roles = BTreeMap::<StableId, BTreeMap<StableId, i32>>::new();
    for index in 0..size {
        let prefix = format!("<Unlocks>k__BackingField.Array.data[{index}]");
        if required_enum(asset, &format!("{prefix}.<TechType>k__BackingField"))? != "Stat Boost" {
            continue;
        }
        let stat = stable_id(
            "stat",
            &slug(&required_enum(
                asset,
                &format!("{prefix}.<StatType>k__BackingField"),
            )?),
        )?;
        let role = required_enum(asset, &format!("{prefix}.<PlayerRole>k__BackingField"))?;
        let amount = technology_i32(asset, &prefix)?;
        let current = if role == "Count" {
            global.entry(stat).or_default()
        } else {
            roles
                .entry(stable_id("role", &slug(&role))?)
                .or_default()
                .entry(stat)
                .or_default()
        };
        *current = current.saturating_add(amount);
    }
    Ok((global, roles))
}

fn aged_buildings(asset: &UnityAsset) -> Result<BTreeSet<StableId>> {
    let size = required_u32(asset, "<Unlocks>k__BackingField.Array.size")?;
    let mut buildings = BTreeSet::new();
    for index in 0..size {
        let prefix = format!("<Unlocks>k__BackingField.Array.data[{index}]");
        if required_enum(asset, &format!("{prefix}.<TechType>k__BackingField"))?
            != "Age Up Building"
        {
            continue;
        }
        let building = required_enum(asset, &format!("{prefix}.<BuildingType>k__BackingField"))?;
        buildings.insert(stable_id("building", &slug(&building))?);
    }
    Ok(buildings)
}

fn technology_i32(asset: &UnityAsset, prefix: &str) -> Result<i32> {
    required_i64(asset, &format!("{prefix}.<IntValue>k__BackingField"))?
        .try_into()
        .with_context(|| format!("{} technology modifier is out of range", asset.path))
}

fn technology_group_name(path: &str) -> String {
    path.split_once("/Groups/")
        .and_then(|(_, suffix)| suffix.split('/').next())
        .unwrap_or("Global")
        .to_owned()
}

fn stable_id(prefix: &str, value: &str) -> Result<StableId> {
    StableId::new(format!("{prefix}:{value}")).map_err(Into::into)
}

fn animation_parameter_name(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<String>()
        .replace("StaffAttackMagic", "StaffMagicAttack")
}

fn slug(value: &str) -> String {
    let mut output = String::new();
    let mut separator = false;
    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            output.push(character);
            separator = false;
        } else if !separator && !output.is_empty() {
            output.push('_');
            separator = true;
        }
    }
    while output.ends_with('_') {
        output.pop();
    }
    if output.is_empty() {
        "unnamed".to_owned()
    } else {
        output
    }
}

fn insert_source_record(
    records: &mut BTreeMap<StableId, AuthoredRecord>,
    id: StableId,
    asset: &UnityAsset,
) -> Result<()> {
    let properties = asset
        .serialized_fields
        .iter()
        .map(|field| Ok((field.path.clone(), authored_value(&field.value)?)))
        .collect::<Result<_>>()?;
    let previous = records.insert(
        id.clone(),
        AuthoredRecord {
            source_guid: asset.guid.clone(),
            source_path: asset.path.clone(),
            source_type: asset
                .unity_type
                .clone()
                .unwrap_or_else(|| asset.name.clone()),
            properties,
        },
    );
    if previous.is_some() {
        bail!("duplicate semantic stable ID {id}");
    }
    Ok(())
}

fn authored_value(value: &Value) -> Result<AuthoredValue> {
    Ok(match value {
        Value::Null => AuthoredValue::Null,
        Value::Bool(value) => AuthoredValue::Bool(*value),
        Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                AuthoredValue::Integer(value)
            } else if let Some(value) = value.as_u64() {
                AuthoredValue::Integer(
                    value
                        .try_into()
                        .context("Unity unsigned integer exceeds the authored i64 range")?,
                )
            } else {
                AuthoredValue::Float(value.as_f64().context("invalid Unity floating value")?)
            }
        }
        Value::String(value) => AuthoredValue::Text(value.clone()),
        Value::Array(values) => AuthoredValue::Sequence(
            values
                .iter()
                .map(authored_value)
                .collect::<Result<Vec<_>>>()?,
        ),
        Value::Object(values) => AuthoredValue::Object(
            values
                .iter()
                .map(|(key, value)| Ok((key.clone(), authored_value(value)?)))
                .collect::<Result<_>>()?,
        ),
    })
}

fn write_ron_atomic<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let encoded = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())?;
    let temporary = path.with_extension("ron.tmp");
    fs::write(&temporary, encoded)
        .with_context(|| format!("failed to write temporary output {}", temporary.display()))?;
    if path.exists() {
        fs::remove_file(path)
            .with_context(|| format!("failed to replace generated output {}", path.display()))?;
    }
    fs::rename(&temporary, path)
        .with_context(|| format!("failed to publish generated output {}", path.display()))
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windmill_axes_follow_the_emitted_glb_mesh_normals() {
        let matches = |actual: [f32; 3], expected: [f32; 3]| {
            actual
                .into_iter()
                .zip(expected)
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        };
        assert!(matches(
            converted_rotating_axis("Age01_Windmill_Blades", [0.0, 0.0, 1.0]),
            [0.0, 1.0, 0.0]
        ));
        assert!(matches(
            converted_rotating_axis("Age02_Windmill_Blades", [0.0, 1.0, 0.0]),
            [0.0, 0.0, 1.0]
        ));
        assert!(matches(
            converted_rotating_axis("LoadingIcon", [0.0, 0.0, 1.0]),
            [0.0, 0.0, 1.0]
        ));
    }

    fn field(path: &str, value: Value) -> UnityField {
        UnityField {
            path: path.to_owned(),
            value,
        }
    }

    fn reference_value(guid: &str, unity_type: &str) -> Value {
        serde_json::json!({ "Guid": guid, "Type": unity_type })
    }

    fn foliage_fixture_fields(list: &str, model: &str, threshold: f64) -> Vec<UnityField> {
        let prefix = format!("{list}.Array.data[0]");
        let mesh = format!("{prefix}.MeshSettings.Array.data[0]");
        vec![
            field(&format!("{list}.Array.size"), Value::from(1)),
            field(&format!("{prefix}.Size"), Value::from(300)),
            field(&format!("{prefix}.LevelOfDetail"), Value::from(2)),
            field(&format!("{prefix}.NoiseScale"), Value::from(10.0)),
            field(&format!("{prefix}.Octaves"), Value::from(1)),
            field(&format!("{prefix}.Persistance"), Value::from(0.8)),
            field(&format!("{prefix}.Lacunarity"), Value::from(1.53)),
            field(&format!("{prefix}.Seed"), Value::from(-430_535_522)),
            field(&format!("{prefix}.Offset.x"), Value::from(130.0)),
            field(&format!("{prefix}.Offset.y"), Value::from(302.0)),
            field(&format!("{prefix}.SpawnThreshold"), Value::from(threshold)),
            field(&format!("{prefix}.Spacing"), Value::from(1)),
            field(&format!("{prefix}.MeshSettings.Array.size"), Value::from(1)),
            field(
                &format!("{mesh}.Mesh"),
                serde_json::json!({ "Guid": "mesh", "Path": format!("Assets/Models/Environment/{model}.fbx") }),
            ),
            field(&format!("{mesh}.BaseScale.x"), Value::from(0.5)),
            field(&format!("{mesh}.BaseScale.y"), Value::from(0.3)),
            field(&format!("{mesh}.BaseScale.z"), Value::from(0.5)),
            field(
                &format!("{prefix}.Material"),
                serde_json::json!({ "Guid": "material", "Path": "Assets/Materials/Environment/Env_Grass.mat" }),
            ),
        ]
    }

    fn asset(
        guid: &str,
        path: &str,
        unity_type: &str,
        serialized_fields: Vec<UnityField>,
    ) -> UnityAsset {
        UnityAsset {
            guid: guid.to_owned(),
            path: path.to_owned(),
            kind: if path.ends_with(".prefab") {
                "prefab".to_owned()
            } else {
                "scriptable_asset".to_owned()
            },
            name: path.to_owned(),
            unity_type: Some(unity_type.to_owned()),
            status: "exported".to_owned(),
            serialized_fields,
            dependencies: Vec::new(),
            game_object: None,
            scene: None,
        }
    }

    fn component(unity_type: &str, fields: Vec<UnityField>) -> UnityComponent {
        component_at("", unity_type, fields)
    }

    fn component_at(
        hierarchy_path: &str,
        unity_type: &str,
        fields: Vec<UnityField>,
    ) -> UnityComponent {
        UnityComponent {
            hierarchy_path: hierarchy_path.to_owned(),
            unity_type: Some(unity_type.to_owned()),
            fields,
        }
    }

    #[test]
    fn stable_slugs_are_portable() {
        assert_eq!(slug("Town Hall / Age 2"), "town_hall_age_2");
        assert_eq!(slug(""), "unnamed");
    }

    #[test]
    fn converts_station_masks_and_ranges() {
        let mut prefab = asset(
            "station",
            "Assets/Prefabs/Buildings/Building_Station_Lumbermill.prefab",
            "UnityEngine.GameObject",
            vec![],
        );
        prefab.game_object = Some(UnityGameObject {
            components: vec![component(
                "Buildings.Station, Assembly-CSharp",
                vec![
                    field("_flags", serde_json::json!({"Index": 3, "Name": "Wood"})),
                    field(
                        "_targetMask",
                        serde_json::json!({"Index": 2, "Name": "Tree"}),
                    ),
                    field("_maxListSize", Value::from(10)),
                    field("_updateRate", Value::from(3.0)),
                    field("_targetSearchRange", Value::from(100.0)),
                ],
            )],
        });
        let station = station_definition(&prefab).unwrap().unwrap();
        assert_eq!(
            station.accepted_role_kinds,
            BTreeSet::from([stable_id("station", "wood").unwrap()])
        );
        assert_eq!(
            station.target_kinds,
            BTreeSet::from([stable_id("target", "tree").unwrap()])
        );
        assert_eq!(station.max_targets, 10);
        assert_eq!(station.update_milliseconds, 3_000);
        assert_eq!(station.search_range_milli_cells, 50_000);
    }

    #[test]
    fn converts_station_target_refresh_policies() {
        let settings = asset(
            "targets",
            TARGET_SETTINGS,
            "ScriptablesProcessorInfrastructure.TargetSettings",
            vec![
                field("_targetableData.Array.size", Value::from(2)),
                field(
                    "_targetableData.Array.data[0].TargetType",
                    serde_json::json!({"Index": 2, "Name": "Tree", "RawValue": 2}),
                ),
                field(
                    "_targetableData.Array.data[0].UpdateType",
                    serde_json::json!({"Index": 0, "Name": "Update", "RawValue": 0}),
                ),
                field(
                    "_targetableData.Array.data[1].TargetType",
                    serde_json::json!({"Index": 6, "Name": "Fish", "RawValue": 32}),
                ),
                field(
                    "_targetableData.Array.data[1].UpdateType",
                    serde_json::json!({"Index": 1, "Name": "Clear", "RawValue": 1}),
                ),
            ],
        );

        assert_eq!(
            station_target_update_modes(&settings).unwrap(),
            BTreeMap::from([
                (
                    StableId::new("target:fish").unwrap(),
                    StationUpdateMode::Clear,
                ),
                (
                    StableId::new("target:tree").unwrap(),
                    StationUpdateMode::Update,
                ),
            ])
        );
    }

    #[test]
    fn converts_targetable_assignment_and_distance_weights() {
        let mut prefab = asset(
            "farm",
            "Assets/Prefabs/Buildings/Building_Farm.prefab",
            "UnityEngine.GameObject",
            vec![],
        );
        prefab.game_object = Some(UnityGameObject {
            components: vec![component(
                "Target.TargetableBuilding, Assembly-CSharp",
                vec![
                    field("_assignmentPenaltyMod", Value::from(10_000.0)),
                    field("_distancePenaltyMod", Value::from(0.05)),
                ],
            )],
        });
        assert_eq!(
            targeting_score_definition(&prefab).unwrap(),
            Some(TargetingScoreDef {
                assignment_penalty_milli: 10_000_000,
                distance_penalty_milli_per_cell: 100,
            })
        );
    }

    #[test]
    fn converts_disable_after_time_lifetime() {
        let mut healing = asset(
            "healing",
            "Assets/Prefabs/VFX/Player/VFX_healing.prefab",
            "UnityEngine.GameObject",
            vec![],
        );
        healing.game_object = Some(UnityGameObject {
            components: vec![component(
                "Utils.SimpleDisableAfterTime, Assembly-CSharp",
                vec![field("_lifeTime", Value::from(1.2))],
            )],
        });
        assert_eq!(disable_after_milliseconds(&healing).unwrap(), Some(1_200));
        healing.game_object.as_mut().unwrap().components[0].fields[0].value = Value::from(0.0);
        assert!(disable_after_milliseconds(&healing).is_err());
    }

    #[test]
    fn converts_unit_health_bar_hide_delay() {
        let mut player = asset("player", PLAYER_PREFAB, "UnityEngine.GameObject", vec![]);
        player.game_object = Some(UnityGameObject {
            components: vec![component(
                "Units.UnitHealthBar, Assembly-CSharp",
                vec![field("_hideDelay", Value::from(3.0))],
            )],
        });
        assert_eq!(health_bar_hide_milliseconds(&player).unwrap(), Some(3_000));
        player.game_object.as_mut().unwrap().components[0].fields[0].value = Value::from(0.0);
        assert!(health_bar_hide_milliseconds(&player).is_err());
    }

    #[test]
    fn converts_loader_timing_and_tooltips() {
        let mut loader = asset("loader", LOADER_SCENE, "UnityEditor.SceneAsset", vec![]);
        loader.scene = Some(UnityScene {
            roots: vec![UnityGameObject {
                components: vec![component_at(
                    "LoadingManager",
                    "UserInterface.MainMenu.LoadingManager, Assembly-CSharp",
                    vec![
                        field("_loadingSpeed", Value::from(0.5)),
                        field("_waitTime", Value::from(0.5)),
                        field("_toolTips.Array.size", Value::from(1)),
                        field(
                            "_toolTips.Array.data[0]",
                            Value::from("This is a tooltip! Isn't that neat?"),
                        ),
                    ],
                )],
            }],
        });
        assert_eq!(
            loading_screen_definition(&loader).unwrap(),
            LoadingScreenDef {
                progress_milli_per_second: 500,
                completion_hold_milliseconds: 500,
                tooltips: vec!["This is a tooltip! Isn't that neat?".to_owned()],
            }
        );
    }

    #[test]
    fn converts_targetable_custom_and_root_collider_sizes() {
        let mut custom = asset("farm", "farm.prefab", "UnityEngine.GameObject", vec![]);
        custom.game_object = Some(UnityGameObject {
            components: vec![component(
                "Target.TargetableBuilding, Assembly-CSharp",
                vec![
                    field("_useCustomSize", Value::Bool(true)),
                    field("_customSize", Value::from(1.0)),
                ],
            )],
        });
        assert_eq!(targetable_size_milli_cells(&custom).unwrap(), 500);

        let mut collider = asset("player", "player.prefab", "UnityEngine.GameObject", vec![]);
        collider.game_object = Some(UnityGameObject {
            components: vec![
                component(
                    "Target.TargetablePlayer, Assembly-CSharp",
                    vec![field("_useCustomSize", Value::Bool(false))],
                ),
                component(
                    "UnityEngine.BoxCollider, UnityEngine.PhysicsModule",
                    vec![field(
                        "size",
                        serde_json::json!({"x": 1.5, "y": 2.0, "z": 1.0}),
                    )],
                ),
            ],
        });
        assert_eq!(targetable_size_milli_cells(&collider).unwrap(), 750);
    }

    #[test]
    fn converts_building_role_slot_modifiers() {
        let mut building = asset(
            "house",
            "Assets/Prefabs/Buildings/Building_House.prefab",
            "UnityEngine.GameObject",
            vec![],
        );
        building.game_object = Some(UnityGameObject {
            components: vec![component(
                "Buildings.RoleSlotModifier, Assembly-CSharp",
                vec![
                    field("_role", serde_json::json!({"Index": 6, "Name": "Logger"})),
                    field("_baseAmount", Value::from(3)),
                    field("_incrementAmount", Value::from(2)),
                ],
            )],
        });
        assert_eq!(
            role_slot_contributions(&building).unwrap(),
            vec![RoleSlotContribution {
                role: stable_id("role", "logger").unwrap(),
                base_amount: 3,
                increment_amount: 2,
            }]
        );
    }

    #[test]
    fn converts_health_revival_and_projectile_shooters() {
        let mut player = asset("player", PLAYER_PREFAB, "UnityEngine.GameObject", vec![]);
        player.game_object = Some(UnityGameObject {
            components: vec![
                component(
                    "Units.HealthHandler, Assembly-CSharp",
                    vec![
                        field("_maxHealth", Value::from(100)),
                        field("_healthGainOnLevel", Value::from(25)),
                        field("_healthRegen", Value::from(1.25)),
                        field("_regenRequiresFood", Value::Bool(true)),
                    ],
                ),
                component(
                    "Character.PlayerDeathHandler, Assembly-CSharp",
                    vec![field("_reviveTime", Value::from(60.0))],
                ),
            ],
        });
        assert_eq!(
            health_definition(&player).unwrap(),
            Some(HealthDef {
                max_health: 100,
                health_gain_per_level: 25,
                regeneration_milli_per_second: 1_250,
                regeneration_requires_food: true,
                revive_milliseconds: Some(60_000),
            })
        );

        let mut tower = asset("tower", "tower.prefab", "UnityEngine.GameObject", vec![]);
        let shooter = || {
            component(
                "Buildings.ProjectileShooter, Assembly-CSharp",
                vec![
                    field(
                        "ProjectilePoolName",
                        Value::String("TowerProjectile".into()),
                    ),
                    field("_moveSpeed", Value::from(30.0)),
                    field("_damage", Value::from(1)),
                    field("_range", Value::from(20.0)),
                    field("_fireRate", Value::from(3.0)),
                ],
            )
        };
        tower.game_object = Some(UnityGameObject {
            components: vec![shooter(), shooter()],
        });
        assert_eq!(
            projectile_shooter_definition(&tower).unwrap(),
            Some(ProjectileShooterDef {
                projectile_pool: "TowerProjectile".into(),
                movement_milli_cells_per_second: 15_000,
                damage: 1,
                range_milli_cells: 10_000,
                fire_milliseconds: 3_000,
            })
        );
    }

    #[test]
    fn converts_pet_follow_settings_and_model_overrides() {
        let mut pet = asset("pet", PET_PREFAB, "UnityEngine.GameObject", vec![]);
        pet.game_object = Some(UnityGameObject {
            components: vec![
                component(
                    "Pets.Pet, Assembly-CSharp",
                    vec![
                        field("_closestDistanceToPlayer", Value::from(1.0)),
                        field("_maxDistanceFromPlayer", Value::from(5.0)),
                        field("_minMoveSpeed", Value::from(0.0)),
                        field("_maxMoveSpeed", Value::from(10.0)),
                        field("_rotationSpeed", Value::from(5.0)),
                    ],
                ),
                component_at(
                    "Critter_Fish3",
                    "UnityEngine.Transform, UnityEngine.CoreModule",
                    vec![
                        field(
                            "localPosition",
                            serde_json::json!({"x": 0.0, "y": 1.403, "z": 2.0}),
                        ),
                        field(
                            "localRotation",
                            serde_json::json!({"x": 0.1, "y": 0.2, "z": 0.3, "w": 0.4}),
                        ),
                        field(
                            "localScale",
                            serde_json::json!({"x": 1.0, "y": 1.0, "z": 1.0}),
                        ),
                    ],
                ),
                component_at(
                    "Critter_Fish3",
                    "Pets.PetModel, Assembly-CSharp",
                    vec![field("_petType", serde_json::json!({"Name": "Fish God"}))],
                ),
                component_at(
                    "Critter_Fish3",
                    "UnityEngine.MeshFilter, UnityEngine.CoreModule",
                    vec![field(
                        "sharedMesh",
                        serde_json::json!({
                            "Guid": "fish",
                            "Path": "Assets/Models/Critters/Critter_Fish3.fbx"
                        }),
                    )],
                ),
            ],
        });

        let definition = pet_definition(&pet).unwrap().unwrap();
        assert!((definition.closest_distance - 1.0).abs() < f32::EPSILON);
        assert!((definition.max_distance - 5.0).abs() < f32::EPSILON);
        assert!(definition.min_move_speed.abs() < f32::EPSILON);
        assert!((definition.max_move_speed - 10.0).abs() < f32::EPSILON);
        assert!((definition.rotation_radians_per_second - 5.0).abs() < f32::EPSILON);
        let fish = &definition.models[&StableId::new("pet:fish_god").unwrap()];
        assert_eq!(
            fish.source_model,
            "Assets/Models/Critters/Critter_Fish3.fbx"
        );
        assert!(
            fish.local_position
                .iter()
                .zip([0.0, 1.403, -2.0])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!(
            fish.local_rotation
                .iter()
                .zip([-0.1, -0.2, 0.3, 0.4])
                .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
        );
        assert!(
            fish.local_scale
                .iter()
                .all(|value| (value - 1.0).abs() < f32::EPSILON)
        );
    }

    #[test]
    fn converts_enemy_combat_and_camp_spawn_data() {
        let enemy_guid = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        let pools = PoolIndex {
            pool_by_prefab_guid: BTreeMap::from([(
                enemy_guid.to_owned(),
                stable_id("pool", "goblin").unwrap(),
            )]),
            archetype_by_pool_name: BTreeMap::from([(
                "goblin".to_owned(),
                stable_id("archetype:prefab", enemy_guid).unwrap(),
            )]),
        };
        let mut enemy = asset(
            enemy_guid,
            "Assets/Prefabs/Enemies/Enemy_Goblin.prefab",
            "UnityEngine.GameObject",
            vec![],
        );
        enemy.game_object = Some(UnityGameObject {
            components: vec![
                component(
                    "Enemies.Enemy, Assembly-CSharp",
                    vec![
                        field("_enemyType", serde_json::json!({"Name": "Goblin"})),
                        field("_additionalHealthPerPlayer", Value::from(0.2)),
                    ],
                ),
                component(
                    "STStateMachine.States.STSM_Action_EnemyAttack, Assembly-CSharp",
                    vec![
                        field("_actionAmount", Value::from(1)),
                        field("_actionRate", Value::from(1.0)),
                        field("_actionRange", Value::from(2.0)),
                    ],
                ),
                component(
                    "Sensors.TargetSensor, Assembly-CSharp",
                    vec![
                        field(
                            "_targetMask",
                            serde_json::json!({"Index": -1, "Name": null, "RawValue": 3841}),
                        ),
                        field("_attackAttacker", Value::from(true)),
                        field("_targetSearchRange", Value::from(8.0)),
                    ],
                ),
                component(
                    "GameResources.ActiveResourceIncrementer, Assembly-CSharp",
                    vec![
                        field(
                            "_resource",
                            serde_json::json!({"Index": 4, "Name": "Gold", "RawValue": 4}),
                        ),
                        field("_amount", Value::from(15)),
                    ],
                ),
            ],
        });
        let converted = enemy_definition(&enemy, &pools).unwrap().unwrap();
        assert_eq!(converted.enemy_type.as_str(), "enemy:goblin");
        assert_eq!(converted.additional_health_milli_per_player, 200);
        assert_eq!(converted.action_milliseconds, 1_000);
        assert_eq!(converted.action_range_milli_cells, 1_000);
        assert_eq!(converted.target_search_range_milli_cells, 4_000);
        assert!(converted.attack_attacker);
        assert_eq!(
            converted.kill_reward,
            ResourceReward {
                resource: StableId::new("resource:gold").unwrap(),
                amount: 15,
            }
        );
        assert_eq!(
            converted.target_kinds,
            BTreeSet::from([
                stable_id("target", "player").unwrap(),
                stable_id("target", "building").unwrap(),
                stable_id("target", "damaged_building").unwrap(),
                stable_id("target", "construction").unwrap(),
                stable_id("target", "injured_player").unwrap(),
            ])
        );

        let mut transform = component(
            "UnityEngine.Transform, UnityEngine.CoreModule",
            vec![field(
                "localPosition",
                serde_json::json!({"x": 3.5, "y": 0.0, "z": 4.5}),
            )],
        );
        transform.hierarchy_path = "Spawner_Goblin".to_owned();
        let mut camp = asset(
            "cccccccccccccccccccccccccccccccc",
            "Assets/Prefabs/Buildings/Enemy/Camp.prefab",
            "UnityEngine.GameObject",
            vec![],
        );
        camp.game_object = Some(UnityGameObject {
            components: vec![
                component(
                    "Enemies.EnemySpawner, Assembly-CSharp",
                    vec![
                        field("_minTotalEnemies", Value::from(3)),
                        field("_maxTotalEnemies", Value::from(50)),
                        field("_timeBetweenSpawns", Value::from(3.0)),
                        field("_enemies._list.Array.size", Value::from(1)),
                        field("_enemies._list.Array.data[0].Object", Value::from("Goblin")),
                        field("_enemies._list.Array.data[0].Chance", Value::from(50.0)),
                        field("_spawnLocations.Array.size", Value::from(1)),
                        field(
                            "_spawnLocations.Array.data[0]",
                            serde_json::json!({"Guid": "camp", "Name": "Spawner_Goblin"}),
                        ),
                    ],
                ),
                transform,
            ],
        });
        let converted = enemy_spawner_definition(&camp, &pools).unwrap().unwrap();
        assert_eq!(converted.spawn_milliseconds, 3_000);
        assert_eq!(converted.spawn_offsets_milli_cells, vec![[1_750, 2_250]]);
        assert_eq!(converted.weighted_enemies[0].weight_milli, 50_000);
    }

    #[test]
    fn converts_enemy_model_and_weapon_animation_contracts() {
        let mut enemy = asset(
            "skeleton",
            "Assets/Prefabs/Enemies/Enemy_Skeleton.prefab",
            "UnityEngine.GameObject",
            vec![],
        );
        enemy.game_object = Some(UnityGameObject {
            components: vec![component(
                "Enemies.EnemyModelHandler, Assembly-CSharp",
                vec![
                    field("_baseModels.Array.size", Value::from(0)),
                    field("_permanentModels.Array.size", Value::from(1)),
                    field(
                        "_permanentModels.Array.data[0]",
                        serde_json::json!({"Name": "SkeletonBody"}),
                    ),
                    field("_optionalModels.Array.size", Value::from(1)),
                    field(
                        "_optionalModels.Array.data[0]",
                        serde_json::json!({"Name": "SkeletonHelmet"}),
                    ),
                    field("_linkedWeaponModels.Array.size", Value::from(1)),
                    field(
                        "_linkedWeaponModels.Array.data[0]._mainWeaponModel",
                        serde_json::json!({"Name": "Enemy_SkelePoleaxe"}),
                    ),
                    field(
                        "_linkedWeaponModels.Array.data[0]._offHandModels.Array.size",
                        Value::from(0),
                    ),
                    field(
                        "_linkedWeaponModels.Array.data[0]._weaponAnimationName",
                        serde_json::json!({"Name": "Hammer Attack"}),
                    ),
                    field(
                        "_linkedWeaponModels.Array.data[0]._animationVariants",
                        Value::from(1),
                    ),
                    field(
                        "_linkedWeaponModels.Array.data[0]._runAnimation",
                        serde_json::json!({"Name": "Two Handed"}),
                    ),
                    field("_baseAnimationVariants", Value::from(1)),
                    field("_randomize", Value::Bool(false)),
                ],
            )],
        });
        let models = enemy_model_definition(&enemy).unwrap().unwrap();
        assert_eq!(models.permanent_models, ["SkeletonBody"]);
        assert_eq!(models.optional_models, ["SkeletonHelmet"]);
        assert_eq!(enemy_model_node_count(&models), 3);
        assert_eq!(models.weapons[0].main_model, "Enemy_SkelePoleaxe");
        assert_eq!(models.weapons[0].action_animation, "HammerAttack");
        assert_eq!(
            models.weapons[0].run_animation,
            EnemyRunAnimation::TwoHanded
        );
    }

    #[test]
    fn decomposes_combined_unity_flag_values() {
        let mut role = asset("role", "role.asset", ROLE_TYPE, vec![]);
        role.serialized_fields = vec![field(
            "TargetFlags",
            serde_json::json!({"Index": -1, "Name": null, "RawValue": 1536}),
        )];
        let (all, targets) = authored_mask(&role, "TargetFlags", "target").unwrap();
        assert!(!all);
        assert_eq!(
            targets,
            BTreeSet::from([
                stable_id("target", "damaged_building").unwrap(),
                stable_id("target", "construction").unwrap(),
            ])
        );
        let all_flags = serde_json::json!({"Index": -1, "Name": null, "RawValue": -1});
        let (all, targets) = mask_ids(all_flags.as_object().unwrap(), "target").unwrap();
        assert!(all);
        assert!(targets.is_empty());
    }

    #[test]
    fn converts_role_equipment_node_bindings() {
        let mut prefab = asset("player", PLAYER_PREFAB, "UnityEngine.GameObject", vec![]);
        let reference = |name: &str| serde_json::json!({"LocalId": 1, "Name": name});
        prefab.game_object = Some(UnityGameObject {
            components: vec![component(
                "Character.CharacterModelHandler, Assembly-CSharp",
                vec![
                    field("_equipmentSets.Array.size", Value::from(1)),
                    field(
                        "_equipmentSets.Array.data[0].RoleName",
                        Value::String("Logger".into()),
                    ),
                    field(
                        "_equipmentSets.Array.data[0].BodieSlim",
                        reference("Body_Logger_Slim"),
                    ),
                    field(
                        "_equipmentSets.Array.data[0].BodieBulk",
                        reference("Body_Logger_Bulk"),
                    ),
                    field(
                        "_equipmentSets.Array.data[0].BodieFeminine",
                        reference("Body_Logger_Feminine"),
                    ),
                    field(
                        "_equipmentSets.Array.data[0].LeftHand",
                        reference("LHand_LoggerCarryWood"),
                    ),
                    field(
                        "_equipmentSets.Array.data[0].RightHand",
                        reference("RHand_LoggerToolAxe"),
                    ),
                    field("_equipmentSets.Array.data[0].Helmet", Value::Null),
                    field(
                        "_equipmentSets.Array.data[0].HasCarryAnimation",
                        Value::Bool(true),
                    ),
                    field(
                        "_equipmentSets.Array.data[0].CarryAnimation",
                        serde_json::json!({"Index": 15, "Name": "Carry Wood"}),
                    ),
                    field(
                        "_equipmentSets.Array.data[0].LeftHandPermanent",
                        Value::Bool(false),
                    ),
                ],
            )],
        });
        let equipment = role_equipment(&prefab).unwrap().remove("logger").unwrap();
        assert_eq!(equipment.body_nodes[0], "Body_Logger_Slim");
        assert_eq!(
            equipment.left_hand_node.as_deref(),
            Some("LHand_LoggerCarryWood")
        );
        assert_eq!(equipment.carry_animation.as_deref(), Some("Carry Wood"));
        assert!(!equipment.left_hand_permanent);
    }

    #[test]
    fn unity_animation_enum_names_match_character_controller_parameters() {
        assert_eq!(animation_parameter_name("Wood Cutting"), "WoodCutting");
        assert_eq!(
            animation_parameter_name("Staff Attack Magic"),
            "StaffMagicAttack"
        );
    }

    #[test]
    fn converts_active_catalog_references_and_round_trips_ron() {
        const BUILDING_GUID: &str = "11111111111111111111111111111111";
        const ROLE_GUID: &str = "22222222222222222222222222222222";
        const ROOT_GUID: &str = "33333333333333333333333333333333";
        const CHILD_GUID: &str = "44444444444444444444444444444444";
        const PREFAB_GUID: &str = "55555555555555555555555555555555";
        let enum_value = |name: &str| serde_json::json!({ "Index": 0, "Name": name });
        let technology_fields = |name: &str, child: Option<&str>| {
            let mut fields = vec![
                field("<TechName>k__BackingField", Value::String(name.to_owned())),
                field(
                    "<Description>k__BackingField",
                    Value::String(format!("{name} description")),
                ),
                field(
                    "<Unlocks>k__BackingField.Array.size",
                    Value::from(if name == "Root" { 6 } else { 0 }),
                ),
                field(
                    "<Objectives>k__BackingField.Array.size",
                    Value::from(i32::from(name == "Child")),
                ),
                field("<Age>k__BackingField", enum_value("Age 1")),
                field("<Tier>k__BackingField", Value::from(1)),
                field("<IsUnlocked>k__BackingField", Value::Bool(name == "Root")),
                field("<Unavailable>k__BackingField", Value::Bool(false)),
                field("<IconPath>k__BackingField", Value::String(String::new())),
            ];
            if let Some(child) = child {
                fields.push(field(
                    "<Children>k__BackingField.Array.data[0].<NextTech>k__BackingField",
                    reference_value(child, TECH_NODE_TYPE),
                ));
            }
            if name == "Child" {
                fields.extend([
                    field(
                        "<Objectives>k__BackingField.Array.data[0].<ObjectiveType>k__BackingField",
                        enum_value("Collect"),
                    ),
                    field(
                        "<Objectives>k__BackingField.Array.data[0].<IntValue>k__BackingField",
                        Value::from(25),
                    ),
                    field(
                        "<Objectives>k__BackingField.Array.data[0].<FloatValue>k__BackingField",
                        Value::from(0.0),
                    ),
                    field(
                        "<Objectives>k__BackingField.Array.data[0].<ResourceType>k__BackingField",
                        enum_value("Wood"),
                    ),
                ]);
            }
            if name == "Root" {
                fields.extend([
                    field(
                        "<Unlocks>k__BackingField.Array.data[0].<TechType>k__BackingField",
                        enum_value("Upgrade Building"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[0].<BuildingType>k__BackingField",
                        enum_value("Townhall"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[0].<IntValue>k__BackingField",
                        Value::from(3),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[1].<TechType>k__BackingField",
                        enum_value("Unlock Building"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[1].<BuildingType>k__BackingField",
                        enum_value("Townhall"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[1].<IntValue>k__BackingField",
                        Value::from(0),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[2].<TechType>k__BackingField",
                        enum_value("Building Cost Reduction"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[2].<BuildingType>k__BackingField",
                        enum_value("Townhall"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[2].<IntValue>k__BackingField",
                        Value::from(5),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[3].<TechType>k__BackingField",
                        enum_value("Storage Boost"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[3].<ResourceType>k__BackingField",
                        enum_value("Wood"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[3].<IntValue>k__BackingField",
                        Value::from(10),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[4].<TechType>k__BackingField",
                        enum_value("Stat Boost"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[4].<PlayerRole>k__BackingField",
                        enum_value("Builder"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[4].<StatType>k__BackingField",
                        enum_value("Action Amount"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[4].<IntValue>k__BackingField",
                        Value::from(20),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[5].<TechType>k__BackingField",
                        enum_value("Age Up Building"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[5].<BuildingType>k__BackingField",
                        enum_value("Townhall"),
                    ),
                    field(
                        "<Unlocks>k__BackingField.Array.data[5].<IntValue>k__BackingField",
                        Value::from(0),
                    ),
                ]);
            }
            fields
        };
        let mut placer = asset(
            "dddddddddddddddddddddddddddddddd",
            BUILDING_PLACER,
            "UnityEngine.GameObject",
            vec![],
        );
        placer.game_object = Some(UnityGameObject {
            components: vec![component(
                "Buildings.BuildingPlacer, Assembly-CSharp",
                vec![
                    field("_buildData.Array.size", Value::from(1)),
                    field(
                        "_buildData.Array.data[0].BuildingType",
                        enum_value("Townhall"),
                    ),
                    field(
                        "_buildData.Array.data[0].Prefab",
                        serde_json::json!({ "Guid": PREFAB_GUID }),
                    ),
                    field("_buildData.Array.data[0].BuildingSize.x", Value::from(8)),
                    field("_buildData.Array.data[0].BuildingSize.y", Value::from(4)),
                ],
            )],
        });
        let mut prefab = asset(
            PREFAB_GUID,
            "Assets/Prefabs/Buildings/Building_Station_TownHall.prefab",
            "UnityEngine.GameObject",
            vec![],
        );
        prefab.game_object = Some(UnityGameObject {
            components: vec![
                component(
                    "Buildings.BuildingBase, Assembly-CSharp",
                    vec![field("_buildingType", enum_value("Townhall"))],
                ),
                component(
                    "Buildings.ResourceStorageModifier, Assembly-CSharp",
                    vec![
                        field("_resource", enum_value("Food")),
                        field("_baseAmount", Value::from(1_000)),
                        field("_incrementAmount", Value::from(2_000)),
                        field("_incrementMultiPerLevel", Value::from(3.0)),
                    ],
                ),
                component(
                    "Buildings.BuildingModelHandler, Assembly-CSharp",
                    vec![
                        field(
                            "FullModel",
                            serde_json::json!({ "Name": "Age01_TownHall_Base" }),
                        ),
                        field(
                            "Stage1",
                            serde_json::json!({ "Name": "Age01_TownHall_Stage_01" }),
                        ),
                        field(
                            "Stage2",
                            serde_json::json!({ "Name": "Age01_TownHall_Stage_02" }),
                        ),
                        field(
                            "Stage3",
                            serde_json::json!({ "Name": "Age01_TownHall_Stage_03" }),
                        ),
                        field("Upgrades.Array.size", Value::from(1)),
                        field(
                            "Upgrades.Array.data[0]",
                            serde_json::json!({ "Name": "Age01_TownHall_Upgrade_01" }),
                        ),
                        field("OtherModels.Array.size", Value::from(0)),
                    ],
                ),
                component(
                    "Buildings.BuildingResourceModelHandler, Assembly-CSharp",
                    vec![
                        field(
                            "EmptyModel",
                            serde_json::json!({ "Name": "Age01_TownHall_Empty" }),
                        ),
                        field(
                            "HalfFullModel",
                            serde_json::json!({ "Name": "Age01_TownHall_Half" }),
                        ),
                        field(
                            "FullModel",
                            serde_json::json!({ "Name": "Age01_TownHall_Full" }),
                        ),
                    ],
                ),
                component_at(
                    "Age01_TownHall/Age01_TownHall_Base/Age01_TownHall_Sign",
                    "Utils.SimpleRotateOnAxis, Assembly-CSharp",
                    vec![
                        field("_axis", serde_json::json!({ "x": 0.0, "y": 1.0, "z": 0.0 })),
                        field("_speed", Value::from(35.0)),
                    ],
                ),
                component(
                    "GameResources.PassiveResourceIncrementer, Assembly-CSharp",
                    vec![
                        field("_resource", enum_value("Gold")),
                        field("_amountPerSecond", Value::from(0.5)),
                        field("_amountPerLevel", Value::from(0.25)),
                    ],
                ),
                component(
                    "Buildings.BuildingLevelHandler, Assembly-CSharp",
                    vec![
                        field(
                            "_onLevelUp.m_PersistentCalls.m_Calls.Array.data[0].m_MethodName",
                            Value::String("OnLevelUp".to_owned()),
                        ),
                        field(
                            "_onLevelUp.m_PersistentCalls.m_Calls.Array.data[1].m_MethodName",
                            Value::String("OnLevelUp".to_owned()),
                        ),
                    ],
                ),
            ],
        });
        prefab.dependencies = vec![UnityReference {
            path: Some("Assets/Models/Buildings/Age01/Age01_TownHall.fbx".to_owned()),
        }];
        let mut player_prefab = asset(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            PLAYER_PREFAB,
            "UnityEngine.GameObject",
            vec![],
        );
        player_prefab.game_object = Some(UnityGameObject {
            components: vec![component(
                "Character.CharacterModelHandler, Assembly-CSharp",
                vec![field("_equipmentSets.Array.size", Value::from(0))],
            )],
        });
        let mut loader = asset(
            "dddddddddddddddddddddddddddddddd",
            LOADER_SCENE,
            "UnityEditor.SceneAsset",
            vec![],
        );
        loader.scene = Some(UnityScene {
            roots: vec![UnityGameObject {
                components: vec![component_at(
                    "LoadingManager",
                    "UserInterface.MainMenu.LoadingManager, Assembly-CSharp",
                    vec![
                        field("_loadingSpeed", Value::from(0.5)),
                        field("_waitTime", Value::from(0.5)),
                        field("_toolTips.Array.size", Value::from(1)),
                        field(
                            "_toolTips.Array.data[0]",
                            Value::from("This is a tooltip! Isn't that neat?"),
                        ),
                    ],
                )],
            }],
        });
        let export = UnityExport {
            schema_version: 1,
            unity_version: "6000.5.6f1".to_owned(),
            warnings: vec![],
            assets: vec![
                loader,
                placer,
                prefab,
                player_prefab,
                asset(
                    "ffffffffffffffffffffffffffffffff",
                    POOL_SETTINGS,
                    "Scriptables.ObjectPoolingSettings",
                    vec![field("_objectsToPool.Array.size", Value::from(0))],
                ),
                asset(
                    "11111111111111111111111111111111",
                    LAND_FOLIAGE_SETTINGS,
                    "ScriptablesProcessorInfrastructure.FoliageGenSettings",
                    foliage_fixture_fields("_foliageGenerationSettings", "Grass", 0.6),
                ),
                asset(
                    "22222222222222222222222222222222",
                    WATER_FOLIAGE_SETTINGS,
                    "ScriptablesProcessorInfrastructure.WaterFoliageGenSettings",
                    foliage_fixture_fields("_waterFoliageGenerationSettings", "Seaweed", 0.7),
                ),
                asset(
                    "33333333333333333333333333333333",
                    TARGET_SETTINGS,
                    "ScriptablesProcessorInfrastructure.TargetSettings",
                    std::iter::once(field("_targetableData.Array.size", Value::from(13)))
                        .chain(
                            [
                                ("Player", "Clear"),
                                ("Tree", "Update"),
                                ("Ore", "Update"),
                                ("Bush", "Update"),
                                ("Farm", "Update"),
                                ("Fish", "Clear"),
                                ("Enemy", "Clear"),
                                ("Boss", "Clear"),
                                ("Building", "Clear"),
                                ("Damaged Building", "Clear"),
                                ("Construction", "Clear"),
                                ("Injured Player", "Clear"),
                                ("Dead Player", "Clear"),
                            ]
                            .into_iter()
                            .enumerate()
                            .flat_map(|(index, (target, mode))| {
                                [
                                    field(
                                        &format!("_targetableData.Array.data[{index}].TargetType"),
                                        enum_value(target),
                                    ),
                                    field(
                                        &format!("_targetableData.Array.data[{index}].UpdateType"),
                                        enum_value(mode),
                                    ),
                                ]
                            }),
                        )
                        .collect(),
                ),
                asset(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    BUILDING_CONTAINER,
                    "Container",
                    vec![field(
                        "BuildingData.Array.data[0]",
                        reference_value(BUILDING_GUID, BUILDING_TYPE),
                    )],
                ),
                asset(
                    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                    ROLE_CONTAINER,
                    "Container",
                    vec![field(
                        "RoleData.Array.data[0]",
                        reference_value(ROLE_GUID, ROLE_TYPE),
                    )],
                ),
                asset(
                    "cccccccccccccccccccccccccccccccc",
                    TECH_TREE,
                    "TechTree.ScriptableObjects.TechTree_SO",
                    vec![
                        field(
                            "<NodeGroups>k__BackingField.list.Array.data[0].Value.Array.data[0]",
                            reference_value(ROOT_GUID, TECH_NODE_TYPE),
                        ),
                        field(
                            "<NodeGroups>k__BackingField.list.Array.data[0].Value.Array.data[1]",
                            reference_value(CHILD_GUID, TECH_NODE_TYPE),
                        ),
                    ],
                ),
                asset(
                    BUILDING_GUID,
                    "Assets/BuildingData/BuildingData_TownHall.asset",
                    BUILDING_TYPE,
                    vec![
                        field("BuildingName", Value::String("Town Hall".to_owned())),
                        field("BuildingType", enum_value("Townhall")),
                        field("BuildResourceCost.WoodCost", Value::from(100)),
                        field("BuildResourceCost.OreCost", Value::from(50)),
                        field("BuildResourceCost.FoodCost", Value::from(0)),
                        field("BuildResourceCost.GoldCost", Value::from(10)),
                        field("CanLevel", Value::Bool(true)),
                        field("Placeable", Value::Bool(true)),
                        field("LevelResourceCost.WoodCost", Value::from(80)),
                        field("LevelResourceCost.OreCost", Value::from(40)),
                        field("LevelResourceCost.FoodCost", Value::from(0)),
                        field("LevelResourceCost.GoldCost", Value::from(8)),
                        field("CostIncreasePerLevelMultiplier", Value::from(2.0)),
                    ],
                ),
                asset(
                    ROLE_GUID,
                    "Assets/RoleData/RoleData_Builder.asset",
                    ROLE_TYPE,
                    vec![
                        field("Role", enum_value("Builder")),
                        field("ActionAnimationName", enum_value("Build")),
                        field("ActionAnimationVariants", Value::from(1)),
                        field("Resource", enum_value("None")),
                        field("BaseActionAmount", Value::from(1)),
                        field("ActionAmountPerLevel", Value::from(0.25)),
                        field("BaseActionSpeed", Value::from(1.0)),
                        field("ActionSpeedPerLevel", Value::from(0.005)),
                        field("BaseActionRange", Value::from(1.0)),
                        field("ActionRangePerLevel", Value::from(0.0)),
                        field("BaseHealth", Value::from(100)),
                        field("HealthPerLevel", Value::from(2.0)),
                        field("BaseHealthRegen", Value::from(0)),
                        field("HealthRegenPerLevel", Value::from(0.001)),
                        field("BaseDamageReduction", Value::from(0)),
                        field("DamageReductionPerLevel", Value::from(0.005)),
                        field("BaseMovementSpeed", Value::from(3)),
                        field("MovementSpeedPerLevel", Value::from(0.05)),
                        field("BaseMaxResource", Value::from(0)),
                        field("MaxResourcePerLevel", Value::from(2.0)),
                        field("HasUserLimit", Value::Bool(false)),
                        field("BaseMaxUserLimit", Value::from(0)),
                        field("ExpModifier", Value::from(3.0)),
                        field("StationFlags", enum_value("Buildings")),
                    ],
                ),
                asset(
                    ROOT_GUID,
                    "Assets/Resources/TechTree/Technologies/TechTreeV2/Groups/Core/Technologies/Root.asset",
                    TECH_NODE_TYPE,
                    technology_fields("Root", Some(CHILD_GUID)),
                ),
                asset(
                    CHILD_GUID,
                    "Assets/Resources/TechTree/Technologies/TechTreeV2/Groups/Core/Technologies/Child.asset",
                    TECH_NODE_TYPE,
                    technology_fields("Child", None),
                ),
            ],
        };

        let (catalog, report) = convert_export(&export, "fixture-sha".to_owned()).unwrap();
        assert_eq!(report.source_assets, 15);
        assert_eq!(report.archetypes, 3);
        assert_eq!(report.archetype_scenes, 1);
        assert_eq!(report.disable_after_time_prefabs, 0);
        assert_eq!(report.foliage_layers, 2);
        assert_eq!(report.foliage_variants, 2);
        assert_eq!(report.buildings, 1);
        assert_eq!(report.building_prefabs, 1);
        assert_eq!(report.building_model_handlers, 1);
        assert_eq!(report.storage_model_handlers, 1);
        assert_eq!(report.rotating_nodes, 1);
        assert_eq!(report.passive_resource_generators, 1);
        assert_eq!(report.roles, 1);
        assert_eq!(report.technology_nodes, 2);
        assert_eq!(report.technology_edges, 1);
        assert_eq!(report.technology_roots, 1);
        assert_eq!(report.objectives, 1);
        assert_eq!(report.station_target_update_modes, 13);
        assert_eq!(catalog.source_records.len(), 4);
        let builder = StableId::new("role:builder").unwrap();
        assert_eq!(catalog.roles[&builder].base_action_amount, 1);
        assert_eq!(catalog.roles[&builder].action_animation, "Build");
        assert_eq!(catalog.roles[&builder].action_animation_variants, 1);
        assert!(!catalog.roles[&builder].has_user_limit);
        assert_eq!(catalog.roles[&builder].base_max_users, 0);
        assert_eq!(catalog.roles[&builder].base_action_milliseconds, 1_000);
        assert_eq!(catalog.roles[&builder].base_health, 100);
        assert_eq!(
            catalog.roles[&builder].experience_multiplier_per_thousand,
            3_000
        );
        assert_eq!(catalog.roles[&builder].action_amount_per_level_milli, 250);
        assert_eq!(
            catalog.roles[&builder].movement_speed_milli_cells_per_second_per_level,
            25
        );
        assert_eq!(
            catalog.roles[&builder].base_movement_speed_milli_cells_per_second,
            1_500
        );
        assert_eq!(catalog.roles[&builder].base_action_range_milli_cells, 500);
        let town_hall = StableId::new("building:townhall").unwrap();
        assert_eq!(catalog.buildings[&town_hall].footprint, [4, 2]);
        assert!(catalog.buildings[&town_hall].can_level);
        assert_eq!(
            catalog.buildings[&town_hall].storage,
            vec![StorageContribution {
                resource: StableId::new("resource:food").unwrap(),
                base_amount: 1_000,
                increment_amount: 2_000,
                level_multiplier_per_thousand: 3_000,
            }]
        );
        assert_eq!(
            catalog.buildings[&town_hall].passive_resources,
            vec![PassiveResourceContribution {
                resource: StableId::new("resource:gold").unwrap(),
                base_milli_per_second: 500,
                increment_milli_per_level: 250,
                level_event_repetitions: 2,
            }]
        );
        assert_eq!(
            catalog.buildings[&town_hall].model_handlers,
            vec![BuildingModelDef {
                age: 1,
                full_model: "Age01_TownHall_Base".to_owned(),
                construction_stages: [
                    "Age01_TownHall_Stage_01".to_owned(),
                    "Age01_TownHall_Stage_02".to_owned(),
                    "Age01_TownHall_Stage_03".to_owned(),
                ],
                upgrades: vec!["Age01_TownHall_Upgrade_01".to_owned()],
                other_models: vec![],
            }]
        );
        assert_eq!(
            catalog.buildings[&town_hall].storage_models,
            vec![StorageModelDef {
                age: 1,
                resource: StableId::new("resource:food").unwrap(),
                empty_model: "Age01_TownHall_Empty".to_owned(),
                half_full_model: "Age01_TownHall_Half".to_owned(),
                full_model: "Age01_TownHall_Full".to_owned(),
            }]
        );
        let town_hall_archetype = &catalog.archetypes[&catalog.buildings[&town_hall].archetype];
        assert_eq!(
            town_hall_archetype.rotating_nodes,
            vec![RotatingNodeDef {
                hierarchy_path: "Age01_TownHall/Age01_TownHall_Base/Age01_TownHall_Sign".to_owned(),
                age: Some(1),
                node: "Age01_TownHall_Sign".to_owned(),
                axis: [0.0, 1.0, 0.0],
                degrees_per_second: 35.0,
            }]
        );
        assert_eq!(
            catalog.buildings[&town_hall].level_cost_multiplier_per_thousand,
            2_000
        );
        let archetype = &catalog.archetypes[&catalog.buildings[&town_hall].archetype];
        assert_eq!(archetype.scenes[0].age, Some(1));
        assert!(archetype.scenes[0].is_default);
        let root = StableId::new(format!("tech:{ROOT_GUID}")).unwrap();
        let child = StableId::new(format!("tech:{CHILD_GUID}")).unwrap();
        assert_eq!(
            catalog.technology.nodes[&root].building_level_caps[&town_hall],
            3
        );
        assert!(
            catalog.technology.nodes[&root]
                .unlocked_buildings
                .contains(&town_hall)
        );
        let root_node = &catalog.technology.nodes[&root];
        assert_eq!(root_node.building_cost_reduction_percent[&town_hall], 5);
        assert_eq!(
            root_node.storage_boost_percent[&StableId::new("resource:wood").unwrap()],
            10
        );
        assert_eq!(
            root_node.role_stat_boost_percent[&StableId::new("role:builder").unwrap()]
                [&StableId::new("stat:action_amount").unwrap()],
            20
        );
        assert!(root_node.aged_buildings.contains(&town_hall));
        assert_eq!(catalog.technology.nodes[&child].prerequisites, vec![root]);
        let objective = &catalog.objectives[&catalog.technology.nodes[&child].objectives[0]];
        assert_eq!(objective.kind, ObjectiveKind::Collect);
        assert_eq!(objective.required_amount, 25);
        assert_eq!(
            objective.resource,
            Some(StableId::new("resource:wood").unwrap())
        );
        catalog.validate().unwrap();

        let encoded = ron::to_string(&catalog).unwrap();
        let decoded: ContentCatalog = ron::from_str(&encoded).unwrap();
        assert_eq!(decoded, catalog);
    }
}
