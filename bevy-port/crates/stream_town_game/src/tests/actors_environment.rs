#[test]
fn agent_facing_matches_unity_rotation_and_action_targets() {
    let mut smooth = Transform::default();
    rotate_agent_toward(&mut smooth, Vec3::X * 4.0, 0.1, false, false);
    let smooth_visible_forward = smooth.rotation * Vec3::Z;
    assert!(smooth_visible_forward.dot(Vec3::X) > 0.7);
    assert!(smooth_visible_forward.dot(Vec3::X) < 0.999);

    let mut snapped = Transform::default();
    rotate_agent_toward(&mut snapped, Vec3::X * 4.0, 0.1, true, false);
    assert!((snapped.rotation * Vec3::Z).dot(Vec3::X) > 0.999);

    let config = GameConfig::default();
    let content = embedded_content();
    let world = generate_world_with_content(&config.world, &content);
    let mut simulation = WorldSimulation::new(world.seed);
    let actor_id = StableId::new("npc:facing_actor").unwrap();
    let target_id = StableId::new("npc:facing_target").unwrap();
    assert!(simulation.join_player(actor_id, GridPos { x: 4, z: 4 }));
    assert!(simulation.join_player(target_id.clone(), GridPos { x: 9, z: 7 }));
    assert_eq!(
        agent_action_facing_grid(&AgentGoal::Heal(target_id), &content, &simulation, &world,),
        Some(GridPos { x: 9, z: 7 })
    );

    let resource = world
        .resources
        .iter()
        .find(|resource| resource.amount > 0)
        .unwrap();
    assert_eq!(
        agent_action_facing_grid(
            &AgentGoal::Gather(resource.id.clone()),
            &content,
            &simulation,
            &world,
        ),
        Some(resource.position)
    );

    let building_id = StableId::new("building:facing_construction").unwrap();
    let building_definition = &content.buildings[&StableId::new("building:townhall").unwrap()];
    simulation.buildings.insert(
        building_id.clone(),
        BuildingState {
            id: building_id.clone(),
            archetype: building_definition.archetype.clone(),
            position: GridPos { x: 12, z: 4 },
            rotation_quarter_turns: 0,
            level: 1,
            health: 1,
            complete: false,
        },
    );
    let origin = GridPos { x: 4, z: 4 };
    let agent = Agent {
        id: StableId::new("npc:waiting_builder").unwrap(),
        kind: ActorKind::Player,
        archetype: StableId::new("archetype:waiting_builder").unwrap(),
        goal: AgentGoal::Construct(building_id),
        spawn: origin,
        origin,
        navigation_position: placement_to_navigation_centre(origin),
        path: vec![origin],
        path_index: 1,
        target: origin,
        action_cooldown_seconds: 1.0,
        action_started: true,
        repath_remaining_seconds: 0.0,
        health_regen_accumulator: 0.0,
        wander_sequence: 0,
        previous_wander_origin: None,
    };
    let mut transform =
        Transform::from_translation(grid_to_world_on_surface(origin, &config, &world));
    for _ in 0..10 {
        assert!(rotate_agent_toward_action(
            &mut transform,
            &agent,
            &content,
            &simulation,
            &world,
            &config,
            0.05,
            false,
        ));
    }
    let target = grid_to_world_on_surface(
        agent_action_facing_grid(&agent.goal, &content, &simulation, &world).unwrap(),
        &config,
        &world,
    );
    let expected = (target - transform.translation)
        .with_y(0.0)
        .normalize_or_zero();
    assert!((transform.rotation * Vec3::Z).dot(expected) > 0.99);
}

#[test]
fn necromancer_tower_godray_binding_and_converted_vertex_colours_are_packaged() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    let archetype = &content.archetypes[&StableId::new("archetype:building:necrotower").unwrap()];
    let scene = archetype
        .scenes
        .iter()
        .find(|scene| scene.age == Some(2))
        .unwrap();
    let binding = presentation.prefab_renderer_materials[&archetype.source_guid]
        .iter()
        .find(|binding| binding.target_path.ends_with("Env_Godrays_08"))
        .unwrap();
    let material_id = &binding.materials["VFX_Godrays"];
    assert_eq!(
        presentation.materials[material_id].source_path,
        GODRAY_MATERIAL_PATH
    );

    let bytes = std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets")
            .join(&scene.asset_path),
    )
    .unwrap();
    let json_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let json = &bytes[20..20 + json_length];
    let json_end = json
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace() && *byte != 0)
        .unwrap()
        + 1;
    let document: serde_json::Value = serde_json::from_slice(&json[..json_end]).unwrap();
    let godray_material_index = document["materials"]
        .as_array()
        .unwrap()
        .iter()
        .position(|material| material["name"].as_str() == Some("God_rays"))
        .expect("converted necromancer tower retains the God_rays material");
    let ray_primitive = document["meshes"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|mesh| mesh["primitives"].as_array().unwrap())
        .find(|primitive| primitive["material"].as_u64() == Some(godray_material_index as u64))
        .expect("converted necromancer tower contains its God_rays primitive");
    assert!(ray_primitive["attributes"].get("COLOR_0").is_some());
}

#[test]
fn main_menu_logo_resolves_the_shipping_drop_shadow_sprite() {
    let presentation = embedded_presentation();
    let logo = presentation
        .textures
        .values()
        .find(|texture| texture.source_path == GAME_LOGO_TEXTURE_PATH)
        .expect("shipping main-menu logo is packaged");
    assert_eq!(
        logo.asset_path,
        "shipping/textures/Sprites/Miscellaneous/Game_Logo_DropShadow.png"
    );
    assert!((GAME_LOGO_ASPECT_RATIO - 1.669_111_7).abs() < 0.000_001);
    assert!(presentation_texture_handle(&presentation, None, GAME_LOGO_TEXTURE_PATH).is_none());
}

#[test]
fn shipping_top_bar_assets_and_metric_formatting_are_complete() {
    let presentation = embedded_presentation();
    for source_path in TOP_BAR_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing top-bar texture {source_path}"
        );
    }
    assert_eq!(hud_play_time(0.0), "00:00");
    assert_eq!(hud_play_time(125.9), "02:05");
    let seconds_per_day = 120;
    assert!(hud_season_meter_percent(0.0, seconds_per_day).abs() <= f32::EPSILON);
    assert!(
        (hud_season_meter_percent(
            f64::from(seconds_per_day * DAYS_PER_SEASON),
            seconds_per_day,
        ) - 24.0)
            .abs()
            <= f32::EPSILON
    );
    assert!(
        hud_season_meter_percent(
            f64::from(seconds_per_day * DAYS_PER_SEASON * SEASONS_PER_YEAR),
            seconds_per_day,
        )
        .abs()
            <= f32::EPSILON
    );
    assert_eq!(
        season_visual_blend(
            f64::from(seconds_per_day * DAYS_PER_SEASON),
            seconds_per_day,
            Season::Summer,
        ),
        (Season::Spring, Season::Summer, 0.0)
    );
    let (_, _, halfway) = season_visual_blend(
        f64::from(seconds_per_day * DAYS_PER_SEASON) + SEASON_TRANSITION_SECONDS * 0.5,
        seconds_per_day,
        Season::Summer,
    );
    assert!((halfway - 0.5).abs() < 0.001);

    let content = embedded_content();
    let mut simulation = WorldSimulation::new(17);
    assert_eq!(
        hud_technology_summary(&content, &simulation),
        HudTechnologySummary {
            title: "TECHNOLOGY".to_owned(),
            requirement: "Ballot pending".to_owned(),
            progress: 0.0,
        }
    );
    let (technology_id, technology) = content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| !technology.objectives.is_empty())
        .expect("shipping technology contains an objective");
    simulation
        .start_technology_vote(technology_id.clone(), TECHNOLOGY_VOTE_DURATION_SECONDS)
        .unwrap();
    assert_eq!(
        hud_technology_summary(&content, &simulation),
        HudTechnologySummary {
            title: "NEXT TECHNOLOGY".to_owned(),
            requirement: "Vote in chat now".to_owned(),
            progress: 0.0,
        }
    );
    simulation.active_vote = None;
    assert!(simulation.start_technology_goal(
        technology_id.clone(),
        &technology.objectives,
        &content.objectives,
        MAX_TOWN_GOALS,
    ));
    let objective = hud_technology_summary(&content, &simulation);
    assert!(objective.requirement.contains("0/"));
    assert!(objective.progress.abs() <= f32::EPSILON);
}

#[test]
fn level_two_technologies_are_the_first_max_level_caps_after_unlocked_roots() {
    let content = embedded_content();
    for (display_name, building) in [
        ("Level2Stonemason", "building:stonemason"),
        ("Level2TownHall", "building:townhall"),
    ] {
        let building = StableId::new(building).unwrap();
        let technology = content
            .technology
            .nodes
            .values()
            .find(|technology| technology.display_name == display_name)
            .expect("shipping max-level technology");
        assert_eq!(technology.building_level_caps.get(&building), Some(&2));
        assert_eq!(technology.prerequisites.len(), 1);
        assert!(content.technology.nodes[&technology.prerequisites[0]].initially_unlocked);
    }
    assert_eq!(
        compact_technology_label("Level2Stonemason").replace('\n', " "),
        "Stonemason Max Lv 2"
    );
}

#[test]
fn shipping_hud_has_no_clickable_gameplay_controls() {
    fn spawn_test_hud(mut commands: Commands, render: Res<RenderAssets>) {
        spawn_hud(
            &mut commands,
            &render,
            3,
            "0123456789abcdef0123456789abcdef",
        );
    }

    let mut render = RenderAssets::default();
    render.ui_slicers.insert(
        VOTE_TEXTURE_PATHS[0].to_owned(),
        TextureSlicer {
            border: BorderRect::all(158.0),
            center_scale_mode: default(),
            sides_scale_mode: default(),
            max_corner_scale: 1.0,
        },
    );
    let mut app = App::new();
    app.insert_resource(render)
        .add_systems(Startup, spawn_test_hud);
    app.update();

    let mut buttons = app
        .world_mut()
        .query_filtered::<Entity, (With<Button>, With<WorldEntity>)>();
    assert_eq!(buttons.iter(app.world()).count(), 0);
    let mut primary_huds = app.world_mut().query::<&Hud>();
    assert_eq!(primary_huds.iter(app.world()).count(), 1);
    let mut command_guidance = app.world_mut().query::<&HudCommandGuidance>();
    assert_eq!(command_guidance.iter(app.world()).count(), 1);
    assert!(
        !app.world()
            .contains_resource::<PointerObjectSelectionEnabled>()
    );
    let mut strips = app
        .world_mut()
        .query_filtered::<&Node, Or<(With<HudResourceStrip>, With<HudStatsStrip>)>>();
    let strips = strips.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(strips.len(), 2);
    for strip in strips {
        assert_eq!(strip.top, percent(0.0));
        assert_eq!(strip.height, percent(100.0));
    }
    let mut rows = app
        .world_mut()
        .query_filtered::<&Node, With<HudMetricRow>>();
    let rows = rows.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(rows.len(), 8);
    assert!(rows.iter().all(|row| row.align_items == AlignItems::Center));
    assert!(rows.iter().all(|row| row.height == px(44)));
    let mut metrics = app.world_mut().query::<&HudMetric>();
    let metrics = metrics.iter(app.world()).copied().collect::<Vec<_>>();
    assert!(metrics.contains(&HudMetric::Players));
    assert!(metrics.contains(&HudMetric::Npcs));
    let mut maximums = app.world_mut().query::<&HudMetricMaximum>();
    let maximums = maximums
        .iter(app.world())
        .map(|maximum| maximum.0)
        .collect::<Vec<_>>();
    assert_eq!(maximums.len(), 5);
    assert!(maximums.contains(&HudMetric::Food));
    assert!(maximums.contains(&HudMetric::Gold));
    assert!(maximums.contains(&HudMetric::Ore));
    assert!(maximums.contains(&HudMetric::Wood));
    assert!(maximums.contains(&HudMetric::Npcs));
    let resource_strip = app
        .world_mut()
        .query_filtered::<&Node, With<HudResourceStrip>>()
        .single(app.world())
        .unwrap();
    assert_eq!(resource_strip.left, px(18));
    let event_panel = app
        .world_mut()
        .query_filtered::<&Node, With<CurrentEventPanel>>()
        .single(app.world())
        .unwrap();
    assert_eq!(event_panel.top, px(76));
    assert_eq!(event_panel.left, px(18));
    assert_eq!(event_panel.width, px(290));
    assert_eq!(event_panel.height, px(104));
    let (objective_panel, objective_panel_image) = app
        .world_mut()
        .query_filtered::<(&Node, Option<&ImageNode>), With<HudTechnologyObjectivePanel>>()
        .single(app.world())
        .unwrap();
    assert_eq!(objective_panel.width, percent(45.0));
    assert_eq!(objective_panel.overflow, Overflow::clip());
    assert!(objective_panel_image.is_none());
    let top_bar_image = app
        .world_mut()
        .query_filtered::<&ImageNode, With<HudTopBar>>()
        .single(app.world())
        .unwrap();
    assert!(matches!(top_bar_image.image_mode, NodeImageMode::Sliced(_)));
    let mut technology_texts = app.world_mut().query::<(&HudTechnologyTextKind, &Text)>();
    let technology_texts = technology_texts.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(technology_texts.len(), 2);
    assert!(
        technology_texts.iter().any(|(kind, text)| {
            **kind == HudTechnologyTextKind::Title && text.0 == "TECHNOLOGY"
        })
    );
    assert!(technology_texts.iter().any(|(kind, text)| {
        **kind == HudTechnologyTextKind::Requirement && text.0 == "Ballot pending"
    }));
}

#[test]
fn shipping_vote_panels_preserve_art_timers_and_deterministic_tallies() {
    let presentation = embedded_presentation();
    for source_path in VOTE_TEXTURE_PATHS
        .into_iter()
        .chain(std::iter::once(RULER_VOTE_TIMER_UNFILLED_PATH))
    {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing vote texture {source_path}"
        );
    }
    assert_eq!(vote_timer_text(0.0), "00:00");
    assert_eq!(vote_timer_text(30.01), "00:31");
    assert_eq!(vote_timer_text(120.0), "02:00");
    assert!((TECHNOLOGY_VOTE_DURATION_SECONDS - 60.0).abs() <= f32::EPSILON);

    let mut simulation = WorldSimulation::new(17);
    let first = StableId::new("viewer:first").unwrap();
    let second = StableId::new("viewer:second").unwrap();
    simulation.join_player(first.clone(), GridPos { x: 1, z: 1 });
    simulation.join_player(second.clone(), GridPos { x: 2, z: 2 });
    simulation
        .start_technology_vote(StableId::new("tech:test_vote").unwrap(), 30.0)
        .unwrap();
    simulation.cast_vote(&first, true).unwrap();
    simulation.cast_vote(&second, false).unwrap();
    assert_eq!(technology_vote_tally(&simulation), Some((1, 2, 0.5)));

    simulation.active_vote = None;
    simulation
        .start_ruler_vote(RulerVoteKind::NewRuler)
        .unwrap();
    simulation.cast_ruler_vote(&first, second.clone()).unwrap();
    simulation.actors.get_mut(&second).unwrap().display_name = Some("Second Viewer".to_owned());
    assert_eq!(ruler_vote_option_text(&simulation), "Second Viewer  (1)");
    assert!((ruler_vote_option_font_size("Amy  (1)") - 17.0).abs() < f32::EPSILON);
    assert!(ruler_vote_option_font_size("A Very Long Twitch Display Name  (12)") < 17.0);
}

#[test]
fn generated_world_starts_numbered_technology_vote_after_unity_delay() {
    fn spawn_test_vote_panels(mut commands: Commands, render: Res<RenderAssets>) {
        spawn_vote_panels(&mut commands, &render);
    }

    let content = embedded_content();
    let mut simulation = WorldSimulation::new(17);
    simulation.unlocked_technology.extend(
        content
            .technology
            .nodes
            .iter()
            .filter(|(_, technology)| technology.initially_unlocked)
            .map(|(id, _)| id.clone()),
    );
    let seconds_per_day = GameConfig::default().time.seconds_per_day;
    simulation.tick(19.0, seconds_per_day);
    assert!(simulation.active_vote.is_none());
    assert_eq!(simulation.technology_vote_cooldown_seconds, Some(1.0));
    simulation.tick(1.0, seconds_per_day);
    let eligible_before_vote = eligible_technology_ids(&content, &simulation);
    let depths = technology_depths(&content);
    let deepest_available = eligible_before_vote
        .iter()
        .filter_map(|technology| depths.get(technology))
        .max()
        .copied()
        .unwrap();
    let shallowest_available = eligible_before_vote
        .iter()
        .filter_map(|technology| depths.get(technology))
        .min()
        .copied()
        .unwrap();
    simulation
        .start_ruler_vote(RulerVoteKind::NewRuler)
        .expect("ruler ballot may overlap the scheduled technology ballot");

    let mut app = App::new();
    app.insert_resource(RuntimeContent(content.clone()))
        .insert_resource(SimulationRuntime(simulation))
        .insert_resource(RuntimePresentation(embedded_presentation()))
        .insert_resource(RenderAssets::default())
        .add_systems(Startup, spawn_test_vote_panels)
        .add_systems(
            Update,
            (start_scheduled_technology_vote, update_vote_panels).chain(),
        );
    app.update();

    let vote = app
        .world()
        .resource::<SimulationRuntime>()
        .0
        .active_vote
        .as_ref()
        .expect("the generated-world technology ballot starts after Unity's delay");
    assert!(
        app.world()
            .resource::<SimulationRuntime>()
            .0
            .ruler_vote
            .is_some(),
        "starting the technology ballot must not dismiss the ruler ballot"
    );
    assert_eq!(vote.options.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    assert_eq!(vote.technology, vote.options[0]);
    assert_eq!(depths[&vote.options[0]], deepest_available);
    assert_eq!(depths[&vote.options[1]], shallowest_available);
    assert_eq!(
        vote.options.iter().collect::<BTreeSet<_>>().len(),
        vote.options.len()
    );
    assert!(vote.option_votes.is_empty());
    assert!((vote.remaining_seconds - TECHNOLOGY_VOTE_DURATION_SECONDS).abs() <= f32::EPSILON);
    let vote_options = vote.options.clone();
    let announcement = technology_vote_announcement(&content, vote);
    assert!(announcement.starts_with("Technology vote started! 1:"));
    for index in 1..=TECHNOLOGY_VOTE_OPTION_COUNT {
        assert!(announcement.contains(&format!("!vote {index}")));
    }
    let mut vote_tags = app.world_mut().query::<(&VoteTextKind, &Text)>();
    let vote_tags = vote_tags
        .iter(app.world())
        .filter_map(|(kind, text)| {
            let VoteTextKind::TechnologyOptionDepthTag(index) = kind else {
                return None;
            };
            Some((*index, text.0.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(vote_tags.get(&0).map(String::as_str), Some("Specialized"));
    assert_eq!(vote_tags.get(&1).map(String::as_str), Some("Fundamental"));
    assert_eq!(vote_tags.get(&2).map(String::as_str), Some(""));
    let mut depth_badges = app.world_mut().query::<(
        &TechnologyVoteDepthBadge,
        &Visibility,
        &Node,
        &BackgroundColor,
    )>();
    let depth_badges = depth_badges.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(depth_badges.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    for (badge, visibility, node, background) in depth_badges {
        assert_eq!(node.left, px(-78));
        assert_eq!(node.top, px(4));
        assert_eq!(node.width, px(80));
        assert_eq!(node.height, px(20));
        assert_eq!(node.border_radius, BorderRadius::all(px(10)));
        assert_eq!(node.align_items, AlignItems::Center);
        assert_eq!(node.justify_content, JustifyContent::Center);
        assert_eq!(background.0, Color::srgb(0.96, 0.76, 0.20));
        assert_eq!(
            *visibility,
            if badge.0 < 2 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        );
    }
    let mut panels = app
        .world_mut()
        .query_filtered::<(&VotePanelKind, &Visibility, &Node), Without<TechnologyVoteOptionRow>>();
    assert!(panels.iter(app.world()).any(|(kind, visibility, node)| {
        *kind == VotePanelKind::Technology
            && *visibility == Visibility::Visible
            && node.width == px(260)
            && node.height == px(TECHNOLOGY_VOTE_PANEL_HEIGHT)
    }));
    let mut rows = app
        .world_mut()
        .query::<(&TechnologyVoteOptionRow, &Visibility, &Node)>();
    assert_eq!(
        rows.iter(app.world())
            .filter(|(_, visibility, _)| **visibility == Visibility::Visible)
            .count(),
        TECHNOLOGY_VOTE_OPTION_COUNT
    );
    let mut positioned_rows = rows.iter(app.world()).collect::<Vec<_>>();
    positioned_rows.sort_by_key(|(row, _, _)| row.0);
    let mut expected_top = TECHNOLOGY_VOTE_FIRST_ROW_TOP;
    for (row, _, node) in positioned_rows {
        assert_eq!(node.top, px(expected_top));
        let requirement_lines = vote_options
            .get(usize::from(row.0))
            .and_then(|technology| content.technology.nodes.get(technology))
            .map_or(1, |technology| technology.objectives.len().max(1));
        expected_top += technology_vote_row_advance(requirement_lines);
    }
    let mut vote_texts = app.world_mut().query::<(&VoteTextKind, &Text)>();
    let requirements = vote_texts
        .iter(app.world())
        .filter_map(|(kind, text)| {
            let VoteTextKind::TechnologyOptionRequirements(index) = kind else {
                return None;
            };
            Some((*index, text.0.as_str()))
        })
        .collect::<Vec<_>>();
    assert_eq!(requirements.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    for (index, text) in requirements {
        let objective_count = vote_options
            .get(usize::from(index))
            .and_then(|technology| content.technology.nodes.get(technology))
            .map_or(0, |technology| technology.objectives.len());
        assert_eq!(text.lines().count(), objective_count.max(1));
        assert!(!text.contains("Requires:"));
        assert!(!text.contains('•'));
    }
    let mut all_text = app.world_mut().query::<&Text>();
    let all_text = all_text.iter(app.world()).collect::<Vec<_>>();
    assert!(all_text.iter().any(|text| text.0 == "TECHNOLOGY VOTE"));
    assert!(
        all_text
            .iter()
            .all(|text| { !text.0.contains("Vote through chat") && !text.0.contains("!vote") })
    );
    let mut vote_fonts = app.world_mut().query::<(&VoteTextKind, &TextFont)>();
    for (kind, font) in vote_fonts.iter(app.world()) {
        let expected = match kind {
            VoteTextKind::TechnologyTitle => Some(18.0),
            VoteTextKind::TechnologyOptionTitle(_) => Some(13.5),
            VoteTextKind::TechnologyOptionRequirements(_) => Some(11.0),
            VoteTextKind::TechnologyTimer => Some(16.0),
            _ => None,
        };
        if let Some(expected) = expected {
            assert_eq!(font.font_size, FontSize::Px(expected));
        }
    }
    let mut vote_text_nodes = app.world_mut().query::<(&VoteTextKind, &Node)>();
    for (kind, node) in vote_text_nodes.iter(app.world()) {
        match kind {
            VoteTextKind::TechnologyTitle => {
                assert_eq!(node.top, px(TECHNOLOGY_VOTE_TITLE_TOP));
            }
            VoteTextKind::TechnologyTimer => {
                assert_eq!(node.width, Val::Auto);
                assert_eq!(node.top, Val::Auto);
            }
            _ => {}
        }
    }
    let mut vote_text_colors = app.world_mut().query::<(&VoteTextKind, &TextColor)>();
    let timer_color = vote_text_colors
        .iter(app.world())
        .find_map(|(kind, color)| (*kind == VoteTextKind::TechnologyTimer).then_some(color.0))
        .expect("technology timer text has a colour");
    assert_eq!(timer_color, Color::srgb(0.06, 0.08, 0.16));
    let mut vote_text_transforms = app.world_mut().query::<(&VoteTextKind, &UiTransform)>();
    let timer_transform = vote_text_transforms
        .iter(app.world())
        .find_map(|(kind, transform)| {
            (*kind == VoteTextKind::TechnologyTimer).then_some(*transform)
        })
        .expect("technology timer text has a transform");
    assert_eq!(
        timer_transform.translation,
        Val2::new(px(0), px(TECHNOLOGY_VOTE_TIMER_TEXT_OFFSET_Y))
    );
    let mut timer_tracks = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerTrack>>();
    let timer_tracks = timer_tracks.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_tracks.len(), 1);
    assert_eq!(timer_tracks[0].top, px(0));
    assert_eq!(timer_tracks[0].left, px(0));
    assert_eq!(timer_tracks[0].right, px(0));
    assert_eq!(timer_tracks[0].bottom, Val::Auto);
    let mut timer_labels = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerLabel>>();
    let timer_labels = timer_labels.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_labels.len(), 1);
    assert_eq!(timer_labels[0].top, px(0));
    assert_eq!(timer_labels[0].left, px(0));
    assert_eq!(timer_labels[0].right, px(0));
    assert_eq!(timer_labels[0].height, px(20));
    assert_eq!(timer_labels[0].align_items, AlignItems::Center);
    assert_eq!(timer_labels[0].justify_content, JustifyContent::Center);
    let mut timer_groups = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerGroup>>();
    let timer_groups = timer_groups.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_groups.len(), 1);
    assert_eq!(timer_groups[0].top, px(TECHNOLOGY_VOTE_TIMER_GROUP_TOP));
    assert_eq!(timer_groups[0].left, px(0));
    assert_eq!(timer_groups[0].right, px(0));
    assert_eq!(timer_groups[0].height, px(24));
    assert_eq!(timer_groups[0].align_items, AlignItems::Center);
    assert_eq!(timer_groups[0].justify_content, JustifyContent::Center);
    let mut timer_bars = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerBar>>();
    let timer_bars = timer_bars.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_bars.len(), 1);
    assert_eq!(timer_bars[0].width, px(TECHNOLOGY_VOTE_TIMER_BAR_WIDTH));
    assert_eq!(timer_bars[0].height, px(20));
    let mut timer_icons = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerIcon>>();
    let timer_icons = timer_icons.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_icons.len(), 1);
    assert_eq!(timer_icons[0].width, px(24));
    assert_eq!(timer_icons[0].height, px(24));
    let mut timer_gaps = app
        .world_mut()
        .query_filtered::<&Node, With<TechnologyVoteTimerGap>>();
    let timer_gaps = timer_gaps.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(timer_gaps.len(), 1);
    assert_eq!(timer_gaps[0].width, px(TECHNOLOGY_VOTE_TIMER_GAP));
    let mut title_bars = app.world_mut().query::<(&TechnologyVoteTitleBar, &Node)>();
    let title_bars = title_bars.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(title_bars.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    for (_, node) in title_bars {
        assert_eq!(node.top, px(TECHNOLOGY_VOTE_LABEL_OFFSET_Y));
        assert_eq!(node.height, px(28));
        assert_eq!(node.align_items, AlignItems::Center);
        assert_eq!(node.justify_content, JustifyContent::Center);
    }
    let mut vote_icons = app.world_mut().query::<(&TechnologyVoteIcon, &Node)>();
    let vote_icons = vote_icons.iter(app.world()).collect::<Vec<_>>();
    assert_eq!(vote_icons.len(), TECHNOLOGY_VOTE_OPTION_COUNT);
    assert!(
        vote_icons
            .iter()
            .all(|(_, node)| node.top == px(TECHNOLOGY_VOTE_ICON_TOP))
    );

    {
        let mut simulation = app.world_mut().resource_mut::<SimulationRuntime>();
        simulation.0.active_vote = None;
        simulation.0.technology_vote_cooldown_seconds = Some(999.0);
    }
    app.update();
    let mut rows = app
        .world_mut()
        .query::<(&TechnologyVoteOptionRow, &Visibility)>();
    assert!(
        rows.iter(app.world())
            .all(|(_, visibility)| *visibility == Visibility::Hidden)
    );
    let mut fills = app.world_mut().query::<(&VoteFillKind, &Node)>();
    assert!(fills.iter(app.world()).all(|(kind, node)| {
        !matches!(
            kind,
            VoteFillKind::TechnologyTimer | VoteFillKind::TechnologyOption(_)
        ) || node.width == percent(0.0)
    }));
}

#[test]
fn actor_nameplates_identify_twitch_players_without_tagging_npcs() {
    assert!(is_stream_player_actor(
        &StableId::new("twitch:human_bean").unwrap()
    ));
    assert!(!is_stream_player_actor(
        &StableId::new("npc:starting_defender").unwrap()
    ));
    assert!(!is_stream_player_actor(
        &StableId::new("npc:recruit_1").unwrap()
    ));
    assert_eq!(
        PlayerSettings::default().interface.display_names,
        NameDisplayMode::AllPlayers
    );
    let health_bottom = ACTOR_HEALTH_OVERLAY_TOP_PX + 10.0;
    assert!(health_bottom < ACTOR_NAME_OVERLAY_TOP_PX);
}

#[test]
fn ruler_vote_results_produce_the_chat_announcement_that_tick_discards() {
    let mut simulation = WorldSimulation::new(17);
    let winner = StableId::new("viewer:winner").unwrap();
    simulation.join_player(winner.clone(), GridPos { x: 1, z: 1 });
    simulation.actors.get_mut(&winner).unwrap().display_name = Some("Town Hero".to_owned());
    simulation.current_ruler = Some(winner.clone());
    assert_eq!(
        ruler_vote_result_announcement(RulerVoteKind::NewRuler, None, &simulation),
        Some("Town Hero has been elected Ruler of Stream Town!".to_owned())
    );
    assert_eq!(
        ruler_vote_result_announcement(RulerVoteKind::KeepRuler, Some(&winner), &simulation,),
        Some("Chat has chosen to keep Town Hero as Ruler of Stream Town!".to_owned())
    );
    simulation.current_ruler = None;
    assert_eq!(
        ruler_vote_result_announcement(RulerVoteKind::KeepRuler, Some(&winner), &simulation,),
        Some("Town Hero was not retained as Ruler. Nominations are now open.".to_owned())
    );
}

#[test]
fn shipping_top_bar_technology_progress_uses_objective_art_and_unity_labels() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    for source_path in OBJECTIVE_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing objective texture {source_path}"
        );
    }
    let (technology, node) = content
        .technology
        .nodes
        .iter()
        .find(|(_, technology)| !technology.objectives.is_empty())
        .expect("shipping technology graph has objective-backed goals");
    let objective = &content.objectives[&node.objectives[0]];
    let label = objective_display_label(objective);
    assert!(!label.is_empty());
    assert!(!label.contains("resource:"));
    assert!(!label.contains("building:"));
    assert!(!label.contains("enemy:"));
    assert!((objective_progress_ratio(500, 1_000) - 0.5).abs() <= f32::EPSILON);
    assert!((objective_progress_ratio(70_000, 100_000) - 0.7).abs() <= f32::EPSILON);
    assert!((objective_progress_ratio(2, 1) - 1.0).abs() <= f32::EPSILON);
    assert!((objective_progress_ratio(0, 0) - 1.0).abs() <= f32::EPSILON);

    let mut simulation = WorldSimulation::new(17);
    assert!(simulation.start_technology_goal(
        technology.clone(),
        &node.objectives,
        &content.objectives,
        MAX_TOWN_GOALS,
    ));
    let summary = hud_technology_summary(&content, &simulation);
    assert_ne!(summary.title, "TECHNOLOGY");
    assert!(summary.requirement.contains("0/"));
    assert!(summary.progress.abs() <= f32::EPSILON);
}

#[test]
fn shipping_current_event_panel_maps_encounters_and_omits_redundant_active_label() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    for source_path in CURRENT_EVENT_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing current-event texture {source_path}"
        );
    }
    let mut simulation = WorldSimulation::new(17);
    assert!(simulation.start_fish_god(true));
    simulation.fish_god.as_mut().unwrap().praises_given = 7;
    let fish = current_event_panel_state(&simulation).unwrap();
    assert_eq!(fish.0, "Fish God");
    assert_eq!(fish.2, "7  /  20");
    assert!((fish.3 - 0.35).abs() <= f32::EPSILON);

    simulation.stop_active_event();
    let enemy =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Minotaur.prefab").unwrap();
    let boss = archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_MinotaurBoss.prefab")
        .unwrap();
    assert!(simulation.start_raid(5, 50, enemy, boss));
    simulation.active_raid.as_mut().unwrap().current_wave = 2;
    let raid = current_event_panel_state(&simulation).unwrap();
    assert_eq!(raid.0, "Raid");
    assert_eq!(raid.2, "Waves Completed: 2/5");
    assert!((raid.3 - 0.4).abs() <= f32::EPSILON);
    simulation
        .active_raid
        .as_mut()
        .unwrap()
        .tracked_enemies
        .insert(StableId::new("enemy:wave").unwrap());
    let active_wave = current_event_panel_state(&simulation).unwrap();
    assert_eq!(active_wave.1, "Enemies In Wave: 1");
    assert_eq!(active_wave.2, "Waves Completed: 1/5");
    assert!((active_wave.3 - 0.2).abs() <= f32::EPSILON);
    simulation.finish_raid();
    assert!(current_event_panel_state(&simulation).is_none());
    simulation.active_community_event = Some(CommunityEvent::Market);
    let market = current_event_panel_state(&simulation).unwrap();
    assert_eq!(market.0, "Market");
    assert!(market.2.is_empty());
    assert!((market.3 - 1.0).abs() <= f32::EPSILON);
}

#[test]
fn selection_panel_uses_packaged_sliders_and_authoritative_actor_state() {
    let content = embedded_content();
    let presentation = embedded_presentation();
    for source_path in SELECTION_PANEL_TEXTURE_PATHS {
        assert!(
            presentation
                .textures
                .values()
                .any(|texture| texture.source_path == source_path),
            "missing selection slider texture {source_path}"
        );
    }
    let position = GridPos { x: 4, z: 7 };
    let actor_id = StableId::new("actor:selection_test").expect("test ID");
    let mut simulation = WorldSimulation::new(17);
    simulation.join_player(actor_id.clone(), position);
    let actor = simulation.actors.get_mut(&actor_id).expect("joined actor");
    actor.display_name = Some("Selection Test".to_owned());
    actor.health = 60;
    actor.max_health = 120;
    let world = generate_world(&GameConfig::default().world);

    let details = selection_panel_details(position, &content, &world, &simulation)
        .expect("actor cell should resolve");
    assert!(details.description.contains("Selection Test"));
    assert!(details.description.contains("Level 1/1000"));
    assert!(details.description.contains("Health 60/120"));
    assert_eq!(details.health_progress, Some(0.5));
    assert_eq!(details.experience_progress, Some(0.0));
    assert_eq!(title_case("wood"), "Wood");

    let resource = world
        .resources
        .iter()
        .find(|resource| resource.amount > 0)
        .expect("generated resource");
    let resource_details = selection_panel_details(
        resource.position,
        &content,
        &world,
        &WorldSimulation::new(17),
    )
    .expect("resource cell should resolve");
    assert!(resource_details.description.contains("remaining"));
    assert_eq!(resource_details.health_progress, None);
    assert_eq!(resource_details.experience_progress, None);

    let enemy_archetype =
        archetype_id_by_source(&content, ArchetypeKind::Enemy, "Enemy_Minotaur.prefab")
            .expect("converted enemy archetype");
    let enemy_position = GridPos { x: 12, z: 14 };
    assert!(simulation.spawn_enemy(
        StableId::new("actor:selection_enemy").unwrap(),
        enemy_archetype,
        enemy_position,
        25,
    ));
    let enemy_details = selection_panel_details(enemy_position, &content, &world, &simulation)
        .expect("enemy cell should resolve");
    assert!(enemy_details.description.starts_with("Minotaur\n"));
    assert!(enemy_details.description.contains("Health 25/25"));
    assert_eq!(enemy_details.health_progress, Some(1.0));
    assert_eq!(enemy_details.experience_progress, None);
}

#[test]
fn selection_panel_and_outline_resolve_the_full_enemy_camp_footprint() {
    let content = embedded_content();
    let world = generate_world(&GameConfig::default().world);
    let (camp_archetype_id, camp_archetype) = content
        .archetypes
        .iter()
        .find(|(_, archetype)| archetype.enemy_spawner.is_some())
        .expect("converted enemy camp archetype");
    let camp_id = StableId::new("enemy_camp:selection_test").unwrap();
    let origin = GridPos { x: 10, z: 12 };
    let selected_cell = GridPos {
        x: origin.x + camp_archetype.footprint[0] - 1,
        z: origin.z + camp_archetype.footprint[1] - 1,
    };
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.enemy_camps.insert(
        camp_id.clone(),
        EnemyCampState {
            id: camp_id,
            archetype: camp_archetype_id.clone(),
            position: origin,
            health: 400,
            spawn_remaining_seconds: 2.0,
            spawned_enemies: BTreeSet::new(),
        },
    );

    assert_eq!(
        selected_enemy_camp_at_cell(selected_cell, &content, &simulation).map(|(camp, _)| &camp.id),
        simulation.enemy_camps.values().next().map(|camp| &camp.id)
    );
    assert_eq!(
        selected_structural_footprint(selected_cell, &content, &simulation),
        Some((origin, camp_archetype.footprint))
    );
    let details = selection_panel_details(selected_cell, &content, &world, &simulation)
        .expect("camp footprint should resolve");
    assert!(details.description.starts_with("Enemy Camp\n"));
    assert!(details.description.contains("Health 400/1000"));
    assert_eq!(details.health_progress, Some(0.4));
    assert_eq!(details.experience_progress, None);
}

#[test]
fn standalone_seagull_resolves_the_authored_critter_material() {
    let id = StableId::new(CRITTER_MATERIAL_ID).expect("critter material id is valid");
    let authored = Handle::<CritterMaterial>::default();
    let mut render = RenderAssets::default();
    render
        .presentation_materials
        .insert(id, ResolvedMaterialHandle::Critter(authored.clone()));

    let resolved = standalone_material_override(&render, CRITTER_MATERIAL_ID)
        .expect("standalone converted scene should resolve its material");
    assert!(resolved.model_materials.is_empty());
    assert!(resolved.renderer_materials.is_empty());
    let Some(ResolvedMaterialHandle::Critter(handle)) = resolved.fallback else {
        panic!("standalone seagull must receive a critter material override");
    };
    assert_eq!(handle, authored);
}

#[test]
fn generated_terrain_mesh_matches_navigation_grid() {
    let config = GameConfig::default();
    let world = generate_world(&config.world);
    let mesh = generated_terrain_mesh(&world, &config);
    let face_count =
        usize::from(config.world.width * 2 - 1) * usize::from(config.world.height * 2 - 1);
    assert_eq!(mesh.count_vertices(), face_count * 4);
    assert_eq!(mesh.indices().unwrap().len(), face_count * 6);
    assert_eq!(
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().len(),
        mesh.count_vertices()
    );
    assert_eq!(
        mesh.attribute(Mesh::ATTRIBUTE_COLOR).unwrap().len(),
        mesh.count_vertices()
    );
    let bevy::mesh::VertexAttributeValues::Float32x3(normals) =
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap()
    else {
        panic!("terrain normals must use float triples");
    };
    assert!(
        normals.iter().all(|normal| normal[1] >= -f32::EPSILON),
        "reflected row-connector winding must not point terrain faces downward"
    );
    let centre = GridPos {
        x: config.world.width / 2,
        z: config.world.height / 2,
    };
    assert!(
        (grid_to_world_on_surface(centre, &config, &world).y - terrain_height(&world, centre))
            .abs()
            <= f32::EPSILON
    );
    let water = generated_water_mesh(&world, &config);
    let padded_width = usize::from(config.world.width + OCEAN_PADDING_CELLS * 2);
    let padded_height = usize::from(config.world.height + OCEAN_PADDING_CELLS * 2);
    assert_eq!(
        water.count_vertices(),
        (padded_width + 1) * (padded_height + 1)
    );
    assert_eq!(
        water.indices().unwrap().len(),
        padded_width * padded_height * 6
    );
    let bevy::mesh::VertexAttributeValues::Float32x4(depth_colors) =
        water.attribute(Mesh::ATTRIBUTE_COLOR).unwrap()
    else {
        panic!("water depth must use float vertex colors");
    };
    assert!(
        depth_colors
            .iter()
            .all(|color| (0.0..=1.0).contains(&color[0]))
    );
    assert!(depth_colors.iter().any(|color| color[0] == 0.0));
    assert!(depth_colors.iter().any(|color| color[0] > 0.0));
}

#[test]
fn generated_terrain_chunks_cover_the_grid_with_watertight_seams() {
    let mut config = GameConfig::default();
    config.world.width = 64;
    config.world.height = 64;
    let world = generate_world(&config.world);
    let chunks = generated_terrain_chunks(&world, &config);
    assert_eq!(chunks.len(), 16);
    assert_eq!(
        chunks
            .iter()
            .map(|chunk| chunk.high.indices().unwrap().len())
            .sum::<usize>(),
        64 * 64 * 6
    );
    assert!(chunks.iter().all(|chunk| {
        chunk.high.count_vertices() == 17 * 17
            && Collider::trimesh_from_mesh(&chunk.high).is_some()
            && chunk.medium.indices().unwrap().len() < chunk.high.indices().unwrap().len()
            && chunk.low.indices().unwrap().len() < chunk.medium.indices().unwrap().len()
    }));

    let positions = |chunk_x: u16, chunk_z: u16| {
        let mesh = &chunks
            .iter()
            .find(|chunk| chunk.chunk_x == chunk_x && chunk.chunk_z == chunk_z)
            .unwrap()
            .high;
        let bevy::mesh::VertexAttributeValues::Float32x3(positions) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!("terrain positions must use float triples");
        };
        positions.clone()
    };
    let left = positions(0, 0);
    let right = positions(1, 0);
    for row in 0..=usize::from(TERRAIN_CHUNK_CELLS) {
        let left_seam = left[row * 17 + usize::from(TERRAIN_CHUNK_CELLS)];
        let right_seam = right[row * 17];
        assert!(
            left_seam
                .into_iter()
                .zip(right_seam)
                .all(|(left, right)| { (left - right).abs() <= f32::EPSILON })
        );
    }

    let mut uneven = config.clone();
    uneven.world.width = 35;
    uneven.world.height = 19;
    let uneven_world = generate_world(&uneven.world);
    let uneven_chunks = generated_terrain_chunks(&uneven_world, &uneven);
    assert_eq!(uneven_chunks.len(), 6);
    assert_eq!(
        uneven_chunks
            .iter()
            .map(|chunk| chunk.high.indices().unwrap().len())
            .sum::<usize>(),
        35 * 19 * 6
    );
}

#[test]
fn terrain_lod_is_hysteretic_and_keeps_partial_chunk_boundaries() {
    assert_eq!(
        terrain_lod_for_distance(TerrainLodLevel::High, TERRAIN_HIGH_DETAIL_RADIUS),
        TerrainLodLevel::High
    );
    assert_eq!(
        terrain_lod_for_distance(
            TerrainLodLevel::High,
            TERRAIN_HIGH_DETAIL_RADIUS + TERRAIN_LOD_HYSTERESIS + 1.0,
        ),
        TerrainLodLevel::Medium
    );
    assert_eq!(
        terrain_lod_for_distance(TerrainLodLevel::Medium, TERRAIN_HIGH_DETAIL_RADIUS),
        TerrainLodLevel::Medium
    );
    assert_eq!(
        terrain_lod_for_distance(
            TerrainLodLevel::Medium,
            TERRAIN_HIGH_DETAIL_RADIUS - TERRAIN_LOD_HYSTERESIS - 1.0,
        ),
        TerrainLodLevel::High
    );
    assert_eq!(terrain_lod_samples(15, 4), vec![0, 4, 8, 12, 15]);
    assert_eq!(terrain_lod_samples(16, 4), vec![0, 4, 8, 12, 16]);

    let focus = camera_ground_focus(&default_town_camera_transform());
    let authored_focus = Vec2::new(
        UNITY_TOWN_CAMERA_OFFSET.x - UNITY_TOWN_CAMERA_FOCUS_BACK_SHIFT
            + UNITY_TOWN_CAMERA_OFFSET.y,
        UNITY_TOWN_CAMERA_OFFSET.z,
    );
    assert!(focus.distance(authored_focus) < 0.000_1);
}

#[test]
fn foliage_ranges_scale_without_exceeding_the_streaming_budget() {
    assert!(
        (foliage_visibility_distance(Vec3::ZERO) - FOLIAGE_VISIBILITY_MIN_RANGE).abs()
            <= f32::EPSILON
    );
    assert!(
        (foliage_visibility_distance(Vec3::splat(100.0)) - FOLIAGE_VISIBILITY_MAX_RANGE).abs()
            <= f32::EPSILON
    );
    assert!(
        foliage_visibility_distance(Vec3::splat(8.0))
            > foliage_visibility_distance(Vec3::splat(2.0))
    );
}

#[test]
fn shipping_foliage_batch_membership_is_deterministic_and_compact() {
    let config = GameConfig::default();
    let content = embedded_content();
    let first = generate_world_with_content(&config.world, &content);
    let second = generate_world_with_content(&config.world, &content);
    let first_keys = first
        .foliage
        .iter()
        .map(foliage_batch_key)
        .collect::<BTreeSet<_>>();
    let second_keys = second
        .foliage
        .iter()
        .map(foliage_batch_key)
        .collect::<BTreeSet<_>>();
    let gpu_batches = first
        .foliage
        .iter()
        .map(|foliage| (foliage.layer.clone(), foliage.variant))
        .collect::<BTreeSet<_>>();

    assert_eq!(first.deterministic_hash, second.deterministic_hash);
    assert_eq!(first_keys, second_keys);
    assert_eq!(first.foliage.len(), 37_320);
    assert_eq!(first_keys.len(), 293);
    assert_eq!(gpu_batches.len(), 12);
    assert!(first_keys.len() * 8 < first.foliage.len());
    assert!(first_keys.iter().all(|key| {
        key.chunk_x <= config.world.width.saturating_sub(1) / FOLIAGE_BATCH_CHUNK_CELLS
            && key.chunk_z <= config.world.height.saturating_sub(1) / FOLIAGE_BATCH_CHUNK_CELLS
    }));
}

#[test]
fn crowd_separation_is_deterministic_bounded_and_balanced() {
    let agents = vec![
        (StableId::new("actor:alpha").unwrap(), Vec2::new(10.0, 10.0)),
        (StableId::new("actor:beta").unwrap(), Vec2::new(10.0, 10.0)),
        (StableId::new("actor:far").unwrap(), Vec2::new(50.0, 50.0)),
    ];
    let first = crowd_separation_offsets(&agents, 5.0, 2.0);
    let second = crowd_separation_offsets(&agents, 5.0, 2.0);
    assert_eq!(first, second);
    assert!(first[0].length() > 0.0);
    assert!((first[0] + first[1]).length() < 0.001);
    assert_eq!(first[2], Vec2::ZERO);
    assert!(first.iter().all(|offset| offset.length() <= 2.0 + 0.001));
}

#[test]
fn predictive_crowd_yielding_is_deterministic_and_preserves_right_of_way() {
    let alpha = StableId::new("actor:alpha").unwrap();
    let beta = StableId::new("actor:beta").unwrap();
    let far = StableId::new("actor:far").unwrap();
    let agents = vec![
        (alpha.clone(), Vec2::new(-0.5, 0.0), Vec2::X),
        (beta.clone(), Vec2::new(0.5, 0.0), Vec2::NEG_X),
        (far.clone(), Vec2::new(0.0, 8.0), Vec2::X),
    ];
    let first = predictive_speed_factors(&agents, 0.5, 1.0, 0.18);
    let second = predictive_speed_factors(&agents, 0.5, 1.0, 0.18);
    assert_eq!(first, second);
    assert!((first[&alpha] - 1.0).abs() <= f32::EPSILON);
    assert!(first[&beta] < 1.0 && first[&beta] >= 0.18);
    assert!((first[&far] - 1.0).abs() <= f32::EPSILON);

    let follower = StableId::new("actor:follower").unwrap();
    let leader = StableId::new("actor:leader").unwrap();
    let following = predictive_speed_factors(
        &[
            (follower.clone(), Vec2::ZERO, Vec2::X),
            (leader.clone(), Vec2::new(0.3, 0.0), Vec2::X * 0.5),
        ],
        0.25,
        1.0,
        0.18,
    );
    assert!(following[&follower] < 1.0);
    assert!((following[&leader] - 1.0).abs() <= f32::EPSILON);
}

#[test]
fn embedded_config_matches_shipping_starting_roster() {
    let config = GameConfig::default();
    assert_eq!(config.gameplay.initial_agents, 5);
    let world = generate_world(&config.world);
    assert_eq!(world.navigation.width(), config.world.width);
}

#[test]
fn initial_actor_identities_match_unity_starting_npc_roles() {
    let expected = [
        ("npc:starting_defender", Some("role:defender")),
        ("npc:starting_logger", Some("role:logger")),
        ("npc:starting_miner", Some("role:miner")),
        ("npc:starting_gatherer", Some("role:gatherer")),
        ("npc:starting_builder", Some("role:builder")),
    ];
    for (index, (expected_id, expected_role)) in expected.into_iter().enumerate() {
        let (id, role) = initial_actor_identity(u16::try_from(index).unwrap());
        assert_eq!(id, expected_id);
        assert_eq!(role, expected_role);
    }
    assert_eq!(
        initial_actor_identity(5),
        ("actor:viewer_0005".to_owned(), None)
    );
}

#[test]
fn embedded_unity_content_catalog_is_valid() {
    let content = embedded_content();
    assert_eq!(content.archetypes.len(), 221);
    assert_eq!(content.foliage.len(), 4);
    assert_eq!(
        content
            .foliage
            .iter()
            .map(|layer| layer.variants.len())
            .sum::<usize>(),
        21
    );
    assert_eq!(content.buildings.len(), 32);
    assert_eq!(content.roles.len(), 18);
    assert_eq!(content.technology.nodes.len(), 598);
    assert_eq!(content.technology.groups.len(), 22);
    let guardhouse_id = StableId::new("building:guardhouse").unwrap();
    let guardhouse = &content.buildings[&guardhouse_id];
    let tower = &content.buildings[&StableId::new("building:tower").unwrap()];
    assert_eq!(guardhouse.footprint, tower.footprint);
    assert_eq!(guardhouse.placement_footprint_thirds, Some([6, 6]));
    assert_eq!(guardhouse.navigation_footprint_thirds, Some([4, 4]));
    assert!(guardhouse.projectile_shooter.is_none());
    for resource in ["food", "gold", "ore", "wood"] {
        let resource = StableId::new(format!("resource:{resource}")).unwrap();
        assert_eq!(guardhouse.cost[&resource], tower.cost[&resource] * 3);
    }
    let guardhouse_tech =
        &content.technology.nodes[&StableId::new("tech:native_guardhouse").unwrap()];
    assert_eq!(guardhouse_tech.age, "Age 1");
    assert!(!guardhouse_tech.initially_unlocked);
    assert!(guardhouse_tech.unlocked_buildings.contains(&guardhouse_id));
    assert_eq!(
        content.objectives[&StableId::new("objective:native_guardhouse:0").unwrap()]
            .required_amount,
        30_000
    );
    let path = &content.buildings[&StableId::new("building:path").unwrap()];
    assert_eq!(path.display_name, "Path");
    assert_eq!(path.footprint, [1, 1]);
    assert!(path.placeable && path.can_level && path.model_handlers.is_empty());
    assert_eq!(path.cost[&StableId::new("resource:wood").unwrap()], 150);
    assert_eq!(path.cost[&StableId::new("resource:ore").unwrap()], 130);
    assert_eq!(path.construction_cost_multiplier_per_thousand, 0);
    let streetlight_id = StableId::new("building:streetlight").unwrap();
    let streetlight = &content.buildings[&streetlight_id];
    assert_eq!(streetlight.placement_footprint_thirds, Some([1, 1]));
    assert_eq!(streetlight.navigation_footprint_thirds, Some([1, 1]));
    assert_eq!(streetlight.construction_cost_multiplier_per_thousand, 0);
    assert!(
        content.technology.nodes[&StableId::new("tech:native_path_unlock").unwrap()]
            .unlocked_buildings
            .contains(&streetlight_id)
    );
    assert_eq!(
        content.technology.nodes[&StableId::new("tech:native_path_level13").unwrap()]
            .building_level_caps[&StableId::new("building:path").unwrap()],
        13
    );
    let logger = &content.roles[&StableId::new("role:logger").unwrap()];
    assert_eq!(logger.base_action_amount, 1);
    assert_eq!(logger.experience_multiplier_per_thousand, 1_000);
    assert_eq!(logger.action_amount_per_level_milli, 500);
    assert_eq!(logger.base_action_milliseconds, 1_000);
    assert_eq!(logger.action_milliseconds_reduction_per_level, 5);
    assert_eq!(logger.base_carry_capacity, 10);
    assert_eq!(logger.carry_capacity_per_level_milli, 2_000);
    assert_eq!(
        logger.resource.as_ref().map(StableId::as_str),
        Some("resource:wood")
    );
    let ranger = &content.roles[&StableId::new("role:ranger").unwrap()];
    assert_eq!(ranger.base_action_range_milli_cells, 6_000);
    let monastery_id = StableId::new("building:monastery").unwrap();
    let monastery = &content.buildings[&monastery_id];
    let monastery_archetype = &content.archetypes[&monastery.archetype];
    assert!(
        default_archetype_scene(monastery_archetype)
            .unwrap()
            .asset_path
            .ends_with("Buildings/Archive/Monastery.glb")
    );
    assert_eq!(monastery.model_handlers.len(), 1);
    assert_eq!(monastery.model_handlers[0].age, 1);
    assert_eq!(monastery.model_handlers[0].full_model, "Monastery");
}

fn regeneration_role_fixture(
    role_name: &str,
    building_name: &str,
) -> (
    GameConfig,
    ContentCatalog,
    GeneratedWorld,
    WorldSimulation,
    RegenerationRoleRuntime,
    StableId,
    GridPos,
) {
    let config = GameConfig::default();
    let content = embedded_content();
    let mut world = generate_world_with_content(&config.world, &content);
    let role = StableId::new(format!("role:{role_name}")).unwrap();
    let building_id = StableId::new(format!("building:{building_name}")).unwrap();
    let definition = &content.buildings[&building_id];
    let hut = find_building_site(
        &world,
        GridPos {
            x: config.world.width / 2,
            z: config.world.height / 2,
        },
        definition.footprint,
    )
    .expect("regeneration test hut has a valid site");
    let runtime_id = StableId::new(format!("building:test_{building_name}")).unwrap();
    let mut simulation = WorldSimulation::new(world.seed);
    simulation.buildings.insert(
        runtime_id.clone(),
        BuildingState {
            id: runtime_id.clone(),
            archetype: definition.archetype.clone(),
            position: hut,
            rotation_quarter_turns: 0,
            level: 1,
            health: 50,
            complete: true,
        },
    );
    let region = building_region(hut, definition.footprint, &world).unwrap();
    world.navigation.set_blocked(region, true).unwrap();
    let actor_position = nearest_walkable(&world, hut).unwrap();
    let actor = StableId::new(format!("twitch:test_{role_name}")).unwrap();
    assert!(simulation.join_player(actor.clone(), actor_position));
    simulation.assign_role(&actor, role.clone()).unwrap();
    simulation
        .actors
        .get_mut(&actor)
        .unwrap()
        .role_progression
        .insert(
            role,
            stream_town_domain::RoleProgress {
                level: 100,
                experience: 0,
            },
        );
    let mut runtime = RegenerationRoleRuntime {
        elapsed_seconds: 1_000.0,
        ..default()
    };
    runtime.workers.insert(
        actor.clone(),
        RegenerationWorkerState {
            initialized: true,
            next_ready_seconds: 0.0,
            station_visit_required: false,
            last_station: Some(runtime_id),
            prospector_step: 0,
            ..default()
        },
    );
    (config, content, world, simulation, runtime, actor, hut)
}

#[test]
fn planting_roles_visit_their_station_before_each_ready_cycle() {
    let (config, content, mut world, mut simulation, mut runtime, actor, _) =
        regeneration_role_fixture("forester", "nursery");
    let from = simulation.actors[&actor].position;
    let worker = runtime.workers.get_mut(&actor).unwrap();
    worker.station_visit_required = true;
    worker.last_station = None;

    let (visit_goal, approach) =
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .expect("ready forester first visits the nursery");
    let AgentGoal::VisitRegenerationStation(station_id) = &visit_goal else {
        panic!("forester skipped the nursery before planting");
    };
    let _ = complete_agent_goal_with_regeneration(
        &mut simulation,
        &mut world,
        &config,
        &content,
        &mut runtime,
        &actor,
        &visit_goal,
        approach,
    );
    assert_eq!(
        runtime.workers[&actor].last_station.as_ref(),
        Some(station_id)
    );
    assert!(!runtime.workers[&actor].station_visit_required);
    assert_eq!(
        actor_idle_anchor(&content, &simulation, &config, &runtime, &actor, approach),
        simulation.buildings[station_id].position,
        "idle wandering remains centered on the last visited nursery"
    );
    assert!(!runtime.workers[&actor].station_visit_required);
}

#[test]
fn losing_the_last_role_building_immediately_reassigns_citizens_to_defender() {
    let (_, content, _, mut simulation, _, actor, _) =
        regeneration_role_fixture("forester", "nursery");
    let first_id = simulation.buildings.keys().next().unwrap().clone();
    let mut second = simulation.buildings[&first_id].clone();
    second.id = StableId::new("building:test_nursery_second").unwrap();
    simulation
        .buildings
        .insert(second.id.clone(), second.clone());

    simulation.buildings.remove(&first_id);
    assert!(reassign_roles_without_prerequisite_buildings(&content, &mut simulation).is_empty());
    assert_eq!(simulation.actors[&actor].role.as_str(), "role:forester");

    simulation.buildings.remove(&second.id);
    assert_eq!(
        reassign_roles_without_prerequisite_buildings(&content, &mut simulation),
        vec![actor.clone()]
    );
    assert_eq!(simulation.actors[&actor].role.as_str(), "role:defender");
}

#[test]
fn regeneration_roles_and_buildings_execute_their_authored_tasks() {
    let (_, content, mut world, mut simulation, mut runtime, actor, hut) =
        regeneration_role_fixture("forester", "nursery");
    let from = simulation.actors[&actor].position;
    let fallen = (0..world.navigation.height())
        .flat_map(|z| (0..world.navigation.width()).map(move |x| GridPos { x, z }))
        .find(|candidate| {
            grid_distance_squared(*candidate, hut) >= 100
                && valid_regeneration_cell(&content, &simulation, &world, *candidate)
                && planting_approach(&content, &simulation, &world, *candidate, from).is_some()
        })
        .expect("test world has a reachable former tree site");
    runtime.recently_fallen_trees.push_back(fallen);
    let trees_before = world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0 && resource.target_kind.as_str() == "target:tree")
        .count();
    let (goal, _) =
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .expect("ready forester chooses a planting task");
    assert!(matches!(goal, AgentGoal::PlantTree(_)));
    assert!(complete_regeneration_goal(
        &mut simulation,
        &mut world,
        &content,
        &mut runtime,
        &actor,
        &goal,
    ));
    assert_eq!(
            world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0
                    && resource.target_kind.as_str() == "target:tree")
                .count(),
            trees_before + 1
        );
    let planted_tree = world
        .resources
        .iter()
        .find(|resource| resource.id.as_str().starts_with("resource:regrown_tree_"))
        .expect("forester creates a persistable regenerated tree");
    let restored_tree = restored_regenerated_resource(&planted_tree.id, planted_tree.amount)
        .expect("regenerated resource ID restores from a native save");
    assert_eq!(restored_tree, *planted_tree);
    assert!(
        planted_tree
            .offset_milli_cells
            .iter()
            .all(|offset| (-250..=250).contains(offset)),
        "regrown resources retain the central-half-cell visual offset contract"
    );
    assert!(
        (regeneration_role_interval_seconds(&StableId::new("role:forester").unwrap(), 100,)
            .unwrap()
            - 13.4)
            .abs()
            < f64::EPSILON
    );
    assert_eq!(
        regeneration_role_interval_seconds(&StableId::new("role:forester").unwrap(), 1),
        Some(201.0)
    );
    let tender = StableId::new("role:tender").unwrap();
    assert_eq!(
        regeneration_role_interval_seconds(&tender, 1),
        Some(3_600.0)
    );
    assert_eq!(
        regeneration_role_interval_seconds(&tender, 100),
        Some(720.0)
    );
    assert_eq!(prospector_discovery_denominator(1), 4_000);
    assert_eq!(prospector_discovery_denominator(100), 400);

    let (_, content, mut world, mut simulation, mut runtime, actor, hut) =
        regeneration_role_fixture("prospector", "prospector_hut");
    let from = simulation.actors[&actor].position;
    let (spiral_goal, _) =
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .expect("prospector starts the authored spiral");
    let AgentGoal::Prospect { cell, .. } = spiral_goal else {
        panic!("prospector did not receive a prospect goal");
    };
    let radius = cell.x.abs_diff(hut.x).max(cell.z.abs_diff(hut.z));
    assert!((5..=20).contains(&radius));
    let nearby_ores = nearby_active_resource_count(
        &world,
        simulation.actors[&actor].position,
        "target:ore",
        LOCAL_RESOURCE_SATURATION_RADIUS_CELLS,
    );
    let saturated_denominator = prospector_discovery_denominator(100)
        .saturating_mul(u64::from(nearby_ores).saturating_add(1));
    let successful_sequence = (0..1_000_000_u32)
        .find(|sequence| {
            seagull_hash(
                world.seed ^ stable_id_hash(&actor),
                u64::from(*sequence),
                0x5052_4F53_5045_4354,
            )
            .is_multiple_of(saturated_denominator)
        })
        .expect("deterministic prospect sequence contains a discovery");
    let ore_before = world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0 && resource.target_kind.as_str() == "target:ore")
        .count();
    assert!(complete_regeneration_goal(
        &mut simulation,
        &mut world,
        &content,
        &mut runtime,
        &actor,
        &AgentGoal::Prospect {
            cell,
            sequence: successful_sequence,
        },
    ));
    let ore_added = world
        .resources
        .iter()
        .filter(|resource| resource.amount > 0 && resource.target_kind.as_str() == "target:ore")
        .count()
        - ore_before;
    assert!((3..=5).contains(&ore_added));

    let (_, content, mut world, mut simulation, mut runtime, actor, hut) =
        regeneration_role_fixture("tender", "greenhouse");
    let mut tree = world
        .resources
        .iter()
        .find(|resource| resource.target_kind.as_str() == "target:tree")
        .cloned()
        .expect("generated fixture contains a tree template");
    let cleared = world
        .resources
        .iter_mut()
        .filter(|resource| resource.target_kind.as_str() != "target:fish")
        .map(|resource| {
            resource.amount = 0;
            resource.position
        })
        .collect::<Vec<_>>();
    for position in cleared {
        world
            .navigation
            .set_blocked(
                stream_town_domain::DirtyRegion {
                    min: position,
                    max: position,
                },
                false,
            )
            .unwrap();
    }
    let tree_position = (-18_i32..=18)
        .flat_map(|z| (-18_i32..=18).map(move |x| (x, z)))
        .filter(|(x, z)| x.abs().max(z.abs()) >= 12)
        .filter_map(|(x, z)| offset_grid(hut, x, z, &world))
        .find(|position| {
            world.navigation.is_walkable(*position)
                && cell_is_clear_of_buildings(&content, &simulation, &world, *position, 10)
        })
        .expect("fixture has a clear tree position near the greenhouse");
    tree.position = tree_position;
    tree.amount = 100;
    world.resources.push(tree);
    world
        .navigation
        .set_blocked(
            stream_town_domain::DirtyRegion {
                min: tree_position,
                max: tree_position,
            },
            true,
        )
        .unwrap();
    let from = simulation.actors[&actor].position;
    let (goal, _) =
        regeneration_agent_goal(&content, &simulation, &world, &mut runtime, &actor, from)
            .expect("ready tender chooses a field planting task");
    let AgentGoal::PlantBush(field) = goal else {
        panic!("tender did not receive a bush planting goal");
    };
    let second_actor = StableId::new("twitch:test_tender_second").unwrap();
    let tender_role = StableId::new("role:tender").unwrap();
    assert!(simulation.join_player(second_actor.clone(), from));
    simulation
        .assign_role(&second_actor, tender_role.clone())
        .unwrap();
    simulation
        .actors
        .get_mut(&second_actor)
        .unwrap()
        .role_progression
        .insert(
            tender_role,
            stream_town_domain::RoleProgress {
                level: 100,
                experience: 0,
            },
        );
    runtime.workers.insert(
        second_actor.clone(),
        RegenerationWorkerState {
            initialized: true,
            next_ready_seconds: 0.0,
            station_visit_required: false,
            last_station: regeneration_hut(
                &content,
                &simulation,
                &StableId::new("role:tender").unwrap(),
                from,
            )
            .map(|building| building.id.clone()),
            ..default()
        },
    );
    let (second_goal, _) = regeneration_agent_goal(
        &content,
        &simulation,
        &world,
        &mut runtime,
        &second_actor,
        from,
    )
    .expect("a second ready tender chooses a field planting task");
    let AgentGoal::PlantBush(second_field) = second_goal else {
        panic!("second tender did not receive a bush planting goal");
    };
    assert_ne!(
        second_field, field,
        "simultaneous tenders reserve distinct planting destinations"
    );
    assert!(cell_is_clear_of_buildings(
        &content,
        &simulation,
        &world,
        field,
        10,
    ));
    let tree_distance = grid_distance_squared(field, tree_position);
    assert!(
        (9..=36).contains(&tree_distance),
        "tender berries must be between three and six cells from a tree"
    );

    // A destination may have been empty when selected and become occupied
    // while the tender walks there. Completion must revalidate the live
    // actor occupancy instead of growing a blocking bush under a citizen.
    simulation.actors.get_mut(&second_actor).unwrap().position = field;
    assert!(!complete_regeneration_goal(
        &mut simulation,
        &mut world,
        &content,
        &mut runtime,
        &actor,
        &AgentGoal::PlantBush(field),
    ));
    simulation.actors.get_mut(&second_actor).unwrap().position = from;
    assert!(complete_regeneration_goal(
        &mut simulation,
        &mut world,
        &content,
        &mut runtime,
        &actor,
        &AgentGoal::PlantBush(field),
    ));
    assert_eq!(
            world
                .resources
                .iter()
                .filter(|resource| resource.amount > 0
                    && resource.target_kind.as_str() == "target:bush")
                .count(),
            1
        );

    for (technology, building, forbidden_resource) in [
        ("tech:native_nursery", "building:nursery", "resource:wood"),
        (
            "tech:native_prospector_hut",
            "building:prospector_hut",
            "resource:ore",
        ),
        (
            "tech:native_greenhouse",
            "building:greenhouse",
            "resource:food",
        ),
    ] {
        let node = &content.technology.nodes[&StableId::new(technology).unwrap()];
        assert!(node.prerequisites.is_empty());
        assert!(!node.initially_unlocked);
        assert!(
            node.unlocked_buildings
                .contains(&StableId::new(building).unwrap())
        );
        assert!(node.objectives.iter().all(|objective| {
            content.objectives[objective]
                .resource
                .as_ref()
                .map(StableId::as_str)
                != Some(forbidden_resource)
        }));
    }
}
