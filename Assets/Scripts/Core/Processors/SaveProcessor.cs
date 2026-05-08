using UnityEngine;
using World.Generation;
using System.Collections.Generic;
using SavingAndLoading.Structs;
using Utils;
using GameResources;
using Units;
using Buildings;
using Character;
using UnityEngine.SceneManagement;
using Twitch.Utils;
using GUIDSystem;
using Sensors;
using PlayerControls;
using System;
using Processors;
using SavingAndLoading;
using Pets.Enumerations;
using TechTree;
using TechTree.ScriptableObjects;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using UserInterface.MainMenu;
using TownGoal;
using TownGoal.Data;
using GameEventSystem;
using Enemies;
using Utils.Pooling;
using SavingAndLoading.SavableObjects;
using Target;
using Reflex.Attributes;
using Reflex.Core;
using System.Threading.Tasks;

namespace Processors
{
	public partial class SaveProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// Runtime data for save data.
		/// Assigned in InjectRuntimeData.
		/// </summary>
		private SaveRuntimeData _saveRuntimeData;

		[Inject] private WorldGenProcessor _worldGenProcessor;
		[Inject] private GameSettings _gameSettings;
		[Inject] private ObjectPoolingProcessor _poolingProcessor;
		[Inject] private TownResourceProcessor _townResourceProcessor;
		[Inject] private TimeProcessor _timeProcessor;
		[Inject] private TechTreeProcessor _techTreeProcessor;
		[Inject] private GameEventProcessor _gameEventProcessor;
		[Inject] private PlayerProcessor _playerProcessor;
		[Inject] private GUIDProcessor _guidProcessor;
		[Inject] private TownGoalProcessor _townGoalProcessor;
		[Inject] private SeasonProcessor _seasonProcessor;
		[Inject] private SaveSettings _saveSettings;
		[Inject] private TimeSettings _timeProcessorScriptable;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		private const float _frameBudgetSeconds = 0.0035f;

		private void EscapePressed()
		{
			SceneManager.LoadScene(0);
		}

		public float AutosaveTime
		{
			get => _saveRuntimeData.AutosaveTime;
			set => SetAutosaveTime(value);
		}

		public bool Autosave
		{
			get => _saveRuntimeData.Autosave;
			set => _saveRuntimeData.Autosave = value;
		}

		public void SetAutosaveTime(float time)
		{
			_saveRuntimeData.AutosaveTime = time;
			_saveRuntimeData.Autosave = _saveRuntimeData.AutosaveTime <= 0.0f ? false : true;
		}

		/// <summary>
		/// Saves the entire game
		/// </summary>
		public void SaveGame()
		{
			_debugProcessor.Log(DebugLogCategory.GameIO, "Saving Game");
			WorldGenSaveData worldGenSave = GetWorldGenerationData();
			List<BuildingSaveData> buildings = GetBuildingsData();
			List<EnemySaveData> enemySaveData = GetEnemySaveData();
			WorldSaveData worldSaveData = GetWorldData();

			List<PlayerSaveData> playerSaveData = GetPlayerSaveData();

			playerSaveData = SetPlayerTargetGUIDs(playerSaveData); // Sets target GUIDS
			enemySaveData = SetEnemyTargetGUIDs(enemySaveData); // Sets enemys GUIDS

			SaveGameData gameSave = new SaveGameData(worldGenSave, buildings, enemySaveData, worldSaveData);
			SavePlayersData playersSave = new SavePlayersData(playerSaveData);
			GameIO.SaveGameData(gameSave);
			GameIO.SavePlayersData(playersSave);
		}

		/// <summary>
		/// Sets the players GUIDs for targets and stations
		/// </summary>
		/// <param name="data">A list of PlayerSaveData to set GUIDs for</param>
		/// <returns>An updated list of PlayerSaveData with GUIDs set</returns>
		private List<PlayerSaveData> SetPlayerTargetGUIDs(List<PlayerSaveData> data)
		{
			List<Player> players = _saveRuntimeData.Players;
			for (int i = 0; i < data.Count; i++)
			{
				// Sets players target
				Target.Targetable target = players[i].TargetSensor.CurrentTarget;
				if (target != null)
					data[i].SetTargetGUID(target.GUIDComponent.GUID);
				else
					data[i].SetTargetGUID(0);

				// Sets players station
				Station station = players[i].StationSensor.CurrentStation;
				if (target != null)
					data[i].SetStationGUID(station.GUIDComponent.GUID);
				else
					data[i].SetStationGUID(0);
			}

			return data;
		}

		private List<EnemySaveData> SetEnemyTargetGUIDs(List<EnemySaveData> data)
		{
			List<Enemies.Enemy> enemies = _saveRuntimeData.Enemies;
			for (int i = 0; i < data.Count; i++)
			{
				// Sets enemies target
				TargetSensor targetSensor = enemies[i].TargetSensor;
				if (targetSensor.CurrentTarget != null)
					data[i].SetTargetGUID(targetSensor.CurrentTarget.GUIDComponent.GUID);

				// Sets enemies station
				StationSensor stationSensor = enemies[i].StationSensor;
				if (stationSensor != null)
					if (stationSensor.CurrentStation != null)
						data[i].SetTargetGUID(stationSensor.CurrentStation.GUIDComponent.GUID);
			}

			return data;
		}

		/// <summary>
		/// Gathers all the enemies data needed for saving
		/// </summary>
		/// <returns>A list of enemy data structs to be saved</returns>
		private List<EnemySaveData> GetEnemySaveData()
		{
			List<EnemySaveData> enemySaveData = new List<EnemySaveData>();
			List<Enemies.Enemy> enemies = new List<Enemies.Enemy>();
			for (int i = 0; i < (int)EnemyType.Count; i++)
			{
				List<PoolableObject> objs = _poolingProcessor.GetAllActivePooledObjectsOfType(((EnemyType)i).ToString());
				for (int o = 0; o < objs.Count; o++)
				{
					enemySaveData.Add((EnemySaveData)((SaveableEnemy)objs[o].SaveableObject).SaveData());
					enemies.Add(((SaveableEnemy)objs[o].SaveableObject).Enemy);
				}
			}
			_saveRuntimeData.InitializeEnemies(enemies);
			return enemySaveData;
		}

		/// <summary>
		/// Gathers all the players data needed for saving
		/// </summary>
		/// <returns>A list of player data structs to be saved</returns>
		private List<PlayerSaveData> GetPlayerSaveData()
		{
			List<PlayerSaveData> playerSaveDatas = new List<PlayerSaveData>();

			List<PoolableObject> players = _poolingProcessor.GetAllActivePooledObjectsOfType("Player");
			List<Player> playerList = new List<Player>();

			for (int i = 0; i < players.Count; i++)
			{
				playerSaveDatas.Add((PlayerSaveData)((SaveablePlayer)players[i].SaveableObject).SaveData());
				playerList.Add(((SaveablePlayer)players[i].SaveableObject).RoleHandler.Player);
			}

			_saveRuntimeData.InitializePlayers(playerList);
			return playerSaveDatas;
		}

		/// <summary>
		/// Gathers all world generation data needed for saving
		/// </summary>
		/// <returns>The struct of world generation data</returns>
		private WorldGenSaveData GetWorldGenerationData()
		{
			WorldGenSaveData worldGenData = new WorldGenSaveData();

			// The generated mesh 
			worldGenData.MapMesh = new MeshSaveData(_worldGenProcessor.GeneratedMesh);

			// The generated resources
			List<ResourceSaveData> resources = new List<ResourceSaveData>();

			for (int i = 0; i < (int)ResourceType.Count; i++)
			{
				if ((ResourceType)i != ResourceType.Fish)
				{
					List<PoolableObject> objs = _poolingProcessor.GetAllActivePooledObjectsOfType(((ResourceType)i).ToString());
					for (int o = 0; o < objs.Count; o++)
					{
						resources.Add((ResourceSaveData)((SaveableResource)objs[o].SaveableObject).SaveData());
					}
				}
			}
			worldGenData.Resources = resources;

			// The generated foliage
			List<FoliageSaveData> foliage = new List<FoliageSaveData>();

			for (int i = 0; i < (int)FoliageSaveType.Count; i++)
			{
				List<PoolableObject> objs = _poolingProcessor.GetAllActivePooledObjectsOfType(((FoliageSaveType)i).ToString());

				for (int o = 0; o < objs.Count; o++)
				{
					foliage.Add((FoliageSaveData)((SaveablFoliage)objs[o].SaveableObject).SaveData());
				}
			}

			worldGenData.Foliage = foliage;

			// The generated enemy camps
			List<EnemyCampSaveData> camps = new List<EnemyCampSaveData>();

			List<PoolableObject> campObjects = _poolingProcessor.GetAllActivePooledObjectsOfType(SaveItem.EnemyCamp_Goblin);
			for (int i = 0; i < campObjects.Count; i++)
			{
				EnemyCampSaveData enemyCampSaveData = new EnemyCampSaveData(campObjects[i].transform, ((SaveableEnemyCamp)campObjects[i].SaveableObject).HealthHandler.Health, _guidProcessor.CreateGUIDandAddToDictionary(campObjects[i]));
				camps.Add(enemyCampSaveData);
			}
			worldGenData.EnemyCamps = camps;

			return worldGenData;
		}

		/// <summary>
		/// Gathers all the buildings data needed for saving
		/// </summary>
		/// <returns>A list of building data structs to be saved to file</returns>
		private List<BuildingSaveData> GetBuildingsData()
		{
			List<BuildingSaveData> buildings = new List<BuildingSaveData>();
			for (int i = 0; i < (int)BuildingType.Count; i++)
			{
				List<PoolableObject> objs = _poolingProcessor.GetAllActivePooledObjectsOfType(((BuildingType)i).ToString());
				if (objs != null)
					for (int o = 0; o < objs.Count; o++)
						buildings.Add((BuildingSaveData)((SaveableBuilding)objs[o].SaveableObject).SaveData());

			}
			return buildings;
		}

		/// <summary>
		/// Gathers the world data needed for saving
		/// </summary>
		/// <returns>The struct of world data to be saved to file</returns>
		private WorldSaveData GetWorldData()
		{
			WorldSaveData worldSaveData = new WorldSaveData();
			worldSaveData.WoodResourceAmount = _townResourceProcessor.GetResourceAmount(Resource.Wood);
			worldSaveData.OreResourceAmount = _townResourceProcessor.GetResourceAmount(Resource.Ore);
			worldSaveData.GoldResourceAmount = _townResourceProcessor.GetResourceAmount(Resource.Gold);
			worldSaveData.FoodResourceAmount = _townResourceProcessor.GetResourceAmount(Resource.Food);
			worldSaveData.WorldAgeInSeconds = _timeProcessor.WorldTimePassed;


			// Tech Tree
			TechTreeSaveData techTree = new TechTreeSaveData();
			techTree.UnlockedTechs = _techTreeProcessor.GetUnlockedTechStates();

			if (_techTreeProcessor.CurrentTech != null)
			{
				techTree.CurrentTechName = _techTreeProcessor.CurrentTech.name;
				List<ObjectiveSaveData> objectives = new List<ObjectiveSaveData>();
				Node_SO currentNode = _techTreeProcessor.CurrentTech;
				Goal goal = _townGoalProcessor.CurrentGoals[0];
				List<Objective> objs = new List<Objective>();

				foreach (KeyValuePair<Objective, bool> obj in goal.ObjectivesStatuses)
				{
					objectives.Add(new ObjectiveSaveData(obj.Key.Amount, obj.Key.RequiredAmount));
				}

				techTree.CurrentTechData = objectives;
				techTree.TechAvailable = true;
			}
			else
				techTree.TechAvailable = false;

			worldSaveData.IsCurrentRuler = _playerProcessor.GetRuler() == null ? false : true;

			worldSaveData.TimeUntillNextRulerVote = _gameEventProcessor.TimeTillRulerVote;
			if (worldSaveData.IsCurrentRuler)
				worldSaveData.RulerName = _playerProcessor.GetRuler().TwitchUser.Username;

			worldSaveData.TechTree = techTree;
			return worldSaveData;
		}


		/// <summary>
		/// Starts the asynchronous load pipeline.
		/// </summary>
		public async void LoadGame()
		{
			await DelayedLoadGameAsync();
		}

		/// <summary>
		/// Loads the game from file using a Task-based staged pipeline (no coroutines).
		/// </summary>
		private async Task DelayedLoadGameAsync(Action<float, string> progressReporter = null)
		{
			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();
			progressReporter?.Invoke(0.02f, "Reading save files...");

			await Task.Yield();

			stopwatch.Restart();
			Task<SaveGameData> gameDataTask = GameIO.LoadGameDataAsync();
			Task<SavePlayersData> playersDataTask = GameIO.LoadPlayersDataAsync();

			while (!gameDataTask.IsCompleted || !playersDataTask.IsCompleted)
				await Task.Yield();

			if (gameDataTask.IsFaulted)
				throw gameDataTask.Exception;

			if (playersDataTask.IsFaulted)
				throw playersDataTask.Exception;

			SaveGameData gameData = gameDataTask.Result;
			SavePlayersData playersData = playersDataTask.Result;
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] File I/O and JSON deserialization: {stopwatch.ElapsedMilliseconds}ms");
			progressReporter?.Invoke(0.08f, "Applying terrain mesh...");

			WorldGenSaveData genData = gameData.WorldGenData;
			List<BuildingSaveData> buildings = gameData.BuildingSaveData;
			WorldSaveData worldData = gameData.WorldSaveData;
			List<EnemySaveData> enemies = gameData.EnemySaveData;

			List<PlayerSaveData> playerSaveDatas = playersData.PlayerSaveDatas;

			// World generation mesh
			stopwatch.Restart();
			Mesh meshData = genData.MapMesh.GetMeshFromData();
			_worldGenProcessor.SetMesh(meshData);
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Mesh reconstruction and application: {stopwatch.ElapsedMilliseconds}ms");

			// Parallel resource and foliage spawning
			_debugProcessor.Log(DebugLogCategory.GameIO, "[SAVE LOAD] Starting parallel resource and foliage spawning");
			UserInterface.MainMenu.ParallelProgressReporter.Reset();
			UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("Resources", 0.5f);
			UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("Foliage", 0.5f);

			System.Diagnostics.Stopwatch resourceFoliageStopwatch = System.Diagnostics.Stopwatch.StartNew();

			Task resourceTask = SpawnResourcesParallelAsync(genData, progressReporter);
			Task foliageTask = SpawnFoliageParallelAsync(genData, progressReporter);

			await Task.WhenAll(resourceTask, foliageTask);

			resourceFoliageStopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Parallel resource and foliage spawning: {resourceFoliageStopwatch.ElapsedMilliseconds}ms");

			// Enemy camps
			progressReporter?.Invoke(0.42f, "Spawning enemy camps...");
			stopwatch.Restart();
			await SpawnEnemyCampsAsync(genData);
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Enemy camp spawning ({genData.EnemyCamps.Count} objects): {stopwatch.ElapsedMilliseconds}ms");

			// Buildings
			stopwatch.Restart();
			progressReporter?.Invoke(0.48f, "Spawning buildings...");
			List<UpdateGraphBounds> buildingsToUpdate = new List<UpdateGraphBounds>();
			await SpawnBuildingsAsync(buildings, progressReporter, buildingsToUpdate);
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Building spawning ({buildings.Count} objects): {stopwatch.ElapsedMilliseconds}ms");

			// Enemies
			stopwatch.Restart();
			progressReporter?.Invoke(0.64f, "Spawning enemies...");
			await SpawnEnemiesAsync(enemies, progressReporter);
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Enemy spawning ({enemies.Count} objects): {stopwatch.ElapsedMilliseconds}ms");

			// Players
			stopwatch.Restart();
			progressReporter?.Invoke(0.72f, "Spawning players...");
			await SpawnPlayersAsync(playerSaveDatas);
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Player spawning ({playerSaveDatas.Count} objects): {stopwatch.ElapsedMilliseconds}ms");

			// Apply world state
			progressReporter?.Invoke(0.82f, "Applying world state...");
			stopwatch.Restart();

			_townResourceProcessor.SetResourceAmount(Resource.Wood, worldData.WoodResourceAmount);
			_townResourceProcessor.SetResourceAmount(Resource.Ore, worldData.OreResourceAmount);
			_townResourceProcessor.SetResourceAmount(Resource.Food, worldData.FoodResourceAmount);
			_townResourceProcessor.SetResourceAmount(Resource.Gold, worldData.GoldResourceAmount);
			_timeProcessor.WorldTimePassed = worldData.WorldAgeInSeconds;
			_timeProcessor.CalculateDayCount(worldData.WorldAgeInSeconds);
			_seasonProcessor.SetSeasonByTimePassed(_timeProcessor.WorldTimePassed);
			_techTreeProcessor.SetUnlockedTechStates(worldData.TechTree.UnlockedTechs);
			if (worldData.TechTree.TechAvailable)
			{
				_techTreeProcessor.StartGoalFromNodeName(worldData.TechTree.CurrentTechName);
			}
			else
			{
				_techTreeProcessor.RequestedTechVoteDelay = 20f;
				_techTreeProcessor.RequestStartTechVote = true;
			}

			_gameEventProcessor.TimeTillRulerVote = worldData.TimeUntillNextRulerVote;

			if (worldData.IsCurrentRuler && _playerProcessor.PlayerExistsByNameToLower(worldData.RulerName, out int index))
				_playerProcessor.SetRuler(_playerProcessor.GetPlayer(index));
			else
			{
				_gameEventProcessor.CanStartNewRulerVote = true;
				_gameEventProcessor.TimeTillRulerVote = _gameEventProcessor.RulerVoteMinTime;
			}
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] World data application: {stopwatch.ElapsedMilliseconds}ms");

			// Finalize building graph bounds
			progressReporter?.Invoke(0.92f, "Finalizing world graph...");
			stopwatch.Restart();
			await FinalizeGraphBoundsAsync(buildingsToUpdate, progressReporter);
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Graph bounds update ({buildingsToUpdate.Count} buildings): {stopwatch.ElapsedMilliseconds}ms");

			progressReporter?.Invoke(1f, "Save load complete");
		}

		private async Task SpawnEnemyCampsAsync(WorldGenSaveData genData)
		{
			float frameStartTime = Time.realtimeSinceStartup;
			for (int i = 0; i < genData.EnemyCamps.Count; i++)
			{
				((SaveableEnemyCamp)((_poolingProcessor.GetPooledObject("EnemyCamp_Goblin", false)).SaveableObject)).LoadData((object)genData.EnemyCamps[i]);

				if (ShouldYieldFrame(ref frameStartTime))
					await Task.Yield();
			}

			_worldGenProcessor.RefreshEnemyCampSpawners();
		}

		private async Task SpawnBuildingsAsync(List<BuildingSaveData> buildings, Action<float, string> progressReporter, List<UpdateGraphBounds> buildingsToUpdate)
		{
			float frameStartTime = Time.realtimeSinceStartup;
			for (int i = 0; i < buildings.Count; i++)
			{
				var building = _poolingProcessor.GetPooledObject(buildings[i].BuildingType, false);
				((SaveableBuilding)((building).SaveableObject)).LoadData((object)buildings[i]);

				UpdateGraphBounds ugb = building.GetComponent<UpdateGraphBounds>();
				if (ugb)
					buildingsToUpdate.Add(ugb);

				if ((i + 1) % 50 == 0)
				{
					float stepProgress = buildings.Count > 0 ? (i + 1f) / buildings.Count : 1f;
					progressReporter?.Invoke(0.48f + (stepProgress * 0.16f), $"Spawning buildings ({i + 1}/{buildings.Count})...");
				}

				if (ShouldYieldFrame(ref frameStartTime))
					await Task.Yield();
			}
		}

		private async Task SpawnEnemiesAsync(List<EnemySaveData> enemies, Action<float, string> progressReporter)
		{
			float frameStartTime = Time.realtimeSinceStartup;
			for (int i = 0; i < enemies.Count; i++)
			{
				PoolableObject temp = _poolingProcessor.GetPooledObject((enemies[i].EnemyType.ToString()), false);
				((SaveableEnemy)temp.SaveableObject).LoadData((object)enemies[i]);

				if ((i + 1) % 30 == 0)
				{
					float stepProgress = enemies.Count > 0 ? (i + 1f) / enemies.Count : 1f;
					progressReporter?.Invoke(0.64f + (stepProgress * 0.08f), $"Spawning enemies ({i + 1}/{enemies.Count})...");
				}

				if (ShouldYieldFrame(ref frameStartTime))
					await Task.Yield();
			}
		}

		private async Task SpawnPlayersAsync(List<PlayerSaveData> playerSaveDatas)
		{
			float frameStartTime = Time.realtimeSinceStartup;
			for (int i = 0; i < playerSaveDatas.Count; i++)
			{
				Player player = playerSaveDatas[i].ToPlayer(playerSaveDatas[i].GUID, playerSaveDatas[i].TargetGUID, playerSaveDatas[i].StationGUID, _gameSettings, _poolingProcessor);
				_playerProcessor.AddExistingPlayer(player);

				if (ShouldYieldFrame(ref frameStartTime))
					await Task.Yield();
			}
		}

		private async Task FinalizeGraphBoundsAsync(List<UpdateGraphBounds> buildingsToUpdate, Action<float, string> progressReporter)
		{
			float frameStartTime = Time.realtimeSinceStartup;
			for (int i = 0; i < buildingsToUpdate.Count; i++)
			{
				buildingsToUpdate[i].SetGraphBounds();

				if ((i + 1) % 25 == 0)
				{
					float stepProgress = buildingsToUpdate.Count > 0 ? (i + 1f) / buildingsToUpdate.Count : 1f;
					progressReporter?.Invoke(0.92f + (stepProgress * 0.08f), $"Finalizing world graph ({i + 1}/{buildingsToUpdate.Count})...");
				}

				if (ShouldYieldFrame(ref frameStartTime))
					await Task.Yield();
			}
		}

		private bool ShouldYieldFrame(ref float frameStartTime)
		{
			if (Time.realtimeSinceStartup - frameStartTime >= _frameBudgetSeconds)
			{
				frameStartTime = Time.realtimeSinceStartup;
				return true;
			}

			return false;
		}

		private async Task SpawnResourcesParallelAsync(WorldGenSaveData genData, Action<float, string> progressReporter)
		{
			UserInterface.MainMenu.ParallelProgressReporter.UpdateTrack("Resources", 0f, "Spawning resources...");
			await Task.Yield();

			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();
			float frameStartTime = Time.realtimeSinceStartup;

			for (int i = 0; i < genData.Resources.Count; i++)
			{
				((SaveableResource)((_poolingProcessor.GetPooledObject(genData.Resources[i].ResourceType, false)).SaveableObject)).LoadData((object)genData.Resources[i]);

				if ((i + 1) % 150 == 0)
				{
					float stepProgress = genData.Resources.Count > 0 ? (i + 1f) / genData.Resources.Count : 1f;
					progressReporter?.Invoke(0.16f + (stepProgress * 0.12f), $"Spawning resources ({i + 1}/{genData.Resources.Count})...");
				}

				if (ShouldYieldFrame(ref frameStartTime))
					await Task.Yield();
			}

			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Resource spawning ({genData.Resources.Count} objects): {stopwatch.ElapsedMilliseconds}ms");
			UserInterface.MainMenu.ParallelProgressReporter.UpdateTrack("Resources", 1f, "Complete");
		}

		private async Task SpawnFoliageParallelAsync(WorldGenSaveData genData, Action<float, string> progressReporter)
		{
			UserInterface.MainMenu.ParallelProgressReporter.UpdateTrack("Foliage", 0f, "Spawning foliage...");
			await Task.Yield();

			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();
			float frameStartTime = Time.realtimeSinceStartup;

			for (int i = 0; i < genData.Foliage.Count; i++)
			{
				((SaveablFoliage)((_poolingProcessor.GetPooledObject(genData.Foliage[i].FoliageType, false)).SaveableObject)).LoadData((object)genData.Foliage[i]);

				if ((i + 1) % 200 == 0)
				{
					float stepProgress = genData.Foliage.Count > 0 ? (i + 1f) / genData.Foliage.Count : 1f;
					progressReporter?.Invoke(0.3f + (stepProgress * 0.12f), $"Spawning foliage ({i + 1}/{genData.Foliage.Count})...");
				}

				if (ShouldYieldFrame(ref frameStartTime))
					await Task.Yield();
			}

			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.GameIO, $"[LOAD TIME] Foliage spawning ({genData.Foliage.Count} objects): {stopwatch.ElapsedMilliseconds}ms");
			UserInterface.MainMenu.ParallelProgressReporter.UpdateTrack("Foliage", 1f, "Complete");
		}

		public void Initialize()
		{
			if (_saveRuntimeData == null)
				throw new InvalidOperationException("SaveProcessor: SaveRuntimeData has not been installed.");
		}

		public void Process()
		{
			UpdateAutosave(Time.deltaTime);
			if (ShouldAutosave())
			{
				ResetAutosaveTimer();
				SaveGame();
			}
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// SaveProcessor does not have scene-specific settings to refresh
		}

		public void InstallBindings(Reflex.Core.ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		/// <summary>
		/// Injects the SaveRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(Reflex.Core.ContainerBuilder containerBuilder)
		{
			if (_saveRuntimeData != null)
				throw new InvalidOperationException("SaveProcessor: SaveRuntimeData has already been installed.");

			_saveRuntimeData = new SaveRuntimeData();
			containerBuilder.AddSingleton(_saveRuntimeData);
		}

		private void UpdateAutosave(float deltaTime)
		{
			if (_saveRuntimeData.Autosave)
			{
				if (_saveRuntimeData.TimeElapsed >= _saveRuntimeData.AutosaveTime)
				{
					_saveRuntimeData.TimeElapsed = 0.0f;
				}
				else
				{
					_saveRuntimeData.TimeElapsed += deltaTime;
				}
			}
		}

		private bool ShouldAutosave()
		{
			return _saveRuntimeData.Autosave && _saveRuntimeData.TimeElapsed >= _saveRuntimeData.AutosaveTime;
		}

		private void ResetAutosaveTimer()
		{
			_saveRuntimeData.TimeElapsed = 0.0f;
		}
	}
}
