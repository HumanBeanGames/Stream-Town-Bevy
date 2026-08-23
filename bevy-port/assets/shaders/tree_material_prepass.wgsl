#import bevy_pbr::{
    prepass_io::{Vertex, VertexOutput},
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import bevy_render::globals::Globals
#import "shaders/tree_wind.wgsl"::{stream_town_tree_deformed_position}

@group(0) @binding(1) var<uniform> globals: Globals;

fn deformed_tree_position(
    vertex: Vertex,
    world_from_local: mat4x4<f32>,
    animation_time: f32,
) -> vec3<f32> {
    let source_world_position = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(vertex.position, 1.0),
    );
    var vertex_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
#ifdef VERTEX_COLORS
    vertex_color = vertex.color;
#endif
    return stream_town_tree_deformed_position(
        vertex.position,
        source_world_position.xyz,
        vertex_color,
        animation_time,
    );
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let local_position = deformed_tree_position(vertex, world_from_local, globals.time);
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
