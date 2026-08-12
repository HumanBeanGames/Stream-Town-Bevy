#import bevy_pbr::{
    forward_io::{FragmentOutput, Vertex, VertexOutput},
    mesh_functions,
    mesh_view_bindings as view_bindings,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
    view_transformations::position_world_to_clip,
}

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

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
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
        view_bindings::globals.time
            + (source_world_position.x + source_world_position.z) * sync,
    );
    let detail_uv = fract(
        (vertex.position.xy + view_bindings::globals.time * direction) * detail_scale
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

fn summer_color(seed: f32) -> vec3<f32> {
    let deep = vec3<f32>(0.2361, 0.3302, 0.0950);
    let teal = vec3<f32>(0.0981, 0.3302, 0.1755);
    let olive = vec3<f32>(0.4314, 0.4063, 0.1098);
    let light = vec3<f32>(0.2980, 0.4471, 0.1059);
    if seed < 0.35 {
        return mix(deep, teal, seed / 0.35);
    }
    if seed < 0.68 {
        return mix(teal, olive, (seed - 0.35) / 0.33);
    }
    return mix(olive, light, (seed - 0.68) / 0.32);
}

fn autumn_color(seed: f32) -> vec3<f32> {
    let brown = vec3<f32>(0.6698, 0.3855, 0.1864);
    let ochre = vec3<f32>(0.6706, 0.5091, 0.1882);
    let red = vec3<f32>(0.6706, 0.1882, 0.2442);
    let olive = vec3<f32>(0.4528, 0.3687, 0.1559);
    if seed < 0.36 {
        return mix(brown, ochre, seed / 0.36);
    }
    if seed < 0.70 {
        return mix(ochre, red, (seed - 0.36) / 0.34);
    }
    return mix(red, olive, (seed - 0.70) / 0.30);
}

fn spring_color(seed: f32) -> vec3<f32> {
    let green = vec3<f32>(0.1904, 0.5890, 0.0593);
    let yellow = vec3<f32>(0.4370, 0.5040, 0.0941);
    let blossom = vec3<f32>(0.7830, 0.1440, 0.4003);
    if seed < 0.47 {
        return mix(green, yellow, seed / 0.47);
    }
    return mix(yellow, blossom, (seed - 0.47) / 0.53);
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    var uv = fract(
        in.world_position.xz * 0.01 * tree_material.main_scale_offset.xy
            + tree_material.main_scale_offset.zw,
    );
#ifdef VERTEX_UVS_A
    uv = in.uv * tree_material.main_scale_offset.xy
        + tree_material.main_scale_offset.zw;
#endif
    var vertex_color = vec4<f32>(1.0, 1.0, 0.0, 1.0);
#ifdef VERTEX_COLORS
    vertex_color = in.color;
#endif
    let main_sample = textureSample(main_texture, main_sampler, uv);
    let cell = floor(in.world_position.xz / 32.0);
    let seed = fract(sin(dot(cell + vec2<f32>(1.0), vec2<f32>(12.9898, 78.233))) * 43758.55);
    let summer = mix(main_sample.rgb, summer_color(seed), 0.3 * (1.0 - vertex_color.b));
    let autumn = mix(autumn_color(seed), main_sample.rgb, vertex_color.b);
    var authored_color = mix(summer, autumn, tree_material.season_controls.x);
    let snow_target = mix(authored_color, vec3<f32>(1.0), vertex_color.g);
    authored_color = mix(
        authored_color,
        snow_target,
        clamp(tree_material.season_controls.y * max(in.world_normal.y, 0.0), 0.0, 1.0),
    );
    authored_color = mix(
        authored_color,
        spring_color(seed),
        tree_material.season_controls.z * (1.0 - vertex_color.b),
    );
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        vec4<f32>(authored_color, main_sample.a),
    );
    pbr_input.material.metallic = 0.0;
    pbr_input.material.perceptual_roughness = 1.0;

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
