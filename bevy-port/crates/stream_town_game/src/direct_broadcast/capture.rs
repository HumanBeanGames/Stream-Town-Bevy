fn sync_stream_only_capture_extract(
    config: Res<RuntimeConfig>,
    gameplay_ready: Option<Res<crate::GameplayReady>>,
    sensitive_screen: Res<SensitiveScreenActive>,
    runtime: Res<DirectBroadcastRuntime>,
    state: Res<StreamOnlyCaptureState>,
    mut extracted: ResMut<StreamOnlyCaptureExtract>,
) {
    extracted.target.clone_from(&state.target);
    extracted.video_sink = runtime
        .controller
        .as_ref()
        .map(BroadcastController::video_sink);
    extracted.width = state.width;
    extracted.height = state.height;
    extracted.active = gameplay_ready.is_some()
        && config.0.twitch.broadcast.render_mode == BroadcastRenderMode::StreamOnly
        && !sensitive_screen.0
        && runtime.controller.is_some()
        && matches!(
            runtime.phase,
            DirectBroadcastPhase::Connecting
                | DirectBroadcastPhase::VerifyingTwitch
                | DirectBroadcastPhase::Broadcasting
                | DirectBroadcastPhase::BandwidthTesting
                | DirectBroadcastPhase::Reconnecting
        );
}

fn receive_stream_only_captured_frames(
    inbox: Res<StreamOnlyCaptureInbox>,
    shared: Res<GpuStreamCaptureShared>,
    runtime: Res<DirectBroadcastRuntime>,
) {
    let mut newest = None;
    let Ok(receiver) = inbox.0.lock() else {
        return;
    };
    while let Ok(frame) = receiver.try_recv() {
        shared.0.queued.fetch_sub(1, Ordering::Relaxed);
        if let Some(replaced) = newest.replace(frame) {
            recycle_stream_capture_pixels(replaced.recycle_pool, replaced.pixels);
            shared.0.dropped.fetch_add(1, Ordering::Relaxed);
            shared.0.queue_drops.fetch_add(1, Ordering::Relaxed);
        }
    }
    drop(receiver);
    let dropped = shared.0.dropped.swap(0, Ordering::Relaxed);
    let map_failures = shared.0.map_failures.swap(0, Ordering::Relaxed);
    let Some(controller) = runtime.controller.as_ref() else {
        if let Some(frame) = newest {
            recycle_stream_capture_pixels(frame.recycle_pool, frame.pixels);
        }
        return;
    };
    if dropped > 0 {
        controller.drop_video_frames(dropped);
    }
    if map_failures > 0 {
        controller.drop_video_frames(map_failures);
        warn!(map_failures, "fixed-ring stream capture mappings failed");
    }
    let Some(frame) = newest else {
        return;
    };
    controller
        .metrics
        .observe_capture_latency(frame.captured_at.elapsed());
    let StreamOnlyCapturedFrame {
        width,
        height,
        pixels,
        recycle_pool,
        ..
    } = frame;
    let _ = controller.send_video(VideoFrame {
        width,
        height,
        pixel_format: VideoPixelFormat::Bgra,
        pixels,
        recycle_pool: Some(recycle_pool),
    });
}

fn take_stream_capture_pixels(pool: &Arc<Mutex<Vec<Vec<u8>>>>, length: usize) -> Vec<u8> {
    let mut pixels = pool
        .lock()
        .ok()
        .and_then(|mut buffers| buffers.pop())
        .unwrap_or_default();
    pixels.resize(length, 0);
    pixels
}

fn recycle_stream_capture_pixels(pool: Arc<Mutex<Vec<Vec<u8>>>>, mut pixels: Vec<u8>) {
    pixels.clear();
    if let Ok(mut buffers) = pool.lock()
        && buffers.len() < STREAM_CAPTURE_RECYCLED_PIXEL_BUFFERS
    {
        buffers.push(pixels);
    }
}

fn copy_gpu_rows_into(data: &[u8], width: u32, height: u32, pixels: &mut Vec<u8>) -> bool {
    let row_bytes = usize::try_from(width).unwrap_or_default().saturating_mul(4);
    let aligned_row_bytes = row_bytes.div_ceil(256).saturating_mul(256);
    let height = usize::try_from(height).unwrap_or_default();
    let source_length = aligned_row_bytes.saturating_mul(height);
    let target_length = row_bytes.saturating_mul(height);
    if row_bytes == 0 || height == 0 || data.len() < source_length {
        pixels.clear();
        return false;
    }
    pixels.resize(target_length, 0);
    if row_bytes == aligned_row_bytes {
        pixels.copy_from_slice(&data[..target_length]);
    } else {
        for row in 0..height {
            let source = row.saturating_mul(aligned_row_bytes);
            let destination = row.saturating_mul(row_bytes);
            pixels[destination..destination + row_bytes]
                .copy_from_slice(&data[source..source + row_bytes]);
        }
    }
    true
}

fn configure_stream_capture_ring(
    ring: &mut GpuStreamCaptureRing,
    render_device: &RenderDevice,
    width: u32,
    height: u32,
) -> bool {
    if ring.width == width && ring.height == height && !ring.slots.is_empty() {
        return true;
    }
    if ring
        .slots
        .iter()
        .any(|slot| slot.busy.load(Ordering::Acquire))
    {
        return false;
    }
    let row_bytes = width.saturating_mul(4);
    let aligned_row_bytes = u32::try_from(RenderDevice::align_copy_bytes_per_row(
        usize::try_from(row_bytes).unwrap_or(usize::MAX),
    ))
    .unwrap_or(u32::MAX);
    let buffer_bytes = u64::from(aligned_row_bytes).saturating_mul(u64::from(height));
    if width == 0 || height == 0 || buffer_bytes == 0 {
        return false;
    }
    ring.slots = (0..STREAM_CAPTURE_STAGING_BUFFERS)
        .map(|_| GpuStreamCaptureSlot {
            buffer: render_device.create_buffer(&BufferDescriptor {
                label: Some("stream-town-fixed-capture-staging"),
                size: buffer_bytes,
                usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            busy: Arc::new(AtomicBool::new(false)),
        })
        .collect();
    ring.width = width;
    ring.height = height;
    ring.aligned_row_bytes = aligned_row_bytes;
    ring.next_slot = 0;
    true
}

fn capture_stream_only_target(
    extracted: Res<StreamOnlyCaptureExtract>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    images: Res<GpuRenderAssets<GpuImage>>,
    mut ring: ResMut<GpuStreamCaptureRing>,
) {
    let _ = render_device.poll(PollType::Poll);
    if !extracted.active {
        return;
    }
    let Some(target) = extracted.target.as_ref() else {
        return;
    };
    let Some(image) = images.get(target) else {
        return;
    };
    if !configure_stream_capture_ring(&mut ring, &render_device, extracted.width, extracted.height)
    {
        return;
    }
    // The Winit producer itself runs at the configured stream cadence, so each
    // rendered offscreen frame must be captured. A second deadline gate here
    // previously forced the producer to run at 7x cadence and was the source of
    // the accumulating transform/upload workload.
    let now = Instant::now();
    let slot_count = ring.slots.len();
    let slot_index = (0..slot_count)
        .map(|offset| (ring.next_slot + offset) % slot_count)
        .find(|index| {
            ring.slots[*index]
                .busy
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        });
    let Some(slot_index) = slot_index else {
        ring.shared.0.dropped.fetch_add(1, Ordering::Relaxed);
        ring.shared.0.slot_misses.fetch_add(1, Ordering::Relaxed);
        return;
    };
    ring.next_slot = (slot_index + 1) % slot_count;
    ring.shared.0.in_flight.fetch_add(1, Ordering::Relaxed);

    let slot = &ring.slots[slot_index];
    let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("stream-town-fixed-capture-copy"),
    });
    encoder.copy_texture_to_buffer(
        image.texture.as_image_copy(),
        TexelCopyBufferInfo {
            buffer: &slot.buffer,
            layout: TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(ring.aligned_row_bytes),
                rows_per_image: None,
            },
        },
        Extent3d {
            width: extracted.width,
            height: extracted.height,
            depth_or_array_layers: 1,
        },
    );
    render_queue.submit([encoder.finish()]);

    let buffer = slot.buffer.clone();
    let busy = Arc::clone(&slot.busy);
    let sender = ring.sender.clone();
    let shared = ring.shared.clone();
    let video_sink = extracted.video_sink.clone();
    let recycle_pool = Arc::clone(&ring.recycled_pixels);
    let width = extracted.width;
    let height = extracted.height;
    let pixel_bytes = usize::try_from(width)
        .unwrap_or_default()
        .saturating_mul(usize::try_from(height).unwrap_or_default())
        .saturating_mul(4);
    let captured_at = now;
    let callback_buffer = buffer.clone();
    buffer.slice(..).map_async(MapMode::Read, move |result| {
        if result.is_ok() {
            let mapped = callback_buffer.slice(..).get_mapped_range();
            let mut pixels = take_stream_capture_pixels(&recycle_pool, pixel_bytes);
            let copied = copy_gpu_rows_into(&mapped, width, height, &mut pixels);
            drop(mapped);
            callback_buffer.unmap();
            if copied {
                if let Some(sink) = video_sink {
                    sink.metrics.observe_capture_latency(captured_at.elapsed());
                    let _ = sink.send_video(VideoFrame {
                        width,
                        height,
                        pixel_format: VideoPixelFormat::Bgra,
                        pixels,
                        recycle_pool: Some(recycle_pool),
                    });
                } else {
                    match sender.try_send(StreamOnlyCapturedFrame {
                        width,
                        height,
                        captured_at,
                        pixels,
                        recycle_pool: Arc::clone(&recycle_pool),
                    }) {
                        Ok(()) => {
                            shared.0.queued.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(TrySendError::Full(frame)) => {
                            recycle_stream_capture_pixels(frame.recycle_pool, frame.pixels);
                            shared.0.dropped.fetch_add(1, Ordering::Relaxed);
                            shared.0.queue_drops.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(TrySendError::Disconnected(frame)) => {
                            recycle_stream_capture_pixels(frame.recycle_pool, frame.pixels);
                        }
                    }
                }
                shared.0.completed.fetch_add(1, Ordering::Relaxed);
            } else {
                recycle_stream_capture_pixels(Arc::clone(&recycle_pool), pixels);
                shared.0.dropped.fetch_add(1, Ordering::Relaxed);
                shared.0.copy_failures.fetch_add(1, Ordering::Relaxed);
            }
        } else {
            shared.0.map_failures.fetch_add(1, Ordering::Relaxed);
            shared.0.total_map_failures.fetch_add(1, Ordering::Relaxed);
        }
        busy.store(false, Ordering::Release);
        shared.0.in_flight.fetch_sub(1, Ordering::Relaxed);
    });
}

fn spawn_stream_operator_view(
    commands: &mut Commands,
    camera: Entity,
    stream_target: Option<&Handle<Image>>,
) -> Entity {
    commands
        .spawn((
            Name::new("Stream-only operator information view"),
            UiTargetCamera(camera),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                padding: UiRect::all(px(48)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.018, 0.024, 0.034)),
            GlobalZIndex(10_000),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("STREAM TOWN · OPERATOR VIEW"),
                TextFont {
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
                TextColor(Color::srgb(0.82, 0.92, 1.0)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(48),
                    top: px(42),
                    ..default()
                },
            ));
            root.spawn((
                StreamOperatorInfoText,
                Text::new("Preparing direct stream…"),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.78, 0.86)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(50),
                    top: px(92),
                    width: px(510),
                    ..default()
                },
            ));
            root.spawn((
                StreamOperatorLiveButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(48),
                    bottom: px(42),
                    width: px(230),
                    height: px(52),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(10)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.08, 0.31, 0.18)),
                BorderColor::all(Color::srgb(0.31, 0.78, 0.46)),
            ))
            .with_child((
                StreamOperatorLiveButtonText,
                Text::new("● GO LIVE"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorRestartButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(292),
                    bottom: px(42),
                    width: px(190),
                    height: px(52),
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(10)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.22, 0.34)),
                BorderColor::all(Color::srgb(0.31, 0.58, 0.82)),
            ))
            .with_child((
                Text::new("RESTART STREAM"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                Text::new("LOCAL SETTINGS · EXCLUDED FROM STREAM"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.64, 0.76, 0.86)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(48),
                    top: px(320),
                    ..default()
                },
            ));
            root.spawn((
                StreamOperatorSettingsText,
                Text::new("Loading settings…"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.68, 0.75, 0.82)),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(48),
                    top: px(348),
                    width: px(470),
                    ..default()
                },
            ));
            let setting_buttons = [
                (
                    StreamOperatorSettingAction::BrightnessDown,
                    "BRIGHT −",
                    48.0,
                    430.0,
                ),
                (
                    StreamOperatorSettingAction::BrightnessUp,
                    "BRIGHT +",
                    148.0,
                    430.0,
                ),
                (
                    StreamOperatorSettingAction::MasterDown,
                    "MASTER −",
                    248.0,
                    430.0,
                ),
                (
                    StreamOperatorSettingAction::MasterUp,
                    "MASTER +",
                    348.0,
                    430.0,
                ),
                (
                    StreamOperatorSettingAction::MusicDown,
                    "MUSIC −",
                    48.0,
                    470.0,
                ),
                (
                    StreamOperatorSettingAction::MusicUp,
                    "MUSIC +",
                    148.0,
                    470.0,
                ),
                (
                    StreamOperatorSettingAction::EffectsDown,
                    "SFX −",
                    248.0,
                    470.0,
                ),
                (
                    StreamOperatorSettingAction::EffectsUp,
                    "SFX +",
                    348.0,
                    470.0,
                ),
                (
                    StreamOperatorSettingAction::AmbienceDown,
                    "AMBIENT −",
                    48.0,
                    510.0,
                ),
                (
                    StreamOperatorSettingAction::AmbienceUp,
                    "AMBIENT +",
                    148.0,
                    510.0,
                ),
                (
                    StreamOperatorSettingAction::ToggleShadows,
                    "SHADOWS",
                    248.0,
                    510.0,
                ),
                (
                    StreamOperatorSettingAction::ToggleReducedMotion,
                    "REDUCE MOTION",
                    348.0,
                    510.0,
                ),
            ];
            for (action, label, left, top) in setting_buttons {
                root.spawn((
                    action,
                    Button,
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(left),
                        top: px(top),
                        width: px(92),
                        height: px(30),
                        border_radius: BorderRadius::all(px(4)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.08, 0.15, 0.22)),
                ))
                .with_child((
                    Text::new(label),
                    TextFont {
                        font_size: FontSize::Px(10.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.78, 0.85, 0.91)),
                    Pickable::IGNORE,
                ));
            }
            let mut preview = root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    right: px(32),
                    top: px(32),
                    width: px(500),
                    height: px(281),
                    padding: UiRect::all(px(8)),
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.035, 0.045, 0.06)),
                BorderColor::all(Color::srgb(0.22, 0.35, 0.48)),
            ));
            if let Some(stream_target) = stream_target {
                preview.with_children(|preview| {
                    preview.spawn((
                        Name::new("Low-resolution stream preview"),
                        ImageNode::new(stream_target.clone()),
                        Node {
                            width: percent(100),
                            height: percent(100),
                            ..default()
                        },
                    ));
                });
            } else {
                preview.with_children(|preview| {
                    preview.spawn((
                        Name::new("Low-resolution stream preview"),
                        Text::new("HEADED MODE\nThe full game remains in the primary window"),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextLayout::justify(Justify::Center),
                        TextColor(Color::srgb(0.58, 0.68, 0.78)),
                        Node {
                            width: percent(100),
                            height: percent(100),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ));
                });
            }
            root.spawn((
                Name::new("Operator Twitch chat"),
                Node {
                    position_type: PositionType::Absolute,
                    right: px(32),
                    top: px(OPERATOR_CHAT_TOP),
                    width: px(OPERATOR_CHAT_WIDTH),
                    height: px(OPERATOR_CHAT_HEIGHT),
                    padding: UiRect::all(px(6)),
                    border: UiRect::all(px(1)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.025, 0.034, 0.048)),
                BorderColor::all(Color::srgb(0.16, 0.25, 0.34)),
            ))
            .with_children(|chat| {
                for slot in 0..OPERATOR_CHAT_VISIBLE_ROWS {
                    chat.spawn((
                        StreamOperatorChatRow {
                            slot,
                            line_id: None,
                            user_id: String::new(),
                            login: String::new(),
                        },
                        Button,
                        Node {
                            width: px(462),
                            height: px(24),
                            padding: UiRect::horizontal(px(5)),
                            border: UiRect::all(px(1)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderColor::all(Color::NONE),
                    ))
                    .with_children(|row| {
                        for (kind, icon, color) in [
                            (
                                OperatorChatBadgeKind::Broadcaster,
                                "★",
                                Color::srgb(1.0, 0.76, 0.2),
                            ),
                            (
                                OperatorChatBadgeKind::Moderator,
                                "◆",
                                Color::srgb(0.35, 0.9, 0.52),
                            ),
                            (
                                OperatorChatBadgeKind::Subscriber,
                                "♥",
                                Color::srgb(0.72, 0.45, 1.0),
                            ),
                        ] {
                            row.spawn((
                                StreamOperatorChatBadge { slot, kind },
                                Text::new(icon),
                                TextFont {
                                    font_size: FontSize::Px(11.0),
                                    ..default()
                                },
                                TextColor(color),
                                Node {
                                    display: Display::None,
                                    width: px(14),
                                    flex_shrink: 0.0,
                                    ..default()
                                },
                                Pickable::IGNORE,
                            ));
                        }
                        row.spawn((
                            StreamOperatorChatRowText(slot),
                            Text::new(""),
                            TextFont {
                                font_size: FontSize::Px(12.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.78, 0.84, 0.9)),
                            Node {
                                flex_shrink: 1.0,
                                ..default()
                            },
                            Pickable::IGNORE,
                        ));
                    });
                }
                for (action, icon, top) in [
                    (StreamOperatorChatScrollAction::Older, "▲", 4.0),
                    (StreamOperatorChatScrollAction::Newer, "▼", 174.0),
                ] {
                    chat.spawn((
                        action,
                        Button,
                        Node {
                            position_type: PositionType::Absolute,
                            right: px(4),
                            top: px(top),
                            width: px(18),
                            height: px(18),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.10, 0.16, 0.22)),
                    ))
                    .with_child((
                        Text::new(icon),
                        TextFont {
                            font_size: FontSize::Px(9.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.72, 0.8, 0.88)),
                        Pickable::IGNORE,
                    ));
                }
                chat.spawn((
                    Name::new("Operator chat scrollbar"),
                    Node {
                        position_type: PositionType::Absolute,
                        right: px(8),
                        top: px(25),
                        width: px(10),
                        height: px(145),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.055, 0.075, 0.1)),
                    Pickable::IGNORE,
                ))
                .with_child((
                    StreamOperatorChatScrollThumb,
                    Node {
                        position_type: PositionType::Absolute,
                        left: px(1),
                        top: px(0),
                        width: px(8),
                        height: px(145),
                        border_radius: BorderRadius::all(px(4)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.28, 0.42, 0.56)),
                    Pickable::IGNORE,
                ));
            });
            root.spawn((
                StreamOperatorChatInput,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(120),
                    top: px(546),
                    width: px(412),
                    height: px(38),
                    padding: UiRect::horizontal(px(10)),
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(5)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.035, 0.047, 0.065)),
                BorderColor::all(Color::srgb(0.22, 0.35, 0.48)),
            ))
            .with_child((
                StreamOperatorChatInputText,
                Text::new("Click to type a Twitch chat message…"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.64, 0.72, 0.8)),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorChatSendButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(32),
                    top: px(546),
                    width: px(80),
                    height: px(38),
                    border_radius: BorderRadius::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.13, 0.31, 0.48)),
            ))
            .with_child((
                Text::new("SEND"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorChatTimeoutButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(164),
                    top: px(594),
                    width: px(128),
                    height: px(34),
                    border_radius: BorderRadius::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.42, 0.26, 0.07)),
            ))
            .with_child((
                Text::new("TIMEOUT 10M"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorChatBanButton,
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(32),
                    top: px(594),
                    width: px(124),
                    height: px(34),
                    border_radius: BorderRadius::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.42, 0.08, 0.07)),
            ))
            .with_child((
                Text::new("BAN"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ));
            root.spawn((
                StreamOperatorChatSelectedText,
                Text::new("Select a chat row to moderate that user"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.62, 0.7, 0.78)),
                Node {
                    position_type: PositionType::Absolute,
                    right: px(300),
                    top: px(603),
                    width: px(232),
                    ..default()
                },
            ));
        })
        .id()
}

fn stream_operator_live_button(
    runtime: Res<DirectBroadcastRuntime>,
    mut control: ResMut<DirectBroadcastControl>,
    mut buttons: StreamOperatorLiveButtonQuery,
    mut labels: Query<&mut Text, With<StreamOperatorLiveButtonText>>,
) {
    let phase = runtime.snapshot().phase;
    let active = phase.is_active();
    let broadcasting = phase == DirectBroadcastPhase::Broadcasting;
    let bandwidth_testing = phase == DirectBroadcastPhase::BandwidthTesting;
    let ending_output = broadcasting || bandwidth_testing;
    if let Ok(mut label) = labels.single_mut() {
        let desired = operator_live_button_label(&phase);
        if **label != desired {
            desired.clone_into(&mut **label);
        }
    }
    for (interaction, mut background, mut border) in &mut buttons {
        let hovered = *interaction == Interaction::Hovered;
        let desired_background = if ending_output {
            if hovered {
                Color::srgb(0.47, 0.08, 0.07)
            } else {
                Color::srgb(0.34, 0.045, 0.04)
            }
        } else if hovered {
            Color::srgb(0.11, 0.42, 0.24)
        } else {
            Color::srgb(0.08, 0.31, 0.18)
        };
        if background.0 != desired_background {
            background.0 = desired_background;
        }
        let desired_border = BorderColor::all(if ending_output {
            Color::srgb(1.0, 0.32, 0.28)
        } else {
            Color::srgb(0.31, 0.78, 0.46)
        });
        if *border != desired_border {
            *border = desired_border;
        }
        if *interaction == Interaction::Pressed {
            if active {
                control.request_stop_and_return_to_main_menu();
            } else {
                control.request_restart();
            }
        }
    }
}

fn stream_operator_restart_button(
    mut control: ResMut<DirectBroadcastControl>,
    mut buttons: StreamOperatorRestartButtonQuery,
) {
    for (interaction, mut background) in &mut buttons {
        let desired_background = if *interaction == Interaction::Hovered {
            Color::srgb(0.17, 0.34, 0.52)
        } else {
            Color::srgb(0.12, 0.22, 0.34)
        };
        if background.0 != desired_background {
            background.0 = desired_background;
        }
        if *interaction == Interaction::Pressed {
            control.request_restart();
        }
    }
}

fn operator_live_button_label(phase: &DirectBroadcastPhase) -> &'static str {
    match phase {
        DirectBroadcastPhase::Broadcasting => "● LIVE · END STREAM",
        DirectBroadcastPhase::BandwidthTesting => "● BANDWIDTH TEST · END TEST",
        DirectBroadcastPhase::VerifyingTwitch => "● VERIFYING TWITCH · CANCEL",
        phase if phase.is_active() => "● NOT LIVE · CANCEL START",
        _ => "● NOT LIVE · GO LIVE",
    }
}

fn update_stream_operator_info(
    runtime: Res<DirectBroadcastRuntime>,
    config: Res<RuntimeConfig>,
    simulation: Option<Res<SimulationRuntime>>,
    mut text: Query<&mut Text, With<StreamOperatorInfoText>>,
    mut next_refresh: Local<Option<Instant>>,
) {
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    if !operator_info_refresh_due(&mut next_refresh, Instant::now()) {
        return;
    }
    let snapshot = runtime.snapshot();
    let fallback = if snapshot.encoder_rejections.is_empty() {
        "Auto selected its first usable backend".to_owned()
    } else {
        format!(
            "Auto fallbacks: {}",
            snapshot.encoder_rejections.join(" | ")
        )
    };
    let twitch_status = runtime
        .verification_status
        .as_deref()
        .unwrap_or("No Twitch public-status check is active");
    let enemy_status = stream_operator_enemy_status(&config.0, simulation.as_deref());
    let desired = format!(
        "Status: {:?}\nTwitch check: {}\nEncoder: {}\n{}\nStream motion: {:.1} FPS\nOutput cadence: {:.1} FPS\nRecent capture replacements: {} · Output cadence skips: {}\nRejected video frames: {} · Audio drops: {}\nEncode latency: {:.2} ms average / {:.2} ms maximum\nNetwork/mux write: {:.2} ms average / {:.2} ms maximum\n{}\nDrop log: {}",
        snapshot.phase,
        twitch_status,
        snapshot.encoder.as_deref().unwrap_or("starting"),
        fallback,
        snapshot.captured_video_fps,
        snapshot.encoded_video_fps,
        snapshot.replaced_video_frames,
        snapshot.skipped_video_frames,
        snapshot.dropped_video_frames,
        snapshot.dropped_audio_frames,
        snapshot.average_encode_ms,
        snapshot.maximum_encode_ms,
        snapshot.average_mux_write_ms,
        snapshot.maximum_mux_write_ms,
        enemy_status,
        DIRECT_BROADCAST_LOG_PATH,
    );
    if **text != desired {
        **text = desired;
    }
}

fn operator_info_refresh_due(next_refresh: &mut Option<Instant>, now: Instant) -> bool {
    if next_refresh.is_some_and(|deadline| now < deadline) {
        return false;
    }
    *next_refresh = Some(now + STREAM_OPERATOR_INFO_REFRESH_INTERVAL);
    true
}

fn stream_operator_enemy_status(
    config: &stream_town_domain::GameConfig,
    simulation: Option<&SimulationRuntime>,
) -> String {
    let Some(simulation) = simulation else {
        return "Enemy threat: world not loaded".to_owned();
    };
    let living_enemies = simulation
        .0
        .actors
        .values()
        .filter(|actor| actor.alive && actor.role.as_str() == "role:enemy")
        .count();
    let camps = simulation.0.enemy_camps.len();
    let timing = if let Some(raid) = &simulation.0.active_raid {
        format!(
            "raid wave {}/{} · {} tracked",
            raid.current_wave,
            raid.total_waves,
            raid.tracked_enemies.len()
        )
    } else if config.time.sample(simulation.0.elapsed_seconds).is_daytime {
        let remaining = seconds_until_enemy_night(&config.time, simulation.0.elapsed_seconds);
        format!("night spawning in {}", format_minutes_seconds(remaining))
    } else {
        "night spawning enabled".to_owned()
    };
    format!("Enemy threat: {living_enemies} active · {camps} camps · {timing}")
}

fn seconds_until_enemy_night(
    time: &stream_town_domain::TimeCycleConfig,
    elapsed_seconds: f64,
) -> u64 {
    let cycle = f64::from(time.seconds_per_day.max(1));
    let phase = elapsed_seconds.max(0.0).rem_euclid(cycle);
    let night_start = cycle * f64::from(time.daylight_per_thousand) / 1_000.0;
    let remaining = Duration::from_secs_f64((night_start - phase).max(0.0));
    remaining
        .as_secs()
        .saturating_add(u64::from(remaining.subsec_nanos() > 0))
}

fn format_minutes_seconds(seconds: u64) -> String {
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

fn stream_operator_settings_controls(
    settings: Option<ResMut<RuntimePlayerSettings>>,
    interactions: Query<(&Interaction, &StreamOperatorSettingAction), Changed<Interaction>>,
) {
    let Some(mut settings) = settings else {
        return;
    };
    let mut changed = false;
    for (interaction, action) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            StreamOperatorSettingAction::BrightnessDown => {
                settings.0.video.brightness_ev =
                    (settings.0.video.brightness_ev - 0.5).clamp(-5.0, 5.0);
            }
            StreamOperatorSettingAction::BrightnessUp => {
                settings.0.video.brightness_ev =
                    (settings.0.video.brightness_ev + 0.5).clamp(-5.0, 5.0);
            }
            StreamOperatorSettingAction::MasterDown => {
                settings.0.audio.master = (settings.0.audio.master - 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::MasterUp => {
                settings.0.audio.master = (settings.0.audio.master + 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::MusicDown => {
                settings.0.audio.music = (settings.0.audio.music - 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::MusicUp => {
                settings.0.audio.music = (settings.0.audio.music + 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::EffectsDown => {
                settings.0.audio.sound_effects =
                    (settings.0.audio.sound_effects - 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::EffectsUp => {
                settings.0.audio.sound_effects =
                    (settings.0.audio.sound_effects + 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::AmbienceDown => {
                settings.0.audio.ambience = (settings.0.audio.ambience - 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::AmbienceUp => {
                settings.0.audio.ambience = (settings.0.audio.ambience + 0.05).clamp(0.0, 1.0);
            }
            StreamOperatorSettingAction::ToggleShadows => {
                settings.0.video.shadows_enabled = !settings.0.video.shadows_enabled;
            }
            StreamOperatorSettingAction::ToggleReducedMotion => {
                settings.0.interface.reduced_motion = !settings.0.interface.reduced_motion;
            }
        }
        changed = true;
    }

    if changed {
        let store = PlayerSettingsStore::new(crate::player_settings_path());
        if let Err(error) = store.write(&settings.0) {
            warn!("could not persist operator player settings: {error}");
        }
    }
}

fn update_stream_operator_settings(
    settings: Option<Res<RuntimePlayerSettings>>,
    mut text: Query<&mut Text, With<StreamOperatorSettingsText>>,
) {
    let Some(settings) = settings else {
        return;
    };
    if !settings.is_changed() {
        return;
    }
    let Ok(mut text) = text.single_mut() else {
        return;
    };
    **text = format!(
        "Brightness {:+.1} EV  ·  Master {:>3.0}%  ·  Music {:>3.0}%\nSFX {:>3.0}%  ·  Ambience {:>3.0}%  ·  Shadows {}  ·  Motion {}",
        settings.0.video.brightness_ev,
        settings.0.audio.master * 100.0,
        settings.0.audio.music * 100.0,
        settings.0.audio.sound_effects * 100.0,
        settings.0.audio.ambience * 100.0,
        if settings.0.video.shadows_enabled {
            "ON"
        } else {
            "OFF"
        },
        if settings.0.interface.reduced_motion {
            "REDUCED"
        } else {
            "FULL"
        },
    );
}

fn stream_operator_chat_controls(
    state: Res<StreamOnlyCaptureState>,
    mut chat: ResMut<OperatorChatRuntime>,
    connection: Option<Res<TwitchConnection>>,
    keyboard: Option<MessageReader<KeyboardInput>>,
    wheel: Option<MessageReader<MouseWheel>>,
    operator_windows: Query<&Window, With<StreamOperatorWindow>>,
    input: Query<&Interaction, (Changed<Interaction>, With<StreamOperatorChatInput>)>,
    send: Query<&Interaction, (Changed<Interaction>, With<StreamOperatorChatSendButton>)>,
    timeout: Query<&Interaction, (Changed<Interaction>, With<StreamOperatorChatTimeoutButton>)>,
    ban: Query<&Interaction, (Changed<Interaction>, With<StreamOperatorChatBanButton>)>,
    rows: Query<(&Interaction, &StreamOperatorChatRow), Changed<Interaction>>,
    scroll: Query<(&Interaction, &StreamOperatorChatScrollAction), Changed<Interaction>>,
) {
    if input
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        chat.input_focused = true;
    }
    for (interaction, row) in &rows {
        if *interaction == Interaction::Pressed
            && let Some(line_id) = row.line_id
        {
            chat.selected_line = Some(line_id);
            chat.selected_user =
                (!row.user_id.is_empty()).then(|| (row.user_id.clone(), row.login.clone()));
        }
    }
    for (interaction, action) in &scroll {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            StreamOperatorChatScrollAction::Older => {
                chat.scroll_older(1, OPERATOR_CHAT_VISIBLE_ROWS);
            }
            StreamOperatorChatScrollAction::Newer => chat.scroll_newer(1),
        }
    }

    let operator_window = state.operator_window;
    if let Some(mut wheel) = wheel {
        let cursor_is_over_chat = operator_window
            .and_then(|entity| operator_windows.get(entity).ok())
            .and_then(Window::cursor_position)
            .is_some_and(|cursor| {
                cursor.x >= OPERATOR_CHAT_LEFT
                    && cursor.x <= OPERATOR_CHAT_LEFT + OPERATOR_CHAT_WIDTH
                    && cursor.y >= OPERATOR_CHAT_TOP
                    && cursor.y <= OPERATOR_CHAT_TOP + OPERATOR_CHAT_HEIGHT
            });
        for event in wheel
            .read()
            .filter(|event| Some(event.window) == operator_window)
        {
            if !cursor_is_over_chat {
                continue;
            }
            let rows = operator_chat_scroll_rows(event.unit, event.y);
            if event.y > 0.0 {
                chat.scroll_older(rows, OPERATOR_CHAT_VISIBLE_ROWS);
            } else if event.y < 0.0 {
                chat.scroll_newer(rows);
            }
        }
    }
    let mut submit = send
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    if let Some(mut keyboard) = keyboard {
        for event in keyboard.read() {
            if !chat.input_focused
                || event.state != ButtonState::Pressed
                || Some(event.window) != operator_window
            {
                continue;
            }
            match &event.logical_key {
                Key::Enter => submit = true,
                Key::Escape => chat.input_focused = false,
                Key::Backspace => {
                    chat.draft.pop();
                }
                _ => {
                    if let Some(text) = &event.text {
                        for character in text.chars().filter(|character| !character.is_control()) {
                            if chat.draft.chars().count() < 500 {
                                chat.draft.push(character);
                            }
                        }
                    }
                }
            }
        }
    }
    if submit {
        send_operator_chat_message(&mut chat, connection.as_deref());
    }

    if timeout
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        moderate_selected_operator_user(&mut chat, connection.as_deref(), false);
    }
    if ban
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        moderate_selected_operator_user(&mut chat, connection.as_deref(), true);
    }
}

fn send_operator_chat_message(
    chat: &mut OperatorChatRuntime,
    connection: Option<&TwitchConnection>,
) {
    let message = chat.draft.trim();
    if message.is_empty() {
        "Enter a message before sending".clone_into(&mut chat.feedback);
        return;
    }
    let Some(transport) = connection.and_then(|connection| connection.transport.as_ref()) else {
        "Twitch chat is not connected".clone_into(&mut chat.feedback);
        return;
    };
    match transport.send(crate::twitch::TwitchControl::SendBroadcasterMessage(
        message.to_owned(),
    )) {
        Ok(()) => {
            "Sending from the broadcaster account…".clone_into(&mut chat.feedback);
            chat.draft.clear();
        }
        Err(error) => chat.feedback = format!("Could not send chat message: {error}"),
    }
}

fn moderate_selected_operator_user(
    chat: &mut OperatorChatRuntime,
    connection: Option<&TwitchConnection>,
    ban: bool,
) {
    let Some((user_id, login)) = chat.selected_user.clone() else {
        "Select a Twitch chat row before moderating".clone_into(&mut chat.feedback);
        return;
    };
    let Some(transport) = connection.and_then(|connection| connection.transport.as_ref()) else {
        "Twitch chat is not connected".clone_into(&mut chat.feedback);
        return;
    };
    let control = if ban {
        crate::twitch::TwitchControl::Ban {
            user_id,
            reason: "Moderated from Stream Town operator panel".to_owned(),
        }
    } else {
        crate::twitch::TwitchControl::Timeout {
            user_id,
            duration_seconds: 600,
            reason: "Timed out from Stream Town operator panel".to_owned(),
        }
    };
    match transport.send(control) {
        Ok(()) => {
            chat.feedback = if ban {
                format!("Ban requested for {login}")
            } else {
                format!("10-minute timeout requested for {login}")
            };
        }
        Err(error) => chat.feedback = format!("Could not moderate {login}: {error}"),
    }
}

fn update_stream_operator_chat(
    chat: Res<OperatorChatRuntime>,
    mut rows: Query<(
        &mut StreamOperatorChatRow,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut row_text: StreamOperatorChatRowTextQuery,
    mut badges: StreamOperatorChatBadgeQuery,
    mut scroll_thumb: Query<
        &mut Node,
        (
            With<StreamOperatorChatScrollThumb>,
            Without<StreamOperatorChatBadge>,
        ),
    >,
    mut input_text: StreamOperatorChatInputTextQuery,
    mut selected_text: StreamOperatorChatSelectedTextQuery,
    mut initialized: Local<bool>,
) {
    if rows.is_empty() || (*initialized && !chat.is_changed()) {
        return;
    }
    *initialized = true;
    let visible = chat.visible_lines(OPERATOR_CHAT_VISIBLE_ROWS);
    for (mut row, mut background, mut border) in &mut rows {
        if let Some(line) = visible.get(row.slot) {
            row.line_id = Some(line.line_id);
            line.user_id.clone_into(&mut row.user_id);
            line.login.clone_into(&mut row.login);
            let selected = chat.selected_line == Some(line.line_id);
            background.0 = if selected {
                Color::srgb(0.12, 0.31, 0.47)
            } else {
                Color::NONE
            };
            *border = BorderColor::all(if selected {
                Color::srgb(0.35, 0.78, 1.0)
            } else {
                Color::NONE
            });
        } else {
            row.line_id = None;
            row.user_id.clear();
            row.login.clear();
            background.0 = Color::NONE;
            *border = BorderColor::all(Color::NONE);
        }
    }
    for (badge, mut node) in &mut badges {
        let show = visible.get(badge.slot).is_some_and(|line| {
            !line.is_system
                && match badge.kind {
                    OperatorChatBadgeKind::Broadcaster => line.badges.broadcaster,
                    OperatorChatBadgeKind::Moderator => line.badges.moderator,
                    OperatorChatBadgeKind::Subscriber => line.badges.subscriber,
                }
        });
        node.display = if show { Display::Flex } else { Display::None };
    }
    for (slot, mut text) in &mut row_text {
        **text = visible.get(slot.0).map_or_else(String::new, |line| {
            if line.is_system {
                format!("SYSTEM · {}", line.message)
            } else {
                format!("{}: {}", line.display_name, line.message)
            }
        });
    }
    if let Ok(mut thumb) = scroll_thumb.single_mut() {
        const TRACK_HEIGHT: f32 = 145.0;
        let total = chat.lines.len();
        let height = if total <= OPERATOR_CHAT_VISIBLE_ROWS {
            TRACK_HEIGHT
        } else {
            (TRACK_HEIGHT * bounded_history_f32(OPERATOR_CHAT_VISIBLE_ROWS)
                / bounded_history_f32(total))
            .max(18.0)
        };
        let maximum_scroll = chat.maximum_scroll(OPERATOR_CHAT_VISIBLE_ROWS);
        let newest_fraction = if maximum_scroll == 0 {
            1.0
        } else {
            1.0 - bounded_history_f32(chat.scroll_from_latest) / bounded_history_f32(maximum_scroll)
        };
        thumb.height = px(height);
        thumb.top = px((TRACK_HEIGHT - height) * newest_fraction);
    }
    if let Ok(mut text) = input_text.single_mut() {
        **text = if chat.draft.is_empty() {
            if chat.input_focused {
                "▌".to_owned()
            } else {
                "Click to type a Twitch chat message…".to_owned()
            }
        } else if chat.input_focused {
            format!("{}▌", chat.draft)
        } else {
            chat.draft.clone()
        };
    }
    if let Ok(mut text) = selected_text.single_mut() {
        let selected = chat.selected_user.as_ref().map_or_else(
            || "Select a chat row to moderate".to_owned(),
            |(_, login)| format!("Selected: {login}"),
        );
        **text = if chat.feedback.is_empty() {
            selected
        } else {
            format!("{selected}\n{}", chat.feedback)
        };
    }
}

fn operator_chat_scroll_rows(unit: MouseScrollUnit, delta: f32) -> usize {
    let mut remaining = match unit {
        MouseScrollUnit::Line => delta.abs(),
        MouseScrollUnit::Pixel => delta.abs() / 24.0,
    };
    let mut rows = 1;
    while remaining > 1.0 && rows < OPERATOR_CHAT_VISIBLE_ROWS {
        rows += 1;
        remaining -= 1.0;
    }
    rows
}

fn bounded_history_f32(value: usize) -> f32 {
    f32::from(u16::try_from(value).unwrap_or(u16::MAX))
}

const fn camera_targets_primary_window(target: &RenderTarget) -> bool {
    matches!(target, RenderTarget::Window(WindowRef::Primary))
}

fn capture_direct_broadcast_frame(
    time: Res<Time>,
    config: Res<RuntimeConfig>,
    sensitive_screen: Res<SensitiveScreenActive>,
    mut runtime: ResMut<DirectBroadcastRuntime>,
) {
    if let Some(controller) = &runtime.controller {
        controller.set_sensitive_screen(sensitive_screen.0);
    }
    if !matches!(
        runtime.phase,
        DirectBroadcastPhase::VerifyingTwitch
            | DirectBroadcastPhase::Broadcasting
            | DirectBroadcastPhase::BandwidthTesting
    ) {
        return;
    }
    if !sensitive_screen.0 {
        runtime.capture_elapsed = 0.0;
        return;
    }
    let frame_period = 1.0 / f32::from(config.0.twitch.broadcast.frames_per_second);
    runtime.capture_elapsed += time.delta_secs();
    if runtime.capture_elapsed < frame_period {
        return;
    }
    let mut due_frames = 0_u64;
    while runtime.capture_elapsed >= frame_period {
        runtime.capture_elapsed -= frame_period;
        due_frames = due_frames.saturating_add(1);
    }
    if due_frames > 1
        && let Some(controller) = &runtime.controller
    {
        controller.drop_video_frames(due_frames - 1);
    }
    let width = u32::from(config.0.twitch.broadcast.width);
    let height = u32::from(config.0.twitch.broadcast.height);
    let rgba = sensitive_rgba_frame(width, height);
    let _ = runtime.controller.as_ref().is_some_and(|controller| {
        controller.send_video(VideoFrame {
            width,
            height,
            pixel_format: VideoPixelFormat::Rgba,
            pixels: rgba,
            recycle_pool: None,
        })
    });
}

fn sensitive_rgba_frame(width: u32, height: u32) -> Vec<u8> {
    labeled_black_rgba_frame(width, height, "SENSITIVE INFORMATION HIDDEN")
}

fn offline_rgba_frame(width: u32, height: u32) -> Vec<u8> {
    labeled_black_rgba_frame(width, height, "OFFLINE")
}

fn labeled_black_rgba_frame(width: u32, height: u32, label: &str) -> Vec<u8> {
    let bytes = usize::try_from(width)
        .unwrap_or(0)
        .saturating_mul(usize::try_from(height).unwrap_or(0))
        .saturating_mul(4);
    let mut rgba = vec![0; bytes];
    for alpha in rgba.iter_mut().skip(3).step_by(4) {
        *alpha = 255;
    }
    draw_centered_label(&mut rgba, width, height, label);
    rgba
}

fn draw_centered_label(rgba: &mut [u8], width: u32, height: u32, label: &str) {
    const GLYPH_WIDTH: u32 = 5;
    const GLYPH_HEIGHT: u32 = 7;
    const GLYPH_GAP: u32 = 1;
    let unscaled_width = u32::try_from(label.chars().count())
        .unwrap_or_default()
        .saturating_mul(GLYPH_WIDTH + GLYPH_GAP)
        .saturating_sub(GLYPH_GAP);
    if width < unscaled_width || height < GLYPH_HEIGHT {
        return;
    }
    let scale = (width / unscaled_width.max(1))
        .min(height / (GLYPH_HEIGHT * 4).max(1))
        .clamp(1, 6);
    let label_width = unscaled_width.saturating_mul(scale);
    let label_height = GLYPH_HEIGHT.saturating_mul(scale);
    let origin_x = width.saturating_sub(label_width) / 2;
    let origin_y = height.saturating_sub(label_height) / 2;
    let stride = usize::try_from(width).unwrap_or_default().saturating_mul(4);

    for (glyph_index, character) in label.chars().enumerate() {
        let glyph = label_glyph(character);
        let glyph_x = origin_x.saturating_add(
            u32::try_from(glyph_index)
                .unwrap_or_default()
                .saturating_mul((GLYPH_WIDTH + GLYPH_GAP) * scale),
        );
        for (row, bits) in glyph.into_iter().enumerate() {
            for column in 0..GLYPH_WIDTH {
                if bits & (1 << (GLYPH_WIDTH - 1 - column)) == 0 {
                    continue;
                }
                for dy in 0..scale {
                    for dx in 0..scale {
                        let x = glyph_x + column * scale + dx;
                        let y = origin_y + u32::try_from(row).unwrap_or_default() * scale + dy;
                        let offset = usize::try_from(y)
                            .unwrap_or_default()
                            .saturating_mul(stride)
                            .saturating_add(
                                usize::try_from(x).unwrap_or_default().saturating_mul(4),
                            );
                        if let Some(pixel) = rgba.get_mut(offset..offset.saturating_add(4)) {
                            pixel.copy_from_slice(&[255, 255, 255, 255]);
                        }
                    }
                }
            }
        }
    }
}

const fn label_glyph(character: char) -> [u8; 7] {
    match character {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        _ => [0; 7],
    }
}
