fn stream_town_tree_deformed_position(
    local_position: vec3<f32>,
    source_world_position: vec3<f32>,
    vertex_color: vec4<f32>,
    animation_time: f32,
) -> vec3<f32> {
    // These are the exact serialized Env_Tree.mat values. Keeping the
    // deformation in this bind-group-free function lets both the visible and
    // shadow pipelines execute the same positions instead of maintaining two
    // subtly different silhouettes.
    let direction = vec2<f32>(1.0, 0.0);
    let sync = 0.7;
    let wind_strength = 0.79;
    let detail_strength = 0.01;
    let detail_scale = 1.0;
    let gust = sin(
        animation_time
            + (source_world_position.x + source_world_position.z) * sync,
    );
    let detail_uv = fract(
        (local_position.xy + animation_time * direction) * detail_scale
            + vec2<f32>(detail_scale),
    );
    let detail_noise = 0.5 + 0.5 * sin(dot(detail_uv, vec2<f32>(39.3468, 11.1351)));
    let detail = smoothstep(0.0, 1.0, detail_noise);
    let displacement = vec3<f32>(direction.x, 0.0, direction.y) * gust * wind_strength
        + vec3<f32>(detail_strength * detail);
    return local_position + displacement * vertex_color.r;
}
