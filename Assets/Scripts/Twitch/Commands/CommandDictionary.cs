using Character;
using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using Reflex.Attributes;
using Utils;

namespace Twitch.Commands
{
	/// <summary>
	/// Contains all commands used in Twitch chat.
	/// </summary>
	public class CommandDictionary
	{
        private PlayerCommands _playerCommands;
        private RoleCommands _roleCommands;
        private BuildingCommands _buildingCommands;
        private MiscCommands _miscCommands;
        private GameMasterCommands _gameMasterCommands;
        private ModeratorCommands _moderatorCommands;
        private RulerCommands _rulerCommands;

        public CommandDictionary(PlayerCommands playerCommands, RoleCommands roleCommands,
            BuildingCommands buildingCommands, MiscCommands miscCommands,
            GameMasterCommands gameMasterCommands, ModeratorCommands moderatorCommands,
            RulerCommands rulerCommands)
        {
            _playerCommands = playerCommands;
            _roleCommands = roleCommands;
            _buildingCommands = buildingCommands;
            _miscCommands = miscCommands;
            _gameMasterCommands = gameMasterCommands;
            _moderatorCommands = moderatorCommands;
            _rulerCommands = rulerCommands;
        }

		private readonly Dictionary<string, string> _commandUsages = new Dictionary<string, string>
		{
			{ "role", "!role <role> (or !role to view your current role)" },
			{ "build", "!build <building>" },
			{ "move", "!move <up|down|left|right|rotate> [amount]" },
			{ "up", "!up [amount]" }, { "down", "!down [amount]" },
			{ "left", "!left [amount]" }, { "right", "!right [amount]" },
			{ "rotate", "!rotate [amount]" },
			{ "level", "!level <role> OR !level <building> <id> [amount]" },
			{ "remove", "!remove <building> <id>" }, { "bid", "!bid <building>" },
			{ "station", "!station <id> (or !station to list IDs)" },
			{ "target", "!target <id> (or !target to list IDs)" },
			{ "hair", "!hair <index>" }, { "facialhair", "!facialhair <index>" },
			{ "eyes", "!eyes <index>" }, { "body", "!body <index>" },
			{ "haircolor", "!haircolor <index>" }, { "eyecolor", "!eyecolor <index>" },
			{ "addresource", "!addresource <resource> <amount>" },
			{ "vote", "!vote <option number>" }, { "modrole", "!modrole <player> <role>" },
			{ "kill", "!kill <player>" }, { "grevive", "!grevive <player>" },
			{ "revive", "!revive [player]" }, { "givexp", "!givexp <player> <amount>" },
			{ "givexpall", "!givexpall <amount>" }, { "levelup", "!levelup <player> [amount]" },
			{ "qevent", "!qevent <event>" },
			{ "buy", "!buy <amount> <resource>" }, { "sell", "!sell <amount> <resource>" },
			{ "levelall", "!levelall <building> <level>" },
			{ "recruit", "!recruit <role> [amount]" },
			{ "givepet", "!givepet <player> <pet>" },
			{ "pet", "!pet <pet> (or !pet to list pets)" },
			{ "cam", "!cam <up|down|left|right|in|out> [amount]" },
			{ "info", "!info <resource|role|building|enemy> [id]" },
			{ "rrole", "!rrole <id> <role>" }, { "rinfo", "!rinfo <id>" },
			{ "rdismiss", "!rdismiss <id>" }, { "resetid", "!resetid <id>" },
			{ "roles", "!roles" }, { "help", "!help" },
			{ "stdiscord", "!stdiscord" }, { "townstats", "!townstats" }
		};

		/// <summary>
		/// Validates command shape before dispatch so individual handlers cannot
		/// silently swallow malformed input. Runtime/business failures remain the
		/// responsibility of the owning command handler.
		/// </summary>
		public bool TryValidateArguments(string command, IReadOnlyList<string> args, out string usage)
		{
			usage = _commandUsages.TryGetValue(command, out string configuredUsage)
				? configuredUsage
				: $"!{command}";
			args ??= Array.Empty<string>();

			if ((SimpleCommands.ContainsKey(command) || CommandsNoArgs.ContainsKey(command)) &&
				!CommandsWithArgs.ContainsKey(command))
				return args.Count == 0;

			switch (command)
			{
				case "role":
					return args.Count == 0 || (args.Count == 1 && IsEnumValue(args[0], PlayerRole.Count));
				case "build":
				case "bid":
					return args.Count == 1 && IsEnumValue(args[0], BuildingType.Count);
				case "remove":
				case "levelall":
					return args.Count == 2 && IsEnumValue(args[0], BuildingType.Count) && IsPositiveInteger(args[1]);
				case "move":
					return ValidateDirectionSequence(args, false);
				case "cam":
					return ValidateDirectionSequence(args, true);
				case "up": case "down": case "left": case "right": case "rotate":
					return args.Count == 0 || (args.Count == 1 && IsInteger(args[0]));
				case "level":
					if (args.Count == 0)
						return true;
					if (args.Count == 1)
						return IsEnumValue(args[0], PlayerRole.Count);
					return args.Count <= 3 && IsEnumValue(args[0], BuildingType.Count) &&
						IsPositiveInteger(args[1]) && (args.Count < 3 || IsPositiveInteger(args[2]));
				case "station":
				case "target":
					return args.Count == 0 || (args.Count == 1 && IsPositiveInteger(args[0]));
				case "hair": case "facialhair": case "eyes": case "body":
				case "haircolor": case "eyecolor": case "vote": case "givexpall":
				case "rinfo": case "rdismiss": case "resetid":
					return args.Count == 1 && IsPositiveInteger(args[0]);
				case "addresource":
					return args.Count == 2 && IsEnumValue(args[0], Resource.Count) &&
						!args[0].Equals(Resource.None.ToString(), StringComparison.OrdinalIgnoreCase) && IsInteger(args[1]);
				case "modrole":
					return args.Count == 2 && !string.IsNullOrWhiteSpace(args[0]) && IsEnumValue(args[1], PlayerRole.Count);
				case "kill": case "grevive": case "qevent": case "info":
					return args.Count >= 1 && !string.IsNullOrWhiteSpace(args[0]);
				case "revive":
				case "pet":
					return args.Count <= 1 && (args.Count == 0 || !string.IsNullOrWhiteSpace(args[0]));
				case "givexp":
					return args.Count == 2 && !string.IsNullOrWhiteSpace(args[0]) && IsPositiveInteger(args[1]);
				case "levelup":
					return (args.Count == 1 || args.Count == 2) && !string.IsNullOrWhiteSpace(args[0]) &&
						(args.Count == 1 || IsPositiveInteger(args[1]));
				case "buy":
				case "sell":
					return args.Count == 2 && IsPositiveInteger(args[0]) && IsEnumValue(args[1], Resource.Count) &&
						!args[1].Equals(Resource.None.ToString(), StringComparison.OrdinalIgnoreCase) &&
						!args[1].Equals(Resource.Gold.ToString(), StringComparison.OrdinalIgnoreCase);
				case "recruit":
					return (args.Count == 1 || args.Count == 2) && IsEnumValue(args[0], PlayerRole.Count) &&
						!args[0].Equals(PlayerRole.Ruler.ToString(), StringComparison.OrdinalIgnoreCase) &&
						(args.Count == 1 || IsPositiveInteger(args[1]));
				case "givepet":
					return args.Count == 2 && !string.IsNullOrWhiteSpace(args[0]) && !string.IsNullOrWhiteSpace(args[1]);
				case "rrole":
					return args.Count == 2 && IsPositiveInteger(args[0]) && IsEnumValue(args[1], PlayerRole.Count);
				default:
					return !CommandsWithArgs.ContainsKey(command) || args.Count > 0;
			}
		}

		private static bool ValidateDirectionSequence(IReadOnlyList<string> args, bool cameraDirections)
		{
			if (args == null || args.Count == 0)
				return false;

			int index = 0;
			while (index < args.Count)
			{
				string direction = args[index];
				bool validDirection = direction == "up" || direction == "down" || direction == "left" ||
					direction == "right" || (cameraDirections ? direction == "in" || direction == "out" : direction == "rotate");
				if (!validDirection)
					return false;

				index++;
				if (index < args.Count && IsInteger(args[index]))
					index++;
			}

			return true;
		}

		private static bool IsInteger(string value) => int.TryParse(value, out _);
		private static bool IsPositiveInteger(string value) => int.TryParse(value, out int parsed) && parsed > 0;

		private static bool IsEnumValue<T>(string value, T countValue) where T : struct, Enum
		{
			return Enum.TryParse(value, true, out T parsed) && !EqualityComparer<T>.Default.Equals(parsed, countValue);
		}

		/// <summary>
		/// The dictionary of commands with arguments.
		/// </summary>
		private Dictionary<string, Action<Player, string, string[]>> _commandsWithArgs;

		/// <summary>
		/// Contains all Game Commands that require further arguments.
		/// </summary>
		public Dictionary<string, Action<Player, string, string[]>> CommandsWithArgs
		{
			get
			{
				if (_commandsWithArgs == null)
				{
					_commandsWithArgs = new Dictionary<string, Action<Player, string, string[]>>();
					_commandsWithArgs.Add("role", _roleCommands.TryChangeRole);
					_commandsWithArgs.Add("build", _buildingCommands.StartBuild);
					_commandsWithArgs.Add("move", _buildingCommands.AdjustBuildingPlacer);
					_commandsWithArgs.Add("up", _buildingCommands.AdjustBuildingPlacer);
					_commandsWithArgs.Add("down", _buildingCommands.AdjustBuildingPlacer);
					_commandsWithArgs.Add("left", _buildingCommands.AdjustBuildingPlacer);
					_commandsWithArgs.Add("right", _buildingCommands.AdjustBuildingPlacer);
					_commandsWithArgs.Add("rotate", _buildingCommands.AdjustBuildingPlacer);
					_commandsWithArgs.Add("level", _miscCommands.Level);
					_commandsWithArgs.Add("remove", _buildingCommands.RemoveBuilding);
					_commandsWithArgs.Add("bid", _buildingCommands.ShowBuildingIDsByType);
					_commandsWithArgs.Add("station", _roleCommands.SwitchStation);
					_commandsWithArgs.Add("target", _roleCommands.SwitchTarget);
					_commandsWithArgs.Add("hair", _playerCommands.ChangeHairStyle);
					_commandsWithArgs.Add("facialhair", _playerCommands.ChangeFacialHair);
					_commandsWithArgs.Add("eyes", _playerCommands.ChangeEyes);
					_commandsWithArgs.Add("body", _playerCommands.ChangeBodyType);
					_commandsWithArgs.Add("haircolor", _playerCommands.ChangeHairColor);
					_commandsWithArgs.Add("eyecolor", _playerCommands.ChangeEyeColor);
					_commandsWithArgs.Add("addresource", _gameMasterCommands.AddResources);
					_commandsWithArgs.Add("vote", _playerCommands.Vote);
					_commandsWithArgs.Add("modrole", _moderatorCommands.ChangePlayerRole);
					_commandsWithArgs.Add("kill", _gameMasterCommands.KillPlayer);
					_commandsWithArgs.Add("grevive", _gameMasterCommands.RevivePlayer);
					_commandsWithArgs.Add("revive", _playerCommands.RevivePlayerWithCost);
					_commandsWithArgs.Add("givexp", _gameMasterCommands.GivePlayerExp);
					_commandsWithArgs.Add("givexpall", _gameMasterCommands.GiveAllExp);
					_commandsWithArgs.Add("levelup", _gameMasterCommands.LevelUpPlayer);
					_commandsWithArgs.Add("qevent", _gameMasterCommands.QueueEvent);
					_commandsWithArgs.Add("buy", _rulerCommands.BuyResource);
					_commandsWithArgs.Add("sell", _rulerCommands.SellResource);
					_commandsWithArgs.Add("levelall", _buildingCommands.LevelAllOfType);
					//_commandsWithArgs.Add("bi", _buildingCommands.GetBuildingInformation);
					_commandsWithArgs.Add("recruit", _rulerCommands.RecruitNPC);
					_commandsWithArgs.Add("givepet", _gameMasterCommands.GivePlayerPet);
					_commandsWithArgs.Add("pet", _playerCommands.SwitchPet);
					_commandsWithArgs.Add("cam", _rulerCommands.MoveCamera);
					_commandsWithArgs.Add("info", _miscCommands.ItemInfo);
					_commandsWithArgs.Add("rrole", _rulerCommands.SwapRecruitRole);
					_commandsWithArgs.Add("rinfo", _rulerCommands.DisplayRecruitInfo);
					_commandsWithArgs.Add("rdismiss", _rulerCommands.DismissRecruit);
					_commandsWithArgs.Add("resetid", _gameMasterCommands.ResetID);
				}
				return _commandsWithArgs;
			}
		}

        /// <summary>
        /// The dictionary of commands without arguments.
        /// </summary>
		private Dictionary<string, Action<Player>> _commandsNoArgs;

		/// <summary>
		/// Contains all Game Commands that do NOT require arguments.
		/// </summary>
		public Dictionary<string, Action<Player>> CommandsNoArgs
		{
			get
			{
				if (_commandsNoArgs == null)
				{
					_commandsNoArgs = new Dictionary<string, Action<Player>>();
					_commandsNoArgs.Add("role", _roleCommands.Experience);
					_commandsNoArgs.Add("level", _roleCommands.Experience);
					_commandsNoArgs.Add("health", _roleCommands.Health);
					_commandsNoArgs.Add("revive", _playerCommands.ReviveWithCost);
					_commandsNoArgs.Add("confirm", _buildingCommands.ConfirmBuildingPlacement);
					_commandsNoArgs.Add("accept", _buildingCommands.ConfirmBuildingPlacement);
					_commandsNoArgs.Add("cancel", _buildingCommands.CancelBuildingPlacement);
					_commandsNoArgs.Add("station", _roleCommands.DisplayStationIDs);
					_commandsNoArgs.Add("target", _roleCommands.DisplayTargetIDs);
					_commandsNoArgs.Add("tbuildcosts", _gameMasterCommands.ToggleBuildCosts);
					_commandsNoArgs.Add("trolelimits", _gameMasterCommands.TogglePlayerRoleLimits);
					_commandsNoArgs.Add("ping", _playerCommands.PingPlayer);
					_commandsNoArgs.Add("rulervote", _moderatorCommands.StartKingVote);
					_commandsNoArgs.Add("stopevent", _gameMasterCommands.StopCurrentEvent);
					_commandsNoArgs.Add("cobj", _gameMasterCommands.CompleteCurrentGoal);
					_commandsNoArgs.Add("randtech", _gameMasterCommands.StartRandomTech);
					_commandsNoArgs.Add("techvote", _gameMasterCommands.StartVoteTech);
					_commandsNoArgs.Add("pet", _playerCommands.PrintPetsList);
					_commandsNoArgs.Add("pets", _playerCommands.PrintPetsList);
					_commandsNoArgs.Add("gaction", _gameMasterCommands.ActionEvent);
					_commandsNoArgs.Add("unlockall", _gameMasterCommands.UnlockAllTech);
					_commandsNoArgs.Add("unlockage2", _gameMasterCommands.UnlockToAge2);
					_commandsNoArgs.Add("resetcam", _rulerCommands.ResetCamera);
					_commandsNoArgs.Add("stuck", _playerCommands.Unstuck);
					_commandsNoArgs.Add("praise", _playerCommands.Praise);
					_commandsNoArgs.Add("buildings", _buildingCommands.PrintUnlockedBuildings);
					_commandsNoArgs.Add("rid", _rulerCommands.ShowRecruitIds);
					_commandsNoArgs.Add("recruits", _rulerCommands.RecruitCount);
					_commandsNoArgs.Add("resign", _rulerCommands.Resign);
				}
				return _commandsNoArgs;
			}
		}

        /// <summary>
        /// The dictionary of simple commands.
        /// </summary>
		private Dictionary<string, Action> _simpleCommands;

		/// <summary>
		/// Contains all Game Commands that do not require a created character or arguments.
		/// </summary>
		public Dictionary<string, Action> SimpleCommands
		{
			get
			{
				if (_simpleCommands == null)
				{
					_simpleCommands = new Dictionary<string, Action>();
					_simpleCommands.Add("stdiscord", _miscCommands.Discord);
					_simpleCommands.Add("help", _miscCommands.Help);
					_simpleCommands.Add("roles", _miscCommands.Roles);
					_simpleCommands.Add("townstats", _miscCommands.TownStats);
				}
				return _simpleCommands;
			}
		}

		/// <summary>
		/// A list of acceptable variants for the word "create".
		/// </summary>
		public static List<string> CreateNameVariants = new List<string>
		{
			"create","crate","crete","join","start","creta","ceate","cate","crtea", "ligma"
		};

		/// <summary>
		/// Allows commands to have multiple variants.
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="args">The arguments.</param>
		/// <param name="command">The command.</param>
		/// <param name="aliases">The aliases.</param>
		private void AliasCommand(Player player, string[] args, Action<Player, string[]> command, params string[] aliases)
		{
			var newArgs = args.ToList();
			for (int i = 0; i < aliases.Length; i++)
			{
				newArgs.Insert(i, aliases[i]);
			}

			command(player, newArgs.ToArray());
		}
	}
}
