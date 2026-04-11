using Character;
using GameEventSystem;
using Managers;
using UnityEngine;

namespace Twitch.Commands
{
	/// <summary>
	/// Handles all Twitch chat commands related to Moderation.
	/// </summary>
	public static class ModeratorCommands
	{
		private static PlayerManager _playerManager;
		private static GameEventManager _gameEventManager;

		public static void Initialize(PlayerManager playerManager, GameEventManager gameEventManager)
		{
			_playerManager = playerManager;
			_gameEventManager = gameEventManager;
		}

		public static void StartKingVote(Player player)
		{
			if (!player.IsModerator())
				return;

			if (_playerManager.Ruler == null)
				_gameEventManager.StartNewRulerVote();
			else
				_gameEventManager.StartKeepRulerVote();
		}

		public static void ChangePlayerRole(Player player, string command, params string[] args)
		{
			if (!player.IsModerator())
				return;

			if (args.Length < 2)
				return;

			string playerNameArg = args[0].ToLower();

			if (_playerManager.PlayerExistsByNameToLower(playerNameArg, out int index))
			{
				Player targetPlayer = _playerManager.GetPlayer(index);
				string[] newArgs = new string[] { args[1] };

				RoleCommands.TryChangeRole(targetPlayer, command, newArgs);
			}
		}

		public static bool IsModerator(this Player player)
		{
			if (player.TwitchUser.TwitchUserType == TwitchLib.Client.Enums.UserType.Moderator)
				return true;
			if (player.TwitchUser.TwitchUserType == TwitchLib.Client.Enums.UserType.Broadcaster)
				return true;
			if (player.TwitchUser.GameUserType == Utils.GameUserType.GameMaster)
				return true;

			return false;
		}
	}
}
