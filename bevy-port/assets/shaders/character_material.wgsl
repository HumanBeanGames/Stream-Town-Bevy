#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct CharacterMaterialUniform {
    albedo_color: vec4<f32>,
    // x: receiver-only world-normal offset. Keeping this in the character
    // material avoids changing terrain and synchronized foliage shadows.
    shadow_controls: vec4<f32>,
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

    // Unity's skinned-character receiver bias is distinct from the world
    // shadow-map bias. Use the offset only for direct-light evaluation and keep
    // the real fragment position for fog, depth, and post processing.
    var lighting_input = pbr_input;
    lighting_input.world_position = vec4<f32>(
        lighting_input.world_position.xyz
            + lighting_input.world_normal * character_material.shadow_controls.x,
        lighting_input.world_position.w,
    );

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(lighting_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
