using Managers;
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
		[SerializeField]
		private int _minTotalEnemies = 3;

		[SerializeField]
		private int _maxTotalEnemies = 50;

		[SerializeField]
		private float _timeBetweenSpawns = 25;

		[SerializeField]
		private ChanceObjectList<string> _enemies;

		[SerializeField]
		private List<Enemy> _spawnedEnemies = new List<Enemy>();

		[SerializeField]
		private Transform[] _spawnLocations;

		[Inject] private ObjectPoolingManager _poolingManager;
		[Inject] private DayAndNightManager _dayNightManager;
		[Inject] private TimeManager _timeManager;
		[Inject] private PlayerManager _playerManager;

		private float _spawnTimer = 0;

		private int _maxEnemies = 2;
		public bool CanSpawnEnemies { get; set; } = true;

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

			PoolableObject obj = _poolingManager.GetPooledObject(enemyName);
			obj.transform.position = spawnTransform.position;
			obj.transform.rotation = spawnTransform.rotation;
			obj.gameObject.SetActive(true);
			Enemy enemy = obj.GetComponent<Enemy>();
			_spawnedEnemies.Add(enemy);
			enemy.OnPooled();
		}

		public Transform GetRandomSpawnLocation()
		{
			return _spawnLocations[Random.Range(0, _spawnLocations.Length)];
		}

		public void AddEnemySpawn(Enemy enemy)
		{
			_spawnedEnemies.Add(enemy);
		}

		public void Update()
		{
			// Check for enemies that have been disabled and remove them from the spawned enemies list.
			// TODO: Have an event to subscribe to that detects when an enemy has been disabled and remove it automatically.
			for (int i = _spawnedEnemies.Count - 1; i >= 0; i--)
			{
				if (!_spawnedEnemies[i].gameObject.activeInHierarchy)
					_spawnedEnemies.RemoveAt(i);
			}

			if (!GameStateManager.ObjectsPooled || !CanSpawnEnemies || _dayNightManager.IsDayTime)
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

		private void OnDayStarted()
		{
			CalculateMaxEnemies();
		}

		private void CalculateMaxEnemies()
		{
			_maxEnemies = Mathf.Max(Mathf.Min((int)(_timeManager.DayCount + _playerManager.Players.Count * 0.1f), _maxTotalEnemies), _minTotalEnemies);
		}

		// Unity Functions.
		private void Awake()
		{
			_spawnTimer = _timeBetweenSpawns;
			_enemies.CalculateTotalChance();
			CalculateMaxEnemies();
		}

		private void Start()
		{
			_dayNightManager.OnDayStarted += OnDayStarted;
		}
	}
}