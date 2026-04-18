using Character;
using GameEventSystem;
using Processors;
using Pets;
using Pets.Enumerations;
using UnityEngine;
using Utils;

namespace Twitch.Utils
{
	/// <summary>
	/// Holds all Color data for different types of Twitch users.
	/// </summary>
	public static class UserColours
	{
        /// <summary>
        /// The game master color.
        /// </summary>
		public static Color GameMaster = new Color(255, 57, 0, 255);

        /// <summary>
        /// The broadcaster color.
        /// </summary>
		public static Color Broadcaster = Color.red;

        /// <summary>
        /// The moderator color.
        /// </summary>
		public static Color Moderator = Color.green;

        /// <summary>
        /// The subscriber color.
        /// </summary>
		public static Color Subscriber = new Color(100, 65, 165, 255);

        /// <summary>
        /// The normal user color.
        /// </summary>
		public static Color Normal = Color.white;

		/// <summary>
		/// Returns a color based on the User's type in Twitch chat.
		/// </summary>
		/// <param name="type">The game user type.</param>
		/// <returns>The color.</returns>
		public static Color GetColourByUserType(GameUserType type)
		{
			switch (type)
			{
				case GameUserType.GameMaster:
					return GameMaster;
				case GameUserType.Broadcaster:
					return Broadcaster;
				case GameUserType.Moderator:
					return Moderator;
				case GameUserType.Subscriber:
					return Subscriber;
				case GameUserType.Normal:
					return Normal;
				default:
					return Normal;
			}
		}
	}

    /// <summary>
    /// Utility class for Twitch-related operations.
    /// </summary>
	public static class TwitchUtils
	{
        /// <summary>
        /// The player processor.
        /// </summary>
		private static PlayerProcessor _playerProcessor;

        /// <summary>
        /// Initializes the Twitch utilities.
        /// </summary>
        /// <param name="playerProcessor">The player processor.</param>
		public static void Initialize(PlayerProcessor playerProcessor)
		{
			_playerProcessor = playerProcessor;
		}

        /// <summary>
        /// Tries to get a player by name.
        /// </summary>
        /// <param name="nameArg">The player name argument.</param>
        /// <param name="player">The player.</param>
        /// <returns>True if the player was found, false otherwise.</returns>
		public static bool TryGetPlayer(string nameArg, out Player player)
		{
			player = null;
			if (_playerProcessor.PlayerExistsByNameToLower(nameArg.ToLower(), out int index))
			{
				player = _playerProcessor.GetPlayer(index);
				return true;
			}

			return false;
		}

        /// <summary>
        /// Gets the pet type from a string.
        /// </summary>
        /// <param name="arg">The string argument.</param>
        /// <returns>The pet type.</returns>
		public static PetType GetPetTypeFromString(string arg)
		{
			arg = arg.ToLower();
			for(int i = 0; i < (int)PetType.Count;i++)
			{
				if (arg == ((PetType)i).ToString().ToLower())
					return (PetType)i;
			}

			return PetType.Count;
		}

        /// <summary>
        /// Converts a string to enum string format.
        /// </summary>
        /// <param name="arg">The string argument.</param>
        /// <returns>The enum string.</returns>
		public static string StringToEnumString(string arg)
		{
			return char.ToUpper(arg[0]) + arg.Substring(1);
		}

        /// <summary>
        /// Converts a string to game event enum.
        /// </summary>
        /// <param name="arg">The string argument.</param>
        /// <returns>The game event type.</returns>
		public static GameEvent.EventType StringToEventEnum(string arg)
		{
			for(int i = 0; i < (int) GameEvent.EventType.Count;i++)
			{
				if(arg == ((GameEvent.EventType)i).ToString().ToLower())
				{
					return (GameEvent.EventType)i;
				}
			}

			return GameEvent.EventType.Count;
		}

        /// <summary>
        /// Gets the resource type from a string.
        /// </summary>
        /// <param name="arg">The string argument.</param>
        /// <returns>The resource type.</returns>
		public static Resource GetResourceFromString(string arg)
		{
			arg = arg.ToLower();

			for (int i = 1; i < (int)Resource.Count; i++)
			{
				if (arg == ((Resource)i).ToString().ToLower())
					return (Resource)i;
			}

			return Resource.None;
		}
	}

	/// <summary>
	/// Dictates what type of user this player is in Twitch chat.
	/// </summary>
	public enum GameUserType
	{
        /// <summary>
        /// Game master user type.
        /// </summary>
		GameMaster,

        /// <summary>
        /// Broadcaster user type.
        /// </summary>
		Broadcaster,

        /// <summary>
        /// Moderator user type.
        /// </summary>
		Moderator,

        /// <summary>
        /// Subscriber user type.
        /// </summary>
		Subscriber,

        /// <summary>
        /// Normal user type.
        /// </summary>
		Normal
	}
}
