using System;
using System.IO;
using UnityEngine;

public static class SettingsIO
{
    private static readonly string SettingsDir =
        Path.Combine(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments), "Panda Belly", "Stream Town");
    private static readonly string SettingsPath = Path.Combine(SettingsDir, "SettingsData.json");

    // Optional: cache so you only hit disk once
    private static SettingsData _cached;

    public static SettingsData LoadOrCreate()
    {
        if (_cached != null) return _cached;

        if (File.Exists(SettingsPath))
        {
            try
            {
                var json = File.ReadAllText(SettingsPath);
                _cached = JsonUtility.FromJson<SettingsData>(json);
                if (_cached != null) return _cached;
                Debug.LogWarning("Settings file existed but failed to parse. Recreating defaults.");
            }
            catch (Exception ex)
            {
                Debug.LogError($"Failed to read settings; recreating defaults. {ex}");
            }
        }

        // Create defaults in memory (no read), then persist once
        _cached = new SettingsData();
        Save(_cached);
        return _cached;
    }

    public static void Save(SettingsData data)
    {
        try
        {
            Directory.CreateDirectory(SettingsDir);
            var json = JsonUtility.ToJson(data, prettyPrint: true);

            // (Nice-to-have) atomic write to avoid partial files
            var tmp = SettingsPath + ".tmp";
            File.WriteAllText(tmp, json);
            if (File.Exists(SettingsPath)) File.Replace(tmp, SettingsPath, null);
            else File.Move(tmp, SettingsPath);

            // Update cache
            _cached = data;
        }
        catch (Exception ex)
        {
            Debug.LogError($"Failed to save settings: {ex}");
        }
    }
}
