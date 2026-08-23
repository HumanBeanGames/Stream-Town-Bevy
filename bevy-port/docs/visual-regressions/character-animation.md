# Character Animation Regression Checklist

Status: **OPEN — the overlapping visible-skin defect is corrected locally and motion is visible in separated captures; user confirmation is still required.**

Baseline audited: `3914e90` on 2026-08-23. This ledger records what the repository actually proves and what the user's visual checks disproved. Passing tests, successful controller attachment logs, or a still screenshot do not establish visible animation.

## Acceptance gate

Do not mark this regression fixed until all of the following are true in the same ordinary in-game run:

- [ ] A stationary starting character has visible, repeating idle motion.
- [ ] A moving starting character has visible limb and body motion distinct from idle.
- [ ] At least one reachable role action visibly enters and exits its authored action animation.
- [ ] The selected body, equipment, hair, facial-hair, and eye variants remain the only visible variants.
- [ ] The character still faces its movement/action direction.
- [ ] A short moving capture demonstrates the result; a still screenshot is insufficient.
- [ ] The user confirms that the visible result is fixed.

## What did work

- [x] **The source animation data is present.** `3965dd2` converted the standalone Unity transform clips, and the later controller series (`c49844d`, `78016d6`, `646596e`, `26264ec`, `e6e86c6`, `522ae54`, `2412ed1`, and `0f24912`) preserved controller state, action parameters, speeds, hierarchy, layers, masks, property curves, and crossfades. This is useful infrastructure, but it proves data and state-machine behavior rather than visible skin deformation.
- [x] **The overlapping-character-variant problem was narrowed separately from animation.** `29a5966` added body/equipment/cosmetic visibility, `e00431e` switched players from the nine independently skinned T-pose export to `Characters.glb`'s single armature, and `2a66489` normalized `_Starter` equipment names. Later user reports stopped describing every character variant as simultaneously visible. Preserve this hierarchy/visibility work while debugging motion.
- [x] **The remaining shoulder overlap was measured and removed (current candidate).** The live inherited-visibility trace showed all three `Body_Default_*` nodes and `Back_CommanderBanner` still rendered under every selected role body because those inactive Unity source nodes are absent from the serialized role list. All equipment-shaped player nodes now enter the same visibility controller. The settled defender trace contains only `Body_Defender_Slim`, its selected face parts, permanent head, helmet, shield, and spear.
- [x] **Visible facing was corrected separately.** `2a66489` aligned the character's visible local `+Z` axis to movement. Later reports no longer describe ordinary characters as walking backward. Do not revisit facing while diagnosing static bones unless a new capture disproves it.
- [x] **A repeatable close-up path exists.** `STREAM_TOWN_SMOKE_STATIC_RIG=1` isolates the static rig and `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1` frames the controller path. These are diagnostic entry points only; a still smoke capture is not an animation acceptance test.
- [x] **The runtime can construct and select controller nodes.** Current tests cover converted clip records, graph composition, layer masks, transition selection, state speed, crossfades, action contracts, and retargeted curve creation. Attachment logs have reported controllers, clips, and targets. This narrows the remaining fault to live playback/target/skin application rather than missing controller RON.
- [x] **The visible skin now receives motion from clips authored for its own consolidated armature (current candidate).** The full translated controller remains authoritative, but each of its 20 motions resolves to the matching native take inside `Characters.glb` when available. This is materially different from the old single native-fallback preference: it preserves layers, transitions, actions, weights, and crossfades while replacing only the incompatible standalone-rig curve source. A live `STREAM_TOWN_DEBUG_ANIMATION_BINDINGS=1` run measured advancing playback and 0.53–0.92-radian rotation deltas on `Thigh_L`/`UpperArm_L`; the measured joints were referenced by the visible glTF skin. This is strong internal evidence, but the visual regression stays open until the acceptance gate and user check pass.

## What did not fix visible animation

- [ ] **FAILED AS A VISUAL FIX — initial Idle/Walk retargeting (`3965dd2`).** It built Bevy curves and an `AnimationGraph`, but subsequent user checks still found characters unanimated.
- [ ] **FAILED AS A VISUAL FIX — full translated controller execution (`c49844d` plus the controller series listed above).** Typed transitions and weights execute in isolation, but adding more controller semantics did not make the rendered skeleton move.
- [ ] **FAILED AS A VISUAL FIX — single native imported-clip preference.** Earlier runtime code replaced the controller with one available native GLB take. The characters remained wrong/static, and an embedded bind/default-pose take could win without exercising the authored controller. Do not confuse that failed shortcut with the current per-motion, skin-compatible sources inside the complete translated controller.
- [ ] **FAILED AS A VISUAL FIX — converted-controller preference and override composition (`2a66489`).** This made the full controller take priority and changed the outer graph from additive to override blending. The user subsequently reported that characters were still not animated.
- [ ] **FAILED AS A VISUAL FIX — broad rig/path suffix retargeting (`e00431e`).** Matching standalone track paths by suffix, keeping joint rotation, limiting skeletal translation, and dropping skeletal scale curves prevented detached body parts, but did not establish visible animation.
- [ ] **FAILED AS A VISUAL FIX — explicit `CharacterArmature` root plus `pelvis/...` suffix mapping (`207e641`).** Tests showed retargeted curves and smoke logs reported five attached starting controllers, 20 clips, and 245 targets per controller. The current user report disproves the claim that this produced visible motion.
- [ ] **NOT EVIDENCE — `player_runtime_uses_the_single_animation_armature`.** This test proves only which GLB path is selected.
- [ ] **NOT EVIDENCE — `embedded_presentation_binds_native_and_converted_animation_paths`.** It proves catalog contents, controller decisions, and synthetic curve counts. Its targets are placeholders; it does not prove that the visible mesh's real joints change at runtime.
- [ ] **NOT EVIDENCE — controller/crossfade/action unit tests.** They prove state selection and weights, not that `AnimationPlayer` reaches the skin used by the rendered primitives.
- [ ] **NOT EVIDENCE — a successful still screenshot or attachment log.** A static pose can satisfy both.

## Do not retry unchanged

- [ ] Do not add another root-name or suffix special case without first logging the exact unmatched real scene paths.
- [ ] Do not swap native-versus-converted priority again without proving which `AnimationPlayer` and clip are active over time.
- [ ] Do not add more controller states, layer logic, crossfades, or action mappings as a response to a completely static mesh.
- [ ] Do not replace the player model, axis correction, cosmetic visibility, materials, or shadow policy in the same change as the motion diagnosis.
- [ ] Do not call clip counts, graph-node counts, attachment logs, unit tests, or still captures a visual fix.

## Next narrow diagnostic pass

Run these in order and record the result before changing behavior:

- [ ] Use one starting actor under `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`; record its stable ID, actor root entity, animation-root entity, `AnimationPlayer`, graph handle, active nodes, weights, elapsed time, target count, and detailed-animation budget decision.
- [ ] Sample the local and global transforms of `CharacterArmature`, `pelvis`, one spine joint, and one arm/leg joint at two separated rendered frames. Store the numerical delta.
- [ ] If player elapsed time does not advance, inspect scheduling/play/repeat/pause state only.
- [ ] If elapsed time advances but joint transforms do not, inspect `AnimationTargetId`/`AnimatedBy` binding only.
- [ ] If joint transforms change but the visible mesh does not, inspect which joint entities the glTF `SkinnedMesh` actually references; do not change retargeting until this is known.
- [ ] If both joints and skin move in a one-clip direct diagnostic, reintroduce the controller one layer at a time to locate the first suppressing weight/mask.
- [ ] Add an integration test that advances real Bevy time and asserts a named real joint transform changes. Keep the existing contract tests, but do not treat them as a substitute.
- [ ] Capture a short idle-to-walk-to-idle video using the same actor and camera. Compare motion, not just pose or attachment logs.
- [ ] Identify the user-observed known-good revision by replaying the same capture while bisecting. No exact known-good commit is currently proven, so do not label a candidate solely from its commit message.

## Current attempt

- [ ] **`pending` — preserve the full controller but source every motion from the visible skin's native armature**
  - Changed: clip source binding only; standalone `PlayerChar_TPose.glb` motion tracks are mapped by authored name to the corresponding `Characters.glb` take.
  - Fixed seed/actor/camera: default deterministic smoke seed; `npc:starting_defender`, `npc:starting_logger`, `npc:starting_miner`, `npc:starting_gatherer`, and `npc:starting_builder`; `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`.
  - Player elapsed delta: `0.0295718` seconds in the sampled rendered frame.
  - Named joint transform delta: `Thigh_L = 0.4646586` radians; `UpperArm_L = 0.9139379` radians.
  - Visible skin result: every sampled joint had `joint_skin_references = 123`; local still capture at `.stream-town/diagnostics/animation-binding-proof.png` (not accepted as motion evidence).
  - User result: `failed` — the next user check still reported visibly static characters and shoulder flicker.
  - Reuse rule: do not change clip priority or retargeting again unless the diagnostic reports zero elapsed time, zero joint delta, or a joint unused by the rendered skin.

- [ ] **`pending` — control inactive source-only model slots and compare the visible skin over separated frames**
  - Changed: model-slot visibility only; `Body_Default_*` and `Back_CommanderBanner` can no longer remain inherited-visible beneath a role body. The animation graph, clip mapping, axis correction, material, and shadow policy were not changed.
  - Fixed seed/actor/camera: default deterministic smoke seed; followed `npc:starting_defender`; `STREAM_TOWN_SMOKE_ANIMATION_CLOSEUP=1`.
  - Player elapsed delta: `0.0143754` seconds in the sampled rendered frame.
  - Named joint transform delta: `Body = 0.2566451` radians; `Thigh_L = 0.5331674` radians on the other starting actors.
  - Visible skin result: settled defender renderers reduced from 123 to 7 primitives across exactly the selected logical slots. The animated poses in `.stream-town/diagnostics/animation-visible-after-slot-fix.png` and `animation-visible-after-slot-fix-frame-2.png` differ from each other and from `animation-static-after-slot-fix.png` while the camera follows the same actor.
  - User result: `not checked`.
  - Reuse rule: if the user still sees no motion, record a short ordinary-run video before changing controller or retargeting code; if shoulder flicker remains, inspect only the seven logged selected primitives and their material/shadow passes.

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
