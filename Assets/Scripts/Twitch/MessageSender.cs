using Character;
using System.Collections.Generic;
using Processors;

namespace Twitch
{
	/// <summary>
	/// Handles sending messages from the bot to Twitch chat.
	/// </summary>
	public class MessageSender
	{
		private readonly TwitchClientProcessor _twitchClientProcessor;

		/// <summary>
		/// The runtime data for Twitch chat.
		/// </summary>
		private readonly TwitchChatRuntimeData _twitchChatRuntimeData;

		public MessageSender(TwitchClientProcessor twitchClientProcessor, TwitchChatRuntimeData runtimeData)
		{
			_twitchClientProcessor = twitchClientProcessor;
			_twitchChatRuntimeData = runtimeData;
		}

		/// <summary>
		/// A static Dictionary of all prebuilt command responses.
		/// </summary>
		public static readonly Dictionary<string, string> CommandResponses = new Dictionary<string, string>
		{
			{ "help", " type !create to start your character, then you can choose a role. type !roles to learn more" },
			{ "characterCreated", "Welcome to the game, your character was successfully created!"},
			{ "characterFailed", "Character already registered into the game!"},
			{ "buildingFailedCost",  "Not enough resources to build that!"},
			{ "buildingLevelFailedCost", "Not enough resources to upgrade that building!" },
			{ "buildingLevelFailedMaxLevel" , "That building is already at max level!" },
			{ "buildingLevelFailed", " Building level failed!" },
			{ "buildingLevelSuccess" , "Succesfully upgraded building!"},
			{ "buildingMultiLevelSuccess" , "Succesfully upgraded buildings!"},
			{ "buildingCancel" , "Building was canceled!" },
			{ "buildingSuccessful", "Building was placed successfully!" },
			{ "buildingRemoved", " Building was removed!" },
			{ "buildingFailedCollision",  "Building can't be placed there!"},
			{ "playerUnstuck", "You were unstuck!" },
			{ "playerRevived", " You have revived, welcome back to life!" },
			{ "roleSwitched" , " Role switched successfully!" },
			{ "noCharacter"," You need to create a character first!" },
			{ "discord","Stream Town Discord: https://discord.gg/By4jvks"}

		};

		/// <summary>
		/// Uses a key to send a prebuilt message to Twitch chat with the player's name.
		/// </summary>
		/// <param name="playerName">The player name.</param>
		/// <param name="key">The message key.</param>
		public void SendPreBuiltMessage(string playerName, string key)
		{
			if (!CommandResponses.ContainsKey(key))
				return;

			string message = $"{playerName} {CommandResponses[key]}";
			SendMessage(message);
		}

		/// <summary>
		/// Uses a key to send a prebuilt message to Twitch chat.
		/// </summary>
		/// <param name="key">The message key.</param>
		public void SendPreBuiltMessage(string key)
		{
			if (!CommandResponses.ContainsKey(key))
				return;

			string message = $"{CommandResponses[key]}";
			SendMessage(message);
		}

		/// <summary>
		/// Sends a custom message to Twitch chat with the Player's name.
		/// </summary>
		/// <param name="playerName">The player name.</param>
		/// <param name="message">The message.</param>
		public void SendMessage(string playerName, string message)
		{
			_twitchClientProcessor.Client.SendMessage(_twitchClientProcessor.Client.JoinedChannels[0], $"{playerName}: {message}");
		}

		/// <summary>
		/// Sends a custom message to Twitch chat.
		/// </summary>
		/// <param name="message">The message.</param>
		public void SendMessage(string message)
		{
			// Always log to Unity Console
			UnityEngine.Debug.Log(message);

			if (_twitchChatRuntimeData == null || !_twitchChatRuntimeData.MessagesAllowed)
				return;

			if (_twitchClientProcessor.Client != null && _twitchClientProcessor.Client.IsConnected)
				_twitchClientProcessor.Client.SendMessage(_twitchClientProcessor.Client.JoinedChannels[0], message);
		}

        /// <summary>
        /// Sends a message from a player.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="message">The message.</param>
		public void SendPlayerMessage(Player player, string message)
		{
			SendMessage($"{player.TwitchUser.Username}: {message}");
		}
	}
}
