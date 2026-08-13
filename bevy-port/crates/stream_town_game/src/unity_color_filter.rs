use bevy::{
    core_pipeline::{
        FullscreenShader,
        schedule::{Core3d, Core3dSystems},
        tonemapping::tonemapping,
        upscaling::upscaling,
    },
    prelude::*,
    render::{
        RenderApp, RenderStartup,
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
            CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, Operations,
            PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType, TextureViewId,
            binding_types::{sampler, texture_2d, uniform_buffer},
        },
        renderer::{RenderContext, RenderDevice, ViewQuery},
        view::ViewTarget,
    },
};

const SHADER_ASSET_PATH: &str = "shaders/unity_color_filter.wgsl";

pub struct UnityColorFilterPlugin;

impl Plugin for UnityColorFilterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<UnityColorFilter>::default(),
            UniformComponentPlugin::<UnityColorFilter>::default(),
        ));
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_systems(RenderStartup, init_unity_color_filter_pipeline)
            .add_systems(
                Core3d,
                unity_color_filter
                    .in_set(Core3dSystems::PostProcess)
                    .after(tonemapping)
                    .before(upscaling),
            );
    }
}

#[derive(Component, Clone, Copy, Debug, ExtractComponent, ShaderType)]
pub struct UnityColorFilter {
    color: Vec4,
}

impl UnityColorFilter {
    pub fn new(color: [f32; 4]) -> Self {
        Self {
            color: Vec4::from_array(color),
        }
    }
}

#[derive(Resource)]
struct UnityColorFilterPipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

fn init_unity_color_filter_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    pipeline_cache: Res<PipelineCache>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "unity_color_filter_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                uniform_buffer::<UnityColorFilter>(true),
            ),
        ),
    );
    let sampler = render_device.create_sampler(&SamplerDescriptor::default());
    let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("unity_color_filter_pipeline".into()),
        layout: vec![layout.clone()],
        vertex: fullscreen_shader.to_vertex_state(),
        fragment: Some(FragmentState {
            shader: asset_server.load(SHADER_ASSET_PATH),
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::Rgba16Float,
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            ..default()
        }),
        ..default()
    });
    commands.insert_resource(UnityColorFilterPipeline {
        layout,
        sampler,
        pipeline_id,
    });
}

#[derive(Default)]
struct UnityColorFilterBindGroupCache {
    cached: Option<(TextureViewId, BindGroup)>,
}

fn unity_color_filter(
    view: ViewQuery<(
        &ViewTarget,
        &UnityColorFilter,
        &DynamicUniformIndex<UnityColorFilter>,
    )>,
    pipeline: Option<Res<UnityColorFilterPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    uniforms: Res<ComponentUniforms<UnityColorFilter>>,
    mut cache: Local<UnityColorFilterBindGroupCache>,
    mut context: RenderContext,
) {
    let Some(pipeline) = pipeline else {
        return;
    };
    let (view_target, _settings, settings_index) = view.into_inner();
    if view_target.main_texture_format() != TextureFormat::Rgba16Float {
        return;
    }
    let Some(render_pipeline) = pipeline_cache.get_render_pipeline(pipeline.pipeline_id) else {
        return;
    };
    let Some(settings_binding) = uniforms.uniforms().binding() else {
        return;
    };
    let post_process = view_target.post_process_write();
    let texture_id = post_process.source.id();
    let bind_group = match &mut cache.cached {
        Some((cached_texture, bind_group)) if *cached_texture == texture_id => bind_group,
        cached => {
            let bind_group = context.render_device().create_bind_group(
                "unity_color_filter_bind_group",
                &pipeline_cache.get_bind_group_layout(&pipeline.layout),
                &BindGroupEntries::sequential((
                    post_process.source,
                    &pipeline.sampler,
                    settings_binding.clone(),
                )),
            );
            let (_, bind_group) = cached.insert((texture_id, bind_group));
            bind_group
        }
    };
    let mut render_pass = context
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("unity_color_filter_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    render_pass.set_pipeline(render_pipeline);
    render_pass.set_bind_group(0, bind_group, &[settings_index.index()]);
    render_pass.draw(0..3, 0..1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn shader_multiplies_all_authored_filter_channels() {
        let shader = include_str!("../../../assets/shaders/unity_color_filter.wgsl");
        assert!(shader.contains("source.rgb * settings.color.rgb"));
        assert!(shader.contains("source.a * settings.color.a"));
    }
}
