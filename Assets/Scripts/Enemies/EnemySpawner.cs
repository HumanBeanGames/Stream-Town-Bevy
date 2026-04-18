using Processors;
using System.Collections.Generic;
using UnityEngine;
using Utils;
using Utils.Pooling;
using Reflex.Attributes;
using Environment;

namespace Enemies
{
	/// <summary>
	/// Handles the spawning of enemies.
	/// </summary>
	public class EnemySpawner : MonoBehaviour
	{
        /// <summary>
        /// The minimum total number of enemies.
        /// </summary>
		[SerializeField]
		private int _minTotalEnemies = 3;

        /// <summary>
        /// The maximum total number of enemies.
        /// </summary>
		[SerializeField]
		private int _maxTotalEnemies = 50;

        /// <summary>
        /// The time between spawns in seconds.
        /// </summary>
		[SerializeField]
		private float _timeBetweenSpawns = 25;

        /// <summary>
        /// List of enemies with spawn chances.
        /// </summary>
		[SerializeField]
		private ChanceObjectList<string> _enemies;

        /// <summary>
        /// List of spawned enemies.
        /// </summary>
		[SerializeField]
		private List<Enemy> _spawnedEnemies = new List<Enemy>();

        /// <summary>
        /// Game state processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GameStateProcessor _gameStateProcessor;

        /// <summary>
        /// Array of spawn locations.
        /// </summary>
		[SerializeField]
		private Transform[] _spawnLocations;

        /// <summary>
        /// Object pooling processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// Day and night processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private DayAndNightProcessor _dayNightProcessor;

        /// <summary>
        /// Time processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TimeProcessor _timeProcessor;

        /// <summary>
        /// Player processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// The spawn timer.
        /// </summary>
		private float _spawnTimer = 0;

        /// <summary>
        /// The maximum number of enemies.
        /// </summary>
		private int _maxEnemies = 2;

        /// <summary>
        /// Gets or sets whether enemies can be spawned.
        /// </summary>
		public bool CanSpawnEnemies { get; set; } = true;

        /// <summary>
        /// Gets the spawn locations.
        /// </summary>
		public Transform[] SpawnLocations => _spawnLocations;

		/// <summary>
		/// Spawns a random enemy from the list on the position of the component holding GameObject.
		/// </summary>
		public void SpawnEnemy()
		{
			string enemyName = _enemies.GetRandomObject();

			if (enemyName == default)
				return;

			Transform spawnTransform = GetRandomSpawnLocation();

			PoolableObject obj = _poolingProcessor.GetPooledObject(enemyName);
			obj.transform.position = spawnTransform.position;
			obj.transform.rotation = spawnTransform.rotation;
			obj.gameObject.SetActive(true);
			Enemy enemy = obj.GetComponent<Enemy>();
			_spawnedEnemies.Add(enemy);
			enemy.OnPooled();
		}

        /// <summary>
        /// Gets a random spawn location.
        /// </summary>
        /// <returns>A random spawn transform.</returns>
		public Transform GetRandomSpawnLocation()
		{
			return _spawnLocations[Random.Range(0, _spawnLocations.Length)];
		}

        /// <summary>
        /// Adds an enemy to the spawned enemies list.
        /// </summary>
        /// <param name="enemy">The enemy to add.</param>
		public void AddEnemySpawn(Enemy enemy)
		{
			_spawnedEnemies.Add(enemy);
		}

        /// <summary>
        /// Updates the spawn timer and spawns enemies when conditions are met.
        /// </summary>
		public void Update()
		{
			// Check for enemies that have been disabled and remove them from the spawned enemies list.
			// TODO: Have an event to subscribe to that detects when an enemy has been disabled and remove it automatically.
			for (int i = _spawnedEnemies.Count - 1; i >= 0; i--)
			{
				if (!_spawnedEnemies[i].gameObject.activeInHierarchy)
					_spawnedEnemies.RemoveAt(i);
			}

			if (!_gameStateProcessor.ObjectsPooled || !CanSpawnEnemies || _dayNightProcessor.IsDayTime)
				return;

			if (_spawnedEnemies.Count < _maxEnemies)
				_spawnTimer += Time.deltaTime;
			else
				_spawnTimer = 0;

			if (_spawnTimer >= _timeBetweenSpawns)
			{
				_spawnTimer -= _timeBetweenSpawns;
				SpawnEnemy();
			}
		}

        /// <summary>
        /// Called when a new day starts.
        /// </summary>
		private void OnDayStarted()
		{
			// Recalculate the maximum number of enemies when a new day starts.
			CalculateMaxEnemies();
		}

        /// <summary>
        /// Calculates the maximum number of enemies based on day count and player count.
        /// </summary>
		private void CalculateMaxEnemies()
		{
			// Calculate the maximum number of enemies based on day count and player count.
			_maxEnemies = Mathf.Max(Mathf.Min((int)(_timeProcessor.DayCount + _playerProcessor.Players.Count * 0.1f), _maxTotalEnemies), _minTotalEnemies);
		}

		// Unity Functions.
        /// <summary>
        /// Initializes the enemy spawner on Awake.
        /// </summary>
		private void Awake()
		{
			// Initialize the spawn timer and calculate the total chance of enemies.
			_spawnTimer = _timeBetweenSpawns;
			_enemies.CalculateTotalChance();
			CalculateMaxEnemies();
		}

        /// <summary>
        /// Subscribes to day started event on Start.
        /// </summary>
		private void Start()
		{
			// Subscribe to the day started event.
			_dayNightProcessor.OnDayStarted += OnDayStarted;
		}
	}
}
