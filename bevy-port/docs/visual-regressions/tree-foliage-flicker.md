# Tree and Foliage Flicker Regression Checklist

Status: **FIXED — the user reported the trees look okay, and two-sided leaf cards, source-space placement, shadows, duplicate audits, and the moving-camera GPU sweep now pass.**

Baseline audited: `3914e90` on 2026-08-23. The most recent explicit scope was in-game trees; earlier reports also covered menu trees and berry bushes, while ore did not exhibit the same problem. Menu and gameplay materials must therefore be tested separately.

## Acceptance gate

Do not mark this regression fixed until all of the following are true:

- [x] A stationary in-game camera shows no black, blue, or brightness flicker on resource trees, generated foliage trees, grass/flowers, or berry bushes.
- [x] A moving and zooming in-game camera shows no flicker or dither popping through the normal visibility range.
- [x] The main-menu tree path is checked separately and remains stable.
- [x] Trees, bushes, and foliage still cast ordinary ground shadows.
- [x] No duplicate renderer occupies the same mesh/transform unless the source prefab intentionally contains it.
- [x] Wind motion remains coherent, without a second shadow or colour silhouette.
- [x] A short capture covers stationary and moving cameras; a still screenshot is insufficient.
- [x] The user confirms that the visible result is fixed.

## What did work

- [x] **The original tight overlapping generation groups were reduced independently of the shader issue.** Generator/placement work in `b5549af`, `a88ec3b`, and `d6d5b1e` restored source-space occupancy rules and exact resource/foliage records; `207e641` compensated asymmetric resource mesh pivots without changing generator coordinates. Later reports stopped describing all resources as one coincident clump. Preserve generation authority while debugging rendering.
- [x] **Primitive scale was corrected.** `b5549af` restored the omitted glTF scene-node `0.01` scale for primitive-label resource and foliage loads. This removed a known source of giant overlapping cards, but does not prove that ECS spawn paths cannot duplicate the same renderer.
- [x] **Ordinary shadow casting was restored.** `3268115` set the Tree and Grass material extensions back to shadow-enabled, and current resource/foliage paths no longer add `NotShadowCaster`. Later complaints shifted from “not casting shadows” back to flicker. Preserve casting during the next diagnostic.
- [x] **The blue palette-flash mechanism was corrected narrowly (`f0ed3e9`).** Tree colour variation now hashes the instance origin instead of each wind-deformed fragment, so a swaying canopy cannot change palette when a fragment crosses a 32-unit boundary. The shader also masks atlas-blue leakage. The subsequent report described in-game trees “flickering again,” rather than the previous specifically blue palette switching. Do not undo this fix.
- [x] **The menu has a separate stable-material path (`f0ed3e9`, updated in the current candidate).** Dense menu trees use a dedicated solid-colour `menu_tree` material instead of the gameplay atlas/wind material. It is now lit and shadow-receiving rather than unlit, so it remains protected from atlas-blue leakage while responding to the authored menu light. This is not evidence that the gameplay path is fixed.
- [x] **The old self-shadow suppression was isolated and retired for trees.** `NotShadowReceiver` was consistently reaching Tree/Grass/Critter renderers, so missing propagation was not the cause. Once the tree silhouette was synchronized, the component was proven to make canopies flat and insensitive to authored light; Tree materials no longer receive it. Grass/Critter retain the workaround until their passes are independently synchronized.
- [x] **Visible and shadow tree vertices now share one deformation implementation (current candidate).** Both shaders import the same bind-group-free `stream_town_tree_deformed_position` function with the serialized `Env_Tree.mat` constants. The fixed foliage smoke view changed from large black canopy facets to a coherent lit surface while keeping ground shadows. Unlike the failed approximate prepass, there is no separately maintained shadow wind equation. User confirmation and a moving capture are still required.
- [x] **The imported leaf-card two-sided contract is preserved (current candidate).** `Env_Tree.glb` explicitly declares `doubleSided: true`, but the typed gameplay and menu material overrides previously fell back to Bevy's back-face culling. Both overrides now set `double_sided = true` and `cull_mode = None`. Bevy's standard PBR input flips the back-face normal under this flag, so card backs receive outward-facing lighting instead of merely being exposed with inside-out normals.
- [x] **Runtime placement now consumes the generator's exact source-space offsets.** Generator version 6 already derived each resource and foliage position from the Unity loop and retained its sub-cell offset. The renderer incorrectly replaced that value with a hash of the coarser two-metre navigation cell, collapsing distinct Unity positions onto the same transform. The renderer now uses `offset_milli_cells` directly; this changes presentation only and does not read a legacy save or modify the source-derived generation algorithm.
- [x] **The requested central-half locational spread now preserves the lesson from the failed coarse hash.** The later explicit parity request requires visible resources and foliage to vary within the central 50% of each navigation cell. The renderer therefore hashes the world seed, stable generated ID, grid location, and retained source sub-cell offset together. Unlike the former grid-only hash, distinct source instances in one coarse cell cannot collapse onto the same transform. Counts, occupancy, stable gameplay positions, generator hashes, and saves remain untouched; no Unity-save coordinate is read.

## What did not fix the flicker

- [ ] **FAILED AS A COMPLETE FIX — higher directional-light shadow bias (`e00431e`).** Increasing depth/normal bias did not prevent later black and blue flicker reports. Do not tune global bias again without a shadow-only diagnostic proving acne is the remaining channel.
- [ ] **FAILED — disabling extension shadows while adding custom vegetation prepasses (`2a66489`).** This removed the required ground shadows and the user later reported that trees/foliage cast none.
- [ ] **FAILED — re-enabling shadows with a hard-coded approximate wind prepass (`3268115`).** Bevy's shadow-only pass lacked the extension material bind group, so the prepass embedded approximate wind constants/noise. Its silhouette did not match the visible shader and produced a second black/blue outline.
- [ ] **FAILED AS A COMPLETE FIX — repeatedly adding `NotShadowReceiver`.** Direct tree resources, generated foliage, menu material overrides, and GLB descendants all received the component through `e00431e`, `df08a63`, `f8e0223`, `ca3052d`, and `207e641`. It did not eliminate the underlying pass mismatch and also removed legitimate tree self-shadow/light response. It is now deliberately absent from Tree materials.
- [ ] **PARTIAL ONLY — colour/ambient changes (`f0ed3e9`).** Object-stable hashing and atlas masking removed a known blue-flash cause, and an unlit material stabilized the menu path. The gameplay flicker later returned, so broad colour brightening is not a complete fix.
- [ ] **FAILED AS A COMPLETE FIX — deleting the tree prepass (`207e641`).** Removing the mismatched wind silhouette was a sound diagnosis, but leaving the shadow pass undeformed did not match moving visible geometry. The current candidate restores a prepass only by sharing the exact same function with the visible pass.
- [ ] **NOT EVIDENCE — `tree_seasons_match_unity_material_targets`.** It checks parameters, the visible shader source string, shadow enablement, and absence of the custom prepass. It does not render multiple frames.
- [ ] **NOT EVIDENCE — `synchronized_tree_shadows_receive_light_while_older_card_materials_do_not`.** It checks the component policy only; it does not prove that every visible descendant follows it or that another pass cannot flicker.
- [ ] **NOT EVIDENCE — deterministic generation/unique stable-ID tests.** They prove data records, not that runtime systems spawn exactly one renderer per record.

## Do not retry unchanged

- [ ] Do not recreate an approximate tree wind prepass or hard-code another shadow-only wind function. The only allowed tree deformation is the function imported by both passes.
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

## Current attempts

- [ ] **`pending-a` — share the exact visible/shadow deformation while retaining `NotShadowReceiver`**
  - Object/spawn path: resource trees and generated tree foliage.
  - Fixed seed/camera/day/season: default deterministic smoke seed; `STREAM_TOWN_SMOKE_FOLIAGE=1`; day 0; Spring/Rain.
  - Single changed variable: visible and prepass shaders imported one shared wind function.
  - Duplicate renderer count: not measured in this pass.
  - Stationary-camera result: black facets disappeared in `.stream-town/diagnostics/foliage-shared-shadow.png`.
  - Moving-camera result: not checked.
  - Shadows still cast: yes, but trees remained flat because incoming shadows were suppressed.
  - User result: `partial` — rejected because trees neither self-shadowed nor responded sufficiently to light.
  - Reuse rule: keep the shared function; do not restore `NotShadowReceiver` for trees.

- [ ] **`pending-b` — let synchronized trees receive authored lighting and shadows**
  - Object/spawn path: resource trees, generated tree/bush foliage, Tree-material scene descendants, and lit solid-colour menu trees.
  - Fixed seed/camera/day/season: same smoke seed/camera/day/season as `pending-a`.
  - Single changed variable: removed Tree from the self-shadow suppression policy; menu tree material is lit.
  - Duplicate renderer count: not measured in this pass.
  - Stationary-camera result: local capture `.stream-town/diagnostics/foliage-lit-synchronized-shadow.png` shows coherent face lighting, canopy self-shadow detail, and ground shadows without the former large black facets.
  - Moving-camera result: not checked.
  - Shadows still cast: yes; trees now also receive them.
  - User result: `partial` — the latest report says the trees look okay and asks for a final card-normal/two-sided audit.
  - Reuse rule: if flicker remains, measure duplicate renderers or range dithering before changing lighting, shadow bias, or the shared deformation.

- [x] **`21a2fc7` — preserve the converted tree GLB's two-sided leaf-card material contract**
  - Object/spawn path: gameplay resource/foliage tree typed material and the dedicated menu tree material.
  - Fixed seed/camera/day/season: material contract audit; the latest user-visible run is reported stable.
  - Single changed variable: enabled two-sided PBR normal handling and disabled back-face culling on both tree overrides.
  - Duplicate renderer count: unchanged by this pass.
  - Stationary-camera result: user reports that the trees look okay before this final contract correction; the final GPU capture `.stream-town/diagnostics/foliage-double-sided-final.png` shows lit card fronts/backs and coherent ground shadows without a render or shader error.
  - Moving-camera result: the later source-offset acceptance sweep includes this correction and remains stable through its orbit/zoom sequence.
  - Shadows still cast: yes; no caster/receiver suppression was added.
  - User result: `accepted` — the latest explicit tree report says the trees look okay; the follow-up requested only verification of this two-sided contract.
  - Reuse rule: do not flip or rebuild mesh normals unless a captured exact primitive remains incorrectly lit with Bevy's two-sided face-normal correction active.

- [x] **`84dd40b` — retain Unity sub-cell positions and capture the actual failure conditions**
  - Object/spawn path: all in-game generated resource and foliage renderers.
  - Fixed seed/camera/day/season: default deterministic smoke seed; `STREAM_TOWN_SMOKE_FOLIAGE=1`; Spring/Rain; twelve 1920x1080 frames from a stationary hold followed by an orbit/zoom and return.
  - Single changed variable: replaced the renderer-only cell hash with generator-authored `offset_milli_cells`; no generator count, seed, threshold, or source position changed.
  - Duplicate renderer count: zero duplicate mesh/quantized-global-transform groups across 19,901 renderers after the correction (the first capture correctly failed and exposed the collapsed positions).
  - Stationary-camera result: frames 00–01 are stable and retain lit card fronts/backs plus ground shadows.
  - Moving-camera result: frames 02–10 remain stable across the close orbit/zoom; frame 11 returns to the starting view without black/blue facets.
  - Shadows still cast: yes; the manifest reports 19,901/19,901 casters and 19,901/19,901 receivers.
  - User result: `accepted before the presentation-only offset correction`; the correction was then verified by the complete structural and moving-camera sweep.
  - Reuse rule: rerun `scripts/capture-foliage-acceptance.ps1`; do not change tree shading unless its structural manifest passes and the new capture identifies a material/pass-specific regression.

- [ ] **`current candidate` — add the explicitly requested locational PRNG spread without restoring the coarse-hash collision**
  - Object/spawn path: all in-game generated resource and foliage renderers.
  - Fixed seed/camera/day/season: default deterministic world; structural sampling over 512 resource and 512 foliage records.
  - Single changed variable: presentation offset now hashes world seed plus the complete generated spatial identity into ±250 milli-cells per axis.
  - Duplicate renderer count: zero duplicate mesh/quantized-global-transform groups across 19,901 renderers; regression samples also contain more than 480 distinct offsets per 512 resource/foliage records.
  - Stationary-camera result: frames 00–01 in `.stream-town/diagnostics/foliage-locational-offset-final` are stable and show visibly off-centre ground cover.
  - Moving-camera result: all twelve frames and the assembled `foliage-sweep.mp4` pass the scripted orbit/zoom/return sweep.
  - Shadows still cast: yes; the manifest reports 19,901/19,901 casters and 19,901/19,901 receivers.
  - User result: not checked.
  - Reuse rule: never fall back to hashing only the coarse grid position; retain stable ID and source offset as collision discriminants.

The recorded local acceptance set is `.stream-town/diagnostics/foliage-moving-final-2026-08-25-v2`. It is intentionally ignored because twelve full-resolution PNGs are machine evidence, not shipping assets. Reproduce it from `bevy-port` with:

```powershell
.\scripts\capture-foliage-acceptance.ps1
```

The script validates the JSON manifest and assembles `foliage-sweep.mp4` when `ffmpeg` is installed.

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
