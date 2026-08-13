#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, main_pass_post_lighting_processing},
}

struct GodrayMaterialUniform {
    emission_alpha: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> godray_material: GodrayMaterialUniform;

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

    // Unity's Godrays_Shaders uses the mesh vertex colour directly for both
    // albedo and emission, then scales vertex alpha by _AlphaStrength. Keep
    // the ray unlit so it remains self-luminous through the day/night cycle.
    let authored_color = vec4<f32>(
        vertex_color.rgb,
        clamp(vertex_color.a * godray_material.emission_alpha.y, 0.0, 1.0),
    );
    pbr_input.material.base_color = alpha_discard(pbr_input.material, authored_color);

    var out: FragmentOutput;
    out.color = vec4<f32>(
        authored_color.rgb * (1.0 + godray_material.emission_alpha.x),
        authored_color.a,
    );
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
