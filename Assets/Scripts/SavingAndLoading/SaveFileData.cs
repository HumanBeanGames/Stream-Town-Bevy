using System;
using SavingAndLoading.Structs;

namespace SavingAndLoading
{
	/// <summary>
	/// Raw, versioned root object written to disk. It deliberately contains no
	/// Unity objects or runtime services.
	/// </summary>
	[Serializable]
	public sealed class SaveFileData
	{
		public const int CurrentSchemaVersion = 3;

		public int SchemaVersion;
		public string SavedAtUtc;
		public SaveGameData Game;
		public SavePlayersData Players;

		public SaveFileData()
		{
		}

		public SaveFileData(SaveGameData game, SavePlayersData players)
		{
			SchemaVersion = CurrentSchemaVersion;
			SavedAtUtc = DateTime.UtcNow.ToString("O");
			Game = game;
			Players = players;
		}
	}
}
