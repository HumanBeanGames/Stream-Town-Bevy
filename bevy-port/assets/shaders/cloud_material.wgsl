#import bevy_pbr::{
    forward_io::{FragmentOutput, VertexOutput},
    mesh_view_bindings as view_bindings,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct CloudMaterialUniform {
    noise_controls: vec4<f32>,
    surface_transform: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> cloud_material: CloudMaterialUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(101)
var cloud_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102)
var cloud_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    let world_uv = in.world_position.xz;
    let drift = vec2<f32>(view_bindings::globals.time * 0.5);
    let scale = cloud_material.surface_transform.zw;
    let cloud1_uv = fract(cloud_material.noise_controls.x * (world_uv - drift) * scale);
    let cloud2_uv = fract(cloud_material.noise_controls.y * (world_uv + drift) * scale);
    let cloud1 = textureSample(cloud_texture, cloud_sampler, cloud1_uv).a;
    let cloud2 = textureSample(cloud_texture, cloud_sampler, cloud2_uv).a;
    let color_sample = clamp(
        (cloud1 - cloud_material.noise_controls.z)
            / max(1.0 - cloud_material.noise_controls.z, 0.001),
        0.0,
        1.0,
    );
    let combined = cloud1 * cloud2;
    let color = color_sample * cloud_material.surface_transform.x * (1.0 - combined);
    let alpha = pow(
        clamp(
            (combined - cloud_material.noise_controls.w)
                / max(1.0 - cloud_material.noise_controls.w, 0.001),
            0.0,
            1.0,
        ),
        max(cloud_material.surface_transform.y, 0.001),
    );
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        vec4<f32>(vec3<f32>(color), alpha),
    );

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
