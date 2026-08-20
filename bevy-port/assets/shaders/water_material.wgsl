#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    mesh_view_bindings as view_bindings,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, main_pass_post_lighting_processing},
}

struct WaterMaterialUniform {
    surface_color: vec4<f32>,
    deep_color: vec4<f32>,
    foam_color: vec4<f32>,
    ice_color: vec4<f32>,
    wind_speed_noise_alpha: vec4<f32>,
    scale_foam_ice: vec4<f32>,
    season_tint: vec4<f32>,
    main_scale_offset: vec4<f32>,
    noise_scale_offset: vec4<f32>,
    depth_foam_controls: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> water_material: WaterMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var main_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var main_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(103)
var noise_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(104)
var noise_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    let wind = water_material.wind_speed_noise_alpha.xy;
    let time = view_bindings::globals.time;
    let scale = water_material.scale_foam_ice.x;
    let world_uv = in.world_position.xz / scale;
    let primary_uv = fract(
        (world_uv + wind * time) * water_material.main_scale_offset.xy
            + water_material.main_scale_offset.zw,
    );
    let secondary_uv = fract(
        (
            in.world_position.xz / 94.0
                + (vec2<f32>(1.0, 1.0) - wind) * 0.01 * time
        ) * water_material.main_scale_offset.xy
            + water_material.main_scale_offset.zw,
    );
    let primary = textureSample(main_texture, main_sampler, primary_uv).a;
    let secondary = textureSample(main_texture, main_sampler, secondary_uv).a;
    let wind_noise = mix(primary, 0.0, clamp(secondary * 3.0, 0.0, 1.0))
        * water_material.wind_speed_noise_alpha.w;

    let broad = textureSample(
        noise_texture,
        noise_sampler,
        fract(
            (in.world_position.xz * 0.002 + wind * time * 0.005)
                * water_material.noise_scale_offset.xy
                + water_material.noise_scale_offset.zw,
        ),
    );
    var depth = broad.g;
#ifdef VERTEX_COLORS
    depth = clamp(in.color.r, 0.0, 1.0);
#endif
    let shallow = pow(1.0 - depth, water_material.depth_foam_controls.y);
    var color = mix(water_material.deep_color, water_material.surface_color, shallow);
    color = mix(color, vec4<f32>(1.0, 1.0, 1.0, 0.0), clamp(wind_noise, 0.0, 1.0));

    let foam_scale = water_material.scale_foam_ice.y * 0.01;
    let foam_a = textureSample(
        noise_texture,
        noise_sampler,
        fract(
            (
                in.world_position.xz * foam_scale
                    + wind * time * water_material.wind_speed_noise_alpha.z
            ) * water_material.noise_scale_offset.xy
                + water_material.noise_scale_offset.zw,
        ),
    ).a;
    let foam_b = textureSample(
        noise_texture,
        noise_sampler,
        fract(
            (
                in.world_position.zx * foam_scale
                    - wind * time * water_material.wind_speed_noise_alpha.z
            ) * water_material.noise_scale_offset.xy
                + water_material.noise_scale_offset.zw,
        ),
    ).a;
    let shoreline = 1.0 - smoothstep(
        0.0,
        water_material.depth_foam_controls.w
            / water_material.depth_foam_controls.x,
        depth,
    );
    let foam_threshold = clamp(
        1.0 - shoreline * water_material.depth_foam_controls.z * 0.1,
        0.05,
        1.95,
    );
    let foam = smoothstep(foam_threshold, foam_threshold + 0.22, foam_a + foam_b)
        * shoreline;
    color = mix(color, water_material.foam_color, foam);

    let ice_pattern = smoothstep(0.2, 0.8, broad.b - primary * 0.35);
    color = mix(
        color,
        water_material.ice_color,
        ice_pattern * water_material.scale_foam_ice.w,
    );
    let tinted_water = color.rgb * water_material.season_tint.rgb;
    pbr_input.material.base_color = vec4<f32>(
        mix(tinted_water, water_material.foam_color.rgb, foam),
        mix(
            mix(1.0, water_material.season_tint.a, depth),
            water_material.scale_foam_ice.z,
            foam,
        ),
    );
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color,
    );

    var out: FragmentOutput;
    // Unity's stylized water colour was authored as the final surface look.
    // Feeding it through Bevy's physical sun response produced HDR values far
    // above one at grazing coastline angles, turning the ocean into a white
    // mirror. Keep the authored colour bounded and still run Bevy's fog/output
    // processing below.
    let bounded = clamp(
        pbr_input.material.base_color.rgb,
        vec3<f32>(0.0),
        vec3<f32>(1.0),
    );
    // Foam and the animated alpha textures may reach authored white, but the
    // Unity look never turns the whole coast into an HDR-white mirror. Keep a
    // visibly blue ceiling and enough opacity that the black clear target does
    // not drain the stylized surface colour.
    out.color = vec4<f32>(
        min(bounded, vec3<f32>(0.18, 0.48, 0.72)),
        max(pbr_input.material.base_color.a, 0.88),
    );
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
