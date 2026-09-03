#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct TerrainMaterialUniform {
    sand_color_a: vec4<f32>,
    sand_color_b: vec4<f32>,
    grass_color_a: vec4<f32>,
    grass_color_b: vec4<f32>,
    season_tint: vec4<f32>,
    texture_uv_blend_tint: vec4<f32>,
    grid_scale_offset: vec4<f32>,
    selection_center_extent: vec4<f32>,
    selection_color: vec4<f32>,
    traversal_grid: vec4<f32>,
    traversal_dirt_color: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> terrain_material: TerrainMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var grid_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var grid_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(103)
var traversal_wear_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(104)
var traversal_wear_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    // Terrain.shader uses saturate(WorldPosition.y + _BlendHeight), not a
    // water-relative smoothstep. This is why authored Y=0 land is fully grass
    // while the -3.5m shoreline shelf remains sand.
    let height_blend = clamp(
        in.world_position.y + terrain_material.texture_uv_blend_tint.z,
        0.0,
        1.0,
    );
    let color_a = mix(
        terrain_material.sand_color_a,
        terrain_material.grass_color_a,
        height_blend,
    );
    let color_b = mix(
        terrain_material.sand_color_b,
        terrain_material.grass_color_b,
        height_blend,
    );
    let grid_uv = fract(
        in.world_position.xz
            * terrain_material.texture_uv_blend_tint.xy
            * terrain_material.grid_scale_offset.xy
            + terrain_material.grid_scale_offset.zw,
    );
    let grid = textureSample(grid_texture, grid_sampler, grid_uv).g;
    let authored_grid = mix(color_a, color_b, grid);
    let broad_noise = textureSample(
        grid_texture,
        grid_sampler,
        fract(in.world_position.xz * vec2<f32>(0.001, 0.001)),
    ).a;
    let authored = mix(
        authored_grid,
        vec4<f32>(vec3<f32>(smoothstep(0.27, 1.86, broad_noise)), 1.0),
        terrain_material.season_tint.w,
    );
    let wear_cell = in.world_position.xz / terrain_material.traversal_grid.z
        + (terrain_material.traversal_grid.xy - vec2<f32>(1.0)) * 0.5;
    let wear_uv = (wear_cell + vec2<f32>(0.5)) / terrain_material.traversal_grid.xy;
    let wear = textureSample(
        traversal_wear_texture,
        traversal_wear_sampler,
        wear_uv,
    ).r * terrain_material.traversal_grid.w;
    let worn_color = mix(authored.rgb, terrain_material.traversal_dirt_color.rgb, wear);
    let terrain_color = vec4<f32>(
        worn_color * terrain_material.season_tint.rgb,
        1.0,
    );
    let selection_delta = abs(
        in.world_position.xz - terrain_material.selection_center_extent.xy,
    );
    let selection_edge_distance = min(
        terrain_material.selection_center_extent.z - selection_delta.x,
        terrain_material.selection_center_extent.w - selection_delta.y,
    );
    let selection_thickness = max(
        min(
            terrain_material.selection_center_extent.z,
            terrain_material.selection_center_extent.w,
        ) * 0.10,
        0.08,
    );
    let selection_aa = max(fwidth(selection_edge_distance), 0.01);
    let selection_inside = smoothstep(
        -selection_aa,
        selection_aa,
        selection_edge_distance,
    );
    let selection_interior = smoothstep(
        selection_thickness - selection_aa,
        selection_thickness + selection_aa,
        selection_edge_distance,
    );
    let selection_active = select(
        0.0,
        1.0,
        terrain_material.selection_center_extent.z > 0.0
            && terrain_material.selection_center_extent.w > 0.0,
    );
    let selection_outline = selection_active
        * selection_inside
        * (1.0 - selection_interior)
        * terrain_material.selection_color.a;
    pbr_input.material.base_color = mix(
        terrain_color,
        vec4<f32>(terrain_material.selection_color.rgb, 1.0),
        selection_outline,
    );
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color,
    );

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    // Unity's baked ambient probe keeps its saturated terrain palette legible
    // in directional-light shadow. Bevy has no equivalent probe in this port,
    // so restore that authored ambient floor without adding specular light.
    out.color = vec4<f32>(
        max(
            out.color.rgb,
            pbr_input.material.base_color.rgb * vec3<f32>(0.36),
        ),
        out.color.a,
    );
    // The selection is part of the terrain pass itself. It cannot z-fight with
    // the terrain, while ordinary world geometry still writes nearer depth and
    // occludes it naturally.
    out.color = vec4<f32>(
        mix(
            out.color.rgb,
            terrain_material.selection_color.rgb,
            selection_outline,
        ),
        out.color.a,
    );
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
