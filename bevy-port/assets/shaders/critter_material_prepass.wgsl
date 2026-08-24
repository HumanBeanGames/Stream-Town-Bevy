#import bevy_pbr::{
    prepass_io::{Vertex, VertexOutput},
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import bevy_render::globals::Globals

struct CritterMaterialUniform {
    animation_controls: vec4<f32>,
    main_scale_offset: vec4<f32>,
}

@group(0) @binding(1) var<uniform> globals: Globals;
@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> critter_material: CritterMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var main_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var main_sampler: sampler;

fn deformed_critter_position(vertex: Vertex, animation_time: f32) -> vec3<f32> {
    var mask_uv = vec2<f32>(0.0);
#ifdef VERTEX_UVS_B
    mask_uv = vertex.uv_b;
#else
#ifdef VERTEX_UVS_A
    mask_uv = vertex.uv;
#endif
#endif
    mask_uv = mask_uv * critter_material.main_scale_offset.xy
        + critter_material.main_scale_offset.zw;
    let mask = textureSampleLevel(main_texture, main_sampler, fract(mask_uv), 0.0).r;
    var local_position = vertex.position;
    local_position.x += mask * sin(
        animation_time * critter_material.animation_controls.x
            + local_position.x * critter_material.animation_controls.y,
    ) * critter_material.animation_controls.z;
    return local_position;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let local_position = deformed_critter_position(vertex, globals.time);
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
    let previous_local_position = deformed_critter_position(
        vertex,
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
