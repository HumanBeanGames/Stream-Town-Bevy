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
    BuildingDef, ContentCatalog, RoleDef, StableId, TechGroup, TechNode, TechTree,
};

const BUILDING_CONTAINER: &str = "Assets/DefaultSettings/D_AllBuildingDataSettings.asset";
const BUILDING_PLACER: &str = "Assets/Prefabs/BuildingPlacer.prefab";
const ROLE_CONTAINER: &str = "Assets/DefaultSettings/D_AllRoleDataSettings.asset";
const TECH_TREE: &str = "Assets/Resources/TechTree/Technologies/TechTreeV2/TechTreeV2.asset";
const BUILDING_TYPE: &str = "ScriptablesProcessorInfrastructure.BuildingDataSettings";
const ROLE_TYPE: &str = "ScriptablesProcessorInfrastructure.RoleDataSettings";
const TECH_NODE_TYPE: &str = "TechTree.ScriptableObjects.Node_SO";

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
    pub buildings: usize,
    pub building_prefabs: usize,
    pub roles: usize,
    pub technology_nodes: usize,
    pub technology_groups: usize,
    pub technology_edges: usize,
    pub technology_roots: usize,
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
struct UnityComponent {
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
    let (archetypes, building_archetypes) = convert_archetypes(export, &placements)?;

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
        for (name, field) in [
            ("wood", "BuildResourceCost.WoodCost"),
            ("ore", "BuildResourceCost.OreCost"),
            ("food", "BuildResourceCost.FoodCost"),
            ("gold", "BuildResourceCost.GoldCost"),
        ] {
            cost.insert(stable_id("resource", name)?, required_u32(asset, field)?);
        }
        buildings.insert(
            id.clone(),
            BuildingDef {
                display_name: building_name,
                archetype: archetype.clone(),
                footprint: *footprint,
                cost,
            },
        );
        insert_source_record(&mut source_records, id, asset)?;
    }

    let mut roles = BTreeMap::new();
    for guid in &role_guids {
        let asset = required_guid_asset(&assets_by_guid, guid, ROLE_TYPE)?;
        let display_name = required_enum(asset, "Role")?;
        let id = stable_id("role", &slug(&display_name))?;
        let base_speed = required_u32(asset, "BaseMovementSpeed")?;
        let movement_speed_multiplier_per_thousand = u16::try_from(base_speed.saturating_mul(1000))
            .with_context(|| format!("{} BaseMovementSpeed is out of range", asset.path))?;
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
                movement_speed_multiplier_per_thousand,
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
    let mut groups = BTreeMap::<StableId, TechGroup>::new();
    for guid in &technology_guids {
        let asset = required_guid_asset(&assets_by_guid, guid, TECH_NODE_TYPE)?;
        let id = node_ids[guid].clone();
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
        let objectives = generated_record_ids(asset, "<Objectives>k__BackingField", "objective")?;
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
                objectives,
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
        schema_version: 2,
        archetypes,
        buildings,
        roles,
        technology: TechTree { nodes, groups },
        source_records,
    };
    catalog.validate().context("converted catalog is invalid")?;

    let report = ContentConversionReport {
        schema_version: 1,
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
        buildings: catalog.buildings.len(),
        building_prefabs: building_archetypes.len(),
        roles: catalog.roles.len(),
        technology_nodes: catalog.technology.nodes.len(),
        technology_groups: catalog.technology.groups.len(),
        technology_edges,
        technology_roots,
        warnings: vec![
            "building footprints use the authored two-unit BuildingPlacer grid; Torch falls back to prefab bounds"
                .to_owned(),
            "prefab archetypes retain spawn-critical component types and converted GLB scene dependencies"
                .to_owned(),
            "detailed Unity building, role, unlock, and objective fields are retained in source_records"
                .to_owned(),
        ],
        outputs: Vec::new(),
    };
    Ok((catalog, report))
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
    let default_index = models
        .iter()
        .position(|path| {
            let path = path.to_ascii_lowercase();
            path.contains("age01") && path.contains("straight")
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

fn required_u32(asset: &UnityAsset, path: &str) -> Result<u32> {
    required_i64(asset, path)?
        .try_into()
        .with_context(|| format!("{} field {path} is outside the u32 range", asset.path))
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

fn technology_group_name(path: &str) -> String {
    path.split_once("/Groups/")
        .and_then(|(_, suffix)| suffix.split('/').next())
        .unwrap_or("Global")
        .to_owned()
}

fn stable_id(prefix: &str, value: &str) -> Result<StableId> {
    StableId::new(format!("{prefix}:{value}")).map_err(Into::into)
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

    fn field(path: &str, value: Value) -> UnityField {
        UnityField {
            path: path.to_owned(),
            value,
        }
    }

    fn reference_value(guid: &str, unity_type: &str) -> Value {
        serde_json::json!({ "Guid": guid, "Type": unity_type })
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
        }
    }

    fn component(unity_type: &str, fields: Vec<UnityField>) -> UnityComponent {
        UnityComponent {
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
                    Value::from(u64::from(name == "Root")),
                ),
                field("<Objectives>k__BackingField.Array.size", Value::from(0)),
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
            components: vec![component(
                "Buildings.BuildingBase, Assembly-CSharp",
                vec![field("_buildingType", enum_value("Townhall"))],
            )],
        });
        prefab.dependencies = vec![UnityReference {
            path: Some("Assets/Models/Buildings/Age01/Age01_TownHall.fbx".to_owned()),
        }];
        let export = UnityExport {
            schema_version: 1,
            unity_version: "6000.5.6f1".to_owned(),
            warnings: vec![],
            assets: vec![
                placer,
                prefab,
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
                    ],
                ),
                asset(
                    ROLE_GUID,
                    "Assets/RoleData/RoleData_Builder.asset",
                    ROLE_TYPE,
                    vec![
                        field("Role", enum_value("Builder")),
                        field("BaseMovementSpeed", Value::from(3)),
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
        assert_eq!(report.source_assets, 9);
        assert_eq!(report.archetypes, 2);
        assert_eq!(report.archetype_scenes, 1);
        assert_eq!(report.buildings, 1);
        assert_eq!(report.building_prefabs, 1);
        assert_eq!(report.roles, 1);
        assert_eq!(report.technology_nodes, 2);
        assert_eq!(report.technology_edges, 1);
        assert_eq!(report.technology_roots, 1);
        assert_eq!(catalog.source_records.len(), 4);
        let town_hall = StableId::new("building:townhall").unwrap();
        assert_eq!(catalog.buildings[&town_hall].footprint, [4, 2]);
        let archetype = &catalog.archetypes[&catalog.buildings[&town_hall].archetype];
        assert_eq!(archetype.scenes[0].age, Some(1));
        assert!(archetype.scenes[0].is_default);
        let root = StableId::new(format!("tech:{ROOT_GUID}")).unwrap();
        let child = StableId::new(format!("tech:{CHILD_GUID}")).unwrap();
        assert_eq!(catalog.technology.nodes[&child].prerequisites, vec![root]);
        catalog.validate().unwrap();

        let encoded = ron::to_string(&catalog).unwrap();
        let decoded: ContentCatalog = ron::from_str(&encoded).unwrap();
        assert_eq!(decoded, catalog);
    }
}
