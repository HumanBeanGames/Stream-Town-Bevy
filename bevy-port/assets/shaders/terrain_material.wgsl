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
    path_grid: vec4<f32>,
    traversal_dirt_color: vec4<f32>,
    constructed_path_color: vec4<f32>,
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
@group(#{MATERIAL_BIND_GROUP}) @binding(105)
var path_surface_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(106)
var path_surface_sampler: sampler;

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
    let worn_color = mix(
        authored.rgb,
        terrain_material.traversal_dirt_color.rgb,
        wear * terrain_material.traversal_dirt_color.a,
    );
    let path_cell = in.world_position.xz / terrain_material.path_grid.z
        + (terrain_material.path_grid.xy - vec2<f32>(1.0)) * 0.5;
    let path_uv = (path_cell + vec2<f32>(0.5)) / terrain_material.path_grid.xy;
    let sampled_path_level = textureSample(
        path_surface_texture,
        path_surface_sampler,
        path_uv,
    ).r * 255.0 * terrain_material.path_grid.w;
    let path_dimensions = vec2<i32>(textureDimensions(path_surface_texture));
    let path_index = vec2<i32>(floor(path_cell + vec2<f32>(0.5)));
    let path_local = path_cell + vec2<f32>(0.5) - vec2<f32>(path_index);
    let path_maximum = path_dimensions - vec2<i32>(1);
    let left_level = textureLoad(
        path_surface_texture,
        clamp(path_index + vec2<i32>(-1, 0), vec2<i32>(0), path_maximum),
        0,
    ).r * 255.0 * terrain_material.path_grid.w;
    let right_level = textureLoad(
        path_surface_texture,
        clamp(path_index + vec2<i32>(1, 0), vec2<i32>(0), path_maximum),
        0,
    ).r * 255.0 * terrain_material.path_grid.w;
    let down_level = textureLoad(
        path_surface_texture,
        clamp(path_index + vec2<i32>(0, -1), vec2<i32>(0), path_maximum),
        0,
    ).r * 255.0 * terrain_material.path_grid.w;
    let up_level = textureLoad(
        path_surface_texture,
        clamp(path_index + vec2<i32>(0, 1), vec2<i32>(0), path_maximum),
        0,
    ).r * 255.0 * terrain_material.path_grid.w;
    // Diagonal A* steps touch at a corner. Round the two adjacent empty-cell
    // corners into a narrow bridge so diagonal routes read as one continuous
    // cobbled path instead of a chain of point-touching squares.
    let corner_radius = 0.36;
    let lower_left = select(
        0.0,
        1.0 - smoothstep(0.0, corner_radius, distance(path_local, vec2<f32>(0.0, 0.0))),
        left_level >= 0.5 && down_level >= 0.5,
    );
    let lower_right = select(
        0.0,
        1.0 - smoothstep(0.0, corner_radius, distance(path_local, vec2<f32>(1.0, 0.0))),
        right_level >= 0.5 && down_level >= 0.5,
    );
    let upper_left = select(
        0.0,
        1.0 - smoothstep(0.0, corner_radius, distance(path_local, vec2<f32>(0.0, 1.0))),
        left_level >= 0.5 && up_level >= 0.5,
    );
    let upper_right = select(
        0.0,
        1.0 - smoothstep(0.0, corner_radius, distance(path_local, vec2<f32>(1.0, 1.0))),
        right_level >= 0.5 && up_level >= 0.5,
    );
    let diagonal_bridge = max(max(lower_left, lower_right), max(upper_left, upper_right));
    let neighbouring_level = max(max(left_level, right_level), max(down_level, up_level));
    let path_level = max(sampled_path_level, neighbouring_level * diagonal_bridge);
    let path_mask = max(select(0.0, 1.0, sampled_path_level >= 0.5), diagonal_bridge);
    // Staggered stones and a narrow mortar line are generated directly in
    // terrain space, so paving conforms to slopes without another mesh.
    let stone_scale = 1.8 + min(path_level, 8.0) * 0.06;
    let stone_row = floor(in.world_position.z * stone_scale);
    let odd_row = abs(i32(stone_row)) % 2 == 1;
    let stone_uv = fract(vec2<f32>(
        in.world_position.x * stone_scale + select(0.0, 0.5, odd_row),
        in.world_position.z * stone_scale,
    ));
    let mortar_distance = min(
        min(stone_uv.x, 1.0 - stone_uv.x),
        min(stone_uv.y, 1.0 - stone_uv.y),
    );
    let mortar = smoothstep(0.035, 0.075, mortar_distance);
    let stone_noise = textureSample(grid_texture, grid_sampler, grid_uv * 0.63).r;
    let cobble = terrain_material.constructed_path_color.rgb
        * mix(0.64, mix(0.88, 1.08, stone_noise), mortar);
    let surfaced_color = mix(
        worn_color,
        cobble,
        path_mask * terrain_material.constructed_path_color.a,
    );
    let terrain_color = vec4<f32>(
        surfaced_color * terrain_material.season_tint.rgb,
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
