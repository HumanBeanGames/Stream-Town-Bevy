using Character;
using GameEventSystem;
using Processors;
using System;
using Twitch.Utils;
using UnityEngine;
using Utils;
using Data.Containers;
using Buildings;
using Reflex.Attributes;

namespace Twitch.Commands
{
    /// <summary>
    /// Handles Twitch chat commands for the Ruler role.
    /// </summary>
	public class RulerCommands
	{
        private PlayerProcessor _playerProcessor;
        private RoleProcessor _roleProcessor;
        private TownResourceProcessor _townResourceProcessor;
        private GameEventProcessor _gameEventProcessor;
        private TradeProcessor _tradeProcessor;
        private Processors.TwitchChatProcessor _twitchChatProcessor;
        private GameMasterCommands _gameMasterCommands;

        public RulerCommands(PlayerProcessor playerProcessor, RoleProcessor roleProcessor,
			TownResourceProcessor townResourceProcessor,
            GameEventProcessor gameEventProcessor, TradeProcessor tradeProcessor,
            Processors.TwitchChatProcessor twitchChatProcessor, GameMasterCommands gameMasterCommands)
        {
            _playerProcessor = playerProcessor;
            _roleProcessor = roleProcessor;
            _townResourceProcessor = townResourceProcessor;
            _gameEventProcessor = gameEventProcessor;
            _tradeProcessor = tradeProcessor;
            _twitchChatProcessor = twitchChatProcessor;
            _gameMasterCommands = gameMasterCommands;
        }

        /// <summary>
        /// Gets the player processor.
        /// </summary>
		public PlayerProcessor PlayerProcessor => _playerProcessor;

        /// <summary>
        /// Gets the role processor.
        /// </summary>
		public RoleProcessor RoleProcessor => _roleProcessor;

		/// <summary>
		/// Used for allowing Rulers to sell resources for gold.
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="command">The command.</param>
		/// <param name="args">The arguments.</param>
		public void SellResource(Player player, string command, params string[] args)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;

			if (args.Length < 2)
				return;

			Resource resource = TwitchUtils.GetResourceFromString(args[1]);

			if (resource == Resource.Gold || resource == Resource.None)
				return;

			if (int.TryParse(args[0], out int amount))
			{
				if (amount <= 0)
					return;

				_tradeProcessor.SellResource(resource, amount, out string message);

				_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} : {message}");
			}
		}

		/// <summary>
		/// Used for allowing Rulers to Purchase Resources for gold.
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="command">The command.</param>
		/// <param name="args">The arguments.</param>
		public void BuyResource(Player player, string command, params string[] args)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;

			if (args.Length < 2)
				return;

			Resource resource = TwitchUtils.GetResourceFromString(args[1]);


			if (resource == Resource.Gold || resource == Resource.None)
				return;

			if (int.TryParse(args[0], out int amount))
			{
				_tradeProcessor.BuyResource(resource, amount, out string message);

				_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} : {message}");
			}
		}

        /// <summary>
        /// Recruits an NPC with the specified role.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments.</param>
		public void RecruitNPC(Player player, string command, params string[] args)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;

			if (args.Length < 1)
				return;
			string r = char.ToUpper(args[0][0]) + args[0].Substring(1);
			if (Enum.TryParse(r, out PlayerRole role))
			{
				if (role == PlayerRole.Ruler)
					return;

				int amount = 1;

				if (args.Length >= 2)
					int.TryParse(args[1], out amount);

				for (int i = 0; i < amount; i++)
				{
					if (_roleProcessor.SlotsFull(role) || _townResourceProcessor.ResourceFull(Resource.Recruit))
						break;

					Player recruit = new Player(new TwitchUser($"{UnityEngine.Random.Range(int.MinValue, 0)}", $""), true);
					_playerProcessor.AddNewPlayer(recruit, role);
					_townResourceProcessor.AddResource(Resource.Recruit, 1);
				}
			}
		}

        /// <summary>
        /// Resets the camera to default position.
        /// </summary>
        /// <param name="player">The player.</param>
		public void ResetCamera(Player player)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;

			_twitchChatProcessor.EnqueueCameraRequest(Vector3.zero, 0, true);
		}

        /// <summary>
        /// Moves the camera based on arguments.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments.</param>
		public void MoveCamera(Player player, string command, params string[] args)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;

			if (args.Length < 1)
				return;

			Vector3 moveVector = Vector3.zero;
			int zoomFactor = 0;

			for (int i = 0; i < args.Length; i++)
			{
				int value = 1;
				if ((i + 1) < args.Length && int.TryParse(args[i + 1], out int parsedValue))
				{
					value = parsedValue;
					i++;
				}

				switch (args[i])
				{
				case "up":
						moveVector += Vector3.forward * value;
						break;
					case "down":
						moveVector += Vector3.back * value;
						break;
					case "left":
						moveVector += Vector3.left * value;
						break;
					case "right":
						moveVector += Vector3.right * value;
						break;
					case "out":
						zoomFactor = 1 * value;
						break;
					case "in":
						zoomFactor = -1 * value;
						break;
				}
			}
			_twitchChatProcessor.EnqueueCameraRequest(moveVector, zoomFactor, false);
		}

        /// <summary>
        /// Displays the recruit count.
        /// </summary>
        /// <param name="player">The player.</param>
		public void RecruitCount(Player player)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;
			_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} The town has {_playerProcessor.RecruitCount()} recruits!");
		}

		// show recruit ids
        /// <summary>
        /// Shows the recruit IDs.
        /// </summary>
        /// <param name="player">The player.</param>
		public void ShowRecruitIds(Player player)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;
			_playerProcessor.ShowRecruitIDs();
		}

		// Dismiss recruit
        /// <summary>
        /// Dismisses a recruit.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments.</param>
		public void DismissRecruit(Player player, string command, params string[] args)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;
			if (args.Length == 0)
				_twitchChatProcessor.SendMessage($"!rdismiss <id>");
			if (int.TryParse(args[0], out int id))
			{
				Player recruit = _playerProcessor.GetRecruitByIndex(id);

				_playerProcessor.DismissRecruit(recruit);
				_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} Successfully Dismissed recruit {id}!");
			}
			else
				_twitchChatProcessor.SendMessage($"{args[0]} is not a valid id");
		}

		// swap recruit role
        /// <summary>
        /// Swaps a recruit's role.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments.</param>
		public void SwapRecruitRole(Player player, string command, params string[] args)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;
			if (args.Length == 0)
				_twitchChatProcessor.SendMessage("!rswap <id> <role>");
			if (args.Length > 0)
			{
				if (int.TryParse(args[0], out int id))
				{
					Player recruit = _playerProcessor.GetRecruitByIndex(id);
					string r = char.ToUpper(args[1][0]) + args[1].Substring(1);
					if (Enum.TryParse(r, out PlayerRole role))
					{
						if (_roleProcessor.IsRoleAvailable(role))
						{
							_playerProcessor.SwapRecruitRole(recruit, role);
							_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} Successfully changed recruit {id} to {role}!");
						}
						else
							_twitchChatProcessor.SendMessage($"{role} is full");
					}
					else
						_twitchChatProcessor.SendMessage($"{args[1]} is not a valid role");
				}
				else
					_twitchChatProcessor.SendMessage($"{args[0]} is not a valid id");
			}
		}

		// show recruit information (role, level) using ID
        /// <summary>
        /// Displays recruit information.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments.</param>
		public void DisplayRecruitInfo(Player player, string command, params string[] args)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;
			if (args.Length == 0)
				_twitchChatProcessor.SendMessage("!rinfo <id> ");
			if (args.Length > 0)
			{
				if (int.TryParse(args[0], out int id))
				{
					Player recruit = _playerProcessor.GetRecruitByIndex(id);

					string info = $"{player.TwitchUser.Username} ----- Recruit {id} | " +
						$"Current role {recruit.RoleHandler.CurrentRole} | " +
						$" Health: {recruit.HealthHandler.Health} / {recruit.HealthHandler.MaxHealth} | " +
						$" Level: {recruit.RoleHandler.PlayerRoleData.CurrentLevel} / {RoleProcessor.MAX_ROLE_LEVEL} | " +
						$" Experience: {recruit.RoleHandler.PlayerRoleData.CurrentExp} / {recruit.RoleHandler.PlayerRoleData.RequiredExp}";

					_twitchChatProcessor.SendMessage(info);
				}
				else
					_twitchChatProcessor.SendMessage($"{args[0]} is not a valid id");
			}
		}

        /// <summary>
        /// Resigns from the ruler role.
        /// </summary>
        /// <param name="player">The player.</param>
		public void Resign(Player player)
		{
			if (_playerProcessor.GetRuler() != player && !_gameMasterCommands.IsGameMaster(player))
				return;

			_gameEventProcessor.StartNewRulerVote();
			player.RoleHandler.TrySetRole(player.RoleHandler.PreviousRole);
			_playerProcessor.SetRuler(null);
			_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} you have been succesfully resigned!");
		}
	}
}
