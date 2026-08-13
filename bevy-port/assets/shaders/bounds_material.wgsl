#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct BoundsMaterialUniform {
    color_alpha: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> bounds_material: BoundsMaterialUniform;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    // Unity's BoundsVisualiser shader is a transparent lit surface. Its
    // runtime component changes only _boundsVisColor; _Alpha remains the
    // authored material value for both valid and blocked states.
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        bounds_material.color_alpha,
    );
    pbr_input.material.metallic = 0.0;
    pbr_input.material.perceptual_roughness = 0.5;

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
