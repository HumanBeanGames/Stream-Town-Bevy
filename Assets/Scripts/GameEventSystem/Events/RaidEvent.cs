using Enemies;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Processors;
using UserInterface;
using Units;
using Utils;
using Utils.Pooling;

namespace GameEventSystem.Events
{
    /// <summary>
    /// Represents a raid event where waves of enemies attack the town.
    /// </summary>
    public class RaidEvent : GameEvent
    {
        /// <summary>
        /// Array of pooled enemy names.
        /// </summary>
        protected string[] _pooledEnemyNames;

        /// <summary>
        /// The boss name.
        /// </summary>
        protected string _bossName;

        /// <summary>
        /// The number of waves.
        /// </summary>
        protected int _waves;

        /// <summary>
        /// The current wave.
        /// </summary>
        protected int _currentWave;

        /// <summary>
        /// Whether the boss appears on the last wave.
        /// </summary>
        protected bool _bossOnLastWave = true;

        /// <summary>
        /// The number of enemies per wave.
        /// </summary>
        protected int _enemiesPerWave = 50;

        /// <summary>
        /// List of tracked enemies.
        /// </summary>
        protected List<Enemy> _trackedEnemies;

        /// <summary>
        /// The object pooling processor.
        /// </summary>
        protected ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// The event interface.
        /// </summary>
        private UserInterface_Event _eventInterface;

        /// <summary>
        /// The previous enemy station mask.
        /// </summary>
        protected StationMask _prevEnemyStationMask;

        /// <summary>
        /// The event processor.
        /// </summary>
        protected GameEventProcessor _eventProcessor;

        /// <summary>
        /// The world generation processor.
        /// </summary>
        protected WorldGenProcessor _worldGenProcessor;

        /// <summary>
        /// The player processor.
        /// </summary>
        protected PlayerProcessor _playerProcessor;

        /// <summary>
        /// Whether to force stop the event.
        /// </summary>
        protected bool _forceStop = false;

        /// <summary>
        /// Initializes a new raid event instance.
        /// </summary>
        /// <param name="delay">The delay before the event starts.</param>
        /// <param name="eventDuration">The event duration.</param>
        /// <param name="enemies">The enemy names.</param>
        /// <param name="poolingProcessor">The object pooling processor.</param>
        /// <param name="eventInterface">The event interface.</param>
        /// <param name="eventProcessor">The event processor.</param>
        /// <param name="worldGenProcessor">The world generation processor.</param>
        /// <param name="playerProcessor">The player processor.</param>
        /// <param name="waves">The number of waves.</param>
        /// <param name="enemiesPerWave">The number of enemies per wave.</param>
        /// <param name="boss">The boss name.</param>
        /// <param name="eventType">The event type.</param>
        /// <param name="data">Additional data.</param>
        /// <param name="overrideCurrentEvent">Whether to override the current event.</param>
        /// <param name="timeout">The timeout.</param>
        public RaidEvent(double delay, double eventDuration, string[] enemies, ObjectPoolingProcessor poolingProcessor, UserInterface_Event eventInterface, GameEventProcessor eventProcessor, WorldGenProcessor worldGenProcessor, PlayerProcessor playerProcessor, int waves = 5, int enemiesPerWave = 50, string boss = null, EventType eventType = EventType.MonsterRaid, object data = null, bool overrideCurrentEvent = false, double timeout = -1) : base(delay, eventDuration, eventType, data, overrideCurrentEvent, timeout)
        {
            _pooledEnemyNames = enemies;
            _waves = waves;
            _enemiesPerWave = enemiesPerWave;

            if (boss == null)
                _bossOnLastWave = false;
            else
            {
                _bossOnLastWave = true;
                _bossName = boss;

            }

            _poolingProcessor = poolingProcessor;
            _eventInterface = eventInterface;
            _eventProcessor = eventProcessor;
            _worldGenProcessor = worldGenProcessor;
            _playerProcessor = playerProcessor;

            _trackedEnemies = new List<Enemy>();
        }

        /// <summary>
        /// Sets the enemy names for the raid.
        /// </summary>
        /// <param name="enemies">The enemy names.</param>
        public void SetEnemies(string[] enemies)
        {
            _pooledEnemyNames = enemies;
        }

        /// <summary>
        /// Called when the event starts.
        /// </summary>
        protected override void OnStarted()
        {
            _eventInterface.Slider.gameObject.SetActive(true);
            UpdateSlider();
            _eventInterface.TitleTMP.text = "Raid";
            _eventInterface.DescriptionTMP.text = "";
            _eventInterface.ActivateEventContainer();
            _eventProcessor.StartCoroutine(HandleWaves());

            _worldGenProcessor?.RefreshEnemyCampSpawners();
            _worldGenProcessor?.SetEnemyCampSpawningEnabled(false);
        }

        /// <summary>
        /// Called when the event stops.
        /// </summary>
        protected override void OnStopped()
        {
            _forceStop = true;
            _eventProcessor.StopCoroutine(HandleWaves());

            for (int i = _trackedEnemies.Count - 1; i >= 0; i--)
            {
                _trackedEnemies[i].HealthHandler.SetHealth(0);
            }

            _eventInterface.DeactivateEventContainer();
            _worldGenProcessor?.SetEnemyCampSpawningEnabled(true);
        }

        /// <summary>
        /// Updates the raid event.
        /// </summary>
        /// <param name="currentTime">The current time.</param>
        public override void Update(double currentTime)
        {
            if (_currentWave < _waves - 1 || !_bossOnLastWave)
                return;
            UpdateSlider();
        }

        /// <summary>
        /// Handles the waves of enemies.
        /// </summary>
        /// <returns>The enumerator for the coroutine.</returns>
        protected IEnumerator HandleWaves()
        {
            for (_currentWave = 0; _currentWave < _waves; _currentWave++)
            {
                while (_trackedEnemies.Count > 0)
                    yield return new WaitForEndOfFrame();
                SpawnNewWave();
            }
        }

        /// <summary>
        /// Updates the slider UI.
        /// </summary>
        protected void UpdateSlider()
        {
            if (_currentWave < _waves || !_bossOnLastWave)
            {
                _eventInterface.SliderTMP.text = $"Waves Completed: {_currentWave}/{_waves}";
                _eventInterface.Slider.value = (float)_currentWave / _waves;
            }
            else
            {
                if (_trackedEnemies.Count > 0 && _trackedEnemies[0] != null)
                {
                    _eventInterface.SliderTMP.text = $"Boss HP: {_trackedEnemies[0].HealthHandler.Health}/{_trackedEnemies[0].HealthHandler.MaxHealth}";
                    _eventInterface.Slider.value = _trackedEnemies[0].HealthHandler.HealthPercentage;
                }
            }
        }

        /// <summary>
        /// Spawns a new wave of enemies.
        /// </summary>
        protected void SpawnNewWave()
        {
            if (_forceStop)
                return;

            if (_worldGenProcessor == null || !_worldGenProcessor.CanSpawnRaidEnemies())
            {
                Stop(false);
                return;
            }

            // On waves prior to last wave or there is no final boss.
            if (_currentWave < _waves - 1 || !_bossOnLastWave)
            {
                for (int i = 0; i < _enemiesPerWave; i++)
                {
                    if (!_worldGenProcessor.TryGetRandomEnemyCampSpawnLocation(out Transform spawnLocation))
                    {
                        Stop(false);
                        return;
                    }

                    string enemyName = _pooledEnemyNames[Random.Range(0, _pooledEnemyNames.Length)];
                    PoolableObject go = _poolingProcessor.GetPooledObject(enemyName);
                    Enemy enemy = go.GetComponent<Enemy>();
                    enemy.OnDied += OnEnemyDeath;
                    _trackedEnemies.Add(enemy);
                    enemy.transform.position = spawnLocation.position;
                    enemy.gameObject.SetActive(true);
                }
            }
            else // On Last Wave and Should spawn boss.
            {
                if (!_worldGenProcessor.TryGetRandomEnemyCampSpawnLocation(out Transform spawnLocation))
                {
                    Stop(false);
                    return;
                }

                PoolableObject go = _poolingProcessor.GetPooledObject(_bossName);
                Enemy enemy = go.GetComponent<Enemy>();
                enemy.OnDied += OnEnemyDeath;
                _trackedEnemies.Add(enemy);
                enemy.HealthHandler.SetMaxHealth(Mathf.Max(1000, 50 * (_playerProcessor.PlayerCount() + _playerProcessor.RecruitCount())));
                enemy.gameObject.SetActive(true);
                enemy.transform.position = spawnLocation.position;
            }

            UpdateSlider();
            UpdateUI();
        }

        /// <summary>
        /// Called when an enemy dies.
        /// </summary>
        /// <param name="enemy">The enemy that died.</param>
        protected void OnEnemyDeath(Enemy enemy)
        {
            enemy.OnDied -= OnEnemyDeath;
            _trackedEnemies.Remove(enemy);

            UpdateUI();

            if (_currentWave == _waves)
                Stop(true);
        }

        /// <summary>
        /// Checks if the raid is complete.
        /// </summary>
        protected void CheckComplete()
        {
            if (_currentWave == _waves && _trackedEnemies.Count == 0)
                OnCompleteEvent();
        }

        /// <summary>
        /// Updates the UI.
        /// </summary>
        protected void UpdateUI()
        {
            _eventInterface.DescriptionTMP.text = $"Enemies In Wave: {_trackedEnemies.Count}";
        }
    }
}
