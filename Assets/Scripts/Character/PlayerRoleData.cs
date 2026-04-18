using Processors;
using Pathfinding;
using System;
using Units;
using UnityEngine;
using Utils;

namespace Character
{
	/// <summary>
	/// Holds all data relating to a player's role and ther role's statistics.
	/// </summary>
	[System.Serializable]
	public class PlayerRoleData
	{
        /// <summary>
        /// Multiplier for calculating acceleration from movement speed.
        /// </summary>
		private const float ACCELERATION_MULTIPLIER = 0.33f;

        /// <summary>
        /// The player's role.
        /// </summary>
		private PlayerRole _role;

        /// <summary>
        /// The player's role type.
        /// </summary>
		private PlayerRoleType _roleType;

        /// <summary>
        /// Whether the role is ranged.
        /// </summary>
		private bool _ranged;

        /// <summary>
        /// The current level of the role.
        /// </summary>
		private int _level;

        /// <summary>
        /// The current experience of the role.
        /// </summary>
		private int _experience;

        /// <summary>
        /// The required experience for the next level.
        /// </summary>
		private int _requiredExp;

        /// <summary>
        /// The action amount per tick.
        /// </summary>
		private int _actionAmount;

        /// <summary>
        /// The action rate in seconds.
        /// </summary>
		private float _actionRate;

        /// <summary>
        /// The action range.
        /// </summary>
		private float _actionRange;

        /// <summary>
        /// The maximum health.
        /// </summary>
		private int _maxHealth;

        /// <summary>
        /// The health regeneration rate.
        /// </summary>
		private float _healthRegen;

        /// <summary>
        /// The movement speed.
        /// </summary>
		private float _movementSpeed;

        /// <summary>
        /// The damage reduction.
        /// </summary>
		private int _damageReduction;

        /// <summary>
        /// Audio clips for actions.
        /// </summary>
		private AudioClip[] _actionClips;

        /// <summary>
        /// The role processor.
        /// </summary>
		private RoleProcessor _roleProcessor;

        /// <summary>
        /// The player inventory.
        /// </summary>
		private PlayerInventory _playerInventory;

        /// <summary>
        /// The AI path component.
        /// </summary>
		private AIPath _aiPath;

        /// <summary>
        /// The health handler.
        /// </summary>
		private HealthHandler _healthHandler;

        /// <summary>
        /// The role handler.
        /// </summary>
		private RoleHandler _roleHandler;

        /// <summary>
        /// The player processor.
        /// </summary>
		private PlayerProcessor _playerProcessor;

        /// <summary>
        /// Gets the player's role.
        /// </summary>
        /// <value>The player's role.</value>
		public PlayerRole Role => _role;

        /// <summary>
        /// Gets the action amount.
        /// </summary>
        /// <value>The action amount.</value>
		public int ActionAmount => _actionAmount;

        /// <summary>
        /// Gets the action rate.
        /// </summary>
        /// <value>The action rate.</value>
		public float ActionRate => _actionRate;

        /// <summary>
        /// Gets the action range.
        /// </summary>
        /// <value>The action range.</value>
		public float ActionRange => _actionRange;

        /// <summary>
        /// Gets the damage reduction.
        /// </summary>
        /// <value>The damage reduction.</value>
		public int DamageReduction => _damageReduction;

        /// <summary>
        /// Gets the health regeneration rate.
        /// </summary>
        /// <value>The health regeneration rate.</value>
		public float HealthRegen => _healthRegen;

        /// <summary>
        /// Gets the maximum health.
        /// </summary>
        /// <value>The maximum health.</value>
		public int MaxHealth => _maxHealth;

        /// <summary>
        /// Gets the movement speed.
        /// </summary>
        /// <value>The movement speed.</value>
		public float MoveSpeed => _movementSpeed;

        /// <summary>
        /// Gets the current level.
        /// </summary>
        /// <value>The current level.</value>
		public int CurrentLevel => _level;

        /// <summary>
        /// Gets the current experience.
        /// </summary>
        /// <value>The current experience.</value>
		public int CurrentExp => _experience;

        /// <summary>
        /// Gets the required experience for the next level.
        /// </summary>
        /// <value>The required experience for the next level.</value>
		public int RequiredExp => _requiredExp;

        /// <summary>
        /// Gets whether the role is at max level.
        /// </summary>
        /// <value>Whether the role is at max level.</value>
		public bool IsMaxLevel => (_level >= RoleProcessor.MAX_ROLE_LEVEL);

        /// <summary>
        /// Gets the action audio clips.
        /// </summary>
        /// <value>The action audio clips.</value>
		public AudioClip[] ActionClips => _actionClips;

        /// <summary>
        /// Event fired when experience changes.
        /// </summary>
        /// <param name="roleHandler">The role handler.</param>
		public event Action<RoleHandler> OnExperienceChange;

        /// <summary>
        /// Gets the health handler.
        /// </summary>
        /// <value>The health handler.</value>
		public HealthHandler HealthHandler => _healthHandler;

		// Constructors.
		public PlayerRoleData(PlayerRole role, RoleProcessor roleProcessor, PlayerInventory inventory, AIPath aiPath, HealthHandler healthHandler, RoleHandler roleHandler, PlayerProcessor playerProcessor)
		{
			_role = role;
			_roleProcessor = roleProcessor;
			_level = 1;
			_experience = 0;
			_requiredExp = roleProcessor.GetRequiredExperience(_level);
			_roleType = roleProcessor.GetRoleData(role).RoleFlags;
			_playerInventory = inventory;
			_aiPath = aiPath;
			_healthHandler = healthHandler;
			_roleHandler = roleHandler;
			_playerProcessor = playerProcessor;
			_actionClips = roleProcessor.GetRoleData(role).ActionClips;
			//TODO: Implement ranged check
			RecalculateStats();
		}


		/// <summary>
		/// Increases the amount of current experience.
		/// </summary>
		/// <param name="amount"></param>
		public void IncreaseExperience(int amount)
		{
			amount = Mathf.Max(1,(int)( amount * _roleProcessor.GetRoleData(_role).ExpModifier));
			if (IsMaxLevel)
			{
				_experience = 0;
				return;
			}

			_experience += amount;
			OnExperienceChanged();
		}

		/// <summary>
		/// Levels up the player's role by one.
		/// </summary>
		public void LevelUp()
		{
			IncreaseExperience(_requiredExp - _experience);
		}

		/// <summary>
		/// Called when the player's role experience has changed.
		/// </summary>
		private void OnExperienceChanged()
		{
			if (_experience >= _requiredExp)
			{
				_experience -= _requiredExp;
				OnLevelUp();
				OnExperienceChanged();
			}
			OnExperienceChange?.Invoke(_roleHandler);
		}

		/// <summary>
		/// Called when the player's role has leveled up.
		/// </summary>
		private void OnLevelUp()
		{
			if (IsMaxLevel)
				return;

			_level++;
			_requiredExp = _roleProcessor.GetRequiredExperience(_level);
			RecalculateStats();
			_healthHandler.SetHealth(_healthHandler.MaxHealth);
		}

		/// <summary>
		/// Recalculates all role stats based on the role's level.
		/// </summary>
		public void RecalculateStats()
		{
			Character.RoleData data = _roleProcessor.GetRoleData(_role);
			StatModifiers statMod = _playerProcessor.GetStatModifiers(_role);

			_actionAmount = data.BaseActionAmount + (int)(data.ActionAmountPerLevel * (_level - 1));
			_actionAmount += AddStatModifiersInt(statMod, StatType.ActionAmount, _actionAmount);
			_actionAmount += AddStatModifiersInt(_roleHandler.GetGlobalPassive(StatType.ActionAmount), _actionAmount);

			_actionRate = Mathf.Max(0.1f, data.BaseActionSpeed - (data.ActionSpeedPerLevel * (_level - 1)));
			_actionRate -= AddStatModifiersFloat(statMod, StatType.ActionSpeed, _actionRate);
			_actionRate += AddStatModifiersFloat(_roleHandler.GetGlobalPassive(StatType.ActionSpeed), _actionRate);

			_actionRange = data.BaseActionRange + (_ranged ? (data.ActionRangePerLevel * (_level - 1)) : 0);
			_actionRange += AddStatModifiersFloat(statMod, StatType.ActionRange, _actionRange);
			_actionRange += AddStatModifiersFloat(_roleHandler.GetGlobalPassive(StatType.ActionRange), _actionRange);

			_maxHealth = data.BaseHealth + (int)(data.HealthPerLevel * (_level - 1));
			_maxHealth += AddStatModifiersInt(statMod, StatType.Health, _maxHealth);
			_maxHealth += AddStatModifiersInt(_roleHandler.GetGlobalPassive(StatType.Health), _maxHealth);

			_healthRegen = data.BaseHealthRegen + (data.HealthRegenPerLevel * (_level - 1));
			_healthRegen += AddStatModifiersFloat(statMod, StatType.HealthRegen, _healthRegen);
			_healthRegen += AddStatModifiersFloat(_roleHandler.GetGlobalPassive(StatType.HealthRegen), _healthRegen);

			_damageReduction = data.BaseDamageReduction + (int)(data.DamageReductionPerLevel * (_level - 1));
			_damageReduction += AddStatModifiersInt(statMod, StatType.Defense, _damageReduction);
			_damageReduction += AddStatModifiersInt(_roleHandler.GetGlobalPassive(StatType.Defense), _damageReduction);

			_movementSpeed = data.BaseMovementSpeed + (data.MovementSpeedPerLevel * (_level - 1));
			_movementSpeed += AddStatModifiersFloat(statMod, StatType.MovementSpeed, _movementSpeed);
			_movementSpeed += AddStatModifiersFloat(_roleHandler.GetGlobalPassive(StatType.MovementSpeed), _movementSpeed);

			if (_roleType == PlayerRoleType.Resource)
			{
				_playerInventory.SetMaxStorage(data.Resource, data.BaseMaxResource + (int)(data.MaxResourcePerLevel * (_level - 1)));
			}

			_aiPath.maxSpeed = _movementSpeed;
			_aiPath.maxAcceleration = _movementSpeed * ACCELERATION_MULTIPLIER;
			_healthHandler.SetMaxHealth(_maxHealth);
			_healthHandler.SetHealthRegen(_healthRegen);
			//TODO:: Add globals here, also implement damage reduction, health regen, max health and movement speed
		}

		/// <summary>
		/// Sets the role
		/// </summary>
		public void SetRole(PlayerRole role)
		{
			_role = role;
		}

		/// <summary>
		/// Sets the experience
		/// </summary>
		public void SetExperience(int experience)
		{
			_experience = experience;
		}

		/// <summary>
		/// Sets the level
		/// </summary>
		public void SetLevel(int level)
		{
			_level = level;
		}

		private int AddStatModifiersInt(StatModifiers statMod, StatType statType, int baseValue)
		{
			return (int)(baseValue * (statMod.GetModifier(statType) / 100.0f));
		}

		private float AddStatModifiersFloat(StatModifiers statMod, StatType statType, float baseValue)
		{
			return baseValue * (float)(statMod.GetModifier(statType) / 100.0f);
		}

		private int AddStatModifiersInt(float mod, int baseValue)
		{
			return (int)(baseValue * (mod / 100.0f));
		}
		private float AddStatModifiersFloat(float mod, int baseValue)
		{
			return (float)(baseValue * (mod / 100.0f));
		}

		private float AddStatModifiersFloat(float mod, float baseValue)
		{
			return (float)(baseValue * (mod / 100.0f));
		}
		private float AddStatModifiersFloat(int mod, int baseValue)
		{
			return (float)(baseValue * (mod / 100.0f));
		}
	}
}
