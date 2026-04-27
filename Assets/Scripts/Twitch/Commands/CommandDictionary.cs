using Character;
using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using Reflex.Attributes;

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
