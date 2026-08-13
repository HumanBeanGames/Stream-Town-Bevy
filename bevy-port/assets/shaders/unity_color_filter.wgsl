@group(0) @binding(0) var source_texture: texture_2d<f32>;
@group(0) @binding(1) var source_sampler: sampler;

struct UnityColorFilter {
    color: vec4<f32>,
};

@group(0) @binding(2) var<uniform> settings: UnityColorFilter;

@fragment
fn fragment(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<f32>(textureDimensions(source_texture));
    let uv = position.xy / size;
    let source = textureSample(source_texture, source_sampler, uv);
    return vec4<f32>(source.rgb * settings.color.rgb, source.a * settings.color.a);
}
