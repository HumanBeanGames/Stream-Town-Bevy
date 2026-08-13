#import bevy_pbr::{
    forward_io::{Vertex, VertexOutput},
    mesh_functions,
    mesh_view_bindings as view_bindings,
    view_transformations::position_world_to_clip,
}

struct CritterMaterialUniform {
    animation_controls: vec4<f32>,
    main_scale_offset: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> critter_material: CritterMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var main_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var main_sampler: sampler;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    var local_position = vertex.position;
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
    let speed = critter_material.animation_controls.x;
    let sync = critter_material.animation_controls.y;
    let stretch = critter_material.animation_controls.z;
    local_position.x += mask * sin(view_bindings::globals.time * speed + local_position.x * sync)
        * stretch;

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
