#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct CharacterMaterialUniform {
    albedo_color: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> character_material: CharacterMaterialUniform;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Custom/CharacterSimple has no vertex-colour masking or transparency:
    // its sole authored surface expression is _characterTexture * _albedoColor.
    // The base StandardMaterial owns the texture and UV transform, which also
    // keeps the shader valid when Unity leaves its texture slot empty.
    let authored_color = pbr_input.material.base_color * character_material.albedo_color;
    pbr_input.material.base_color = alpha_discard(pbr_input.material, authored_color);

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
