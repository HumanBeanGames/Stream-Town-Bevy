#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_tabs_cover_authoring_and_operator_workflows() {
        assert_eq!(
            ToolTab::ALL.map(ToolTab::label),
            [
                "Game Authority",
                "Models + Assets",
                "Buildings",
                "Roles",
                "Progression",
                "Technology",
                "Terrain",
                "Music",
                "World + Nav",
                "Player Settings",
                "Runtime",
                "Twitch",
                "Validation",
            ]
        );
        assert_eq!(
            AssetEditorSection::ALL.map(AssetEditorSection::label),
            ["Models", "Textures", "Materials", "Animations"]
        );
    }

    #[test]
    fn character_model_choices_include_converted_hierarchy_nodes() {
        let state = ToolState::default();
        let choices = equipment_node_choices(&state.catalog, &state.presentation);
        assert!(choices.iter().any(|node| node == "Body_Blacksmith_Slim"));
        assert!(choices.iter().any(|node| node == "Body_Logger_Feminine"));
        assert!(choices.iter().all(|node| !node.trim().is_empty()));
    }

    #[test]
    fn discovered_model_assets_are_project_relative_glbs() {
        let assets = discover_model_assets();
        assert!(!assets.is_empty());
        assert!(assets.iter().all(|path| {
            std::path::Path::new(path)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
        }));
        assert!(
            assets
                .iter()
                .all(|path| !std::path::Path::new(path).is_absolute() && !path.contains('\\'))
        );
    }

    #[test]
    fn texture_discovery_and_gltf_metadata_are_typed_project_assets() {
        let textures = discover_texture_assets();
        assert!(!textures.is_empty());
        assert!(textures.iter().all(|path| {
            path.starts_with("shipping/textures/")
                && !path.contains('\\')
                && matches!(
                    std::path::Path::new(path)
                        .extension()
                        .unwrap()
                        .to_string_lossy()
                        .to_ascii_lowercase()
                        .as_str(),
                    "png" | "tga" | "jpg" | "jpeg"
                )
        }));
        let metadata =
            inspect_gltf_asset("shipping/models/Models/Characters/Characters.glb").unwrap();
        assert!(!metadata.nodes.is_empty());
        assert!(!metadata.materials.is_empty());
        assert!(!metadata.animations.is_empty());
    }

    #[test]
    fn preview_material_resolution_matches_runtime_binding_precedence() {
        let mut assets = Assets::<StandardMaterial>::default();
        let fallback = assets.add(StandardMaterial::default());
        let model = assets.add(StandardMaterial::default());
        let renderer = assets.add(StandardMaterial::default());
        let overrides = PreviewMaterialOverrides {
            fallback: Some(fallback.clone()),
            model_materials: BTreeMap::from([("MainMaterial".to_owned(), model.clone())]),
            renderer_materials: vec![PreviewRendererMaterialBinding {
                target_path: "Root/Body".to_owned(),
                materials: BTreeMap::from([("MainMaterial".to_owned(), renderer.clone())]),
            }],
        };

        let resolved = resolved_preview_material(
            &overrides,
            "Imported/Root/Body/Primitive0",
            Some("Body"),
            Some("MainMaterial"),
        )
        .unwrap();
        assert_eq!(resolved, &renderer);
        assert_eq!(
            resolved_preview_material(
                &overrides,
                "Unmatched/Primitive0",
                None,
                Some("MainMaterial")
            ),
            Some(&model)
        );
        assert_eq!(
            resolved_preview_material(&overrides, "Unmatched", None, None),
            Some(&fallback)
        );
    }

    #[test]
    fn preview_camera_starts_in_front_and_pans_in_camera_space() {
        let controls = ModelPreviewControls::default();
        let offset = preview_camera_offset(controls.yaw, controls.pitch, controls.distance);
        assert!(
            offset.z > 0.0,
            "the default camera must view +Z-facing models from the front"
        );

        let front_pan = preview_pan_delta(0.0, 0.0, 1.0, 0.0, 1.0);
        let side_pan = preview_pan_delta(std::f32::consts::FRAC_PI_2, 0.0, 1.0, 0.0, 1.0);
        assert!(front_pan.x < -0.99 && front_pan.z.abs() < 0.01);
        assert!(side_pan.z > 0.99 && side_pan.x.abs() < 0.01);
    }

    #[test]
    fn presentation_asset_crud_stays_valid_and_reference_safe() {
        let mut state = ToolState::default();

        state.new_texture_id = "texture:test_authoring".to_owned();
        state.new_texture_name = "Test Texture".to_owned();
        state.new_texture_asset = state.discovered_texture_assets[0].clone();
        create_texture_definition(&mut state).unwrap();
        let texture = StableId::new("texture:test_authoring").unwrap();
        assert!(state.presentation.textures.contains_key(&texture));
        delete_selected_texture(&mut state).unwrap();

        state.new_material_id = "material:test_authoring".to_owned();
        state.new_material_name = "Test Material".to_owned();
        create_material_definition(&mut state).unwrap();
        let material = StableId::new("material:test_authoring").unwrap();
        assert!(state.presentation.materials.contains_key(&material));
        delete_selected_material(&mut state).unwrap();

        state.new_clip_id = "clip:test_authoring".to_owned();
        state.new_clip_name = "Test Clip".to_owned();
        state.new_clip_asset = "shipping/models/Models/Characters/Characters.glb".to_owned();
        create_animation_clip(&mut state).unwrap();
        let clip = StableId::new("clip:test_authoring").unwrap();
        assert!(state.presentation.clips.contains_key(&clip));
        delete_selected_animation_clip(&mut state).unwrap();

        state.new_controller_id = "animation_controller:test_authoring".to_owned();
        state.new_controller_name = "Test Controller".to_owned();
        create_animation_controller(&mut state).unwrap();
        let controller = StableId::new("animation_controller:test_authoring").unwrap();
        assert!(state.presentation.controllers.contains_key(&controller));
        delete_selected_animation_controller(&mut state).unwrap();
        state.presentation.validate().unwrap();
    }

    #[test]
    fn model_archetype_and_variant_lifecycle_remains_valid() {
        let mut state = ToolState::default();
        state.new_archetype_id = "archetype:test:model".to_owned();
        state.new_archetype_name = "Test Model".to_owned();
        state.new_archetype_asset = state.discovered_model_assets[0].clone();

        create_model_archetype(&mut state).unwrap();
        let id = StableId::new("archetype:test:model").unwrap();
        assert_eq!(state.catalog.archetypes[&id].scenes.len(), 1);
        state.new_archetype_asset = state.discovered_model_assets[1].clone();
        add_archetype_scene(&mut state, &id).unwrap();
        assert_eq!(state.catalog.archetypes[&id].scenes.len(), 2);
        remove_archetype_scene(&mut state, &id, 1).unwrap();
        delete_model_archetype(&mut state, &id).unwrap();
        assert!(!state.catalog.archetypes.contains_key(&id));
        state.catalog.validate().unwrap();
    }

    #[test]
    fn technology_editor_rejects_cycles_without_mutating_catalog() {
        let mut state = ToolState::default();
        let (node_id, prerequisite) = state
            .catalog
            .technology
            .nodes
            .iter()
            .find_map(|(id, node)| {
                node.prerequisites
                    .first()
                    .map(|parent| (id.clone(), parent.clone()))
            })
            .expect("shipping graph has an edge");
        state.technology_draft = technology_draft(&state.catalog, &prerequisite);
        state.technology_draft.as_mut().unwrap().value.prerequisites = vec![node_id];
        let before = state.catalog.clone();
        assert!(apply_technology_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);
    }

    #[test]
    fn technology_socket_connection_rejects_cycles_without_mutation() {
        let mut state = ToolState::default();
        let (dependent, prerequisite) = state
            .catalog
            .technology
            .nodes
            .iter()
            .find_map(|(id, node)| {
                node.prerequisites
                    .first()
                    .map(|parent| (id.clone(), parent.clone()))
            })
            .expect("shipping graph has an edge");
        let before = state.catalog.clone();

        assert!(connect_technology_nodes(&mut state, &dependent, &prerequisite).is_err());
        assert_eq!(state.catalog, before);
    }

    #[test]
    fn technology_connections_can_be_removed_and_restored() {
        let mut state = ToolState::default();
        let (dependent, prerequisite) = state
            .catalog
            .technology
            .nodes
            .iter()
            .find_map(|(id, node)| {
                node.prerequisites
                    .first()
                    .map(|parent| (id.clone(), parent.clone()))
            })
            .expect("shipping graph has an edge");
        disconnect_technology_nodes(&mut state, &prerequisite, &dependent).unwrap();
        assert!(
            !state.catalog.technology.nodes[&dependent]
                .prerequisites
                .contains(&prerequisite)
        );
        connect_technology_nodes(&mut state, &prerequisite, &dependent).unwrap();
        assert!(
            state.catalog.technology.nodes[&dependent]
                .prerequisites
                .contains(&prerequisite)
        );
    }

    #[test]
    fn vote_requirement_lifecycle_is_typed_validated_and_reference_safe() {
        let mut state = ToolState::default();
        let original_count = state.catalog.objectives.len();
        state.new_objective_id = "objective:test".to_owned();
        duplicate_selected_objective(&mut state).unwrap();
        let id = StableId::new("objective:test").unwrap();
        assert!(state.catalog.objectives.contains_key(&id));

        state
            .objective_draft
            .as_mut()
            .unwrap()
            .value
            .required_amount = 0;
        let before = state.catalog.clone();
        assert!(apply_objective_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);

        refresh_objective_draft(&mut state);
        delete_selected_objective(&mut state).unwrap();
        assert_eq!(state.catalog.objectives.len(), original_count);
        state.catalog.validate().unwrap();
    }

    #[test]
    fn technology_authoring_create_move_delete_and_undo_remains_valid() {
        let mut state = ToolState {
            new_group_id: "technology_group:test".to_owned(),
            new_group_name: "Test Group".to_owned(),
            ..ToolState::default()
        };
        create_technology_group(&mut state).unwrap();
        let group_id = StableId::new("technology_group:test").unwrap();
        assert_eq!(state.selected_group.as_ref(), Some(&group_id));

        state.new_technology_id = "technology:test".to_owned();
        state.new_technology_name = "Test Technology".to_owned();
        create_technology_node(&mut state).unwrap();
        let node_id = StableId::new("technology:test").unwrap();
        assert_eq!(
            state.catalog.technology.nodes[&node_id].group.as_ref(),
            Some(&group_id)
        );
        assert!(
            state.catalog.technology.groups[&group_id]
                .nodes
                .contains(&node_id)
        );
        assert!(delete_selected_technology_group(&mut state).is_err());

        delete_selected_technology_node(&mut state).unwrap();
        assert!(!state.catalog.technology.nodes.contains_key(&node_id));
        delete_selected_technology_group(&mut state).unwrap();
        assert!(!state.catalog.technology.groups.contains_key(&group_id));
        state.catalog.validate().unwrap();

        let previous = state.undo_authoring.pop().unwrap();
        state.catalog = previous.catalog;
        state.technology_layout = previous.technology_layout;
        assert!(state.catalog.technology.groups.contains_key(&group_id));
        state.catalog.validate().unwrap();
        state
            .technology_layout
            .validate(&state.catalog.technology)
            .unwrap();
    }

    #[test]
    fn technology_editor_preserves_the_complete_effect_record() {
        let mut state = ToolState::default();
        let (id, before) = state
            .catalog
            .technology
            .nodes
            .iter()
            .find(|(_, node)| {
                !node.building_level_caps.is_empty()
                    || !node.global_stat_boost_percent.is_empty()
                    || !node.role_stat_boost_percent.is_empty()
            })
            .map(|(id, node)| (id.clone(), node.clone()))
            .expect("shipping technology graph has authored effects");
        state.technology_draft = technology_draft(&state.catalog, &id);
        state
            .technology_draft
            .as_mut()
            .unwrap()
            .value
            .description
            .push_str(" [edited]");

        apply_technology_draft(&mut state).unwrap();

        let mut expected = before;
        expected.description.push_str(" [edited]");
        assert_eq!(state.catalog.technology.nodes[&id], expected);
    }

    #[test]
    fn building_editor_preserves_the_complete_template_record() {
        let mut state = ToolState::default();
        let (id, before) = state
            .catalog
            .buildings
            .iter()
            .find(|(_, building)| {
                !building.model_handlers.is_empty()
                    && (!building.storage.is_empty() || building.station.is_some())
            })
            .map(|(id, building)| (id.clone(), building.clone()))
            .expect("shipping catalog has a fully authored building");
        state.selected_building = Some(id.clone());
        state.building_draft = building_draft(&state.catalog, &id);
        state
            .building_draft
            .as_mut()
            .unwrap()
            .value
            .display_name
            .push_str(" Edited");

        apply_building_draft(&mut state).unwrap();

        let mut expected = before;
        expected.display_name.push_str(" Edited");
        assert_eq!(state.catalog.buildings[&id], expected);
    }

    #[test]
    fn logical_footprint_sync_updates_the_runtime_archetype_record() {
        let mut state = ToolState::default();
        let building = state.catalog.buildings.keys().next().unwrap().clone();
        state.selected_building = Some(building.clone());
        state.building_draft = building_draft(&state.catalog, &building);
        let archetype = state
            .building_draft
            .as_ref()
            .unwrap()
            .value
            .archetype
            .clone();
        state.building_draft.as_mut().unwrap().value.footprint = [7, 3];

        apply_building_draft(&mut state).unwrap();

        assert_eq!(state.catalog.buildings[&building].footprint, [7, 3]);
        assert_eq!(state.catalog.archetypes[&archetype].footprint, [7, 3]);
        state.building_draft.as_mut().unwrap().value.footprint = [0, 3];
        assert!(apply_building_draft(&mut state).is_err());
    }

    #[test]
    fn role_preview_uses_shipping_rig_animation_and_composition_rules() {
        let state = ToolState::default();
        let logger = state
            .catalog
            .roles
            .get(&StableId::new("role:logger").unwrap())
            .unwrap();
        let animation_state =
            matching_role_animation_state(&state.presentation, &logger.action_animation).unwrap();
        let (asset_path, animation_index, _) =
            role_preview_animation_request(&state.presentation, &animation_state).unwrap();
        assert_eq!(asset_path, PLAYER_ANIMATED_MODEL_PATH);
        assert_eq!(
            animation_index, 24,
            "logger preview should use CharacterWoodCutting"
        );

        let equipment = logger.equipment.as_ref().unwrap();
        let idle = role_preview_visible_nodes(
            logger,
            2,
            false,
            Some("Eyes_Normal"),
            Some("Hair_Short_Normal"),
            None,
        );
        assert!(idle.contains(&equipment.body_nodes[2]));
        if let Some(left_hand) = equipment.left_hand_node.as_ref() {
            assert_eq!(idle.contains(left_hand), equipment.left_hand_permanent);
            let carrying = role_preview_visible_nodes(
                logger,
                2,
                true,
                Some("Eyes_Normal"),
                Some("Hair_Short_Normal"),
                None,
            );
            assert!(carrying.contains(left_hand));
        }
    }

    #[test]
    fn technology_catalog_save_is_atomic_validated_and_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("catalog.ron");
        let catalog = ToolState::default().catalog;

        save_content_catalog(&catalog, path.to_str().unwrap()).unwrap();
        save_content_catalog(&catalog, path.to_str().unwrap()).unwrap();

        let reloaded: ContentCatalog = ron::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(reloaded, catalog);
        assert!(PathBuf::from(format!("{}.bak", path.display())).is_file());
        assert!(!PathBuf::from(format!("{}.tmp", path.display())).exists());
    }

    #[test]
    fn technology_layout_save_is_atomic_validated_and_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("technology_layout.ron");
        let state = ToolState::default();

        save_technology_layout(
            &state.technology_layout,
            &state.catalog,
            path.to_str().unwrap(),
        )
        .unwrap();
        save_technology_layout(
            &state.technology_layout,
            &state.catalog,
            path.to_str().unwrap(),
        )
        .unwrap();

        let reloaded = load_technology_layout(path.to_str().unwrap(), &state.catalog).unwrap();
        assert_eq!(reloaded, state.technology_layout);
        assert!(PathBuf::from(format!("{}.bak", path.display())).is_file());
        assert!(!PathBuf::from(format!("{}.tmp", path.display())).exists());
    }

    #[test]
    fn presentation_save_is_atomic_validated_and_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("presentation.ron");
        let presentation = ToolState::default().presentation;

        save_presentation_catalog(&presentation, path.to_str().unwrap()).unwrap();
        save_presentation_catalog(&presentation, path.to_str().unwrap()).unwrap();

        let reloaded: PresentationCatalog =
            ron::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(reloaded, presentation);
        assert!(PathBuf::from(format!("{}.bak", path.display())).is_file());
        assert!(!PathBuf::from(format!("{}.tmp", path.display())).exists());
    }

    #[test]
    fn game_config_save_is_atomic_validated_and_round_trips() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("game.ron");
        let mut config = GameConfig::default();
        config.world.seed = 42;

        save_game_config(&config, path.to_str().unwrap()).unwrap();
        save_game_config(&config, path.to_str().unwrap()).unwrap();

        assert_eq!(load_game_config(path.to_str().unwrap()).unwrap(), config);
        assert!(PathBuf::from(format!("{}.bak", path.display())).is_file());
        assert!(!PathBuf::from(format!("{}.tmp", path.display())).exists());
        config.world.cell_size = 0.0;
        assert!(save_game_config(&config, path.to_str().unwrap()).is_err());
        assert_eq!(
            load_game_config(path.to_str().unwrap()).unwrap().world.seed,
            42
        );
    }

    #[test]
    fn authoring_apply_preserves_the_complete_local_twitch_setup() {
        let mut authored = GameConfig::default();
        authored.time.seconds_per_day = 900;
        authored.world.seed = 42;

        let mut runtime = GameConfig::default();
        runtime.twitch.enabled = true;
        runtime.twitch.client_id = "local-public-client-id".to_owned();
        runtime.twitch.bot_login = "localbot".to_owned();
        runtime.twitch.channel_login = "localchannel".to_owned();
        runtime.twitch.broadcast.enabled = true;
        runtime.twitch.broadcast.width = 1_920;
        runtime.twitch.broadcast.height = 1_080;
        runtime.twitch.broadcast.frames_per_second = 60;
        runtime.twitch.broadcast.video_bitrate_kbps = 6_000;
        runtime.twitch.broadcast.audio_bitrate_kbps = 160;
        runtime.twitch.broadcast.encoder = BroadcastEncoderPreference::Amd;
        runtime.twitch.broadcast.ingest = "Sydney".to_owned();

        let merged = merge_authoring_config_with_runtime(&authored, Some(&runtime));

        assert_eq!(merged.time.seconds_per_day, 900);
        assert_eq!(merged.world.seed, 42);
        assert_eq!(merged.twitch, runtime.twitch);
    }

    #[test]
    fn first_authoring_apply_uses_authored_twitch_defaults_without_a_runtime_override() {
        let authored = GameConfig::default();

        let merged = merge_authoring_config_with_runtime(&authored, None);

        assert_eq!(merged, authored);
    }

    #[test]
    fn twitch_tool_normalizes_game_master_ids_and_reward_field() {
        let ids = parse_game_master_ids(" 42,7,42, ");
        assert_eq!(format_game_master_ids(&ids), "42, 7");

        let mut state = ToolState::default();
        state.config.twitch.game_master_ids = ids;
        state.config.twitch.fish_god_reward_id = None;
        state.game_master_ids.clear();
        state.fish_god_reward_id = "stale".to_owned();
        sync_twitch_tool_fields(&mut state);
        assert_eq!(state.game_master_ids, "42, 7");
        assert!(state.fish_god_reward_id.is_empty());
    }

    #[test]
    fn role_editor_applies_every_reference_family_without_partial_mutation() {
        let mut state = ToolState::default();
        let id = StableId::new("role:gatherer").unwrap();
        state.selected_role = Some(id.clone());
        state.role_draft = role_draft(&state.catalog, &id);
        let draft = state.role_draft.as_mut().unwrap();
        draft.value.base_action_amount = 7;
        draft.value.station_kinds = ["station:food", "station:fish"]
            .into_iter()
            .map(|value| StableId::new(value).unwrap())
            .collect();
        draft.value.target_kinds = ["target:bush", "target:fish"]
            .into_iter()
            .map(|value| StableId::new(value).unwrap())
            .collect();
        draft.value.granted_abilities = ["resource:food", "role_flag:resource"]
            .into_iter()
            .map(|value| StableId::new(value).unwrap())
            .collect();

        apply_role_draft(&mut state).unwrap();
        let role = &state.catalog.roles[&id];
        assert_eq!(role.base_action_amount, 7);
        assert!(
            role.station_kinds
                .contains(&StableId::new("station:fish").unwrap())
        );
        state.catalog.validate().unwrap();

        let before = state.catalog.clone();
        state
            .role_draft
            .as_mut()
            .unwrap()
            .value
            .equipment
            .as_mut()
            .unwrap()
            .body_nodes[0]
            .clear();
        assert!(apply_role_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);
    }

    #[test]
    fn foliage_editor_rejects_invalid_generation_values_without_mutation() {
        let mut state = ToolState::default();
        let before = state.catalog.clone();
        state.foliage_draft.as_mut().unwrap().noise_scale = 0.0;

        assert!(apply_foliage_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);
    }

    #[test]
    fn resource_generation_layers_are_explicit_editable_and_validated() {
        let mut state = ToolState::default();
        assert_eq!(state.catalog.resource_generation.len(), 4);
        let baseline =
            stream_town_domain::generate_world_with_content(&state.config.world, &state.catalog)
                .deterministic_hash;

        let before = state.catalog.clone();
        state.resource_generation_draft.as_mut().unwrap().amount = 0;
        assert!(apply_resource_generation_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);

        refresh_resource_generation_draft(&mut state);
        state.new_resource_generation_id = "resource_generation:test".to_owned();
        state.new_resource_generation_name = "Test layer".to_owned();
        duplicate_resource_generation_layer(&mut state).unwrap();
        assert_eq!(state.catalog.resource_generation.len(), 5);
        delete_resource_generation_layer(&mut state).unwrap();
        assert_eq!(state.catalog.resource_generation.len(), 4);
        assert_eq!(
            stream_town_domain::generate_world_with_content(&state.config.world, &state.catalog)
                .deterministic_hash,
            baseline
        );
    }

    #[test]
    fn enemy_camp_generation_layer_lifecycle_is_complete() {
        let mut state = ToolState::default();
        let original_count = state.catalog.enemy_camp_generation.len();
        let before = state.catalog.clone();
        state
            .enemy_camp_generation_draft
            .as_mut()
            .unwrap()
            .minimum_distance_between_camps_milli_cells = 0;
        assert!(apply_enemy_camp_generation_draft(&mut state).is_err());
        assert_eq!(state.catalog, before);

        refresh_enemy_camp_generation_draft(&mut state);
        state.new_enemy_camp_generation_id = "enemy_camp_generation:test".to_owned();
        duplicate_enemy_camp_generation_layer(&mut state).unwrap();
        assert_eq!(
            state.catalog.enemy_camp_generation.len(),
            original_count + 1
        );
        delete_enemy_camp_generation_layer(&mut state).unwrap();
        assert_eq!(state.catalog.enemy_camp_generation.len(), original_count);
        state.catalog.validate().unwrap();
    }

    #[test]
    fn checked_in_authoring_assets_pass_headless_validation() {
        let summary = validate_authoring_assets().unwrap();
        assert!(summary.contains("roles"));
        assert!(summary.contains("technologies"));
    }
}
