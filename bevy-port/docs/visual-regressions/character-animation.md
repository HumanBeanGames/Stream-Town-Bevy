# Character Animation Regression Checklist

Status: **FIXED AND USER-CONFIRMED — rig-native motion takes provide articulated deformation, and the controller Exit repair preserves clean continuous looping.**

Baseline audited: `3914e90` on 2026-08-23. This ledger records what the repository actually proves and what the user's visual checks disproved. Passing tests, successful controller attachment logs, or a still screenshot do not establish visible animation.

## Acceptance gate

Do not mark this regression fixed until all of the following are true in the same ordinary in-game run:

- [x] A stationary starting character has visible, repeating idle motion.
- [x] A moving starting character has visible limb and body motion distinct from idle.
- [x] Reachable role actions enter and exit their authored action animations.
- [x] The selected body, equipment, hair, facial-hair, and eye variants remain the only visible variants.
- [x] The character still faces its movement/action direction.
- [x] A short moving capture demonstrates the result; a still screenshot is insufficient.
- [x] The user confirms that the visible result is fixed.
- [x] The user confirms that characters now visibly animate.

## What did work

- [x] **The source animation data is present.** `3965dd2` converted the standalone Unity transform clips, and the later controller series (`c49844d`, `78016d6`, `646596e`, `26264ec`, `e6e86c6`, `522ae54`, `2412ed1`, and `0f24912`) preserved controller state, action parameters, speeds, hierarchy, layers, masks, property curves, and crossfades. This is useful infrastructure, but it proves data and state-machine behavior rather than visible skin deformation.
- [x] **The overlapping-character-variant problem was narrowed separately from animation.** `29a5966` added body/equipment/cosmetic visibility, `e00431e` switched players from the nine independently skinned T-pose export to `Characters.glb`'s single armature, and `2a66489` normalized `_Starter` equipment names. Later user reports stopped describing every character variant as simultaneously visible. Preserve this hierarchy/visibility work while debugging motion.
- [x] **The remaining shoulder overlap was measured and removed (current candidate).** The live inherited-visibility trace showed all three `Body_Default_*` nodes and `Back_CommanderBanner` still rendered under every selected role body because those inactive Unity source nodes are absent from the serialized role list. All equipment-shaped player nodes now enter the same visibility controller. The settled defender trace contains only `Body_Defender_Slim`, its selected face parts, permanent head, helmet, shield, and spear.
- [x] **Visible facing was corrected separately.** `2a66489` aligned the character's visible local `+Z` axis to movement. Later reports no longer describe ordinary characters as walking backward. Do not revisit facing while diagnosing static bones unless a new capture disproves it.
- [x] **A repeatable close-up path exists.** `STREAM_TOWN_SMOKE_STATIC_RIG=1` isolates the static rig and `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1` frames the controller path. These are diagnostic entry points only; a still smoke capture is not an animation acceptance test.
- [x] **The runtime can construct and select controller nodes.** Current tests cover converted clip records, graph composition, layer masks, transition selection, state speed, crossfades, action contracts, and retargeted curve creation. Attachment logs have reported controllers, clips, and targets. This narrows the remaining fault to live playback/target/skin application rather than missing controller RON.
- [x] **The visible rig's native takes produce real skinned deformation.** Direct GLB evaluation and the earlier lifecycle-repaired runtime produced changing limb poses from the per-motion `Characters.glb` takes. The user subsequently confirmed visible animation before reporting the loop defect. That was the strongest known-good motion source; the loop failure was later isolated to a controller restart, not to missing native deformation.
- [x] **The exported defender skin and native run clip deform correctly without Bevy runtime assumptions.** Direct GLB evaluation moved all 476 `Body_Defender_Slim` vertices at a quarter-cycle (mean `1.071888`, maximum `2.59697` model units), changed the joint skin matrices by up to `4.2767`, and found normalized vertex weights. This proves skinning, weights, and inverse bind poses; it does not prove that the exported clip timeline has a seamless zero-based loop.
- [x] **The first live animation player was being discarded by Bevy scene finalization, and the replacement now rebinds (current candidate).** The same frame previously contained one active converted player during `Update` and zero before Bevy animation evaluation in `PostUpdate`. Converted rigs now wait for `WorldInstanceReady`; a later instance-ready event clears the stale applied marker, and a fallback system detects any root whose player disappeared. After replacement, one player remained active while seek time advanced from `0.028` to `0.528` seconds and visible-body vertex deltas reached `1.1801` model units. Twenty internal GPU screenshots were assembled into `.stream-town/diagnostics/animation-lifecycle-internal-capture.mp4`, which visibly contains changing character poses.
- [x] **Short navigation gaps no longer restart locomotion.** `AgentLocomotion` derives the controller signal from measured displacement, preserves the authored playback speed, and applies a 120-ms stop grace so a one-frame path/occupancy handoff does not produce Run → Idle → Run chatter.
- [x] **Root state-machine exits no longer replay the active fallback loop.** The decisive live heartbeat previously showed the run player's elapsed time falling from `0.489` to `0.320` and its completion count remaining at zero. A destination-less root Exit was re-entering the already-active locomotion fallback and calling `AnimationPlayer::start`. That no-op exit is now ignored. The final GPU run advanced the same defender player monotonically to `39.15` seconds and 51 completed wraps without a playback restart.
- [x] **Looping authored curves close at the actual clip boundary.** The raw run fixture contains a measured `0.54435`-radian end/start leg-pose gap and some sparse tracks end before the declared duration. Only looping curves receive a progressive end-to-start correction plus an explicit boundary key; non-looping attacks/death remain unchanged. Regression tests retain the raw seam assertion and prove corrected translation/rotation/scale endpoints close at the declared duration.

## What did not fix visible animation

- [ ] **FAILED AS A VISUAL FIX — initial Idle/Walk retargeting (`3965dd2`).** It built Bevy curves and an `AnimationGraph`, but subsequent user checks still found characters unanimated.
- [ ] **FAILED AS A VISUAL FIX — full translated controller execution (`c49844d` plus the controller series listed above).** Typed transitions and weights execute in isolation, but adding more controller semantics did not make the rendered skeleton move.
- [ ] **FAILED AS A VISUAL FIX — single native imported-clip shortcut.** Earlier runtime code replaced the controller with one arbitrary available GLB take before the scene-lifecycle repair. A bind/default-pose take could win and the first animation player was later discarded by scene finalization, so that experiment did not test the current per-motion native mapping fairly.
- [ ] **FAILED AS A VISUAL FIX — converted-controller preference and override composition (`2a66489`).** This made the full controller take priority and changed the outer graph from additive to override blending. The user subsequently reported that characters were still not animated.
- [ ] **FAILED AS A VISUAL FIX — broad rig/path suffix retargeting (`e00431e`).** Matching standalone track paths by suffix, keeping joint rotation, limiting skeletal translation, and dropping skeletal scale curves prevented detached body parts, but did not establish visible animation.
- [ ] **FAILED AS A VISUAL FIX — explicit `CharacterArmature` root plus `pelvis/...` suffix mapping (`207e641`).** Tests showed retargeted curves and smoke logs reported five attached starting controllers, 20 clips, and 245 targets per controller. The current user report disproves the claim that this produced visible motion.
- [ ] **NOT EVIDENCE — `player_runtime_uses_the_single_animation_armature`.** This test proves only which GLB path is selected.
- [ ] **NOT EVIDENCE — `embedded_presentation_binds_native_and_converted_animation_paths`.** It proves catalog contents, controller decisions, and synthetic curve counts. Its targets are placeholders; it does not prove that the visible mesh's real joints change at runtime.
- [ ] **NOT EVIDENCE — controller/crossfade/action unit tests.** They prove state selection and weights, not that `AnimationPlayer` reaches the skin used by the rendered primitives.
- [ ] **NOT EVIDENCE — a successful still screenshot or attachment log.** A static pose can satisfy both.
- [ ] **FAILED AS A LOOP FIX — zero-basing the native FBX timelines alone.** Re-exporting the takes removed their nonzero first timestamp, but the user still reported a broken run loop. The active controller motion was also not the same take as the native name-based substitute.
- [ ] **FAILED AS A COMPLETE LOOP FIX — curve seam correction without controller-lifecycle tracing.** Closing the endpoint pose was necessary, but it could not help while a destination-less root Exit replayed the locomotion state before Bevy recorded a completion.
- [ ] **FAILED AS A VISUAL FIX — preferring standalone Unity transform curves over visible-rig takes (`c93c70c`).** Although their paths could be bound and the controller completed repeated wraps, the user observed the assembled models swinging in their entirety instead of showing useful skeletal animation. Do not infer compatible deformation from target-name matches or completion counters.

## Do not retry unchanged

- [ ] Do not add another root-name or suffix special case without first logging the exact unmatched real scene paths.
- [ ] Do not prefer standalone transform curves over a take authored on the rendered rig unless a moving capture proves correct limb deformation on that exact skin.
- [ ] Do not add more controller states, layer logic, crossfades, or action mappings as a response to a completely static mesh.
- [ ] Do not replace the player model, axis correction, cosmetic visibility, materials, or shadow policy in the same change as the motion diagnosis.
- [ ] Do not call clip counts, graph-node counts, attachment logs, unit tests, or still captures a visual fix.
- [ ] Do not revisit hierarchy binding, retargeting, or skin weights for a loop-seam hold while visible motion continues to work.
- [ ] Do not treat matching bone names as proof that standalone Unity curves use the same root/pelvis space as the assembled visible rig.
- [ ] Do not smooth or replace the run again unless a live heartbeat first proves that elapsed time remains monotonic and completion counts increase.

## Next narrow diagnostic pass

Run these in order and record the result before changing behavior:

- [x] Use one starting actor under `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`; record its stable ID, actor root entity, animation-root entity, `AnimationPlayer`, graph handle, active nodes, weights, elapsed time, target count, and detailed-animation budget decision.
- [x] Sample representative named-joint transforms at separated rendered frames and store the numerical delta.
- [x] Determine whether player elapsed time advances. It advances after the replacement hierarchy is rebound; the original player was removed between schedules.
- [x] Determine whether animated target transforms change. The visible skin's referenced `Body`, `Thigh_L`, and `UpperArm_L` joints change.
- [x] Determine whether the real visible mesh references and deforms with those joints. `Body_Defender_Slim` references the changed joints, and direct live skinning of its 476 vertices changes every sampled pose.
- [x] Prove the full controller can drive the same visible skin after the lifecycle repair. The controller retains 20 clips/two layers and produces visible internal screenshot sequences.
- [x] Add a lifecycle regression test that verifies a replaced hierarchy becomes eligible for rebinding while a live driver keeps its marker.
- [x] Capture a short same-actor sequence from GPU-rendered frames. `.stream-town/diagnostics/animation-lifecycle-internal-capture.mp4` covers 20 consecutive internal captures.
- [x] Inspect the actual native GLB sampler timelines. Character idle/run/walk began at `0.0166667` seconds rather than zero and ended one sample later than the Unity duration, creating a held leading sample at every repeat seam.
- [x] The user-observed known-good revision is `334b9dc`; it restored the rig-native motion sources while retaining the already-proven lifecycle and loop repairs, and the user accepted that result.

## Current attempt

- [ ] **`failed` — preserve the full controller but source every motion from the visible skin's native armature**
  - Changed: clip source binding only; standalone `PlayerChar_TPose.glb` motion tracks are mapped by authored name to the corresponding `Characters.glb` take.
  - Fixed seed/actor/camera: default deterministic smoke seed; `npc:starting_defender`, `npc:starting_logger`, `npc:starting_miner`, `npc:starting_gatherer`, and `npc:starting_builder`; `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`.
  - Player elapsed delta: `0.0295718` seconds in the sampled rendered frame.
  - Named joint transform delta: `Thigh_L = 0.4646586` radians; `UpperArm_L = 0.9139379` radians.
  - Visible skin result: every sampled joint had `joint_skin_references = 123`; local still capture at `.stream-town/diagnostics/animation-binding-proof.png` (not accepted as motion evidence).
  - User result: `failed` — the next user check still reported visibly static characters and shoulder flicker.
  - Reuse rule: do not change clip priority or retargeting again unless the diagnostic reports zero elapsed time, zero joint delta, or a joint unused by the rendered skin.

- [ ] **`failed` — control inactive source-only model slots and compare the visible skin over separated frames**
  - Changed: model-slot visibility only; `Body_Default_*` and `Back_CommanderBanner` can no longer remain inherited-visible beneath a role body. The animation graph, clip mapping, axis correction, material, and shadow policy were not changed.
  - Fixed seed/actor/camera: default deterministic smoke seed; followed `npc:starting_defender`; `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`.
  - Player elapsed delta: `0.0143754` seconds in the sampled rendered frame.
  - Named joint transform delta: `Body = 0.2566451` radians; `Thigh_L = 0.5331674` radians on the other starting actors.
  - Visible skin result: settled defender renderers reduced from 123 to 7 primitives across exactly the selected logical slots. The animated poses in `.stream-town/diagnostics/animation-visible-after-slot-fix.png` and `animation-visible-after-slot-fix-frame-2.png` differ from each other and from `animation-static-after-slot-fix.png` while the camera follows the same actor.
  - User result: `failed` — the latest user check still reported no visible animation, which led to the scene-lifecycle diagnosis below.
  - Reuse rule: if the user still sees no motion, record a short ordinary-run video before changing controller or retargeting code; if shoulder flicker remains, inspect only the seven logged selected primitives and their material/shadow passes.

- [x] **`0aaced7` — bind only to a finalized imported hierarchy and recover after instance replacement**
  - Changed: scene/player lifecycle only; converted rigs wait for `WorldInstanceReady`, rebind on later instance-ready events, and clear stale applied markers if a driver disappears. Clip selection, retargeting, controller logic, visibility, facing, materials, and shadows were not changed.
  - Fixed seed/actor/camera: default deterministic smoke seed; `npc:starting_defender`; `STREAM_TOWN_DEBUG_INITIAL_AGENTS=1`; `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`.
  - Player elapsed delta: replacement player advanced from `0.0281846` to `0.5281846` seconds and remained active in every later sampled stage.
  - Named joint transform delta: `Body = 0.3868502` radians between sampled frames; playback remained at speed `1.0`.
  - Visible skin result: `Body_Defender_Slim` has 476 vertices/25 joints; live mean vertex deltas reached `0.3845894` and maximum deltas reached `1.1801370`. Visible motion is recorded in `.stream-town/diagnostics/animation-lifecycle-internal-capture.mp4`.
  - User result: `partial` — the user confirmed that characters now animate, but reported that the loop does not look correct.
  - Reuse rule: if this still fails for the user, first check whether their log contains a second `attached translated Unity animation controller` after scene finalization and capture ordinary-run frames after that point; do not alter clip or rig data without evidence that the replacement player survives but renders statically.

- [ ] **`failed` — zero-base native take timelines and preserve authored repeat modes**
  - Changed: animation seam timing and repeat policy only; every FBX action is rebased from its first authored frame to frame zero before glTF export, and runtime playback uses each migrated clip's `looping` flag instead of forcing every state to repeat forever.
  - Fixed seed/actor/camera: default deterministic smoke seed; `npc:starting_defender`; `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`.
  - Player elapsed delta: playback advanced, but subsequent traces proved the controller replayed locomotion before recording a completion.
  - Named joint transform delta: already established by the accepted lifecycle candidate; this pass must not change skin binding.
  - Visible skin result: visible motion remained, but its repeat seam remained incorrect.
  - User result: `failed` — the user reported the run still did not loop cleanly twice.
  - Reuse rule: if the seam remains, compare the final two and first two samples of the active take before changing controller transitions or blend weights.

- [ ] **`failed` — preserve continuous playback and close the standalone authored boundary**
  - Changed: the controller used the migrated Unity source curves; looping curves closed at the declared duration; short movement gaps used locomotion grace; and a destination-less root Exit could not replay an already-active fallback state.
  - Fixed seed/actor/camera: default deterministic smoke seed; `npc:starting_defender`; `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`.
  - Player elapsed delta: `39.1476` seconds continuous in the final trace, with no reset.
  - Loop completion evidence: `51` consecutive `PlayerChar_Run_01` completions on the same player; the earlier failing trace remained at `0` while elapsed repeatedly reset.
  - Visible skin result: 12-second ordinary GPU capture at `.stream-town/diagnostics/animation-run-final-visual.mp4`; runtime log at `.stream-town/diagnostics/animation-run-final-live.log`.
  - User result: `failed` — the user reported that useful animation disappeared and the models swung around in their entirety.
  - Reuse rule: retain the no-op Exit repair and its monotonic completion diagnostic, but do not reuse the standalone curves for the visible player rig without new deformation evidence.

- [x] **`334b9dc` — combine the known-good visible-rig takes with the proven loop lifecycle**
  - Changed: clip-source selection only; each controller motion prefers its mapped take inside `Characters.glb`, and retargeted Unity curves remain a fallback only when no skin-compatible take exists.
  - Preserved: finalized-scene rebinding, selected-variant visibility, facing, locomotion grace, native zero-based timelines, authored repeat modes, and the destination-less root Exit no-op.
  - Fixed seed/actor/camera: default deterministic smoke seed; `npc:starting_defender`; `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`.
  - Player elapsed delta: `37.1738` seconds continuous in the final trace, with `45` completed wraps and no playback restart.
  - Named joint/skin delta: the sampled `Body` joint rotated by `0.3240` radians and all 476 sampled body vertices deformed; the moving capture visibly shows independent arm and leg poses.
  - Visible skin result: 12-second ordinary GPU capture at `.stream-town/diagnostics/animation-native-source-visual.mp4`; runtime log at `.stream-town/diagnostics/animation-native-source-live.log`.
  - User result: `accepted` — the user confirmed, “That worked!”
  - Reuse rule: require both named-limb deformation and multiple monotonically counted loop completions in the same run before changing this source policy again.

## Attempt record template

Append every future attempt here before claiming completion:

- [ ] **`<commit>` — `<hypothesis>`**
  - Changed: `<one narrowly scoped variable>`
  - Fixed seed/actor/camera: `<values>`
  - Player elapsed delta: `<value>`
  - Named joint transform delta: `<value>`
  - Visible skin result: `<video/capture path>`
  - User result: `not checked | failed | partial | accepted`
  - Reuse rule: `<what new evidence would justify retrying this approach>`
