#import bevy_pbr::{
    prepass_io::{Vertex, VertexOutput},
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

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

fn grass_wind_uv(world_position: vec3<f32>, animation_time: f32) -> vec2<f32> {
    let direction = grass_material.wind_direction_smoothness.xy;
    let crawl_speed = grass_material.wind_controls.x;
    let texture_size = max(abs(grass_material.wind_controls.w), 0.0001);
    return world_position.xz / texture_size + animation_time * crawl_speed * direction;
}

fn deformed_grass_position(
    vertex: Vertex,
    world_from_local: mat4x4<f32>,
    animation_time: f32,
) -> vec3<f32> {
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
    let panner = grass_wind_uv(source_world_position.xyz, animation_time);
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
    return local_position;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let local_position = deformed_grass_position(
        vertex,
        world_from_local,
        globals.time,
    );
    out.world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(local_position, 1.0),
    );
    out.position = position_world_to_clip(out.world_position.xyz);

#ifdef UNCLIPPED_DEPTH_ORTHO_EMULATION
    out.unclipped_depth = out.position.z;
    out.position.z = min(out.position.z, 1.0);
#endif
#ifdef VERTEX_UVS_A
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_UVS_B
    out.uv_b = vertex.uv_b;
#endif
#ifdef NORMAL_PREPASS_OR_DEFERRED_PREPASS
#ifdef VERTEX_NORMALS
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        vertex.instance_index,
    );
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        vertex.instance_index,
    );
#endif
#endif
#ifdef MOTION_VECTOR_PREPASS
    let previous_world_from_local =
        mesh_functions::get_previous_world_from_local(vertex.instance_index);
    let previous_local_position = deformed_grass_position(
        vertex,
        previous_world_from_local,
        globals.time - globals.delta_time,
    );
    out.previous_world_position = mesh_functions::mesh_position_local_to_world(
        previous_world_from_local,
        vec4<f32>(previous_local_position, 1.0),
    );
#endif
#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index = vertex.instance_index;
#endif
#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif
#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex.instance_index,
        world_from_local[3],
    );
#endif
    return out;
}
