#import bevy_pbr::{
    forward_io::{FragmentOutput, Vertex, VertexOutput},
    mesh_functions,
    mesh_view_bindings as view_bindings,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    view_transformations::position_world_to_clip,
}

struct FlagMaterialUniform {
    colour_1: vec4<f32>,
    colour_2: vec4<f32>,
    controls: vec4<f32>,
    noise_scale_offset: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> flag_material: FlagMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var noise_sampler: sampler;

fn rotate_around_y(position: vec3<f32>, angle: f32) -> vec3<f32> {
    let cosine = cos(angle);
    let sine = sin(angle);
    return vec3<f32>(
        cosine * position.x + sine * position.z,
        position.y,
        -sine * position.x + cosine * position.z,
    );
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    var local_position = vertex.position;
    var uv = vec2<f32>(0.0);
#ifdef VERTEX_UVS_A
    uv = vertex.uv;
#endif
    var vertex_color = vec4<f32>(1.0);
#ifdef VERTEX_COLORS
    vertex_color = vertex.color;
#endif

    let noise_uv = uv
        * flag_material.controls.x
        * flag_material.noise_scale_offset.xy
        + flag_material.noise_scale_offset.zw
        + vec2<f32>(view_bindings::globals.time * 0.35);
    let noise = textureSampleLevel(noise_texture, noise_sampler, fract(noise_uv), 0.0).r;
    let source = vec3<f32>(local_position.x, 0.0, local_position.y);
    let rotated = rotate_around_y(source, sin(view_bindings::globals.time));
    let animated = mix(vec3<f32>(noise), rotated, flag_material.controls.y);
    let displacement = vec3<f32>(animated.x, animated.z, 0.0)
        * (1.0 - vertex_color.a)
        * flag_material.controls.z;
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
    var vertex_color = vec4<f32>(1.0);
#ifdef VERTEX_COLORS
    vertex_color = in.color;
#endif
    let authored_color = mix(
        flag_material.colour_1,
        flag_material.colour_2,
        vertex_color.r,
    );
    let metal_edge = mix(flag_material.controls.w, 0.0, vertex_color.r);
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        vec4<f32>(authored_color.rgb, 1.0),
    );
    pbr_input.material.metallic = metal_edge;
    pbr_input.material.perceptual_roughness = 1.0 - metal_edge;

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
