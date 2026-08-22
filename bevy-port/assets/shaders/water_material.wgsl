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
    // A flat menu plane has no vertex-depth data. Treating animated broad
    // noise as physical depth divided it into conspicuous light- and dark-blue
    // regions, so menu materials can request one fixed authored depth through
    // opacity_controls.z/w while the generated world keeps its vertex depth.
    if water_material.opacity_controls.z > 0.5 {
        depth = clamp(water_material.opacity_controls.w, 0.0, 1.0);
    }
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

    var out: FragmentOutput;
    // Unity authored this as a stylized final colour, so keep it unlit to avoid
    // the grazing-angle HDR mirror. Compress overbright foam uniformly instead
    // of clipping each RGB channel: the latter collapsed the surface, depth,
    // and foam into a flat cyan strip.
    let authored = max(pbr_input.material.base_color.rgb, vec3<f32>(0.0));
    let peak = max(authored.r, max(authored.g, authored.b));
    let bounded = authored / max(1.0, peak / 0.92);
    out.color = vec4<f32>(
        bounded,
        clamp(
            pbr_input.material.base_color.a,
            water_material.opacity_controls.x,
            water_material.opacity_controls.y,
        ),
    );
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
