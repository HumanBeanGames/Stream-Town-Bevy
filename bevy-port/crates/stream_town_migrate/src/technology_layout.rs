use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{Context, Result, bail};
use serde::Serialize;
use stream_town_domain::{
    ContentCatalog, GraphPoint, GraphSize, TechnologyGraphLayout, TechnologyGroupLayout,
    TechnologyNodeLayout,
};

const NODE_WIDTH: f32 = 220.0;
const NODE_HEIGHT: f32 = 76.0;
const GROUP_PADDING: f32 = 54.0;
const GROUP_HEADER: f32 = 42.0;

#[derive(Clone, Debug, Serialize)]
pub struct TechnologyLayoutConversionReport {
    pub schema_version: u32,
    pub source_groups: usize,
    pub source_nodes: usize,
    pub converted_groups: usize,
    pub converted_nodes: usize,
    pub output: String,
}

#[derive(Clone, Debug)]
struct AuthoredGroup {
    name: String,
    position: GraphPoint,
}

#[derive(Clone, Debug)]
struct AuthoredNode {
    name: String,
    position: GraphPoint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Section {
    None,
    Groups,
    Nodes,
}

pub fn convert(
    graph_path: &Path,
    catalog_path: &Path,
    output_path: &Path,
) -> Result<TechnologyLayoutConversionReport> {
    let catalog: ContentCatalog = ron::from_str(
        &fs::read_to_string(catalog_path)
            .with_context(|| format!("failed to read {}", catalog_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", catalog_path.display()))?;
    catalog.validate().context("content catalog is invalid")?;
    let source = fs::read_to_string(graph_path)
        .with_context(|| format!("failed to read {}", graph_path.display()))?;
    let (groups, nodes) = parse_unity_graph(&source)?;
    let layout = build_layout(&catalog, &groups, &nodes)?;
    layout.validate(&catalog.technology)?;

    let parent = output_path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let encoded = ron::ser::to_string_pretty(&layout, ron::ser::PrettyConfig::default())?;
    fs::write(output_path, encoded)
        .with_context(|| format!("failed to write {}", output_path.display()))?;
    let reloaded: TechnologyGraphLayout = ron::from_str(&fs::read_to_string(output_path)?)?;
    reloaded.validate(&catalog.technology)?;
    if reloaded != layout {
        bail!("reloaded technology layout differs from the converted layout");
    }

    Ok(TechnologyLayoutConversionReport {
        schema_version: layout.schema_version,
        source_groups: groups.len(),
        source_nodes: nodes.len(),
        converted_groups: layout.groups.len(),
        converted_nodes: layout.nodes.len(),
        output: output_path.display().to_string(),
    })
}

fn build_layout(
    catalog: &ContentCatalog,
    authored_groups: &[AuthoredGroup],
    authored_nodes: &[AuthoredNode],
) -> Result<TechnologyGraphLayout> {
    let groups_by_name: BTreeMap<_, _> = authored_groups
        .iter()
        .map(|group| (group.name.as_str(), group))
        .collect();
    if groups_by_name.len() != authored_groups.len() {
        bail!("Unity technology graph contains duplicate group names");
    }
    let nodes_by_name: BTreeMap<_, _> = authored_nodes
        .iter()
        .map(|node| (node.name.as_str(), node))
        .collect();
    if nodes_by_name.len() != authored_nodes.len() {
        bail!("Unity technology graph contains duplicate node names");
    }

    let mut layout = TechnologyGraphLayout::default();
    for (id, node) in &catalog.technology.nodes {
        let authored = nodes_by_name
            .get(node.display_name.as_str())
            .with_context(|| {
                format!(
                    "catalog technology {id} ({}) is absent from the Unity graph",
                    node.display_name
                )
            })?;
        layout.nodes.insert(
            id.clone(),
            TechnologyNodeLayout {
                position: authored.position,
            },
        );
    }

    for (group_id, group) in &catalog.technology.groups {
        let authored = groups_by_name
            .get(group.display_name.as_str())
            .with_context(|| {
                format!(
                    "catalog technology group {group_id} ({}) is absent from the Unity graph",
                    group.display_name
                )
            })?;
        let mut left = authored.position.x;
        let mut top = authored.position.y;
        let mut right = left + GraphSize::default().width;
        let mut bottom = top + GraphSize::default().height;
        for node_id in &group.nodes {
            let position = layout.nodes[node_id].position;
            left = left.min(position.x - GROUP_PADDING);
            top = top.min(position.y - GROUP_HEADER);
            right = right.max(position.x + NODE_WIDTH + GROUP_PADDING);
            bottom = bottom.max(position.y + NODE_HEIGHT + GROUP_PADDING);
        }
        layout.groups.insert(
            group_id.clone(),
            TechnologyGroupLayout {
                position: GraphPoint { x: left, y: top },
                size: GraphSize {
                    width: right - left,
                    height: bottom - top,
                },
            },
        );
    }
    Ok(layout)
}

fn parse_unity_graph(source: &str) -> Result<(Vec<AuthoredGroup>, Vec<AuthoredNode>)> {
    let mut section = Section::None;
    let mut groups = Vec::new();
    let mut nodes = Vec::new();
    let mut name = None;
    let mut position = None;

    let flush = |section: Section,
                 name: &mut Option<String>,
                 position: &mut Option<GraphPoint>,
                 groups: &mut Vec<AuthoredGroup>,
                 nodes: &mut Vec<AuthoredNode>|
     -> Result<()> {
        if name.is_none() && position.is_none() {
            return Ok(());
        }
        let name = name
            .take()
            .context("Unity graph record is missing a name")?;
        let position = position
            .take()
            .with_context(|| format!("Unity graph record {name} is missing a position"))?;
        match section {
            Section::Groups => groups.push(AuthoredGroup { name, position }),
            Section::Nodes => nodes.push(AuthoredNode { name, position }),
            Section::None => bail!("Unity graph record appeared before a section"),
        }
        Ok(())
    };

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "<Groups>k__BackingField:" {
            section = Section::Groups;
            continue;
        }
        if trimmed == "<Nodes>k__BackingField:" {
            flush(section, &mut name, &mut position, &mut groups, &mut nodes)?;
            section = Section::Nodes;
            continue;
        }
        if trimmed.starts_with("- <ID>k__BackingField:") {
            flush(section, &mut name, &mut position, &mut groups, &mut nodes)?;
            continue;
        }
        if let Some(value) = trimmed.strip_prefix("<Name>k__BackingField:") {
            name = Some(value.trim().to_owned());
            continue;
        }
        if let Some(value) = trimmed.strip_prefix("<Position>k__BackingField:") {
            position = Some(parse_point(value.trim())?);
        }
    }
    flush(section, &mut name, &mut position, &mut groups, &mut nodes)?;
    if groups.is_empty() || nodes.is_empty() {
        bail!("Unity technology graph has no authored groups or nodes");
    }
    Ok((groups, nodes))
}

fn parse_point(value: &str) -> Result<GraphPoint> {
    let value = value
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .context("Unity graph position must use {x: ..., y: ...} syntax")?;
    let mut x = None;
    let mut y = None;
    for part in value.split(',') {
        let (key, value) = part
            .split_once(':')
            .context("Unity graph position component is malformed")?;
        let coordinate: f32 = value.trim().parse()?;
        match key.trim() {
            "x" => x = Some(coordinate),
            "y" => y = Some(coordinate),
            _ => {}
        }
    }
    Ok(GraphPoint {
        x: x.context("Unity graph position has no x coordinate")?,
        y: y.context("Unity graph position has no y coordinate")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_group_and_node_positions_from_unity_yaml() {
        let source = r"
<Groups>k__BackingField:
- <ID>k__BackingField: group-guid
  <Name>k__BackingField: TownHall
  <Position>k__BackingField: {x: -12.5, y: 42}
<Nodes>k__BackingField:
- <ID>k__BackingField: node-guid
  <Name>k__BackingField: RootTech
  <NodeTitle>k__BackingField: Root
  <Position>k__BackingField: {x: 100, y: 200.25}
";
        let (groups, nodes) = parse_unity_graph(source).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "TownHall");
        assert_eq!(groups[0].position, GraphPoint { x: -12.5, y: 42.0 });
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "RootTech");
        assert_eq!(
            nodes[0].position,
            GraphPoint {
                x: 100.0,
                y: 200.25
            }
        );
    }

    #[test]
    fn checked_in_layout_exactly_matches_the_unity_graph_conversion() {
        let catalog: ContentCatalog =
            ron::from_str(include_str!("../../../assets/content/catalog.ron")).unwrap();
        let (groups, nodes) = parse_unity_graph(include_str!(
            "../../../../Assets/Scripts/TechTree/Editor/Graphs/TechTreeV2Graph.asset"
        ))
        .unwrap();
        let converted = build_layout(&catalog, &groups, &nodes).unwrap();
        let checked_in: TechnologyGraphLayout = ron::from_str(include_str!(
            "../../../assets/content/technology_layout.ron"
        ))
        .unwrap();

        assert_eq!(converted, checked_in);
        checked_in.validate(&catalog.technology).unwrap();
    }
}
