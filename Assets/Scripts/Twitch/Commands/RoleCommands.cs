using Character;
using Processors;
using System;
using Utils;
using Data.Containers;
using Reflex.Attributes;

namespace Twitch.Commands
{
	/// <summary>
	/// Handles all Twitch chat commands related to Player Roles.
	/// </summary>
	public class RoleCommands
	{
        private PlayerProcessor _playerProcessor;
        private StationProcessor _stationProcessor;
        private RoleProcessor _roleProcessor;
        private Processors.TwitchChatProcessor _twitchChatProcessor;

        public RoleCommands(PlayerProcessor playerProcessor, StationProcessor stationProcessor,
            RoleProcessor roleProcessor, Processors.TwitchChatProcessor twitchChatProcessor)
        {
            _playerProcessor = playerProcessor;
            _stationProcessor = stationProcessor;
            _roleProcessor = roleProcessor;
            _twitchChatProcessor = twitchChatProcessor;
        }

		/// <summary>
		/// Attempts to change the role of the User.
		/// </summary>
		/// <param name="player"></param>
		/// <param name="command"></param>
		/// <param name="args"></param>
		public void TryChangeRole(Player player, string command, params string[] args)
		{
			// Convert first letter to Uppercase to work with enum parse
			string r = char.ToUpper(args[0][0]) + args[0].Substring(1);

			if (Enum.TryParse(r, out PlayerRole role))
			{
				if (role == PlayerRole.Ruler && _playerProcessor.GetRuler() != player)
					return;

				if (player.RoleHandler.TrySetRole(role))
				{
					_twitchChatProcessor.SendPreBuiltMessage(player.TwitchUser.Username, "roleSwitched");
				}
			}
		}

		/// <summary>
		/// Sends a message detailing the User's role and level.
		/// </summary>
		/// <param name="player"></param>
		public void Role(Player player)
		{
			string message = $"{player.TwitchUser.Username} you are currently a level {player.RoleHandler.PlayerRoleData.CurrentLevel} {player.RoleHandler.CurrentRole}";
			_twitchChatProcessor.SendMessage(message);
		}

		/// <summary>
		/// Sends a message detailing the User's health.
		/// </summary>
		/// <param name="player"></param>
		public void Health(Player player)
		{
			string message = $"{player.TwitchUser.Username} your health is: ({player.HealthHandler.Health}/{player.HealthHandler.MaxHealth})";
			_twitchChatProcessor.SendMessage(message);
		}

		/// <summary>
		/// Sends a message detailing the User's Role, level and experience.
		/// </summary>
		/// <param name="player"></param>
		public void Experience(Player player)
		{
			string message = $"{player.TwitchUser.Username} you are a level ({player.RoleHandler.PlayerRoleData.CurrentLevel}/{RoleProcessor.MAX_ROLE_LEVEL}) {player.RoleHandler.CurrentRole}. Current Exp: ({player.RoleHandler.PlayerRoleData.CurrentExp}/{player.RoleHandler.PlayerRoleData.RequiredExp}).";
			_twitchChatProcessor.SendMessage(message);

		}

		/// <summary>
		/// Sends a message detailing the User's Role, level and experience.
		/// </summary>
		/// <param name="player"></param>
		public void ExperienceForRole(Player player, PlayerRole role)
		{
			if (player.RoleHandler.TryGetRoleData(role, out PlayerRoleData data))
			{
				string message = $"{player.TwitchUser.Username} you are a level ({data.CurrentLevel}/{RoleProcessor.MAX_ROLE_LEVEL}) {data.Role}. Current Exp: ({data.CurrentExp}/{data.RequiredExp}).";
				_twitchChatProcessor.SendMessage(message);
			}
			else
				_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} you currenty don't have data for {role}");
		}

		/// <summary>
		/// Displays the Station IDs based on Player's Station Flags.
		/// </summary>
		/// <param name="player"></param>
		public void DisplayStationIDs(Player player)
		{
			_stationProcessor.DisplayStationIdByType(player.StationSensor.StationMask);
		}

		/// <summary>
		/// Attempts to change the User's station.
		/// </summary>
		/// <param name="player"></param>
		/// <param name="command"></param>
		/// <param name="args"></param>
		public void SwitchStation(Player player, string command, params string[] args)
		{

			if (int.TryParse(args[0], out int index))
			{
				var station = _stationProcessor.GetStationByFlaggedIndex(player.StationSensor.StationMask, index - 1);
				player.StationSensor.UpdateStation = false;
				player.StationSensor.TrySetStation(station);
				_twitchChatProcessor.SendPlayerMessage(player, "Station Switched!");
			}
		}

		/// <summary>
		/// Attempts to display the ID's of the targets currently stored in their station.
		/// </summary>
		/// <param name="player"></param>
		public void DisplayTargetIDs(Player player)
		{
			player.StationSensor.CurrentStation.DisplayTargetIDsByMask(player.RoleHandler.RoleData_SO.TargetFlags);
		}

		/// <summary>
		/// Attempts to switch the User's current Target.
		/// </summary>
		/// <param name="player"></param>
		/// <param name="command"></param>
		/// <param name="args"></param>
		public void SwitchTarget(Player player, string command, params string[] args)
		{
			if (int.TryParse(args[0], out int index))
			{
				var targetable = player.StationSensor.CurrentStation.GetTargetByFlaggedIndex(player.RoleHandler.RoleData_SO.TargetFlags, index - 1);

				if (targetable)
				{
					if (player.TargetSensor.TrySetTarget(targetable))
						_twitchChatProcessor.SendPlayerMessage(player, "Target Switched!");
				}
			}
		}
	}
}
