using System;
using System.IO;
using UnityEngine;

namespace SavingAndLoading
{
	/// <summary>
	/// Persists user settings only. Game snapshots are owned by SaveProcessor and
	/// cross the ISaveStorage boundary as a single versioned binary snapshot.
	/// </summary>
	public static class GameIO
	{
		// Kept for existing graphics bootstrap code that ensures the data folder exists.
		public const string SAVE_FILEPATH = "/Panda Belly/Stream Town/Saves/";

		private const string SettingsDirectoryName = "Panda Belly/Stream Town";
		private const string SettingsFileName = "SettingsData.json";

		private static string SettingsPath => Path.Combine(
			System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments),
			SettingsDirectoryName,
			SettingsFileName);

		public static void SaveSettingsData(SettingsData settings)
		{
			if (settings == null)
				throw new ArgumentNullException(nameof(settings));

			string directory = Path.GetDirectoryName(SettingsPath);
			if (!string.IsNullOrEmpty(directory))
				Directory.CreateDirectory(directory);

			File.WriteAllText(SettingsPath, JsonUtility.ToJson(settings, true));
			Debug.Log($"Settings saved to {SettingsPath}");
		}

		public static SettingsData LoadSettingsData()
		{
			if (!File.Exists(SettingsPath))
				throw new FileNotFoundException("No settings file exists.", SettingsPath);

			return JsonUtility.FromJson<SettingsData>(File.ReadAllText(SettingsPath));
		}

		/// <summary>
		/// Compatibility shim for older callers. Both legacy file kinds now refer
		/// to the one aggregate snapshot.
		/// </summary>
		[Obsolete("Resolve SaveProcessor and use HasSaveGame instead.")]
		public static bool DoesSaveFileExist(SaveFileType type)
		{
			return new BinarySaveStorage().SaveExists;
		}

		[Obsolete("Game and player data are stored together by SaveProcessor.")]
		public enum SaveFileType
		{
			GameSave,
			PlayersSave
		}
	}
}
