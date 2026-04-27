using Animation;
using Behaviours;
using Processors;
using Pathfinding;
using Sensors;
using System;
using System.Collections.Generic;
using Units;
using UnityEngine;
using UnityEngine.Events;
using Utils;
using Reflex.Attributes;

namespace Character
{
	/// <summary>
	/// Handles the current role of the Player character.
	/// </summary>
	public class RoleHandler : MonoBehaviour
	{
        /// <summary>
        /// The player associated with this role handler.
        /// </summary>
		private Player _player = null;

        /// <summary>
        /// The current role of the character.
        /// </summary>
		[SerializeField]
		private PlayerRole _currentRole = PlayerRole.Builder;

        /// <summary>
        /// The previous role of the character.
        /// </summary>
		private PlayerRole _prevRole = PlayerRole.Builder;

        /// <summary>
        /// The role type of the character.
        /// </summary>
		private PlayerRoleType _roleType = PlayerRoleType.Other;

        /// <summary>
        /// Unity event fired when the role changes.
        /// </summary>
		[SerializeField]
		private UnityEvent<PlayerRole, PlayerRole, BodyType> _onRoleChanged;

        /// <summary>
        /// The player inventory.
        /// </summary>
		private PlayerInventory _inventory;

        /// <summary>
        /// The collect resource behavior.
        /// </summary>
		private CollectResource _collectResource;

        /// <summary>
        /// The animation handler.
        /// </summary>
		private AnimationHandler _animationHandler;

        /// <summary>
        /// The character model handler.
        /// </summary>
		private CharacterModelHandler _equipmentHandler;

        /// <summary>
        /// Role processor for role management. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private RoleProcessor _roleProcessor;

        /// <summary>
        /// Tech tree processor for tech tree events. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TechTreeProcessor _techTreeProcessor;

        /// <summary>
        /// Game coordinator. Injected via Reflex dependency injection.
        /// </summary>

        /// <summary>
        /// Player processor for player management. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerProcessor _playerProcessor;

		//Sensors
        /// <summary>
        /// The target sensor.
        /// </summary>
		private TargetSensor _targetSensor;

        /// <summary>
        /// The station sensor.
        /// </summary>
		private StationSensor _stationSensor;

        /// <summary>
        /// The starter role for the character.
        /// </summary>
		private PlayerRole _starterRole = PlayerRole.Builder;

        /// <summary>
        /// The current role data scriptable object.
        /// </summary>
		private Character.RoleData _currentRoleData_SO;

        /// <summary>
        /// The current player role data.
        /// </summary>
		private PlayerRoleData _currentPlayerRoleData;

        /// <summary>
        /// The AI path component.
        /// </summary>
		private AIPath _aiPath;

        /// <summary>
        /// The health handler.
        /// </summary>
		private HealthHandler _healthHandler;

        /// <summary>
        /// Array of player role data for all roles.
        /// </summary>
		[SerializeField]
		private PlayerRoleData[] _playerRoleData;

        /// <summary>
        /// Dictionary of character global passives.
        /// </summary>
		private Dictionary<StatType, float> _characterGlobalPassives;

		/// <summary>
		/// Gets the current role of the character.
		/// </summary>
		public PlayerRole CurrentRole => _currentRole;

		/// <summary>
		/// Gets the previous role of the character.
		/// </summary>
		public PlayerRole PreviousRole => _prevRole;

		/// <summary>
		/// Gets the current role data scriptable object.
		/// </summary>
		public RoleData RoleData_SO => _currentRoleData_SO;

		/// <summary>
		/// Gets the current player role data.
		/// </summary>
		public PlayerRoleData PlayerRoleData => _currentPlayerRoleData;

		/// <summary>
		/// Gets the array of player role data for all roles.
		/// </summary>
		public PlayerRoleData[] PlayerRolesData => _playerRoleData;

        /// <summary>
        /// Gets the player inventory.
        /// </summary>
		public PlayerInventory Inventory => _inventory;

        /// <summary>
        /// Gets the animation handler.
        /// </summary>
		public AnimationHandler AnimationHandler => _animationHandler;

        /// <summary>
        /// Gets the character model handler.
        /// </summary>
		public CharacterModelHandler EquipmentHandler => _equipmentHandler;

        /// <summary>
        /// Gets or sets the player.
        /// </summary>
		public Player Player
		{
			get { return _player; }
			set { _player = value; }
		}

        /// <summary>
        /// Event fired when the role changes.
        /// </summary>
		public event Action<RoleHandler> OnRoleChanged;

		/// <summary>
		/// Attempts to set the role of the character if it is available.
		/// </summary>
		/// <param name="role">The role to set.</param>
		/// <param name="decrement">Whether to decrement the role count.</param>
		/// <returns>True if the role was set, false otherwise.</returns>
		public bool TrySetRole(PlayerRole role, bool decrement = true)
		{
			if (!_roleProcessor.TryChangeRole(_currentRole, role, decrement))
				return false;

			_onRoleChanged.Invoke(_currentRole, role, _equipmentHandler.CurrentBodyType);
			_currentRoleData_SO = _roleProcessor.GetRoleData(role);
			_targetSensor.TargetMask = _currentRoleData_SO.TargetFlags;
			_stationSensor.StationMask = _currentRoleData_SO.StationFlags;
			_roleType = _currentRoleData_SO.RoleFlags;

			_prevRole = (CurrentRole == PlayerRole.Ruler ? role : _currentRole); 

			_currentRole = role;
			_currentPlayerRoleData = _playerRoleData[(int)_currentRole];
			_aiPath.maxSpeed = _currentPlayerRoleData.MoveSpeed;
			OnRoleChanged?.Invoke(this);
			return true;
		}

		/// <summary>
		/// Sets the starter role that the character will spawn in as.
		/// </summary>
		/// <param name="role">The starter role.</param>
		public void SetStarterRole(PlayerRole role)
		{
			if (_roleProcessor.SlotsFull(role) && _roleProcessor.PlayerRoleLimits)
				return;

			_starterRole = role;
		}

		/// <summary>
		/// Sets all the role datas from file.
		/// </summary>
		/// <param name="data">The role data array.</param>
		public void SetRoleData(PlayerRoleData[] data)
		{
			_playerRoleData = data;

			for (int i = 0; i < data.Length; i++)
			{
				_playerRoleData[i].RecalculateStats();
			}
		}

		/// <summary>
		/// Recalculates all role stats.
		/// </summary>
		public void RecalculateRoles()
		{
			for (int i = 0; i < _playerRoleData.Length; i++)
			{
				_playerRoleData[i].RecalculateStats();
			}
		}

        /// <summary>
        /// Adds to the global passive for a stat type.
        /// </summary>
        /// <param name="statType">The stat type.</param>
        /// <param name="amount">The amount to add.</param>
		public void AddToGlobalPassive(StatType statType, float amount)
		{
			_characterGlobalPassives[statType] += amount;
		}

        /// <summary>
        /// Gets the global passive for a stat type.
        /// </summary>
        /// <param name="statType">The stat type.</param>
        /// <returns>The global passive value.</returns>
		public float GetGlobalPassive(StatType statType)
		{
			if (_characterGlobalPassives == null || !_characterGlobalPassives.ContainsKey(statType))
				return 0;

			return _characterGlobalPassives[statType];
		}

		// Unity Events.
        // Initializes the role handler.
		private void Awake()
		{
			// Initialize sensors and handlers.
			_targetSensor = GetComponent<TargetSensor>();
			_stationSensor = GetComponent<StationSensor>();
			_inventory = GetComponent<PlayerInventory>();
			_collectResource = GetComponent<CollectResource>();
			_animationHandler = GetComponentInChildren<AnimationHandler>();
			_equipmentHandler = GetComponent<CharacterModelHandler>();
			_healthHandler = GetComponent<HealthHandler>();
			_aiPath = GetComponent<AIPath>();

			// Initialize player role data array.
			_playerRoleData = new PlayerRoleData[(int)PlayerRole.Count];

			for (int i = 0; i < (int)PlayerRole.Count; i++)
			{
				_playerRoleData[i] = new PlayerRoleData((PlayerRole)i, _roleProcessor, _inventory, _aiPath, _healthHandler, this, _playerProcessor);
			}

			// Initialize character global passives dictionary.
			_characterGlobalPassives = new Dictionary<StatType, float>();

			for (int i = 0; i < (int)StatType.Count; i++)
			{
				_characterGlobalPassives.Add((StatType)i, 0.0f);
			}
		}

        // Sets the starter role and health on Start.
		private void Start()
		{
			// Try to set the starter role.
			TrySetRole(_starterRole, false);

			// Set the health to maximum.
			_healthHandler.SetHealth(_healthHandler.MaxHealth);
		}

        // Subscribes to tech tree events on Enable.
		private void OnEnable()
		{
			// Subscribe to stat boost unlocked event.
			_techTreeProcessor.OnStatBoostUnlocked += OnStatBoostUnlocked;
		}

        // Called when a stat boost is unlocked for a role.
		private void OnStatBoostUnlocked(PlayerRole role, StatType type)
		{
			// Recalculate stats if the role matches the current role.
			if (role == _currentRole)
				_currentPlayerRoleData.RecalculateStats();
		}

        /// <summary>
        /// Tries to get the role data for a specific role.
        /// </summary>
        /// <param name="role">The role to get data for.</param>
        /// <param name="data">The role data output.</param>
        /// <returns>True if the role data was found, false otherwise.</returns>
		public bool TryGetRoleData(PlayerRole role, out PlayerRoleData data)
		{
			for(int i = 0; i < _playerRoleData.Length; i ++)
				if(role == _playerRoleData[i].Role)
				{
					data = _playerRoleData[i];
					return true;
				}
			data = null;
			return false;
		}
	}
}
