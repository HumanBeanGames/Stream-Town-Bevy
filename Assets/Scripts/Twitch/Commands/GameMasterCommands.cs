using Character;
using Enemies;
using GameEventSystem;
using GameEventSystem.Events;
using Processors;
using Core;
using Pets.Enumerations;
using System;
using System.Linq;
using Twitch.Utils;
using Utils;
using Reflex.Attributes;
using TechTree;
using TownGoal;
using ScriptablesProcessorInfrastructure;

namespace Twitch.Commands
{
	/// <summary>
	/// Handles all Twitch Chat messages relating to Game Management.
	/// </summary>
	public class GameMasterCommands
	{
	        /// <summary>
        /// The game coordinator. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private Coordinator _gameProcessor;
	        /// <summary>
        /// The role runtime scriptable. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private RoleProcessor _roleProcessor;
	        /// <summary>
        /// The UI processor. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private UIProcessor _uiProcessor;
	        /// <summary>
        /// The enemy spawner. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private EnemySpawner _enemySpawner;
	        /// <summary>
        /// The game settings scriptable. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private GameSettings _gameSettings;
	        /// <summary>
        /// The town resource processor. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private TownResourceProcessor _townResourceProcessor;
	        /// <summary>
        /// The player processor. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private PlayerProcessor _playerProcessor;
	        /// <summary>
        /// The game event processor. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private GameEventProcessor _gameEventProcessor;
	        /// <summary>
        /// The town goal processor. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private TownGoalProcessor _townGoalProcessor;
	        /// <summary>
        /// The tech tree processor. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private TechTreeProcessor _techTreeProcessor;
	        /// <summary>
        /// The building processor. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private BuildingProcessor _buildingProcessor;
	        /// <summary>
        /// The object pooling processor. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private ObjectPoolingProcessor _poolingProcessor;
	        /// <summary>
        /// The message sender. Injected via Reflex dependency injection.
        /// </summary>
	[Inject] private MessageSender _messageSender;

        /// <summary>
        /// Toggles whether buildings cost resources.
        /// </summary>
        /// <param name="player">The player.</param>
		public void ToggleBuildCosts(Player player)
		{
			if (!IsGameMaster(player))
				return;

			// TODO: Add BuildingsCostResources property to Coordinator
			// _gameProcessor.BuildingsCostResources = !_gameProcessor.BuildingsCostResources;
			// _messageSender.SendMessage($"Buildings Cost Resources: {_gameProcessor.BuildingsCostResources}");
		}

        /// <summary>
        /// Toggles player role limits.
        /// </summary>
        /// <param name="player">The player.</param>
		public void TogglePlayerRoleLimits(Player player)
		{
			if (!IsGameMaster(player))
				return;

			_roleProcessor.PlayerRoleLimits = !_roleProcessor.PlayerRoleLimits;
			_messageSender.SendMessage($"Player Role Limits: {_roleProcessor.PlayerRoleLimits}");
		}

        /// <summary>
        /// Adds resources to the town.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing resource type and amount.</param>
		public void AddResources(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;

			if (args.Length < 2)
				return;

			string resourceArg = args[0].ToLower();
			Resource resource = Resource.None;
			//TODO:: Make static helper function
			for (int i = 1; i < (int)Resource.Count - 1; i++)
			{
				if (resourceArg == ((Resource)i).ToString().ToLower())
					resource = (Resource)i;
			}

			if (resource == Resource.None)
				return;

			if (int.TryParse(args[1], out int amount))
			{
				_townResourceProcessor.AddResource(resource, amount);
			}
		}

        /// <summary>
        /// Kills a target player.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the target player name.</param>
		public void KillPlayer(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;

			if (Utils.TwitchUtils.TryGetPlayer(args[0], out Player targetPlayer))
				targetPlayer.HealthHandler.SetHealth(0);
		}

        /// <summary>
        /// Revives a target player.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the target player name.</param>
		public void RevivePlayer(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;

			if (Utils.TwitchUtils.TryGetPlayer(args[0], out Player targetPlayer))
				targetPlayer.HealthHandler.Revive();

		}

        /// <summary>
        /// Gives experience to a target player.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the target player name and experience amount.</param>
		public void GivePlayerExp(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;

			if (args.Length < 2)
				return;

			if (Utils.TwitchUtils.TryGetPlayer(args[0], out Player targetPlayer))
			{
				if (int.TryParse(args[1], out int amount))
				{
					if (amount <= 0)
						return;

					targetPlayer.RoleHandler.PlayerRoleData.IncreaseExperience(amount);
				}
			}
		}

        /// <summary>
        /// Levels up a target player by a specified amount.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the target player name and level amount.</param>
		public void LevelUpPlayer(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;

			int amount = 1;
			if (args.Length >= 2)
				int.TryParse(args[1], out amount);

			if (amount <= 0)
				return;

			if (Utils.TwitchUtils.TryGetPlayer(args[0], out Player targetPlayer))
			{
				for (int i = 0; i < amount; i++)
				{
					targetPlayer.RoleHandler.PlayerRoleData.LevelUp();
				}
			}
		}

        /// <summary>
        /// Gives experience to all players.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the experience amount.</param>
		public void GiveAllExp(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;

			if (int.TryParse(args[0], out int amount))
			{
				if (amount <= 0)
					return;

				for (int i = 0; i < _playerProcessor.PlayerCount(); i++)
				{
					_playerProcessor.GetPlayer(i).RoleHandler.PlayerRoleData.IncreaseExperience(amount);
				}
			}
		}

        /// <summary>
        /// Stops the current game event.
        /// </summary>
        /// <param name="player">The player.</param>
		public void StopCurrentEvent(Player player)
		{
			if (!IsGameMaster(player))
				return;

			GameEvent currentEvent = _gameEventProcessor.CurrentEvent;

			if (currentEvent == null)
				return;

			currentEvent.Stop();
		}

        /// <summary>
        /// Gives a pet to a target player.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the target player name and pet type.</param>
		public void GivePlayerPet(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;

			if (args.Length < 2)
				return;

			PetType type = TwitchUtils.GetPetTypeFromString(args[1]);

			if (type == PetType.Count)
				return;

			if (TwitchUtils.TryGetPlayer(args[0], out Player targetPlayer))
			{
				targetPlayer.PetsUnlocked[type] = true;
			}

		}

        /// <summary>
        /// Queues a game event.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the event type.</param>
		public void QueueEvent(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;
			GameEvent.EventType type = TwitchUtils.StringToEventEnum(args[0]);
			switch (type)
			{
				case GameEvent.EventType.None:
					break;
				case GameEvent.EventType.FishGod:
					_gameEventProcessor.AddEvent(new FishGodEvent(0, gameEventProcessor: _gameEventProcessor, townResourceProcessor: _townResourceProcessor, playerProcessor: _playerProcessor, poolingProcessor: _poolingProcessor, messageSender: _messageSender, eventInterface: _uiProcessor.EventInterface));
					break;
				case GameEvent.EventType.NightRaid:
					break;
				case GameEvent.EventType.BloodMoonRaid:
					break;
				case GameEvent.EventType.AdventureLandNecro:
					break;
				case GameEvent.EventType.AdventureLandFishGod:
					break;
				case GameEvent.EventType.DragonFire:
					break;
				case GameEvent.EventType.DragonForest:
					break;
				case GameEvent.EventType.DragonIce:
					break;
				case GameEvent.EventType.DragonTwoHeaded:
					break;
				case GameEvent.EventType.DragonUndead:
					break;
				case GameEvent.EventType.Subscription:
					break;
				case GameEvent.EventType.BitsDonated:
					break;
				case GameEvent.EventType.Vote:
					break;
				case GameEvent.EventType.MonsterRaid:
					string[] enemies = new string[] { "Minotaur" };
					_gameEventProcessor.AddEvent(new RaidEvent(0, 1200, enemies, poolingProcessor: _poolingProcessor, eventInterface: _uiProcessor.EventInterface, eventProcessor: _gameEventProcessor, enemySpawner: _enemySpawner, playerProcessor: _playerProcessor, boss: "MinotaurBoss"));
					break;
				default:
					break;
			}
		}

        /// <summary>
        /// Completes the current town goal.
        /// </summary>
        /// <param name="player">The player.</param>
		public void CompleteCurrentGoal(Player player)
		{
			if (!IsGameMaster(player))
				return;

			if (_townGoalProcessor.CurrentGoals.Count > 0)
				_townGoalProcessor.CurrentGoals[0].ForceComplete();
		}

        /// <summary>
        /// Starts a random tech research.
        /// </summary>
        /// <param name="player">The player.</param>
		public void StartRandomTech(Player player)
		{
			if (!IsGameMaster(player))
				return;

			_techTreeProcessor.StartNewRandomTech();
		}

        /// <summary>
        /// Starts a tech vote.
        /// </summary>
        /// <param name="player">The player.</param>
		public void StartVoteTech(Player player)
		{
			if (!IsGameMaster(player))
				return;

			_techTreeProcessor.StartNewTechVote();
		}

        /// <summary>
        /// Triggers the action for the current event.
        /// </summary>
        /// <param name="player">The player.</param>
		public void ActionEvent(Player player)
		{
			if (!IsGameMaster(player))
				return;

			var ev = _gameEventProcessor.CurrentEvent;

			if (ev == null)
				return;

			ev.Action();
		}

        /// <summary>
        /// Unlocks all tech tree technologies.
        /// </summary>
        /// <param name="player">The player.</param>
		public void UnlockAllTech(Player player)
		{
			if (!IsGameMaster(player))
				return;

			_techTreeProcessor.UnlockAllTech();
		}

        /// <summary>
        /// Unlocks all tech tree technologies up to Age 2.
        /// </summary>
        /// <param name="player">The player.</param>
		public void UnlockToAge2(Player player)
		{
			if (!IsGameMaster(player))
				return;

			_techTreeProcessor.UnlockToAge2Tech();
		}

		public bool IsGameMaster(Player player)
		{
			return _gameSettings.GM_IDs.Contains(player.TwitchUser.UserID);
		}

        /// <summary>
        /// Resets the ID counter for a specified building type.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the building type and ID.</param>
		public void ResetID(Player player, string command, params string[] args)
		{
			if (!IsGameMaster(player))
				return;

			if (args.Length == 2)
			{
				if (args[0] == "building" && args[0] != (BuildingType.Townhall).ToString().ToLower())
				{
					if (Enum.TryParse(args[1], true, out BuildingType building))
						_buildingProcessor.ResetBuilding(building);
				}

				_messageSender.SendMessage($"Reset ID: {args[0]}, {args[1]}");
			}
		}
	}
}
