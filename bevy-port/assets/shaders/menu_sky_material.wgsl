#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    mesh_view_bindings as view_bindings,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::main_pass_post_lighting_processing,
}

struct MenuSkyMaterialUniform {
    horizon_color: vec4<f32>,
    zenith_color: vec4<f32>,
    haze_color: vec4<f32>,
    sun_direction_strength: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> menu_sky: MenuSkyMaterialUniform;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    let pbr_input = pbr_input_from_standard_material(in, is_front);
    let view_direction = normalize(
        in.world_position.xyz - view_bindings::view.world_position.xyz,
    );
    let vertical = smoothstep(-0.18, 0.38, view_direction.y);
    let horizon_band = exp(-abs(view_direction.y + 0.035) * 8.0);
    let sun_direction = normalize(menu_sky.sun_direction_strength.xyz);
    let sun_disc = pow(max(dot(view_direction, sun_direction), 0.0), 96.0);
    let sun_glow = pow(max(dot(view_direction, sun_direction), 0.0), 9.0);
    let high_wisps = pow(
        0.5 + 0.5 * sin(
            view_direction.x * 11.0
                + sin(view_direction.z * 7.0) * 1.8
                + view_direction.z * 4.0,
        ),
        8.0,
    ) * smoothstep(0.08, 0.48, view_direction.y)
        * (1.0 - smoothstep(0.58, 0.82, view_direction.y));
    var color = mix(menu_sky.horizon_color.rgb, menu_sky.zenith_color.rgb, vertical);
    color = mix(color, menu_sky.haze_color.rgb, horizon_band * 0.16);
    color = mix(color, menu_sky.haze_color.rgb, high_wisps * 0.045);
    color += menu_sky.haze_color.rgb
        * (sun_disc * 0.72 + sun_glow * menu_sky.sun_direction_strength.w * 0.24);

    var out: FragmentOutput;
    out.color = vec4<f32>(max(color, vec3<f32>(0.0)), 1.0);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
