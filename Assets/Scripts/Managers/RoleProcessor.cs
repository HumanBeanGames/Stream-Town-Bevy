using System.Collections.Generic;
using Utils;
using UnityEngine;
using Character;
using UnityEngine.Events;
using System;
using ScriptablesProcessorInfrastructure;
using Reflex.Attributes;
using Reflex.Core;
using Data.Containers;

#if UNITY_EDITOR
using UnityEditor;
#endif

namespace Processors
{
    /// <summary>
    /// Processor that manages the role system for players.
    /// Handles role assignments, slot management, and role limits.
    /// </summary>
    public class RoleProcessor : MonoBehaviour, IInstaller, IProcessor
    {
        /// <summary>
        /// Maximum level a role can reach.
        /// </summary>
        public const int MAX_ROLE_LEVEL = 99;

        /// <summary>
        /// Container for all role data definitions.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private RoleDataContainer _roleDataContainer;

        /// <summary>
        /// The debug processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Processors.DebugProcessor _debugProcessor;

        /// <summary>
        /// Nested runtime data class for role data.
        /// Created and bound in InjectRuntimeData().
        /// </summary>
        private RoleRuntimeData _roleRuntimeData;

        /// <summary>
        /// Gets or sets whether player role limits are enforced.
        /// </summary>
        public bool PlayerRoleLimits
        {
            get => _roleRuntimeData.PlayerRoleLimits;
            set => _roleRuntimeData.PlayerRoleLimits = value;
        }

        /// <summary>
        /// Player processor for accessing player data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// Gets the role data container.
        /// </summary>
        public RoleDataContainer AllRoleData => _roleDataContainer;

        /// <summary>
        /// Event fired when role slots change.
        /// </summary>
        public event System.Action<PlayerRole> OnRoleSlotsChangedEvent;

        /// <summary>
        /// Event fired when a role slot is removed.
        /// </summary>
        public event System.Action<PlayerRole> OnSlotRemoved;

        /// <summary>
        /// Returns the stored RoleData for that specified PlayerRole.
        /// </summary>
        /// <param name="role">The role to get data for.</param>
        /// <returns>The role data domain copy, or null if not found.</returns>
        public Character.RoleData GetRoleData(PlayerRole role)
        {
            if (!_roleRuntimeData.RoleDataContainer.RoleDataDictionary.ContainsKey(role))
            {
                _debugProcessor.LogError(DebugLogCategory.RoleProcessor, $"Attempted to get role data for role {role} but it was not found!");
                return null;
            }

            return new Character.RoleData(_roleRuntimeData.RoleDataContainer.RoleDataDictionary[role]);
        }

        /// <summary>
        /// Gets a list of available role names as strings.
        /// </summary>
        /// <returns>List of available role names excluding Ruler.</returns>
        public List<string> GetAvailableRolesAsString()
        {
            List<string> roles = new List<string>();
            foreach (PlayerRole role in _roleRuntimeData.RoleSlotsDictionary.Keys)
            {
                if (_roleRuntimeData.RoleSlotsDictionary[role].Available && role != PlayerRole.Ruler)
                    roles.Add(role.ToString());
            }
            return roles;
        }

        /// <summary>
        /// Gets a list of available roles.
        /// </summary>
        /// <returns>List of available roles excluding Ruler.</returns>
        public List<PlayerRole> GetAvailableRoles()
        {
            List<PlayerRole> roles = new List<PlayerRole>();
            foreach (PlayerRole role in _roleRuntimeData.RoleSlotsDictionary.Keys)
            {
                if (_roleRuntimeData.RoleSlotsDictionary[role].Available && role != PlayerRole.Ruler)
                    roles.Add(role);
            }
            return roles;
        }

        /// <summary>
        /// Gets an available role by index.
        /// </summary>
        /// <param name="index">The index of the role to get.</param>
        /// <returns>The available role at the specified index.</returns>
        public PlayerRole GetAvailableRoleFromIndex(int index)
        {
            List<PlayerRole> availableRoles = GetAvailableRoles();
            return availableRoles[index];
        }

        /// <summary>
        /// Gets the index of a role in the available roles list.
        /// </summary>
        /// <param name="playerRole">The role to find the index for.</param>
        /// <returns>The index of the role, or 0 if not found.</returns>
        public int GetRoleIndex(PlayerRole playerRole)
        {
            List<PlayerRole> availableRoles = GetAvailableRoles();
            for (int i = 0; i < availableRoles.Count; i++)
            {
                if (availableRoles[i].ToString() == playerRole.ToString())
                    return i;
            }
            return 0;
        }

        /// <summary>
        /// Removes a slot from the specified role.
        /// </summary>
        /// <param name="role">The role to remove a slot from.</param>
        public void TakeFromRole(PlayerRole role)
        {
            _roleRuntimeData.RoleSlotsDictionary[role].OnSlotRemoved();
			OnSlotRemoved?.Invoke(role);
        }

        /// <summary>
        /// Gets the required experience for a given level.
        /// </summary>
        /// <param name="level">The level to get required experience for.</param>
        /// <returns>The required experience for the level.</returns>
        public int GetRequiredExperience(int level)
        {
            return _roleRuntimeData.RoleDataContainer.GetRequiredExperience(level);
        }

        /// <summary>
        /// Checks if a role is available (not full).
        /// </summary>
        /// <param name="role">The role to check.</param>
        /// <returns>True if the role has available slots.</returns>
        public bool IsRoleAvailable(PlayerRole role)
        {
            return !_roleRuntimeData.RoleSlotsDictionary[role].Full;
        }

        /// <summary>
        /// Called when a character unit attempts to change it's role.
        /// </summary>
        /// <param name="previousRole">The role to switch from.</param>
        /// <param name="newRole">The role to switch to.</param>
        /// <param name="decrement">Whether to decrement the previous role slot count.</param>
        /// <returns>True if the role can be switched to.</returns>
        public bool TryChangeRole(PlayerRole previousRole, PlayerRole newRole, bool decrement = true)
        {
            Dictionary<PlayerRole, RoleSlot> roleSlotsDictionary = _roleRuntimeData.RoleSlotsDictionary;

            if (!roleSlotsDictionary.ContainsKey(newRole))
            {
                _debugProcessor.LogError(DebugLogCategory.RoleProcessor, $"Attempted to change to a role that is not stored: {newRole}");
                return false;
            }

            if (roleSlotsDictionary[newRole].Full && _roleRuntimeData.PlayerRoleLimits)
            {
                if (TryReplaceInactivePlayer(newRole, out Player player))
                {
                    player.RoleHandler.TrySetRole(PlayerRole.Builder);
                }
                else
                    return false;
            }

            if (decrement)
			{
                _roleRuntimeData.RoleSlotsDictionary[previousRole].OnSlotRemoved();
				OnSlotRemoved?.Invoke(previousRole);
			}

            _roleRuntimeData.RoleSlotsDictionary[newRole].OnSlotTaken();

            _roleRuntimeData.OnRoleSlotsChangedEvent.Invoke(previousRole);
            _roleRuntimeData.OnRoleSlotsChangedEvent.Invoke(newRole);
            OnRoleSlotsChangedEvent?.Invoke(previousRole);
            OnRoleSlotsChangedEvent?.Invoke(newRole);

            return true;
        }

        /// <summary>
        /// Adds more total slots to the specified PlayerRole.
        /// </summary>
        /// <param name="role">The role to add slots to.</param>
        /// <param name="amount">The number of slots to add.</param>
        public void AddSlots(PlayerRole role, int amount)
        {
            _roleRuntimeData.RoleSlotsDictionary[role].IncreaseMaxSlots(amount);
            _roleRuntimeData.OnRoleSlotsChangedEvent.Invoke(role);
            OnRoleSlotsChangedEvent?.Invoke(role);
        }

        /// <summary>
        /// Removes total slots available for the specified PlayerRole.
        /// </summary>
        /// <param name="role">The role to remove slots from.</param>
        /// <param name="amount">The number of slots to remove.</param>
        public void RemoveSlots(PlayerRole role, int amount)
        {
            _roleRuntimeData.RoleSlotsDictionary[role].DecreaseMaxSlots(amount);
            _roleRuntimeData.OnRoleSlotsChangedEvent.Invoke(role);
            OnRoleSlotsChangedEvent?.Invoke(role);
        }

        /// <summary>
        /// Checks if all slots for the PlayerRole are taken.
        /// </summary>
        /// <param name="role">The role to check.</param>
        /// <returns>True if all slots for the role are taken.</returns>
        public bool SlotsFull(PlayerRole role)
        {
            return _roleRuntimeData.RoleSlotsDictionary[role].Full;
        }

        /// <summary>
        /// Gets a formatted string displaying number of role slots taken and slots available.
        /// </summary>
        /// <param name="role">The role to get slot information for.</param>
        /// <returns>A formatted string showing slot usage.</returns>
        public string GetSlotPrint(PlayerRole role)
        {
            return _roleRuntimeData.RoleSlotsDictionary[role].SlotDataAsString;
        }

        /// <summary>
        /// Gets the maximum number of slots for a role.
        /// </summary>
        /// <param name="role">The role to get max slots for.</param>
        /// <returns>The maximum number of slots for the role.</returns>
        public int GetMaxSlots(PlayerRole role)
        {
            return _roleRuntimeData.RoleSlotsDictionary[role].MaxSlots;
        }

        /// <summary>
        /// Checks if a role has infinite slots.
        /// </summary>
        /// <param name="role">The role to check.</param>
        /// <returns>True if the role has infinite slots.</returns>
        public bool RoleIsInfinite(PlayerRole role)
        {
            return _roleRuntimeData.RoleSlotsDictionary[role].Infinite;
        }

        /// <summary>
        /// Tries to find and replace an inactive player in the specified role.
        /// </summary>
        /// <param name="role">The role to search for an inactive player in.</param>
        /// <param name="player">Output parameter for the inactive player if found.</param>
        /// <returns>True if an inactive player was found and replaced.</returns>
        public bool TryReplaceInactivePlayer(PlayerRole role, out Player player)
        {
            player = null;

            for (int i = 0; i < _playerProcessor.PlayerCount(); i++)
            {
                Player targetPlayer = _playerProcessor.GetPlayer(i);

                if (targetPlayer.RoleHandler.CurrentRole != role)
                    continue;

                if (targetPlayer.TwitchUser.ActivityStatus != Character.Enumerations.ActivityStatus.Inactive)
                    continue;

                player = targetPlayer;
                return true;
            }

            return false;
        }

        // Initializes role slot data from the role data container.
        private void InitializeRoleData()
        {
            Dictionary<PlayerRole, RoleSlot> roleSlotsDictionary = new Dictionary<PlayerRole, RoleSlot>();

            for (int i = 0; i < (int)PlayerRole.Count; i++)
            {
                var role = (PlayerRole)i;
                if (roleSlotsDictionary.ContainsKey(role))
                {
                    _debugProcessor.LogError(DebugLogCategory.RoleProcessor, $"Attempted to add the same role multiple times {role}.");
                    continue;
                }
                var roleData = _roleDataContainer.RoleDataDictionary[role];
                roleSlotsDictionary.Add(role, new RoleSlot(role, roleData.BaseMaxUserLimit, !roleData.HasUserLimit));
            }

            _roleRuntimeData.InitializeRoleSlots(roleSlotsDictionary);
            _roleRuntimeData.InitializeRoleData(_roleDataContainer);
        }

        /// <summary>
        /// Initializes the role processor.
        /// Sets up role slot data from the container.
        /// </summary>
        public void Initialize()
        {
            if (_roleRuntimeData == null)
                throw new InvalidOperationException("RoleProcessor: RoleRuntimeData has not been installed.");

            InitializeRoleData();
        }

		/// <summary>Resets role occupancy and authored slot limits before players are restored.</summary>
		public void ResetWorldState()
		{
			InitializeRoleData();
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
            if (_roleRuntimeData != null)
                throw new InvalidOperationException("RoleProcessor: RoleRuntimeData has already been installed.");

            _roleRuntimeData = new RoleRuntimeData();
            containerBuilder.AddSingleton(_roleRuntimeData);
        }

        /// <summary>
        /// Processes role logic every frame.
        /// Called every frame by the Coordinator.
        /// RoleProcessor does not require per-frame updates.
        /// </summary>
        public void Process()
        {
            // RoleProcessor does not require per-frame updates
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // RoleProcessor does not have scene-specific settings to refresh
        }
    }
}
