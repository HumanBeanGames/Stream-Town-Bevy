// SettingsManager.cs
//
// High-level overview:
// - Bridges between your persistent SettingsData model, the in-game UI widgets (Access_* wrappers),
//   and the actual engine/runtime (URP, QualitySettings, AudioMixer, etc).
// - When the settings panel opens, it pushes the current SettingsData values into the UI
//   (LoadedSettingsToUI) and snapshots a "baseline" copy used to detect unsaved changes.
// - While the panel is open, the user interacts with UI controls. When they hit "Apply/Confirm",
//   SaveSettings pulls all UI values back into SettingsData, persists them, and applies to the engine.
// - Presets are applied by copying values from VideoSettingsPreset assets into the model, then
//   reloading the UI to reflect those values.
// - Some small UI-only helpers update labels or enable/disable related controls based on toggles.
//

using UnityEngine;
using UnityEngine.Rendering.Universal;
using UnityEngine.Audio;
using URP;
using ShadowResolution = UnityEngine.Rendering.Universal.ShadowResolution;
using PlayerControls;
using Managers;
using Reflex.Attributes;
using System.Linq;

namespace Settings
{
    public class SettingsManager : MonoBehaviour
    {
        // UI: Resolution dropdown wrapper (DI via Reflex)
        [Inject] Access_ResolutionDropdown _resolutionDropdown;

        // Visual quality presets exposed in the UI.
        public enum Preset { Low, Medium, High, Ultra, Custom };

        // Tracks which preset button is currently highlighted/selected.
        private Preset _preset = Preset.Low;

        // Cache of the URP SSAO renderer feature so we don�t index into rendererFeatures by position.
        private ScriptableRendererFeature _ssaoFeatureCached;

        // Snapshot of settings used to detect "dirty" state (unsaved changes) while panel is open.
        private SettingsData _uiBaseline = new();

        // On scene start, push model -> UI, then apply to the engine to ensure runtime matches.
        public void Start()
        {
            TryInitialize();
        }

        private bool _initialized = false;

        private void TryInitialize()
        {
            if (_initialized)
                return;

            if (_displayModeDropdown == null || _renderPipeline == null)
                return;

            LoadedSettingsToUI();
            ApplySettingsToEngine();
            _initialized = true;
        }

        private void Update()
        {
            TryInitialize();
        }

        // Reference wrappers injected for camera and preset buttons.
        [Inject] ProjectCamera _camera;
        [Inject] PresetButtons _presetButtons;

        // Called by the Camera AA UI control to mark preset mode as "Custom".
        public void CameraAAOnChange() => SwitchToCustomPresetMode();

        // --- Camera sensitivity readout helpers (update numeric text labels as sliders move) ---

        [Inject] Access_PanningSensitivitySlider _panningSensitivitySlider;
        [Inject] Access_PanningSensitivityText _panningSensitivityText;
        public void PanSensitivityOnChange()
        {
            // Show slider value as 0.0 � 10.0 (slider val / 10) with one decimal.
            _panningSensitivityText.val = (_panningSensitivitySlider.val / 10).ToString("F1");
        }

        [Inject] Access_ZoomingSensitivitySlider _zoomingSensitivitySlider;
        [Inject] Access_ZoomingSensitivityText _zoomingSensitivityText;
        public void ZoomSensitivityOnChange()
        {
            _zoomingSensitivityText.val = (_zoomingSensitivitySlider.val / 10).ToString("F1");
        }

        [Inject] Access_WasdSensitivitySlider _wasdSensitivitySlider;
        [Inject] Access_WasdSensitivityText _wasdSensitivityText;
        public void WASDSensitivityOnChange()
        {
            _wasdSensitivityText.val = (_wasdSensitivitySlider.val / 10).ToString("F1");
        }

        [Inject] Access_EdgeScrollingSensitivitySlider _edgeScrollingSensitivity;
        [Inject] Access_EdgeScrollingSensitivityText _edgeScrollingSensitivityText;
        public void EdgeScrollingSensitivityOnChange()
        {
            _edgeScrollingSensitivityText.val = (_edgeScrollingSensitivity.val / 10).ToString("F1");
        }

        // --- Dependent interactivity helpers (enable/disable controls based on toggles) ---

        [Inject] Access_EdgeScrollingToggle _edgeScrollingToggle;
        public void EdgeScrollingToggle()
        {
            // Enable/disable the edge scrolling sensitivity slider when the feature is toggled.
            _edgeScrollingSensitivity.interactable = _edgeScrollingToggle.isOn;
        }

        [Inject] Access_MouseControlsToggle _mouseControlsToggle;
        public void MouseMovementToggle()
        {
            // If mouse movement is off, disable the pan sensitivity slider so it�s visually clear.
            _panningSensitivitySlider.interactable = _mouseControlsToggle.isOn;
        }

        // FOV label readout when the slider changes.
        [Inject] Access_FOVLevelSlider _fovLevelSlider;
        [Inject] Access_FOVLevelText _fovLevelText;
        public void FOVOnChange()
        {
            _fovLevelText.val = (_fovLevelSlider.val).ToString();
        }

        // Optional helper if a Unity Dropdown calls us with an int.
        public void PresetOnChangeInt(int v) => PresetOnChange((Preset)v);

        // Current save system + ScriptableObject presets injected via Reflex.
        [Inject] SaveState SaveState;
        [Inject] VideoSettingsPreset[] _settingPreset;

        // Called when a preset button is selected. Applies preset to the model, updates UI, and highlights the button.
        public void PresetOnChange(Preset pre)
        {
            if (pre != Preset.Custom)
            {
                int v = (int)pre;

                // Clear all other preset button highlights.
                for (int i = 0; i < _presetButtons.Buttons.Count; i++)
                {
                    _presetButtons.Buttons[i].transform.GetChild(0).GetChild(0).gameObject.SetActive(false);
                }

                // Copy the ScriptableObject preset values into the model.
                _settingPreset[v].ApplyTo(CurrentSettings);

                // Refresh widgets to reflect new model values.
                LoadedSettingsToUI();

                // Highlight this preset button and record the selection.
                _preset = pre;
                _presetButtons.Buttons[v].transform.GetChild(0).GetChild(0).gameObject.SetActive(true);
            }
        }

        // Marks the preset state as "Custom" and clears the preset button highlights.
        private void SwitchToCustomPresetMode()
        {
            foreach (var btn in _presetButtons.Buttons)
                btn.transform.GetChild(0).GetChild(0).gameObject.SetActive(false);
            _preset = Preset.Custom;
        }

        // These mark the preset state as custom when respective UI changes occur.
        public void AAOnChange() => SwitchToCustomPresetMode();
        public void ShadowQualityOnChange() => SwitchToCustomPresetMode();

        // Shadows-related UI widgets (type controls the interactivity of the quality dropdown).
        [Inject] Access_ShadowQualityDropdown _shadowQualityDropdown;
        [Inject] Access_ShadowTypeDropdown _shadowsTypeDropdown;
        public void ShadowsOnChange()
        {
            // Switch to custom then enable/disable the quality dropdown based on type (0=None, 1=Hard, 2=Soft, etc.)
            SwitchToCustomPresetMode();
            int type = _shadowsTypeDropdown.val;
            _shadowQualityDropdown.dropDown.interactable = (type == 1 || type == 2);
        }

        // More preset-busting toggles.
        public void VSyncToggle() => SwitchToCustomPresetMode();
        public void AOToggle() => SwitchToCustomPresetMode();

        // --- Audio label helpers (update % readouts as user drags sliders) ---

        [Inject] Access_MasterVolumeSlider _masterVolumeSlider;
        [Inject] Access_MasterVolumeText _masterVolumeText;
        [Inject] private AudioMixer _mixer;
        public void MasterVolumeOnChange()
        {
            _masterVolumeText.val = Mathf.RoundToInt(_masterVolumeSlider.val * 50).ToString();
        }

        [Inject] Access_MusicVolumeSlider _musicVolumeSlider;
        [Inject] Access_MusicVolumeText _musicVolumeText;
        public void MusicVolumeOnChange()
        {
            _musicVolumeText.val = Mathf.RoundToInt(_musicVolumeSlider.val * 50).ToString();
        }

        [Inject] Access_SoundEffectsVolumeSlider _soundEffectsVolumeSlider;
        [Inject] Access_SoundEffectsVolumeText _soundEffectsVolumeText;
        public void SoundEffectsVolumeOnChange()
        {
            _soundEffectsVolumeText.val = Mathf.RoundToInt(_soundEffectsVolumeSlider.val * 50).ToString();
        }

        [Inject] Access_AmbienceVolumeSlider _ambienceVolumeSlider;
        [Inject] Access_AmbienceVolumeText _ambienceVolumeText;
        public void AmbienceVolumeOnChange()
        {
            _ambienceVolumeText.val = Mathf.RoundToInt(_ambienceVolumeSlider.val * 50).ToString();
        }

        // Autosave widgets + model write helper when the dropdown changes.
        [Inject] private Autosave _autoSave;
        [Inject] Access_AutosaveTimerDropdown _autosaveTimerDropdown;
        public void AutoSaveTimer(int v)
        {
            _autosaveTimerDropdown.val = v;
            CurrentSettings.autosaveTime = v;
        }

        /// <summary>
        /// Disables/enables the connection tab in world game (UI hook).
        /// </summary>
        [Inject] ConnectionTab _connectionTab;
        public void TogglingConnectionTab(bool val)
        {
            _connectionTab.enabled = val;
        }

        /// <summary>
        /// Opens or closes the settings panel.
        /// On open: pushes model -> UI and snapshots the baseline for dirty checking.
        /// On close: if there are unsaved changes, shows confirm; else closes immediately.
        /// </summary>
        [Inject] SettingsPanel _settingsPanel;
        [Inject] ConfirmSettingsPanel _confirmSettingsPanel;
        public void ToggleSettingsPanel()
        {
            if (_settingsPanel.Enabled)
            {
                // Attempting to close
                if (HasUnsavedUIChanges())
                {
                    _confirmSettingsPanel.Enabled = true; // �You have unsaved changes�
                    return;
                }

                ChangeTab(0);
                _settingsPanel.Enabled = false;
                return;
            }

            // Opening
            LoadedSettingsToUI();          // model -> widgets
            SnapshotBaselineFromCurrent(); // baseline = saved model
            _confirmSettingsPanel.Enabled = false;
            _settingsPanel.Enabled = true;
        }

        /// <summary>
        /// "Apply/Confirm" button:
        /// Reads UI -> model, saves to disk, applies to engine, updates baseline, and closes UI state.
        /// </summary>
        public void ConfirmSettings()
        {
            SaveSettings();
            LoadedSettingsToUI();
            ChangeTab(0);
            _confirmSettingsPanel.Enabled = false;
            _settingsPanel.Enabled = false;
        }

        // URP & graphics system refs used to apply settings to the engine/runtime.
        [Inject] UniversalRenderPipelineAsset _renderPipeline;
        [Inject] UniversalRendererData _forwardRenderer;
        [Inject] GraphicsManager _graphics;

        /// <summary>
        /// Pushes SettingsData into Unity/URP/Audio systems so the game actually reflects the chosen values.
        /// </summary>
        private void ApplySettingsToEngine()
        {
            SettingsData s = CurrentSettings;

            // --- VIDEO / GRAPHICS ---

            // MSAA sample count (pipeline-wide)
            switch (s.antiAliasing)
            {
                case 1: _renderPipeline.msaaSampleCount = 2; break;
                case 2: _renderPipeline.msaaSampleCount = 4; break;
                case 3: _renderPipeline.msaaSampleCount = 8; break;
                default: _renderPipeline.msaaSampleCount = 0; break;
            }

            // Camera AA (FXAA/SMAA/None) on the main camera, if present
            if (_camera.Exists)
            {
                switch (s.cameraAA)
                {
                    case 1: _camera.Data.antialiasing = AntialiasingMode.FastApproximateAntialiasing; break;
                    case 2: _camera.Data.antialiasing = AntialiasingMode.SubpixelMorphologicalAntiAliasing; break;
                    default: _camera.Data.antialiasing = AntialiasingMode.None; break;
                }
                _camera.Data.antialiasingQuality = AntialiasingQuality.High;

                // Field of View
                _camera.Cam.fieldOfView = s.fov;
            }

            // Shadow casting mode (main + additional, hard/soft/none)
            switch (s.shadowType)
            {
                case 1:
                    UnityGraphics.MainLightCastShadows = true;
                    UnityGraphics.AdditionalLightCastShadows = true;
                    UnityGraphics.SoftShadowsEnabled = false;
                    break;
                case 2:
                    UnityGraphics.MainLightCastShadows = true;
                    UnityGraphics.AdditionalLightCastShadows = true;
                    UnityGraphics.SoftShadowsEnabled = true;
                    break;
                default:
                    UnityGraphics.AdditionalLightCastShadows = false;
                    UnityGraphics.MainLightCastShadows = false;
                    break;
            }

            // Shadow map resolution for main and additional lights
            switch (s.shadowResolution)
            {
                case 1: UnityGraphics.MainLightShadowResolution = ShadowResolution._512; UnityGraphics.AdditionalLightShadowResolution = ShadowResolution._512; break;
                case 2: UnityGraphics.MainLightShadowResolution = ShadowResolution._1024; UnityGraphics.AdditionalLightShadowResolution = ShadowResolution._1024; break;
                case 3: UnityGraphics.MainLightShadowResolution = ShadowResolution._2048; UnityGraphics.AdditionalLightShadowResolution = ShadowResolution._2048; break;
                case 4: UnityGraphics.MainLightShadowResolution = ShadowResolution._4096; UnityGraphics.AdditionalLightShadowResolution = ShadowResolution._4096; break;
                default: UnityGraphics.MainLightShadowResolution = ShadowResolution._256; UnityGraphics.AdditionalLightShadowResolution = ShadowResolution._256; break;
            }

            // SSAO toggle (via cached renderer feature)
            SetSsaoActive(s.enabledAO);

            // VSync on/off
            QualitySettings.vSyncCount = s.vSync ? 1 : 0;

            // FPS limiter -> Application.targetFrameRate (-1 means platform default)
            switch (s.fpsLimiter)
            {
                case 0: Application.targetFrameRate = 24; break;
                case 1: Application.targetFrameRate = 30; break;
                case 2: Application.targetFrameRate = 60; break;
                case 3: Application.targetFrameRate = 120; break;
                case 4: Application.targetFrameRate = 240; break;
                default: Application.targetFrameRate = -1; break;
            }

            // Display mode / windowing
            switch (s.displayMode)
            {
                case 1: Screen.fullScreenMode = FullScreenMode.FullScreenWindow; break;
                case 2: Screen.fullScreenMode = FullScreenMode.ExclusiveFullScreen; break;
                default: Screen.fullScreenMode = FullScreenMode.Windowed; break;
            }

            // Resolution (read from GraphicsManager list using stored index)
            if (s.resolution >= 0 && s.resolution < _graphics.Resolutions.Count)
            {
                var r = _graphics.Resolutions[s.resolution];
                Screen.SetResolution(r.width, r.height, Screen.fullScreenMode);
            }

            // Post-processing: gamma & exposure
            _graphics.LiftGammaGain.gamma.value = new Vector4(1, 1, 1, s.gamma);
            _graphics.ColorAdjustments.postExposure.value = s.brightness;

            // --- AUDIO ---
            // Convert linear [0..1] to mixer dB scale (log10 * 40 is common mapping)
            _mixer.SetFloat("_masterVolume", Mathf.Log10(Mathf.Max(s.masterVolume, 0.0001f)) * 40);
            _mixer.SetFloat("_musicVolume", Mathf.Log10(Mathf.Max(s.musicVolume, 0.0001f)) * 40);
            _mixer.SetFloat("_soundEffectsVolume", Mathf.Log10(Mathf.Max(s.playerVolume, 0.0001f)) * 40);
            _mixer.SetFloat("_ambienceVolume", Mathf.Log10(Mathf.Max(s.environmentVolume, 0.0001f)) * 40);

            // --- GAMEPLAY / MISC ---
            // Example of wiring autosave interval to SaveManager
            if (GameManager.Instance)
                GameManager.Instance.SaveManager.SetAutosaveTime(_autoSave.Intervals[s.autosaveTime] * 60.0f);
        }

        // Try to find SSAO feature once and cache it (robust against renderer feature ordering).
        private void CacheSsaoFeature()
        {
            if (_forwardRenderer == null) return;

            if (_ssaoFeatureCached != null && _forwardRenderer.rendererFeatures.Contains(_ssaoFeatureCached))
                return;

            var ssaoType = typeof(ScreenSpaceAmbientOcclusion); // strong type
            _ssaoFeatureCached = _forwardRenderer.rendererFeatures
                .FirstOrDefault(f => f != null && ssaoType.IsAssignableFrom(f.GetType()));
        }

        /// <summary>
        /// Toggle SSAO feature if present. Safe no-op if feature is missing from the renderer.
        /// </summary>
        private void SetSsaoActive(bool enabled)
        {
            CacheSsaoFeature();
            if (_ssaoFeatureCached != null)
                _ssaoFeatureCached.SetActive(enabled);
        }

        /// <summary>
        /// Reads current UI widget values into the model, saves to disk, applies to engine, and updates the baseline snapshot.
        /// </summary>
        [Inject] SettingsData CurrentSettings;
        public void SaveSettings()
        {
            // Pull UI -> model and persist.
            ReadUIInto(CurrentSettings);
            SettingsIO.Save(CurrentSettings);
            ApplySettingsToEngine();

            // After saving, the model reflects the UI, so make it the new "clean" baseline.
            SnapshotBaselineFromCurrent();
        }

        /// <summary>
        /// Reset to default settings (new SettingsData) and refresh the UI to show those defaults.
        /// </summary>
        public void SetToDefaultSettings()
        {
            CurrentSettings.GetFrom(new SettingsData());
            LoadedSettingsToUI();
        }

        /// <summary>
        /// Close the panel without saving: restore model from the baseline snapshot and update UI.
        /// </summary>
        public void CloseSettingPanel()
        {
            ChangeTab(0);
            CurrentSettings.GetFrom(_uiBaseline);
            LoadedSettingsToUI();
            _confirmSettingsPanel.SetActive(false);
            _settingsPanel.SetActive(false);
        }

        // --- Model <-> UI binding ---

        [Inject] Access_DisplayModeDropdown _displayModeDropdown;
        [Inject] Access_AADropdown _AADropdown;
        [Inject] Access_VsyncToggle _vSyncToggle;
        [Inject] Access_AOToggle _AOToggle;
        [Inject] Access_GammaSlider _gammaSlider;
        [Inject] Access_BrightnessSlider _brightnessSlider;
        [Inject] Access_FPSLimiterDropdown _fpsLimiterDropdown;
        [Inject] Access_ChannelNameInput _channelNameInput;
        [Inject] Access_CameraAADropdown _cameraAADropdown;
        [Inject] Access_DisplayNameDropdown _displayNameDropdown;
        [Inject] Access_DisplayBuildingDamageDropdown _displayBuildingDamageDropdown;

        /// <summary>
        /// Push model -> UI (sets widget values), then runs UI-only refresh helpers
        /// (e.g., enabling/disabling controls and updating numeric readouts).
        /// </summary>
        public void LoadedSettingsToUI()
        {
            // --- 1) Model -> Widgets ---
            // Video / Display
            _displayModeDropdown.val = CurrentSettings.displayMode;
            _resolutionDropdown.val = CurrentSettings.resolution;

            // Shadows & AA
            _shadowsTypeDropdown.val = CurrentSettings.shadowType;
            _shadowQualityDropdown.val = CurrentSettings.shadowResolution;
            _AADropdown.val = CurrentSettings.antiAliasing;
            // If you have a Camera AA dropdown wrapper, set it here too:
            // _cameraAADropdown.val        = CurrentSettings.cameraAA;

            // Toggles
            _vSyncToggle.isOn = CurrentSettings.vSync;
            _AOToggle.isOn = CurrentSettings.enabledAO;

            // PostFX
            _gammaSlider.val = CurrentSettings.gamma;
            _brightnessSlider.val = CurrentSettings.brightness;

            // FPS / Autosave
            _fpsLimiterDropdown.val = CurrentSettings.fpsLimiter;
            _autosaveTimerDropdown.val = CurrentSettings.autosaveTime;

            // Audio
            _masterVolumeSlider.val = CurrentSettings.masterVolume;
            _musicVolumeSlider.val = CurrentSettings.musicVolume;
            _soundEffectsVolumeSlider.val = CurrentSettings.playerVolume;
            _ambienceVolumeSlider.val = CurrentSettings.environmentVolume;

            // Camera/Input
            _panningSensitivitySlider.val = CurrentSettings.panSensitivity;
            _zoomingSensitivitySlider.val = CurrentSettings.zoomSensitivity;
            _wasdSensitivitySlider.val = CurrentSettings.wasdSensitivity;
            _edgeScrollingSensitivity.val = CurrentSettings.edgeScrollingSensitivity;
            _edgeScrollingToggle.isOn = CurrentSettings.edgeScrolling;
            _fovLevelSlider.val = CurrentSettings.fov;
            _mouseControlsToggle.isOn = CurrentSettings.mouseControls;
            _cameraAADropdown.val = CurrentSettings.cameraAA;

            // Game options
            _displayNameDropdown.val = CurrentSettings.displayNames;
            _displayBuildingDamageDropdown.val = CurrentSettings.displayBuildingDamage;

            // Text inputs (if any)
            _channelNameInput.text = CurrentSettings.channelName;

            // --- 2) UI-only refresh (no model writes) ---
            // These update labels or enable/disable related controls based on the above values.
            ShadowsOnChange();

            MasterVolumeOnChange();
            MusicVolumeOnChange();
            SoundEffectsVolumeOnChange();
            AmbienceVolumeOnChange();

            PanSensitivityOnChange();
            ZoomSensitivityOnChange();
            WASDSensitivityOnChange();
            EdgeScrollingSensitivityOnChange();
            EdgeScrollingToggle();      // set interactivity for edge sensitivity slider
            MouseMovementToggle();      // set interactivity for pan sensitivity slider
            FOVOnChange();

            // Example of pushing autosave interval to SaveManager
            if (GameManager.Instance)
                GameManager.Instance.SaveManager.SetAutosaveTime(_autoSave.Intervals[CurrentSettings.autosaveTime] * 60.0f);
        }

        /// <summary>
        /// Pull UI -> the provided SettingsData instance (used by SaveSettings and snapshots).
        /// </summary>
        private void ReadUIInto(SettingsData dst)
        {
            dst.displayMode = _displayModeDropdown.val;
            dst.resolution = _resolutionDropdown.val;

            dst.shadowType = _shadowsTypeDropdown.val;
            dst.shadowResolution = _shadowQualityDropdown.val;
            dst.antiAliasing = _AADropdown.val;

            dst.vSync = _vSyncToggle.isOn;
            dst.enabledAO = _AOToggle.isOn;

            dst.gamma = _gammaSlider.val;
            dst.brightness = _brightnessSlider.val;

            dst.fpsLimiter = _fpsLimiterDropdown.val;
            dst.autosaveTime = _autosaveTimerDropdown.val;

            dst.masterVolume = _masterVolumeSlider.val;
            dst.musicVolume = _musicVolumeSlider.val;
            dst.playerVolume = _soundEffectsVolumeSlider.val;
            dst.environmentVolume = _ambienceVolumeSlider.val;

            dst.panSensitivity = _panningSensitivitySlider.val;
            dst.zoomSensitivity = _zoomingSensitivitySlider.val;
            dst.wasdSensitivity = _wasdSensitivitySlider.val;
            dst.edgeScrollingSensitivity = _edgeScrollingSensitivity.val;
            dst.edgeScrolling = _edgeScrollingToggle.isOn;
            dst.fov = (int)_fovLevelSlider.val;
            dst.mouseControls = _mouseControlsToggle.isOn;

            dst.cameraAA = _cameraAADropdown.val;

            dst.displayBuildingDamage = _displayBuildingDamageDropdown.val;
            dst.displayNames = _displayNameDropdown.val;

            dst.channelName = _channelNameInput.text;
        }

        /// <summary>
        /// Build a temporary SettingsData snapshot from current UI state for dirty comparison.
        /// </summary>
        private SettingsData BuildCandidateFromUI()
        {
            var s = new SettingsData();
            ReadUIInto(s);
            return s;
        }

        /// <summary>
        /// Copy the current (saved) model into the baseline snapshot for later dirty checks.
        /// </summary>
        private void SnapshotBaselineFromCurrent()
        {
            // reuse your copy helper
            _uiBaseline.GetFrom(CurrentSettings);
        }

        /// <summary>
        /// Returns true if UI differs from the baseline snapshot (unsaved changes exist).
        /// </summary>
        private bool HasUnsavedUIChanges()
        {
            var candidate = BuildCandidateFromUI();
            return !SettingsData.SettingsEqual(candidate, _uiBaseline);
        }

        // --- Tab/page switching for the settings UI ---

        [Inject] Access_SettingsMenus _menus;
        [Inject] Access_SettingsTabs _tabs;
        public void ChangeTab(int v)
        {
            // Clear all tab highlights
            for (int i = 0; i < _tabs.list.Count; i++)
            {
                _tabs.list[i].transform.GetChild(0).GetChild(0).gameObject.SetActive(false);
            }

            // Highlight the selected tab
            _tabs.list[v].transform.GetChild(0).GetChild(0).gameObject.SetActive(true);

            // Show only the selected page/panel
            for (int i = 0; i < _menus.list.Count; i++)
            {
                _menus.list[i].SetActive(false);
            }

            _menus.list[v].SetActive(true);
        }
    }
}
