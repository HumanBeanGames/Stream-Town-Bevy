#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct BuildingMaterialUniform {
    detail_color: vec4<f32>,
    emissive_color: vec4<f32>,
    ambient_occlusion: vec4<f32>,
    surface_controls: vec4<f32>,
    snow_damage: vec4<f32>,
    main_scale_offset: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> building_material: BuildingMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var main_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var main_sampler: sampler;

fn roof_color(seed: f32) -> vec3<f32> {
    let brown = vec3<f32>(0.6706, 0.3855, 0.1864);
    let ochre = vec3<f32>(0.6706, 0.5091, 0.1882);
    let red = vec3<f32>(0.6706, 0.1882, 0.2442);
    let olive = vec3<f32>(0.4151, 0.3516, 0.1351);
    if seed < 0.34 {
        return mix(brown, ochre, seed / 0.34);
    }
    if seed < 0.68 {
        return mix(ochre, red, (seed - 0.34) / 0.34);
    }
    return mix(red, olive, (seed - 0.68) / 0.32);
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    var uv = fract(
        in.world_position.xz * 0.01 * building_material.main_scale_offset.xy
            + building_material.main_scale_offset.zw,
    );
#ifdef VERTEX_UVS_A
    uv = in.uv * building_material.main_scale_offset.xy
        + building_material.main_scale_offset.zw;
#endif
    var detail_uv = uv;
#ifdef VERTEX_UVS_B
    detail_uv = in.uv_b * building_material.main_scale_offset.xy
        + building_material.main_scale_offset.zw;
#endif
    var vertex_color = vec4<f32>(1.0, 1.0, 0.0, 1.0);
#ifdef VERTEX_COLORS
    vertex_color = in.color;
#endif

    let main_sample = textureSample(main_texture, main_sampler, uv);
    let detail_alpha = textureSample(main_texture, main_sampler, detail_uv).a;
    let detail_mix = detail_alpha * building_material.surface_controls.x;
    let detail = mix(vec4<f32>(1.0), building_material.detail_color, detail_mix);
    let base_color = main_sample * detail;
    let occlusion = 1.0 - smoothstep(
        building_material.ambient_occlusion.x,
        building_material.ambient_occlusion.y,
        vertex_color.r,
    );
    var authored_color = vec4<f32>(base_color.rgb * occlusion, base_color.a);

    let upward = clamp(building_material.snow_damage.x * max(in.world_normal.y, 0.0), 0.0, 1.0);
    let snow_exclusion = clamp(vertex_color.r * building_material.snow_damage.y, 0.0, 1.0);
    authored_color = mix(authored_color, vec4<f32>(1.0), upward * (1.0 - snow_exclusion));

    let damage_sample = textureSample(
        main_texture,
        main_sampler,
        fract(in.world_position.xy * 0.5),
    ).a;
    // Unity's effect multiplies its texture by absolute world height. The Bevy
    // terrain uses a different vertical datum, so preserve the authored 0.01–2
    // health threshold while deriving a datum-independent wear mask.
    let damage_amount = clamp((2.0 - building_material.snow_damage.z) / 1.99, 0.0, 1.0);
    let damage = smoothstep(0.15, 0.85, damage_sample) * damage_amount;
    let damage_color = mix(vec3<f32>(0.0944), vec3<f32>(0.0), vertex_color.r);
    authored_color = vec4<f32>(
        mix(authored_color.rgb, damage_color, damage * 0.9),
        authored_color.a,
    );

    let roof_cell = floor(in.world_position.xz / 128.0);
    let roof_seed = fract(sin(dot(roof_cell, vec2<f32>(12.9898, 78.233))) * 43758.55);
    authored_color = vec4<f32>(
        mix(authored_color.rgb, roof_color(roof_seed), (1.0 - vertex_color.a) * 0.3),
        authored_color.a,
    );

    let emission_source = mix(
        main_sample * building_material.surface_controls.y,
        building_material.emissive_color,
        vertex_color.a,
    );
    pbr_input.material.base_color = alpha_discard(pbr_input.material, authored_color);
    pbr_input.material.emissive = emission_source
        * building_material.surface_controls.z
        * vertex_color.b;
    pbr_input.material.metallic = clamp(
        building_material.surface_controls.w * vertex_color.g,
        0.0,
        1.0,
    );
    pbr_input.material.perceptual_roughness = 1.0 - clamp(
        building_material.snow_damage.w * vertex_color.g,
        0.0,
        1.0,
    );

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
