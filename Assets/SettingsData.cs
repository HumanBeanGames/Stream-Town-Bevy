// SettingsData.cs
//
// Purpose:
// --------
// A serializable, plain-data container for all user-configurable settings.
// This class is the single source of truth that:
//   • Stores current settings (what the user last chose or what defaults specify)
//   • Moves values between disk (save/load), the settings UI (model <-> widgets),
//     and the runtime engine (URP, AudioMixer, etc.)
// It intentionally contains no engine calls — only data and small helpers
// (copy, clamp, and equality checks to support "unsaved changes" prompts).

using UnityEngine;

[System.Serializable]
public class SettingsData
{
    // =========================
    // VIDEO / GRAPHICS SETTINGS
    // =========================

    /// <summary>
    /// Which preset the UI currently represents. This is informational and used to light up the correct button;
    /// values below still hold the actual applied configuration.
    /// </summary>
    public Settings.SettingsProcessor.Preset preset;

    /// <summary>
    /// Display mode index (e.g., 0=Windowed, 1=Borderless, 2=Exclusive Fullscreen).
    /// Exact mapping is defined by the UI/SettingsProcessor.
    /// </summary>
    public int displayMode;

    /// <summary>
    /// Resolution index into your GraphicsProcessor.Resolutions list.
    /// </summary>
    public int resolution;

    /// <summary>
    /// Shadow type index (0=None, 1=Hard, 2=Soft). Matches your UI dropdown.
    /// </summary>
    public int shadowType;

    /// <summary>
    /// Shadow map resolution index (0=256, 1=512, 2=1024, 3=2048, 4=4096). Matches your UI dropdown.
    /// </summary>
    public int shadowResolution;

    /// <summary>
    /// Screen-space ambient occlusion toggle.
    /// </summary>
    public bool enabledAO;

    /// <summary>
    /// VSync toggle. (QualitySettings.vSyncCount = isOn ? 1 : 0)
    /// </summary>
    public bool vSync;

    /// <summary>
    /// FPS limiter index (e.g., 0=24,1=30,2=60,3=120,4=240,5=Unlimited). Exact mapping is in SettingsProcessor.
    /// </summary>
    public int fpsLimiter;

    /// <summary>
    /// Post-exposure (in EV) applied via ColorAdjustments.postExposure.
    /// </summary>
    public float brightness;

    /// <summary>
    /// Gamma lift (stored as the 'A' of LiftGammaGain.gamma). Exact mapping defined in SettingsProcessor.
    /// </summary>
    public float gamma;

    /// <summary>
    /// Hardware MSAA setting (URP asset). 0=Off, 1=2x, 2=4x, 3=8x.
    /// </summary>
    public int antiAliasing;   // MSAA: 0=Off,1=2x,2=4x,3=8x

    /// <summary>
    /// Camera post AA (per-camera). 0=None, 1=FXAA, 2=SMAA.
    /// </summary>
    public int cameraAA;       // Post AA: 0=None,1=FXAA,2=SMAA

    // ==========
    // AUDIO MIX
    // ==========

    /// <summary> Linear [0..1] master bus volume. Converted to dB in SettingsProcessor. </summary>
    public float masterVolume;
    /// <summary> Linear [0..1] music bus volume. </summary>
    public float musicVolume;
    /// <summary> Linear [0..1] SFX/player bus volume. </summary>
    public float playerVolume;
    /// <summary> Linear [0..1] ambience/environment bus volume. </summary>
    public float environmentVolume;

    // =================
    // GAMEPLAY / CAMERA
    // =================

    /// <summary> Mouse pan sensitivity (UI uses a 0..100 or similar scale; shown as val/10 in labels). </summary>
    public float panSensitivity;
    /// <summary> Mouse wheel zoom sensitivity. </summary>
    public float zoomSensitivity;
    /// <summary> Keyboard WASD pan sensitivity. </summary>
    public float wasdSensitivity;
    /// <summary> Edge scrolling speed sensitivity. </summary>
    public float edgeScrollingSensitivity;
    /// <summary> Camera field-of-view in degrees. </summary>
    public int fov;

    /// <summary> Autosave interval index; mapped to seconds via Autosave.Intervals in SettingsProcessor. </summary>
    public int autosaveTime;

    // ============
    // UI DISPLAY
    // ============

    /// <summary>
    /// Username display mode. 0=None, 1=AllPlayers, 2=ModeratorsAndSubscribers.
    /// </summary>
    public int displayNames;

    /// <summary>
    /// Building health overlay mode. 0=None, 1=DamagedOnly, 2=Always.
    /// </summary>
    public int displayBuildingDamage;

    // ======
    // INPUT
    // ======

    /// <summary> Enable edge scrolling at screen borders. </summary>
    public bool edgeScrolling;

    /// <summary> Enable keyboard (WASD) movement. (Reserved for use by input layer/UI, not enforced here.) </summary>
    public bool keyboardMovement;

    /// <summary> Enable mouse controls (e.g., drag-to-pan). </summary>
    public bool mouseControls;

    // =============
    // INTEGRATIONS
    // =============

    /// <summary> Optional integration channel string (e.g., community/stream channel name). </summary>
    public string channelName;

    // ==========
    // DEFAULTS
    // ==========
    public SettingsData()
    {
        // Preset recorded as "Custom" by default; actual values below define the effective state.
        preset = Settings.SettingsProcessor.Preset.Custom;

        // Sensible engine defaults (match these to your project's expected out-of-box settings)
        displayMode = 2;      // Example: Exclusive Fullscreen
        resolution = 18;      // Example index into your GraphicsProcessor.Resolutions

        shadowType = 2;       // Soft shadows
        shadowResolution = 4; // 4096
        enabledAO = true;

        vSync = true;
        fpsLimiter = 5;       // If 5 = Unlimited; ensure UI mapping matches.

        brightness = 0f;      // EV
        gamma = 0f;           // Lift A

        antiAliasing = 3;     // MSAA x8
        cameraAA = 2;         // SMAA

        masterVolume = 1f;
        musicVolume = 1f;
        playerVolume = 1f;
        environmentVolume = 1f;

        panSensitivity = 10f;
        zoomSensitivity = 10f;
        wasdSensitivity = 10f;
        edgeScrollingSensitivity = 10f;

        fov = 60;

        autosaveTime = 3;

        // UI dropdown defaults (ensure these are within the actual dropdown ranges 0..2)
        displayNames = 0;            // None
        displayBuildingDamage = 1;   // Damaged Only

        edgeScrolling = true;
        keyboardMovement = true;
        mouseControls = true;

        channelName = "";
    }

    // =========================
    // STRUCTURAL DATA OPERATIONS
    // =========================

    /// <summary>
    /// Copy all fields from another SettingsData (shallow field-wise copy).
    /// Useful for snapshots (e.g., taking a "baseline" when opening the settings UI).
    /// </summary>
    public void GetFrom(SettingsData other)
    {
        preset = other.preset;
        displayMode = other.displayMode;
        resolution = other.resolution;

        shadowType = other.shadowType;
        shadowResolution = other.shadowResolution;
        enabledAO = other.enabledAO;

        vSync = other.vSync;
        fpsLimiter = other.fpsLimiter;

        brightness = other.brightness;
        gamma = other.gamma;

        antiAliasing = other.antiAliasing;
        cameraAA = other.cameraAA;

        masterVolume = other.masterVolume;
        musicVolume = other.musicVolume;
        playerVolume = other.playerVolume;
        environmentVolume = other.environmentVolume;

        panSensitivity = other.panSensitivity;
        zoomSensitivity = other.zoomSensitivity;
        wasdSensitivity = other.wasdSensitivity;
        edgeScrollingSensitivity = other.edgeScrollingSensitivity;
        fov = other.fov;

        autosaveTime = other.autosaveTime;

        displayNames = other.displayNames;
        displayBuildingDamage = other.displayBuildingDamage;

        edgeScrolling = other.edgeScrolling;
        keyboardMovement = other.keyboardMovement;
        mouseControls = other.mouseControls;

        channelName = other.channelName;
    }

    /// <summary>
    /// Guard against invalid indices when loading older or corrupted saves.
    /// Call this right after deserialization, before pushing values into UI/engine.
    /// </summary>
    public void ClampToUiRanges()
    {
        // Adjust ranges to match your actual dropdowns / UI mappings.
        displayMode = Mathf.Clamp(displayMode, 0, 2);
        shadowType = Mathf.Clamp(shadowType, 0, 2);
        shadowResolution = Mathf.Clamp(shadowResolution, 0, 4);
        antiAliasing = Mathf.Clamp(antiAliasing, 0, 3);
        cameraAA = Mathf.Clamp(cameraAA, 0, 2);

        displayNames = Mathf.Clamp(displayNames, 0, 2);
        displayBuildingDamage = Mathf.Clamp(displayBuildingDamage, 0, 2);

        // If 5 == Unlimited in your UI. Adjust if your mapping differs.
        fpsLimiter = Mathf.Clamp(fpsLimiter, 0, 5);
    }

    // ==========================
    // EQUALITY / DIRTY CHECKING
    // ==========================

    /// <summary>
    /// Small float comparison helper for settings sliders (tolerates minor rounding).
    /// </summary>
    private static bool Nearly(float a, float b, float eps = 0.0005f)
        => Mathf.Abs(a - b) <= eps;

    /// <summary>
    /// Field-by-field comparison of two SettingsData instances to determine if anything visible in the UI changed.
    /// Used by SettingsProcessor to trigger "unsaved changes" prompts.
    /// Update this if you add/remove settings visible to the user.
    /// </summary>
    public static bool SettingsEqual(SettingsData x, SettingsData y)
    {
        // --- Video/Graphics ---
        if (x.displayMode != y.displayMode) return false;
        if (x.resolution != y.resolution) return false;
        if (x.shadowType != y.shadowType) return false;
        if (x.shadowResolution != y.shadowResolution) return false;
        if (x.antiAliasing != y.antiAliasing) return false;
        if (x.cameraAA != y.cameraAA) return false;
        if (x.vSync != y.vSync) return false;
        if (x.enabledAO != y.enabledAO) return false;

        if (!Nearly(x.gamma, y.gamma)) return false;
        if (!Nearly(x.brightness, y.brightness)) return false;

        if (x.fpsLimiter != y.fpsLimiter) return false;
        if (x.autosaveTime != y.autosaveTime) return false;

        // --- Audio ---
        if (!Nearly(x.masterVolume, y.masterVolume)) return false;
        if (!Nearly(x.musicVolume, y.musicVolume)) return false;
        if (!Nearly(x.playerVolume, y.playerVolume)) return false;
        if (!Nearly(x.environmentVolume, y.environmentVolume)) return false;

        // --- Camera / Input ---
        if (!Nearly(x.panSensitivity, y.panSensitivity)) return false;
        if (!Nearly(x.zoomSensitivity, y.zoomSensitivity)) return false;
        if (!Nearly(x.wasdSensitivity, y.wasdSensitivity)) return false;
        if (!Nearly(x.edgeScrollingSensitivity, y.edgeScrollingSensitivity)) return false;
        if (x.edgeScrolling != y.edgeScrolling) return false;
        if (x.mouseControls != y.mouseControls) return false;
        if (x.fov != y.fov) return false;

        // --- UI Display prefs ---
        if (x.displayNames != y.displayNames) return false;
        if (x.displayBuildingDamage != y.displayBuildingDamage) return false;

        // --- Integrations / Misc ---
        if (!string.Equals(x.channelName, y.channelName)) return false;

        return true;
    }
}
