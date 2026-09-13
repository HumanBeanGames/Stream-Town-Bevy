use super::*;

mod bootstrap_flow;
mod credits_flow;
mod gameplay_flow;
mod main_menu_flow;
mod presentation_flow;
mod shared_runtime;
mod world_loading_flow;

use bootstrap_flow::BootstrapFlowPlugin;
use credits_flow::CreditsFlowPlugin;
use gameplay_flow::GameplayFlowPlugin;
use main_menu_flow::MainMenuFlowPlugin;
use presentation_flow::PresentationFlowPlugin;
use shared_runtime::SharedRuntimePlugin;
use world_loading_flow::WorldLoadingFlowPlugin;

pub struct StreamTownGamePlugin;

impl Plugin for StreamTownGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TabNavigationPlugin);
        app.add_plugins(runtime::timelapse::CityTimelapsePlugin);
        #[cfg(target_os = "windows")]
        app.add_plugins(direct_broadcast::DirectTwitchBroadcastPlugin);
        let render_schedule_available = app.get_sub_app(RenderApp).is_some();
        let presented_frames = PresentedRenderFrames::new(render_schedule_available);
        let gpu_readiness = GpuReadinessProbe::default();
        let automatic_resume_path = automatic_resume_save_path();
        let fixed_save_path = std::env::var_os("STREAM_TOWN_SAVE_PATH").map(PathBuf::from);
        let requested_new_town = if automatic_resume_path.is_none() {
            std::env::var("STREAM_TOWN_NEW_TOWN_NAME")
                .ok()
                .filter(|name| !name.trim().is_empty())
        } else {
            None
        };
        app.insert_resource(presented_frames.clone());
        app.insert_resource(gpu_readiness.clone());
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .insert_resource(presented_frames)
                .insert_resource(gpu_readiness)
                .add_systems(
                    Render,
                    (record_gpu_readiness, record_presented_render_frame)
                        .chain()
                        .in_set(RenderSystems::Cleanup),
                );
        }
        if !app.world().contains_resource::<RuntimeConfig>() {
            app.insert_resource(RuntimeConfig(GameConfig::default()));
        }
        // Select the final seed before any world geometry exists so terrain,
        // water, resources, fish, and foliage all share one world. A named new
        // town uses the same stable seed derivation as the interactive menu.
        let startup_seed = requested_new_town
            .as_deref()
            .map(town_name_seed)
            .or_else(|| automatic_resume_world_seed(automatic_resume_path.as_deref()));
        if let Some(seed) = startup_seed {
            app.world_mut().resource_mut::<RuntimeConfig>().0.world.seed = seed;
        }
        if !app.world().contains_resource::<RuntimeContent>() {
            app.insert_resource(RuntimeContent(embedded_content()));
        }
        if !app.world().contains_resource::<RuntimePresentation>() {
            app.insert_resource(RuntimePresentation(embedded_presentation()));
        }
        if !app.world().contains_resource::<RuntimeAssetRoot>() {
            app.insert_resource(RuntimeAssetRoot(locate_asset_root()));
        }
        if !app.world().contains_resource::<RuntimePlayerSettings>() {
            app.insert_resource(RuntimePlayerSettings(PlayerSettings::default()));
        }
        app.add_message::<AccessibilityActionRequest>();
        app.add_message::<TownPersistenceRequest>();
        app.init_state::<GameState>()
            .configure_sets(
                Update,
                GameplaySimulationSet
                    .run_if(in_state(GameState::InGame))
                    .run_if(resource_exists::<GameplayReady>),
            )
            .configure_sets(
                Update,
                AgentSimulationSet
                    .in_set(GameplaySimulationSet)
                    .run_if(agent_simulation_due),
            )
            .configure_sets(
                Update,
                NightLightSyncSet
                    .in_set(GameplaySimulationSet)
                    .run_if(night_light_sync_due),
            )
            .init_resource::<SessionStats>()
            .init_resource::<WorldRenderStats>()
            .init_resource::<TraversalWearRuntime>()
            .init_resource::<PathSurfaceRuntime>()
            .init_resource::<AgentSimulationCadence>()
            .init_resource::<FineNavigationRuntime>()
            .init_resource::<WorldDiagnosticRuntime>()
            .init_resource::<CrowdSeparationRuntime>()
            .init_resource::<StationTargetRuntime>()
            .init_resource::<StationResourceTargetIndex>()
            .init_resource::<EnemyNavigationRuntime>()
            .init_resource::<RegenerationRoleRuntime>()
            .init_resource::<RulerVoteAnnouncementRuntime>()
            .init_resource::<CitizenDeathAnnouncementRuntime>()
            .init_resource::<InjectedCommands>()
            .init_resource::<CommandFeedback>()
            .init_resource::<CommandAcknowledgementRuntime>()
            .init_resource::<CameraDamageRuntime>()
            .init_resource::<RetreatingCitizens>()
            .init_resource::<MenuRuntime>()
            .init_resource::<TownRestartRuntime>()
            .init_resource::<NightEnemyWaveRuntime>()
            .init_resource::<SettingsUiCache>()
            .init_resource::<SecretsRuntime>()
            .init_resource::<SensitiveScreenActive>()
            .init_resource::<AccessibilityRuntime>()
            .init_resource::<InputFocus>()
            .init_resource::<InputFocusVisible>()
            .init_resource::<MenuIoRequest>()
            .init_resource::<RuntimeConsoleRuntime>()
            .init_resource::<RuntimeCaptureRequest>()
            .init_resource::<CameraCommandQueue>()
            .init_resource::<AgentCommandQueue>()
            .init_resource::<BuildingCommandQueue>()
            .init_resource::<BuildingPlacers>()
            .init_resource::<PathFailureRuntime>()
            .init_resource::<TwitchConnection>()
            .init_resource::<OperatorChatRuntime>()
            .init_resource::<SelectedCell>()
            .init_resource::<SelectedActor>()
            .init_resource::<EnvironmentPresentation>()
            .init_resource::<PostProcessPresentation>()
            .init_resource::<BuildingMaterialInstances>()
            .init_resource::<BuildingMaterialUpdateRuntime>()
            .init_resource::<BuildingVisualSignatures>()
            .init_resource::<CosmeticMaterialCache>()
            .init_resource::<CharacterBaseMaterialCache>()
            .init_resource::<RoleActionAudioCache>()
            .init_resource::<WorldAudioRuntime>()
            .init_resource::<tidal_music::TidalMusicRuntime>()
            .init_resource::<tidal_music::IntensitySongInput>()
            .init_resource::<NativeAnimationCache>()
            .init_resource::<ConvertedAnimationCache>()
            .init_resource::<GateAnimationCache>()
            .init_resource::<LevelUpPresentation>()
            .add_observer(mark_converted_animation_instance_ready)
            .insert_resource(PlayerSettingsRuntime {
                autosave_elapsed_seconds: 0.0,
            })
            .insert_resource(TownSaveCatalogRuntime::from_startup_paths(
                fixed_save_path.clone(),
                automatic_resume_path.clone(),
            ))
            .insert_resource(SaveRuntime {
                store: NativeSaveStore::new(startup_save_path(
                    automatic_resume_path.clone(),
                    fixed_save_path,
                )),
            })
            .insert_resource(AutomaticResumeRuntime::new(automatic_resume_path));
        app.add_plugins((
            BootstrapFlowPlugin,
            SharedRuntimePlugin,
            MainMenuFlowPlugin,
            WorldLoadingFlowPlugin,
            GameplayFlowPlugin,
            PresentationFlowPlugin,
            CreditsFlowPlugin,
        ));
    }
}
