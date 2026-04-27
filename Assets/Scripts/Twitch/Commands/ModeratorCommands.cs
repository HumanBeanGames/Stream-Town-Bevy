using Character;
using GameEventSystem;
using Processors;
using UnityEngine;
using Data.Containers;
using Reflex.Attributes;
using Twitch.Utils;

namespace Twitch.Commands
{
	/// <summary>
	/// Handles all Twitch chat commands related to Moderation.
	/// </summary>
	public class ModeratorCommands
	{
        private PlayerProcessor _playerProcessor;
        private GameEventProcessor _gameEventProcessor;
        private RoleCommands _roleCommands;

        public ModeratorCommands(PlayerProcessor playerProcessor, GameEventProcessor gameEventProcessor,
            RoleCommands roleCommands)
        {
            _playerProcessor = playerProcessor;
            _gameEventProcessor = gameEventProcessor;
            _roleCommands = roleCommands;
        }

	        /// <summary>
        /// Starts a vote for the ruler position.
        /// </summary>
        /// <param name="player">The player.</param>
		public void StartKingVote(Player player)
		{
			if (!IsModerator(player))
				return;

			if (_playerProcessor.GetRuler() == null)
				_gameEventProcessor.StartNewRulerVote();
			else
				_gameEventProcessor.StartKeepRulerVote();
		}

	        /// <summary>
        /// Changes a target player's role.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the target player name and new role.</param>
		public void ChangePlayerRole(Player player, string command, params string[] args)
		{
			if (!IsModerator(player))
				return;

			if (args.Length < 2)
				return;

			string playerNameArg = args[0].ToLower();

			if (_playerProcessor.PlayerExistsByNameToLower(playerNameArg, out int index))
			{
				Player targetPlayer = _playerProcessor.GetPlayer(index);
				string[] newArgs = new string[] { args[1] };

				_roleCommands.TryChangeRole(targetPlayer, command, newArgs);
			}
		}

	        /// <summary>
        /// Checks if a player has moderator privileges.
        /// </summary>
        /// <param name="player">The player to check.</param>
        /// <returns>True if the player is a moderator, broadcaster, or game master.</returns>
		public static bool IsModerator(Player player)
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
