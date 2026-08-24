# Accessibility

Stream Town exposes its shipping Bevy UI through AccessKit and supports complete
non-pointer menu operation. Accessibility preferences are part of the same
atomic, backup-protected player-settings file as video, audio, and camera
preferences.

## Persisted preferences

Open **Settings > Accessibility** in the game or the Settings tab in
`stream_town_tools`:

- **UI Scale:** 75%, 90%, 100%, 110%, 125%, or 150%.
- **High Contrast:** adds a dark text separation shadow and stronger persistent
  control outlines without replacing authored UI art.
- **Reduced Motion:** freezes non-essential looping presentation motion:
  loading spinners, tree and grass wind, water ripples, menu clouds, menu fish,
  windmill blades, rain/snow presentation particles, and Credits fireworks.
  Actor movement and other authoritative gameplay state continue normally.

These fields use player-settings schema 3. Schema-1 and schema-2 files upgrade
automatically; existing files receive 100% scale with high contrast and reduced
motion off, preserving their previous appearance.

## Keyboard operation

- **Main Menu, in-game HUD and vote controls, Credits:** Tab and Shift+Tab move
  between visible enabled controls. Once keyboard focus is visible, arrow keys
  also move focus. Enter or Space activates the focused control.
- **Game menu:** Up/Down selects a row and Enter activates it. Escape closes the
  menu.
- **Settings:** Tab and Shift+Tab change category; Up/Down selects a setting;
  Left/Right changes its value; Enter activates Apply, Defaults, or Back; Escape
  follows the same unsaved-draft confirmation path as Back.
- **Modal confirmation:** keyboard and assistive-technology focus is confined to
  Apply or Discard until the dialog is resolved.
- **Pointer handoff:** clicking a control moves logical focus to it while hiding
  the keyboard-only focus ring. The next keyboard navigation action restores
  the ring.

Focused controls use a four-pixel gold outline. High Contrast retains a subtle
outline on other visible controls as an additional boundary cue.

## Screen-reader contract

Bevy's Windows accessibility adapter publishes AccessKit nodes for shipping
buttons and text. Runtime enhancement provides:

- labels for authored actions and dynamic setting values;
- Button, Tab, CheckBox, Label, Status, and ProgressIndicator roles;
- selected, checked, disabled, and hidden state where applicable;
- Click, Focus, and Blur action handling;
- a polite live status region for scene/menu transitions, loading state,
  selected settings, and gameplay command feedback; and
- a numeric 0-100 loading progress range with its displayed percentage.

AccessKit Click requests set the same Bevy `Interaction::Pressed` state used by
pointer input, so assistive technology cannot bypass the normal validation or
action handlers.

## Automated verification

From `bevy-port`:

```powershell
cargo test -p stream_town_domain settings::tests
cargo test -p stream_town_game --lib accessibility
cargo test -p stream_town_game --lib screen_reader_click_uses_the_same_shipping_menu_action
cargo test -p stream_town_game --lib loading_progress_exposes_accesskit_range_and_live_value
cargo xtask validate
```

The full workspace gate remains:

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo xtask stress --agents 300
```

## Windows Narrator acceptance

1. Start Narrator with **Windows+Ctrl+Enter**, then run
   `cargo run -p stream_town_game`.
2. After the loading screen, press Tab. Confirm Narrator announces the focused
   Main Menu action and the gold focus outline is visible.
3. Navigate to Settings, open Accessibility, and confirm each tab and setting
   value is announced. Change UI Scale, High Contrast, and Reduced Motion,
   choose Apply, then restart the game and confirm all three persisted.
4. Reopen Settings, change a value, press Escape, and confirm focus remains
   inside the Apply/Discard dialog until one option is activated.
5. Start a town. Tab through HUD and vote actions, use Enter or Space on an
   enabled action, and confirm disabled actions are announced but skipped by
   keyboard traversal.
6. During a cold load, confirm loading progress and the transition to the town
   are announced. Trigger a local UI action and confirm its result reaches the
   polite status announcement.
7. Enable Reduced Motion and confirm decorative loops stop while camera,
   selection, actor simulation, and menu operation remain responsive.
