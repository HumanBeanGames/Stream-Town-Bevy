using UnityEngine;
using Character;
using System.Collections.Generic;
using Utils;
using Units;
using Sensors;
using UserInterface;
using System;
using Pets;
using GUIDSystem;
using Utils.Pooling;
using Target;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Processors;
using Twitch;

namespace Processors
{
    /// <summary>
    /// Processor that manages all players in the game.
    /// Handles player spawning, role assignment, stat modifiers, and player state.
    /// </summary>
    public class PlayerProcessor : MonoBehaviour, IInstaller, IProcessor
    {
        /// <summary>
        /// Runtime data for player data.
        /// Assigned in InjectRuntimeData.
        /// </summary>
        private PlayerRuntimeData _playerRuntimeData;

        /// <summary>
        /// Gets or sets the user-controlled player.
        /// </summary>
        public Player UserPlayer
        {
            get => _playerRuntimeData.UserPlayer;
            set => _playerRuntimeData.UserPlayer = value;
        }

        /// <summary>
        /// Gets the global stat modifiers.
        /// </summary>
        public StatModifiers GlobalStatModifiers => _playerRuntimeData.GlobalStatModifiers;

        /// <summary>
        /// Gets the player spawn position transform.
        /// </summary>
        public Transform PlayerSpawnPosition => _playerRuntimeData.PlayerSpawnPosition;

        /// <summary>
        /// Object pooling processor for managing pooled objects.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// Town resource processor for managing resources.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TownResourceProcessor _townResourceProcessor;

        /// <summary>
        /// Role processor for managing role data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private RoleProcessor _roleProcessor;

        /// <summary>
        /// Time processor for time-related data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TimeProcessor _timeProcessor;

        /// <summary>
        /// The Twitch chat processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Processors.TwitchChatProcessor _twitchChatProcessor;

        /// <summary>
        /// Event invoked when the ruler changes.
        /// </summary>
        public Action<Player> OnRulerChanged;

        /// <summary>
        /// Gets the list of all active players.
        /// </summary>
        public List<Player> Players => _playerRuntimeData.Players;

        /// <summary>
        /// Gets the current ruler player.
        /// </summary>
        /// <returns>The ruler player, or null if no ruler is set.</returns>
        public Player GetRuler()
        {
            return _playerRuntimeData.Ruler;
        }

		public void SetUserPlayer(Player player)
		{
			UserPlayer = player;
		}

        /// <summary>
        /// Shows numbered IDs above all recruits for selection purposes.
        /// </summary>
        public void ShowRecruitIDs()
        {
            List<Player> recruits = _playerRuntimeData.Recruits;
            for (int i = 0; i < recruits.Count; i++)
            {
                var textDisplay = _poolingProcessor.GetPooledObject("TextDisplay");
                textDisplay.gameObject.SetActive(true);
                var rectTransform = textDisplay.GetComponent<RectTransform>();
                rectTransform.SetParent(recruits[i].PlayerTarget.TextDisplayTransform, false);
                rectTransform.localPosition = recruits[i].PlayerTarget.TextDisplayTransform.localPosition;

                var display = textDisplay.GetComponent<UnitTextDisplay>();
                display.Targetable = recruits[i].PlayerTarget;
                display.SetDisplayText($"{i + 1}");
                display.SetDisplayTextAfterTime("", 15.0f);
            }
        }

        /// <summary>
        /// Dismisses a recruit from the recruit pool.
        /// Removes the player, decreases recruit resource count, and deactivates the character.
        /// </summary>
        /// <param name="player">The recruit player to dismiss.</param>
        public void DismissRecruit(Player player)
        {
            if (_playerRuntimeData.Recruits.Contains(player))
            {
                _playerRuntimeData.Recruits.Remove(player);
                _townResourceProcessor.RemoveResource(Resource.Recruit, 1);
				_roleProcessor.TakeFromRole(player.RoleHandler.CurrentRole);
                player.Character.SetActive(false);
            }
        }

        /// <summary>
        /// Swaps a recruit's role to a different role.
        /// </summary>
        /// <param name="player">The recruit whose role to change.</param>
        /// <param name="role">The new role to assign.</param>
        public void SwapRecruitRole(Player player, PlayerRole role)
        {
            player.RoleHandler.TrySetRole(role);
        }

        /// <summary>
        /// Initializes the player processor.
        /// Creates RuntimeData after all processors are confirmed ready.
        /// Sets up stat modifiers and player update queue.
        /// </summary>
        public void Initialize()
        {
            if (_playerRuntimeData == null)
                throw new InvalidOperationException("PlayerProcessor runtime data has not been installed.");

            Dictionary<PlayerRole, StatModifiers> roleStatModifiers = new Dictionary<PlayerRole, StatModifiers>();
            StatModifiers globalStatModifier = new StatModifiers();
            Queue<Player> playerUpdateQueue = new Queue<Player>();

            for (int i = 0; i < (int)PlayerRole.Count; i++)
            {
                roleStatModifiers.Add((PlayerRole)i, new StatModifiers());
            }

            _playerRuntimeData.InitializePlayerState(roleStatModifiers, globalStatModifier, playerUpdateQueue);

            // Set up player data access for Twitch chat
            _twitchChatProcessor.SetPlayerDataAccess((string userID, out int index) => PlayerExistsByID(userID, out index), GetPlayer);
        }

        /// <summary>
        /// Registers this processor as a singleton in the dependency injection container.
        /// Called by Reflex during container initialization.
        /// </summary>
        /// <param name="containerBuilder">The container builder to register bindings with.</param>
        public void InstallBindings(ContainerBuilder containerBuilder)
        {
            containerBuilder.AddSingleton(this);
            InjectRuntimeData(containerBuilder);
        }

        public void InjectRuntimeData(ContainerBuilder containerBuilder)
        {
            if (_playerRuntimeData != null)
                throw new InvalidOperationException("PlayerProcessor runtime data has already been installed.");

            _playerRuntimeData = new PlayerRuntimeData();
            containerBuilder.AddSingleton(_playerRuntimeData);
        }

        /// <summary>
        /// Adds a new player to the game, initializing their character and data.
        /// </summary>
        /// <param name="data">The player data to add.</param>
        /// <param name="startingRole">The starting role for the player.</param>
        public void AddNewPlayer(Player data, PlayerRole startingRole = PlayerRole.Builder)
        {
            if (_playerRuntimeData.Players.Contains(data))
                return;

            if (_playerRuntimeData.Recruits.Contains(data))
                return;

            // TODO: Optimize this, store it when objects are pooled.
            PoolableObject obj = _poolingProcessor.GetPooledObject("Player");
            obj.gameObject.SetActive(true);
            obj.transform.position = _playerRuntimeData.PlayerSpawnPosition.position;
            data.RoleHandler = obj.GetComponent<RoleHandler>();
            data.RoleHandler.Player = data;
            data.StationSensor = obj.GetComponent<StationSensor>();
            data.HealthHandler = obj.GetComponent<HealthHandler>();
            data.HealthHandler.OnDeath += (attacked) => data.OnCharacterDied(attacked, _twitchChatProcessor);
            data.HealthHandler.OnRevived += () => data.OnCharacterRespawned(_twitchChatProcessor);
            data.TargetSensor = obj.GetComponent<TargetSensor>();
            data.EquipmentHandler = obj.GetComponent<CharacterModelHandler>();
            data.GUIDComponent = obj.GetComponent<GUIDComponent>();
            data.PlayerTarget = obj.GetComponent<TargetablePlayer>();
            data.RoleHandler.SetStarterRole(startingRole);
            data.Character = obj.gameObject;
            data.StationSensor.Player = data;
            var unitText = obj.GetComponentInChildren<UnitTextDisplay>();

            unitText.gameObject.SetActive(true);
            unitText.SetDisplayText(data.TwitchUser.Username);
            unitText.SetTextColor(Twitch.Utils.UserColours.GetColourByUserType(data.TwitchUser.GameUserType));
            data.UnitTextDisplay = unitText;
            PoolableObject petObj = _poolingProcessor.GetPooledObject("Pet");
            Pet pet = petObj.GetComponent<Pet>();

            pet.SetOwner(obj.transform, data);
            petObj.gameObject.SetActive(true);
            _playerRuntimeData.PlayerUpdateQueue.Enqueue(data);
            data.Pet = pet;

            if (data.TwitchUser.Username == "")
                if (_playerRuntimeData.Recruits.Count < 200)
                {
                    _playerRuntimeData.Recruits.Add(data);
                    _townResourceProcessor.AddResource(Resource.Recruit, 1);
                    data.TwitchUser.GameUserType = Twitch.Utils.GameUserType.Normal;
                }

                else
                    return;
            else
                _playerRuntimeData.Players.Add(data);
        }

        /// <summary>
        /// Adds an existing player to the game, initializing their character and data.
        /// </summary>
        /// <param name="data">The player data to add.</param>
        /// <param name="startingRole">The starting role for the player.</param>
        /// <returns>The added player, or null if the player already exists or recruit pool is full.</returns>
        public Player AddExistingPlayer(Player data, PlayerRole startingRole = PlayerRole.Builder)
        {
            if (_playerRuntimeData.Players.Contains(data))
                return null;

            if (data.TwitchUser.Username == "")
                if (_playerRuntimeData.Recruits.Count < 200)
                {
                    _playerRuntimeData.Recruits.Add(data);
                    _townResourceProcessor.AddResource(Resource.Recruit, 1);
                }
                else
                    return null;
            else
                _playerRuntimeData.Players.Add(data);

            data.StationSensor.Player = data;
            var unitText = data.Character.GetComponentInChildren<UnitTextDisplay>();
            unitText.SetDisplayText(data.TwitchUser.Username);
            unitText.SetTextColor(Twitch.Utils.UserColours.GetColourByUserType(data.TwitchUser.GameUserType));
            data.UnitTextDisplay = unitText;
            _playerRuntimeData.PlayerUpdateQueue.Enqueue(data);
            return data;
        }

        /// <summary>
        /// Removes a player from the game.
        /// </summary>
        /// <param name="data">The player to remove.</param>
        public void RemovePlayer(Player data)
        {
            if (!_playerRuntimeData.Players.Contains(data))
                return;

            _playerRuntimeData.Players.Remove(data);
        }

        /// <summary>
        /// Gets a Player by index.
        /// </summary>
        /// <param name="index">The index of the player to get.</param>
        /// <returns>The player at the specified index, or null if index is out of range.</returns>
        public Player GetPlayer(int index)
        {
            if (_playerRuntimeData.Players.Count <= index)
                return null;

            return _playerRuntimeData.Players[index];
        }

        /// <summary>
        /// Checks if a player exists.
        /// </summary>
        /// <param name="data">The player to check.</param>
        /// <returns>True if the player exists in the players list.</returns>
        public bool PlayerExists(Player data)
        {
            return _playerRuntimeData.Players.Contains(data);
        }

        /// <summary>
        /// Sets the ruler player.
        /// Restores the previous ruler's role and invokes the ruler changed event.
        /// </summary>
        /// <param name="player">The player to set as ruler.</param>
        public void SetRuler(Player player)
        {
            if (_playerRuntimeData.Ruler != null)
                _playerRuntimeData.Ruler.RoleHandler.TrySetRole(_playerRuntimeData.Ruler.RoleHandler.PreviousRole);

            _playerRuntimeData.Ruler = player;
            OnRulerChanged?.Invoke(player);
        }

        /// <summary>
        /// Checks if a player with the specified userID exists and outputs an index.
        /// </summary>
        /// <param name="userID">The Twitch user ID to search for.</param>
        /// <param name="index">Output parameter for the player index if found.</param>
        /// <returns>True if a player with the specified userID exists.</returns>
        public bool PlayerExistsByID(string userID, out int index)
        {
            index = -1;

            for (int i = 0; i < _playerRuntimeData.Players.Count; i++)
            {
                if (_playerRuntimeData.Players[i].TwitchUser.UserID == userID)
                {
                    index = i;
                    return true;
                }
            }
            return false;
        }

        /// <summary>
        /// Checks if a player with the specified lowercase name exists and outputs an index.
        /// </summary>
        /// <param name="playerName">The lowercase player name to search for.</param>
        /// <param name="index">Output parameter for the player index if found.</param>
        /// <returns>True if a player with the specified name exists.</returns>
        public bool PlayerExistsByNameToLower(string playerName, out int index)
        {
            index = -1;

            for (int i = 0; i < _playerRuntimeData.Players.Count; i++)
            {
                if (_playerRuntimeData.Players[i].TwitchUser.Username.ToLower() == playerName)
                {
                    index = i;
                    return true;
                }
            }
            return false;
        }

        /// <summary>
        /// Returns the current number of Players.
        /// </summary>
        /// <returns>The count of active players.</returns>
        public int PlayerCount()
        {
            return _playerRuntimeData.Players.Count;
        }

        /// <summary>
        /// Returns the current number of recruits.
        /// </summary>
        /// <returns>The count of recruits in the recruit pool.</returns>
        public int RecruitCount()
        {
            return _playerRuntimeData.Recruits.Count;
        }

        /// <summary>
        /// Gets a recruit by index (1-based).
        /// </summary>
        /// <param name="index">The 1-based index of the recruit to get.</param>
        /// <returns>The recruit at the specified index, or null if index is out of range.</returns>
        public Player GetRecruitByIndex(int index)
        {
            int adjustedIndex = index - 1;
            if (adjustedIndex <= _playerRuntimeData.Recruits.Count - 1)
                return _playerRuntimeData.Recruits[adjustedIndex];
            else
                return null;
        }

        /// <summary>
        /// Returns the Player's Twitch name by index.
        /// </summary>
        /// <param name="index">The index of the player.</param>
        /// <returns>The Twitch username of the player, or empty string if index is out of range.</returns>
        public string GetPlayerTwitchName(int index)
        {
            if (_playerRuntimeData.Players.Count <= index)
                return "";

            return _playerRuntimeData.Players[index].TwitchUser.Username;
        }

        /// <summary>
        /// Gets the stat modifiers for a specific role.
        /// </summary>
        /// <param name="role">The role to get stat modifiers for.</param>
        /// <returns>The stat modifiers for the specified role, or a new StatModifiers if not found.</returns>
        public StatModifiers GetStatModifiers(PlayerRole role)
        {
            if (_playerRuntimeData.RoleStatModifiers.ContainsKey(role))
                return _playerRuntimeData.RoleStatModifiers[role];
            return new StatModifiers();
        }

        /// <summary>
        /// Updates player activity by cycling through the update queue.
        /// Called every frame by the Coordinator.
        /// </summary>
        public void Process()
        {
            if (_playerRuntimeData.PlayerUpdateQueue.Count > 0)
            {
                var playerToUpdate = _playerRuntimeData.PlayerUpdateQueue.Dequeue();

                playerToUpdate.TwitchUser.UpdateActivity(_timeProcessor.WorldTimePassed);

                _playerRuntimeData.PlayerUpdateQueue.Enqueue(playerToUpdate);
            }
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // PlayerProcessor does not have scene-specific settings to refresh
        }
    }
}
