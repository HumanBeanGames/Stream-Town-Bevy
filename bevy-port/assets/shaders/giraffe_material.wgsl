#import bevy_pbr::{
    forward_io::{FragmentOutput, Vertex, VertexOutput},
    mesh_functions,
    mesh_view_bindings as view_bindings,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    skinning,
    view_transformations::position_world_to_clip,
}

struct GiraffeMaterialUniform {
    animation_controls: vec4<f32>,
    mask_controls: vec4<f32>,
    rotation_controls: vec4<f32>,
    main_scale_offset: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> giraffe_material: GiraffeMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var main_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var main_sampler: sampler;

// Preserve Amplify's generated RotateAroundAxis matrix literally. The authored
// axis is intentionally not normalized; doing so changes the original motion.
fn rotate_around_axis(original: vec3<f32>, axis: vec3<f32>, angle: f32) -> vec3<f32> {
    let cosine = cos(angle);
    let sine = sin(angle);
    let t = 1.0 - cosine;
    let m00 = t * axis.x * axis.x + cosine;
    let m01 = t * axis.x * axis.y - sine * axis.z;
    let m02 = t * axis.x * axis.z + sine * axis.y;
    let m10 = t * axis.x * axis.y + sine * axis.z;
    let m11 = t * axis.y * axis.y + cosine;
    let m12 = t * axis.y * axis.z - sine * axis.x;
    let m20 = t * axis.x * axis.z - sine * axis.y;
    let m21 = t * axis.y * axis.z + sine * axis.x;
    let m22 = t * axis.z * axis.z + cosine;
    return vec3<f32>(
        m00 * original.x + m01 * original.y + m02 * original.z,
        m10 * original.x + m11 * original.y + m12 * original.z,
        m20 * original.x + m21 * original.y + m22 * original.z,
    );
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let mesh_world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    var local_position = vertex.position;
    var vertex_color = vec4<f32>(1.0);
#ifdef VERTEX_COLORS
    vertex_color = vertex.color;
#endif

    let time = view_bindings::globals.time;
    let sway = sin(time * giraffe_material.animation_controls.z);
    let head_animation = vec3<f32>(
        local_position.x + sway,
        giraffe_material.animation_controls.x
            + giraffe_material.animation_controls.y
                * sin(time * giraffe_material.animation_controls.w),
        local_position.z + sway,
    );
    let neck_weight = smoothstep(
        giraffe_material.mask_controls.x,
        giraffe_material.mask_controls.y,
        vertex_color.r,
    );
    let animated_head = mix(vec3<f32>(0.0), head_animation, neck_weight);
    let axis = vec3<f32>(
        giraffe_material.rotation_controls.x,
        giraffe_material.rotation_controls.y,
        0.0,
    );
    let rotated_head = rotate_around_axis(
        vec3<f32>(local_position.x, 0.0, local_position.z),
        axis,
        sin(time) * giraffe_material.rotation_controls.z,
    );
    let head_weight = smoothstep(
        giraffe_material.mask_controls.z,
        giraffe_material.mask_controls.w,
        vertex_color.r,
    );
    local_position += mix(vec3<f32>(0.0), animated_head + rotated_head, head_weight);

#ifdef SKINNED
    var world_from_local = skinning::skin_model(
        vertex.joint_indices,
        vertex.joint_weights,
        vertex.instance_index,
    );
#else
    var world_from_local = mesh_world_from_local;
#endif

#ifdef VERTEX_NORMALS
#ifdef SKINNED
    out.world_normal = skinning::skin_normals(world_from_local, vertex.normal);
#else
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        vertex.instance_index,
    );
#endif
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
        mesh_world_from_local[3],
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
    var uv = vec2<f32>(0.0);
#ifdef VERTEX_UVS_A
    uv = in.uv;
#endif
    uv = uv * giraffe_material.main_scale_offset.xy
        + giraffe_material.main_scale_offset.zw;
    let authored_color = textureSample(main_texture, main_sampler, uv);
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
