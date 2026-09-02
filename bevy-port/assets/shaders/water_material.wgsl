#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    mesh_view_bindings as view_bindings,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
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
    opacity_controls: vec4<f32>,
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
    // A negative authored speed is the persisted reduced-motion sentinel. It
    // freezes every decorative water phase without changing the sampled shape.
    let animation_time = select(time, 0.0, water_material.wind_speed_noise_alpha.z < 0.0);
    let foam_speed = abs(water_material.wind_speed_noise_alpha.z);
    let scale = water_material.scale_foam_ice.x;
    let world_uv = in.world_position.xz / scale;
    let primary_uv = fract(
        (world_uv + wind * animation_time) * water_material.main_scale_offset.xy
            + water_material.main_scale_offset.zw,
    );
    let secondary_uv = fract(
        (
            in.world_position.xz / 94.0
                + (vec2<f32>(1.0, 1.0) - wind) * 0.01 * animation_time
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
            (in.world_position.xz * 0.002 + wind * animation_time * 0.005)
                * water_material.noise_scale_offset.xy
                + water_material.noise_scale_offset.zw,
        ),
    );
    var depth = broad.g;
#ifdef VERTEX_COLORS
    depth = clamp(in.color.r, 0.0, 1.0);
#endif
    // A flat menu plane has no vertex-depth data. Keep one authored base depth,
    // but retain a small continuous portion of the broad noise. This avoids the
    // old two-tone split without reducing the whole ocean to one flat colour.
    if water_material.opacity_controls.z > 0.5 {
        let menu_wave = 0.5 + 0.25 * (
            sin(
                in.world_position.x * 0.105
                    + animation_time * 0.46
                    + sin(in.world_position.z * 0.052 - animation_time * 0.21) * 1.7,
            )
                + sin(in.world_position.z * 0.143 - animation_time * 0.37)
        );
        depth = clamp(
            water_material.opacity_controls.w
                + (broad.g - 0.5) * 0.16
                + (menu_wave - 0.5) * 0.18,
            0.0,
            1.0,
        );
    }
    let shallow = pow(1.0 - depth, water_material.depth_foam_controls.y);
    var color = mix(water_material.deep_color, water_material.surface_color, shallow);
    color = mix(color, vec4<f32>(1.0, 1.0, 1.0, 0.0), clamp(wind_noise, 0.0, 1.0));
    if water_material.opacity_controls.z > 0.5 {
        let ripple = 0.5 + 0.25 * (
            sin(in.world_position.x * 0.18 + animation_time * 0.72)
                + sin(in.world_position.z * 0.23 - animation_time * 0.54)
        );
        color = mix(color, water_material.surface_color, ripple * 0.28);
        let broad_ripple = 0.5 + 0.5 * sin(
            in.world_position.x * 0.027
                + animation_time * 0.20
                + sin(in.world_position.z * 0.019 - animation_time * 0.16) * 2.4,
        );
        color = vec4<f32>(
            color.rgb * (0.82 + broad_ripple * 0.32)
                + water_material.foam_color.rgb
                    * smoothstep(0.78, 1.0, broad_ripple)
                    * 0.045,
            color.a,
        );
    }

    let foam_scale = water_material.scale_foam_ice.y * 0.01;
    let foam_a = textureSample(
        noise_texture,
        noise_sampler,
        fract(
            (
                in.world_position.xz * foam_scale
                    + wind * animation_time * foam_speed
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
                    - wind * animation_time * foam_speed
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
    // `_FoamCuttoff` is serialized by Unity on a 0-10 inspector scale. The
    // earlier port subtracted it from one and compared it with the *sum* of two
    // 0-1 samples, so virtually every shallow pixel became solid foam.
    let foam_threshold = clamp(
        water_material.depth_foam_controls.z * 0.1,
        0.05,
        0.95,
    );
    let foam_noise = (foam_a + foam_b) * 0.5;
    let foam = smoothstep(
        foam_threshold,
        min(foam_threshold + 0.14, 1.0),
        foam_noise,
    ) * shoreline;
    let foam_colour_strength = foam * water_material.scale_foam_ice.z;
    color = mix(color, water_material.foam_color, foam_colour_strength);

    let ice_pattern = smoothstep(0.2, 0.8, broad.b - primary * 0.35);
    color = mix(
        color,
        water_material.ice_color,
        ice_pattern * water_material.scale_foam_ice.w,
    );
    // Seasonal colour is a tint, not a replacement albedo. Multiplying all the
    // way down to the palette target made transparent water expose the dark
    // terrain checker underneath. Retain most of the authored cyan surface.
    let tinted_water = mix(
        color.rgb,
        color.rgb * water_material.season_tint.rgb,
        0.35,
    );
    pbr_input.material.base_color = vec4<f32>(
        tinted_water,
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

    // Preserve the authored colour variation while feeding it through Bevy's
    // PBR lighting. The material remains rough and non-reflective on the Rust
    // side, avoiding the old grazing-angle HDR mirror without making the water
    // ignore sunlight, ambient light, and night entirely.
    let authored = max(pbr_input.material.base_color.rgb, vec3<f32>(0.0));
    let peak = max(authored.r, max(authored.g, authored.b));
    let bounded = authored / max(1.0, peak / 0.92);
    pbr_input.material.base_color = vec4<f32>(
        bounded,
        clamp(
            pbr_input.material.base_color.a,
            water_material.opacity_controls.x,
            water_material.opacity_controls.y,
        ),
    );
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
