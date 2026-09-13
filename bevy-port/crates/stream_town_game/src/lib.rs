#![allow(clippy::wildcard_imports)]

mod app;
mod bootstrap;
#[cfg(target_os = "windows")]
pub mod direct_broadcast;
mod presentation;
#[cfg(all(feature = "stream-profiling", target_os = "windows"))]
pub mod profiling;
mod runtime;
mod tidal_music;
pub mod twitch;

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex, OnceLock, PoisonError,
        atomic::{AtomicU64, Ordering as AtomicOrdering},
        mpsc,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use accesskit::{Action as AccessAction, Live, Node as AccessibleNode, Role};
use anyhow::{Context, Result as AnyResult};
use avian3d::prelude::{Collider, PhysicsPlugins, RigidBody, SpatialQuery, SpatialQueryFilter};
#[cfg(target_os = "windows")]
use bevy::render::settings::Backends;
use bevy::{
    a11y::{AccessibilityNode, ActionRequest as AccessibilityActionRequest},
    animation::{
        AnimatedBy, AnimationClip, AnimationTargetId, RepeatAnimation, animated_field,
        graph::{AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationNodeType},
        prelude::{AnimatableCurve, AnimatableKeyframeCurve},
        transition::AnimationTransitions,
    },
    anti_alias::{fxaa::Fxaa, smaa::Smaa},
    app::AnimationSystems,
    asset::{AssetId, AssetPlugin, LoadState, RenderAssetUsages, UntypedAssetId, UntypedHandle},
    audio::{AudioSink, AudioSinkPlayback, AudioSource, SpatialScale, Volume},
    camera::{Hdr, RenderTarget, ScalingMode},
    camera::{primitives::Aabb, visibility::NoFrustumCulling},
    color::LinearRgba,
    core_pipeline::tonemapping::Tonemapping,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::{query::QueryData, system::SystemParam},
    gltf::{GltfMaterialName, GltfMeshName},
    image::ImageSampler,
    input::mouse::{MouseScrollUnit, MouseWheel},
    input_focus::{
        FocusCause, InputFocus, InputFocusVisible,
        tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
    },
    light::{
        DirectionalLightShadowMap, NotShadowCaster,
        cluster::{ClusterConfig, ClusterFarZMode, ClusterZConfig},
    },
    math::Affine2,
    mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes},
    mesh::{Indices, VertexAttributeValues},
    pbr::ScreenSpaceAmbientOcclusion,
    pbr::{ExtendedMaterial, MaterialExtension, PreparedMaterial, RenderMeshInstances},
    post_process::{
        bloom::{Bloom, BloomCompositeMode, BloomPrefilter},
        effect_stack::Vignette,
        motion_blur::MotionBlur,
    },
    prelude::*,
    render::erased_render_asset::ErasedRenderAssets,
    render::error_handler::{ErrorType, RenderError, RenderErrorHandler, RenderErrorPolicy},
    render::mesh::RenderMesh,
    render::render_asset::RenderAssets as GpuRenderAssets,
    render::render_resource::{
        AsBindGroup, CachedPipelineState, Extent3d, PipelineCache, PrimitiveTopology, ShaderType,
        TextureDimension, TextureFormat,
    },
    render::sync_world::MainEntity,
    render::texture::GpuImage,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
    render::view::{ColorGrading, Msaa},
    render::{Render, RenderApp, RenderPlugin, RenderSystems, settings::WgpuSettings},
    shader::ShaderRef,
    sprite::{BorderRect, TextureSlicer},
    tasks::{AsyncComputeTaskPool, Task, block_on, poll_once},
    text::{EditableText, TextCursorStyle},
    transform::TransformSystems,
    ui_widgets::SelectAllOnFocus,
    window::{
        CursorOptions, MonitorSelection, OnMonitor, PresentMode, PrimaryWindow, WindowMode,
        WindowResolution,
    },
    winit::{UpdateMode, WinitSettings},
    world_serialization::{WorldInstance, WorldInstanceReady},
};
use stream_town_domain::{
    ActorCustomization, ActorKind, ActorState, AdaptiveMusicConfig, AnimationBlendSelection,
    AnimationClipDef, AnimationControllerRuntime, AnimationLayerBlendMode, AnimationLayerDef,
    AnimationTransformTrack, AnimationTransitionPlayback, ArchetypeDef, ArchetypeKind,
    ArchetypeScene, AvatarMaskDef, BUILDING_MAX_HEALTH, BroadcastConfig,
    BroadcastEncoderPreference, BroadcastRenderMode, BuildingAction, BuildingDef,
    BuildingDirection, BuildingHealthDisplayMode, BuildingModelDef, BuildingState,
    CURRENT_RUNTIME_CONSOLE_SCHEMA, CURRENT_SIMULATION_SCHEMA, CURRENT_WORLD_SNAPSHOT_SCHEMA,
    CameraAction, CameraDirection, ChatCommand, ChimneySmokeDef, CommunityEvent,
    CommunityVoteProposal, ContentCatalog, CustomizationKind, DAYS_PER_SEASON, DisplayMode,
    EnemyCampGenerationDef, EnemyCampState, EnemyModelSetDef, EnemyRunAnimation, FireworksVfxDef,
    FoliageHabitat, GameConfig, GeneratedFoliage, GeneratedWorld, GridPos, HealingBurstVfxDef,
    HealingChannelVfxDef, MAX_TRAVERSAL_WEAR_SCORE, MainMenuSceneReference,
    MaterialAlphaMode as AuthoredAlphaMode, MaterialDef, NameDisplayMode, NativeSaveStore,
    ObjectiveEvent, ObjectiveKind, PetDef, PetModelDef, PlayerSettings, PlayerSettingsStore,
    PostProcessAntiAliasing, PostProcessProfileDef, PostProcessTonemapping, PresentationCatalog,
    RainingFishVfxDef, RoleEquipmentDef, RulerVoteKind, RuntimeConsoleAction, RuntimeConsoleStatus,
    RuntimeConsoleStore, SEASON_TRANSITION_SECONDS, SEASONS_PER_YEAR, SavedActor, Season, StableId,
    StationDef, StationUpdateMode, StorageModelDef, StreamUserType, TargetingScoreDef,
    TimelapseInterval, TownEvent, VISIBLE_WATER_SURFACE_LIFT_METRES, VfxGradientDef, Weather,
    WorldGenerationStage, WorldSimulation, WorldSnapshot, foliage_visual_variant,
    foliage_visual_yaw_milliradians, generate_world_with_content,
    generate_world_with_content_observed, navigation_corner_height_metres,
    navigation_surface_height_at_world, parse_chat_commands, resource_visual_variant,
    visible_water_surface_height,
};

include!("core/foundation.rs");
include!("core/runtime_state.rs");
include!("core/loading_state.rs");
include!("core/bootstrap_runtime.rs");
include!("core/world_runtime.rs");

#[cfg(test)]
mod tests;
