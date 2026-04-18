using UnityEngine;
using System.IO;
using System.Runtime.Serialization.Formatters.Binary;
using System.Threading.Tasks;
using SavingAndLoading.Structs;

namespace SavingAndLoading
{
    /// <summary>
    /// Static class for game save/load operations.
    /// </summary>
	public static class GameIO
	{
        /// <summary>
        /// The save file path.
        /// </summary>
		public const string SAVE_FILEPATH = "/Panda Belly/Stream Town/Saves/";

        /// <summary>
        /// The game save file path.
        /// </summary>
		public const string GAME_SAVE_FILEPATH = "GameSave.pog";

        /// <summary>
        /// The player save file path.
        /// </summary>
		public const string PLAYER_SAVE_FILEPATH = "PlayersSave.pog";

        /// <summary>
        /// Loads the game save data.
        /// </summary>
        /// <returns>The save game data.</returns>
		public static SaveGameData LoadGameData()
		{
			BinaryFormatter formatter = new BinaryFormatter();
			FileStream fStream = new FileStream(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + GAME_SAVE_FILEPATH, FileMode.Open);
			SaveGameData data = formatter.Deserialize(fStream) as SaveGameData;
			fStream.Close();
			Debug.Log("GameIO: Loading from -> " + System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + GAME_SAVE_FILEPATH);
			return data;
		}

        /// <summary>
        /// Loads the game save data asynchronously.
        /// </summary>
        /// <returns>A task that returns the save game data.</returns>
		public static Task<SaveGameData> LoadGameDataAsync()
		{
			return Task.Run(LoadGameData);
		}

        /// <summary>
        /// Saves the game save data.
        /// </summary>
        /// <param name="data">The save game data.</param>
		public static void SaveGameData(SaveGameData data)
		{
			BinaryFormatter formatter = new BinaryFormatter();
			FileStream fStream = new FileStream(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + GAME_SAVE_FILEPATH, FileMode.Create);
			formatter.Serialize(fStream, data);
			fStream.Close();
			Debug.Log("GameIO: Saving to -> " + System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + GAME_SAVE_FILEPATH);
		}

        /// <summary>
        /// Loads the players save data.
        /// </summary>
        /// <returns>The save players data.</returns>
		public static SavePlayersData LoadPlayersData()
		{
			BinaryFormatter formatter = new BinaryFormatter();
			FileStream fStream = new FileStream(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + PLAYER_SAVE_FILEPATH, FileMode.Open);
			SavePlayersData data = formatter.Deserialize(fStream) as SavePlayersData;
			fStream.Close();
			Debug.Log("GameIO: Loading from -> " + System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + PLAYER_SAVE_FILEPATH);
			return data;
		}

        /// <summary>
        /// Loads the players save data asynchronously.
        /// </summary>
        /// <returns>A task that returns the save players data.</returns>
		public static Task<SavePlayersData> LoadPlayersDataAsync()
		{
			return Task.Run(LoadPlayersData);
		}

        /// <summary>
        /// Saves the players save data.
        /// </summary>
        /// <param name="data">The save players data.</param>
		public static void SavePlayersData(SavePlayersData data)
		{
			BinaryFormatter formatter = new BinaryFormatter();
			FileStream fStream = new FileStream(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + PLAYER_SAVE_FILEPATH, FileMode.Create);
			formatter.Serialize(fStream, data);
			fStream.Close();
			Debug.Log("GameIO: Saving to -> " + System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + PLAYER_SAVE_FILEPATH);
		}

        /// <summary>
        /// Saves the settings data.
        /// </summary>
        /// <param name="savePreset">The settings data.</param>
		public static void SaveSettingsData(SettingsData savePreset)
		{
			string data = JsonUtility.ToJson(savePreset);
			File.WriteAllText(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + "/Panda Belly/Stream Town/SettingsData.json", data);
			Debug.Log("File location : " + System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + "/Panda Belly/Stream Town/SettingsData.json");
		}

        /// <summary>
        /// Loads the settings data.
        /// </summary>
        /// <returns>The settings data.</returns>
		public static SettingsData LoadSettingsData()
		{
			string fileContents = File.ReadAllText(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + "/Panda Belly/Stream Town/SettingsData.json");
			return JsonUtility.FromJson<SettingsData>(fileContents);
		}

        /// <summary>
        /// Checks if a save file exists.
        /// </summary>
        /// <param name="type">The type of save file.</param>
        /// <returns>True if the save file exists, false otherwise.</returns>
		public static bool DoesSaveFileExist(SaveFileType type)
		{
			string directory = " ";

			switch (type)
			{
				case SaveFileType.GameSave:
					directory = GAME_SAVE_FILEPATH;
					break;
				case SaveFileType.PlayersSave:
					directory = PLAYER_SAVE_FILEPATH;
					break;
			}

			return File.Exists(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + SAVE_FILEPATH + directory);
		}

        /// <summary>
        /// Enum representing different types of save files.
        /// </summary>
		public enum SaveFileType
		{
            /// <summary>
            /// Game save file.
            /// </summary>
			GameSave,

            /// <summary>
            /// Players save file.
            /// </summary>
			PlayersSave
		}
	}
}
