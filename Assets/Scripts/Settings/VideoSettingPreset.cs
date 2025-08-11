using UnityEngine;

[System.Serializable]
[CreateAssetMenu(menuName = "Settings/Setting Preset", fileName = "NewSettingPreset")]
public class VideoSettingsPreset : ScriptableObject
{
    public string presetName;
    public int antiAliasing;
    public int shadowType;
    public int shadowResolution;
    public bool vSync;
    public bool enabledAO;
    public int cameraAA;

    public void ApplyTo(SettingsData s)
    {
        s.antiAliasing = antiAliasing;
        s.shadowType = shadowType;
        s.shadowResolution = shadowResolution;
        s.vSync = vSync;
        s.enabledAO = enabledAO;
        s.cameraAA = cameraAA;
    }
}