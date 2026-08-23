# Tree and Foliage Flicker Regression Checklist

Status: **OPEN — trees are visibly flickering in the current user-tested build.**

Baseline audited: `3914e90` on 2026-08-23. The most recent explicit scope was in-game trees; earlier reports also covered menu trees and berry bushes, while ore did not exhibit the same problem. Menu and gameplay materials must therefore be tested separately.

## Acceptance gate

Do not mark this regression fixed until all of the following are true:

- [ ] A stationary in-game camera shows no black, blue, or brightness flicker on resource trees, generated foliage trees, grass/flowers, or berry bushes.
- [ ] A moving and zooming in-game camera shows no flicker or dither popping through the normal visibility range.
- [ ] The main-menu tree path is checked separately and remains stable.
- [ ] Trees, bushes, and foliage still cast ordinary ground shadows.
- [ ] No duplicate renderer occupies the same mesh/transform unless the source prefab intentionally contains it.
- [ ] Wind motion remains coherent, without a second shadow or colour silhouette.
- [ ] A short capture covers stationary and moving cameras; a still screenshot is insufficient.
- [ ] The user confirms that the visible result is fixed.

## What did work

- [x] **The original tight overlapping generation groups were reduced independently of the shader issue.** Generator/placement work in `b5549af`, `a88ec3b`, and `d6d5b1e` restored source-space occupancy rules and exact resource/foliage records; `207e641` compensated asymmetric resource mesh pivots without changing generator coordinates. Later reports stopped describing all resources as one coincident clump. Preserve generation authority while debugging rendering.
- [x] **Primitive scale was corrected.** `b5549af` restored the omitted glTF scene-node `0.01` scale for primitive-label resource and foliage loads. This removed a known source of giant overlapping cards, but does not prove that ECS spawn paths cannot duplicate the same renderer.
- [x] **Ordinary shadow casting was restored.** `3268115` set the Tree and Grass material extensions back to shadow-enabled, and current resource/foliage paths no longer add `NotShadowCaster`. Later complaints shifted from “not casting shadows” back to flicker. Preserve casting during the next diagnostic.
- [x] **The blue palette-flash mechanism was corrected narrowly (`f0ed3e9`).** Tree colour variation now hashes the instance origin instead of each wind-deformed fragment, so a swaying canopy cannot change palette when a fragment crosses a 32-unit boundary. The shader also masks atlas-blue leakage. The subsequent report described in-game trees “flickering again,” rather than the previous specifically blue palette switching. Do not undo this fix.
- [x] **The menu has a separate stable-material path (`f0ed3e9`).** Dense menu trees use the dedicated unlit `menu_tree` material instead of the gameplay wind/shadow material. This addressed the menu-specific blue/black lighting instability at that time; it is not evidence that the gameplay path is fixed.
- [x] **Self-shadow suppression is consistently encoded.** `e00431e`, the menu passes, and `207e641` propagate `NotShadowReceiver` to Tree/Grass/Critter scene overrides and direct resource/foliage primitives. Tests prove the policy is selected. The current visual failure shows that this policy alone is insufficient, not that it was never added.

## What did not fix the flicker

- [ ] **FAILED AS A COMPLETE FIX — higher directional-light shadow bias (`e00431e`).** Increasing depth/normal bias did not prevent later black and blue flicker reports. Do not tune global bias again without a shadow-only diagnostic proving acne is the remaining channel.
- [ ] **FAILED — disabling extension shadows while adding custom vegetation prepasses (`2a66489`).** This removed the required ground shadows and the user later reported that trees/foliage cast none.
- [ ] **FAILED — re-enabling shadows with a hard-coded approximate wind prepass (`3268115`).** Bevy's shadow-only pass lacked the extension material bind group, so the prepass embedded approximate wind constants/noise. Its silhouette did not match the visible shader and produced a second black/blue outline.
- [ ] **FAILED AS A COMPLETE FIX — repeatedly adding `NotShadowReceiver`.** Direct tree resources, generated foliage, menu material overrides, and GLB descendants all received the component through `e00431e`, `df08a63`, `f8e0223`, `ca3052d`, and `207e641`. Flicker remains, so another unconditional insertion is not a new hypothesis.
- [ ] **PARTIAL ONLY — colour/ambient changes (`f0ed3e9`).** Object-stable hashing and atlas masking removed a known blue-flash cause, and an unlit material stabilized the menu path. The gameplay flicker later returned, so broad colour brightening is not a complete fix.
- [ ] **FAILED AS A COMPLETE FIX — deleting the tree prepass (`207e641`).** Removing the known mismatched wind silhouette was correct and should remain, but the current user report proves another flicker source still exists.
- [ ] **NOT EVIDENCE — `tree_seasons_match_unity_material_targets`.** It checks parameters, the visible shader source string, shadow enablement, and absence of the custom prepass. It does not render multiple frames.
- [ ] **NOT EVIDENCE — `animated_card_materials_share_the_no_self_shadow_policy`.** It checks an enum match only; it does not prove that every visible descendant receives the component or that another pass cannot flicker.
- [ ] **NOT EVIDENCE — deterministic generation/unique stable-ID tests.** They prove data records, not that runtime systems spawn exactly one renderer per record.

## Do not retry unchanged

- [ ] Do not recreate an approximate tree wind prepass or hard-code another shadow-only wind function.
- [ ] Do not globally increase shadow bias, ambient light, exposure, or emissive colour as a first response.
- [ ] Do not add `NotShadowReceiver` again without first showing a visible tree entity that lacks it.
- [ ] Do not make gameplay trees unlit merely because the menu-only unlit material was stable.
- [ ] Do not disable shadow casting; keeping ground shadows is part of acceptance.
- [ ] Do not alter generation counts, seeds, density, or saved coordinates to treat a render flicker.
- [ ] Do not assume z-fighting is eliminated until runtime renderer multiplicity is measured, even though generated data IDs are unique.

## Next narrow diagnostic pass

Use the same fixed seed, camera, day, season, and tree for every toggle. Change one variable per capture:

- [ ] Identify whether the failing object is a resource tree, a generated foliage variant, a berry bush, or a menu tree. Record mesh handle, material variant, entity, transform, and spawn path.
- [ ] Count visible `Mesh3d` entities by exact mesh handle plus quantized global transform. If duplicates exist, trace their two spawn ancestors before changing materials.
- [ ] Freeze only Tree/Grass wind deformation. If flicker remains, stop changing wind math; if it disappears, compare vertex positions/normals across visible/depth/shadow passes.
- [ ] Disable only directional shadow maps for the capture. If flicker remains, stop changing shadow bias/receiver policy; if it disappears, inspect pass/component propagation on the exact renderer.
- [ ] Remove only `VisibilityRange` from the failing entity. If flicker disappears, isolate range dithering/AABB behavior instead of touching materials or generation.
- [ ] Apply a temporary flat unlit debug material to one gameplay tree. If flicker remains, suspect duplicate geometry/depth/visibility; if it disappears, isolate PBR, normals, culling, or the custom fragment path.
- [ ] Hold the camera stationary and then move it. Camera-only flicker points toward LOD/range/depth precision; time-only flicker points toward wind, shadow, or temporal material input.
- [ ] Inspect front/back-face behavior and imported normals on the exact failing primitive. Berry bushes and trees share card-like material/geometry characteristics that ore lacks.
- [ ] Add a runtime diagnostic asserting each generated resource/foliage stable ID owns exactly one visible render root and reporting duplicate mesh/transform pairs.
- [ ] Capture the matrix of toggles as short videos. Do not combine the winning toggle with unrelated rendering changes until the fault channel is established.
- [ ] Identify the user-observed known-good revision with the same capture and a Git bisect. `f0ed3e9` is known to have fixed the blue palette mechanism, but no revision is yet proven to have fixed all in-game flicker.

## Attempt record template

Append every future attempt here before claiming completion:

- [ ] **`<commit>` — `<hypothesis>`**
  - Object/spawn path: `<resource | foliage | menu | GLB descendant>`
  - Fixed seed/camera/day/season: `<values>`
  - Single changed variable: `<value>`
  - Duplicate renderer count: `<value>`
  - Stationary-camera result: `<video/capture path>`
  - Moving-camera result: `<video/capture path>`
  - Shadows still cast: `yes | no`
  - User result: `not checked | failed | partial | accepted`
  - Reuse rule: `<what new evidence would justify retrying this approach>`
