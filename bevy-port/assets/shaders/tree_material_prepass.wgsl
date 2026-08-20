#import bevy_pbr::{
    prepass_io::{Vertex, VertexOutput},
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

struct TreeMaterialUniform {
    wind_direction_smoothness: vec4<f32>,
    wind_controls: vec4<f32>,
    season_controls: vec4<f32>,
    main_scale_offset: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> tree_material: TreeMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var main_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var main_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(103)
var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(104)
var noise_sampler: sampler;

fn deformed_tree_position(
    vertex: Vertex,
    world_from_local: mat4x4<f32>,
    animation_time: f32,
) -> vec3<f32> {
    var local_position = vertex.position;
    let source_world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(local_position, 1.0),
    );
    let direction = tree_material.wind_direction_smoothness.xy;
    let sync = tree_material.wind_controls.x;
    let wind_strength = tree_material.wind_controls.y;
    let detail_strength = tree_material.wind_controls.z;
    let detail_scale = tree_material.wind_controls.w;
    var wind_weight = 1.0;
#ifdef VERTEX_COLORS
    wind_weight = vertex.color.r;
#endif
    let gust = sin(
        animation_time + (source_world_position.x + source_world_position.z) * sync,
    );
    let detail_uv = fract(
        (vertex.position.xy + animation_time * direction) * detail_scale
            + vec2<f32>(detail_scale),
    );
    let detail_noise = textureSampleLevel(noise_texture, noise_sampler, detail_uv, 0.0).a;
    let detail = smoothstep(
        tree_material.wind_direction_smoothness.z,
        tree_material.wind_direction_smoothness.w,
        detail_noise,
    );
    let displacement = vec3<f32>(direction.x, 0.0, direction.y) * gust * wind_strength
        + vec3<f32>(detail_strength * detail);
    local_position += displacement * wind_weight;
    return local_position;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let local_position = deformed_tree_position(
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
    let previous_local_position = deformed_tree_position(
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
