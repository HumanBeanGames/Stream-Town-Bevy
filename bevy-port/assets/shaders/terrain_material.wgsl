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
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> terrain_material: TerrainMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var grid_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var grid_sampler: sampler;

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
    pbr_input.material.base_color = vec4<f32>(
        authored.rgb * terrain_material.season_tint.rgb,
        1.0,
    );
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color,
    );

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
