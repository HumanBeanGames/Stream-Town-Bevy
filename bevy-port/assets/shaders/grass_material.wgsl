#import bevy_pbr::{
    forward_io::{FragmentOutput, Vertex, VertexOutput},
    mesh_functions,
    mesh_view_bindings as view_bindings,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    view_transformations::position_world_to_clip,
}

struct GrassMaterialUniform {
    grid_color_1: vec4<f32>,
    grid_color_2: vec4<f32>,
    wind_color: vec4<f32>,
    wind_direction_smoothness: vec4<f32>,
    wind_controls: vec4<f32>,
    surface_controls: vec4<f32>,
    world_strength_transform: vec4<f32>,
    main_scale_offset: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> grass_material: GrassMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var main_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var main_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(103)
var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(104)
var noise_sampler: sampler;

fn wind_uv(world_position: vec3<f32>) -> vec2<f32> {
    let direction = grass_material.wind_direction_smoothness.xy;
    let crawl_speed = grass_material.wind_controls.x;
    let texture_size = max(abs(grass_material.wind_controls.w), 0.0001);
    return world_position.xz / texture_size
        + view_bindings::globals.time * crawl_speed * direction;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    var local_position = vertex.position;
    let source_world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(local_position, 1.0),
    );
    var vertex_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
#ifdef VERTEX_COLORS
    vertex_color = vertex.color;
#endif
    let vertex_mask_max = max(abs(grass_material.surface_controls.w), 0.0001);
    let vertex_mask = smoothstep(0.0, vertex_mask_max, 1.0 - vertex_color.b);
    let panner = wind_uv(source_world_position.xyz);
    let wind_sample = textureSampleLevel(main_texture, main_sampler, fract(panner), 0.0).a;
    let wind_texture = smoothstep(
        grass_material.wind_direction_smoothness.z,
        grass_material.wind_direction_smoothness.w,
        wind_sample,
    );
    var local_uv = vec2<f32>(0.0);
#ifdef VERTEX_UVS_A
    local_uv = vertex.uv;
#endif
    let noise_uv = local_uv * grass_material.wind_controls.y + panner;
    let noise = textureSampleLevel(noise_texture, noise_sampler, fract(noise_uv), 0.0).a;
    let noise_displacement = vec3<f32>(noise * 2.0, 0.0, noise * 2.0);
    let displacement = mix(
        wind_texture * noise_displacement,
        noise_displacement,
        wind_texture,
    ) * vertex_mask * grass_material.wind_controls.z * 0.05;
    local_position += displacement;

#ifdef VERTEX_NORMALS
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        vertex.instance_index,
    );
#endif
#ifdef VERTEX_POSITIONS
    out.world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(local_position, 1.0),
    );
    out.position = position_world_to_clip(out.world_position.xyz);
#endif
#ifdef VERTEX_UVS_A
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_UVS_B
    out.uv_b = vertex.uv_b;
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        vertex.instance_index,
    );
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index = vertex.instance_index;
#endif
#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex.instance_index,
        world_from_local[3],
    );
#endif
    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    var vertex_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
#ifdef VERTEX_COLORS
    vertex_color = in.color;
#endif
    let noise_uv = fract(in.world_position.xz * grass_material.world_strength_transform.xy);
    let grid_noise = textureSample(noise_texture, noise_sampler, noise_uv).g;
    let grid_color = mix(
        grass_material.grid_color_1,
        grass_material.grid_color_2,
        grid_noise,
    );
    let color_blend = clamp(vertex_color.b * grass_material.surface_controls.x, 0.0, 1.0);
    let wind_color = mix(grass_material.wind_color, grid_color, color_blend);
    let wind_sample = textureSample(
        main_texture,
        main_sampler,
        fract(wind_uv(in.world_position.xyz)),
    ).a;
    let wind_texture = smoothstep(
        grass_material.wind_direction_smoothness.z,
        grass_material.wind_direction_smoothness.w,
        wind_sample,
    );
    let animated_color = mix(grid_color, wind_color, wind_texture);
    var main_uv = in.world_position.xz * 0.01;
#ifdef VERTEX_UVS_A
    main_uv = in.uv;
#endif
    main_uv = main_uv * grass_material.main_scale_offset.xy
        + grass_material.main_scale_offset.zw;
    let main_sample = textureSample(main_texture, main_sampler, fract(main_uv));
    let textured_color = mix(animated_color, main_sample, vertex_color.r);
    let spring_color = mix(
        textured_color,
        main_sample,
        vertex_color.g * grass_material.surface_controls.y,
    );
    let winter_noise = smoothstep(
        0.27,
        1.86,
        textureSample(
            noise_texture,
            noise_sampler,
            fract(in.world_position.xz * grass_material.world_strength_transform.zw),
        ).a,
    );
    let authored_color = mix(
        spring_color,
        vec4<f32>(vec3<f32>(winter_noise), winter_noise),
        grass_material.surface_controls.z,
    );
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        vec4<f32>(authored_color.rgb, 1.0),
    );
    pbr_input.material.metallic = 0.0;
    pbr_input.material.perceptual_roughness = 1.0;

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
