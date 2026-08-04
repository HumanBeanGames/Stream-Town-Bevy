using GameResources;
using GUIDSystem;
using Processors;
using Sensors;
using System;
using Units;
using UnityEngine;
using Utils;
using Reflex.Attributes;

namespace Enemies
{
	/// <summary>
	/// Base class for all Enemy Units in the game.
	/// </summary>
	public class Enemy : MonoBehaviour
	{
        /// <summary>
        /// The type of the enemy.
        /// </summary>
		[SerializeField]
		private EnemyType _enemyType;
        /// <summary>
        /// Event processor for game events. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private EventProcessor _eventProcessor;
        /// <summary>
        /// Additional health per player.
        /// </summary>
		[SerializeField]
		private float _additionalHealthPerPlayer = 0.05f;
        /// <summary>
        /// The health handler.
        /// </summary>
		private HealthHandler _healthHandler;
        /// <summary>
        /// The target sensor.
        /// </summary>
		private TargetSensor _targetSensor;
        /// <summary>
        /// The GUID component.
        /// </summary>
		private GUIDComponent _gUIDComponent;
        /// <summary>
        /// The station sensor.
        /// </summary>
		private StationSensor _stationSensor;
        /// <summary>
        /// Player processor for player management. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerProcessor _playerProcessor;
        /// <summary>
        /// The active resource incrementer.
        /// </summary>
		private ActiveResourceIncrementer _activeResourceIncrementer;

        /// <summary>
        /// Gets the health handler.
        /// </summary>
		public HealthHandler HealthHandler => _healthHandler;
        /// <summary>
        /// Gets the target sensor.
        /// </summary>
		public TargetSensor TargetSensor => _targetSensor;
        /// <summary>
        /// Gets the GUID component.
        /// </summary>
		public GUIDComponent GUIDComponent => _gUIDComponent;
        /// <summary>
        /// Gets the station sensor.
        /// </summary>
		public StationSensor StationSensor => _stationSensor;
        /// <summary>
        /// Event fired when the enemy dies.
        /// </summary>
		public Action<Enemy> OnDied;
        /// <summary>
        /// Gets the enemy type.
        /// </summary>
		public EnemyType EnemyType => _enemyType;
        /// <summary>
        /// Called when the enemy is pooled.
        /// </summary>
        public void OnPooled()
		{
		}

		/// <summary>
		/// Rebuilds player-count-derived maximum health before applying saved health.
		/// </summary>
		public void RestoreHealth(int health)
		{
			if (_healthHandler.BaseMaxHealth > 0)
			{
				_healthHandler.SetMaxHealth(
					_healthHandler.BaseMaxHealth +
					(int)(_additionalHealthPerPlayer * (_playerProcessor.PlayerCount() + _playerProcessor.RecruitCount())));
			}

			_healthHandler.SetHealth(health);
		}

        /// <summary>
        /// Initializes all required data and components.
        /// </summary>
		private void Init()
		{
			if (_healthHandler)
				return;

			_healthHandler = GetComponent<HealthHandler>();
			_activeResourceIncrementer = GetComponent<ActiveResourceIncrementer>();
			_targetSensor = GetComponent<TargetSensor>();
			_stationSensor = GetComponent<StationSensor>();
			_gUIDComponent = GetComponent<GUIDComponent>();
		}

		// Unit Events.
        // Initializes the enemy on Awake.
		private void Awake()
		{
			Init();
		}
        // Subscribes to death event on Start.
		private void Start()
		{
			_healthHandler.OnDeath += OnDeath;
		}
        // Initializes the enemy and sets health based on player count on Enable.
		private void OnEnable()
		{
			Init();
			if (_healthHandler.BaseMaxHealth <= 0)
				return;

			_healthHandler.SetMaxHealth(_healthHandler.BaseMaxHealth + (int)(_additionalHealthPerPlayer * (_playerProcessor.PlayerCount() + _playerProcessor.RecruitCount())));
			_healthHandler.SetHealth(_healthHandler.MaxHealth);
		}

        /// <summary>
        /// Called when the enemy dies.
        /// </summary>
        /// <param name="killedByPlayer">Whether the enemy was killed by a player.</param>
		private void OnDeath(bool killedByPlayer)
		{
			if (killedByPlayer)
			{
				_eventProcessor.EnemyKilled?.Invoke(_enemyType);
				_activeResourceIncrementer.Increment();
			}
			OnDied?.Invoke(this);
		}
	}
}
