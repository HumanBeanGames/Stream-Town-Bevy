using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Buildings;
using Character;
using Enemies;
using GameEventSystem;
using GameEventSystem.Events.Voting;
using GameResources;
using Pets;
using Pets.Enumerations;
using Reflex.Attributes;
using Reflex.Core;
using SavingAndLoading;
using SavingAndLoading.SavableObjects;
using SavingAndLoading.Structs;
using ScriptablesProcessorInfrastructure;
using Sensors;
using Target;
using TechTree.ScriptableObjects;
using TownGoal;
using TownGoal.Data;
using Twitch;
using Twitch.Commands;
using Twitch.Utils;
using TwitchLib.Client.Enums;
using UnityEngine;
using UserInterface.MainMenu;
using Utils;
using Utils.Pooling;
using World.Generation;

namespace Processors
{
	/// <summary>
	/// Coordinates snapshot capture and world restoration. All mutable operation
	/// state lives in SaveRuntimeData and all file access is delegated to
	/// ISaveStorage.
	/// </summary>
	public partial class SaveProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		private const float FrameBudgetSeconds = 0.0035f;

		private SaveRuntimeData _saveRuntimeData;

		[Inject] private ISaveStorage _saveStorage;
		[Inject] private SaveDataMapper _mapper;
		[Inject] private WorldGenProcessor _worldGenProcessor;
		[Inject] private GameSettings _gameSettings;
		[Inject] private ObjectPoolingProcessor _poolingProcessor;
		[Inject] private TownResourceProcessor _townResourceProcessor;
		[Inject] private TimeProcessor _timeProcessor;
		[Inject] private TechTreeProcessor _techTreeProcessor;
		[Inject] private GameEventProcessor _gameEventProcessor;
		[Inject] private PlayerProcessor _playerProcessor;
		[Inject] private RoleProcessor _roleProcessor;
		[Inject] private GUIDProcessor _guidProcessor;
		[Inject] private TownGoalProcessor _townGoalProcessor;
		[Inject] private SeasonProcessor _seasonProcessor;
		[Inject] private ResourceProcessor _resourceProcessor;
		[Inject] private FoliageProcessor _foliageProcessor;
		[Inject] private BuildingProcessor _buildingProcessor;
		[Inject] private ResourceGenSettings _resourceGenerationSettings;
		[Inject] private WaterResourceGenSettings _waterResourceGenerationSettings;
		[Inject] private FoliageGenSettings _foliageGenerationSettings;
		[Inject] private WaterFoliageGenSettings _waterFoliageGenerationSettings;
		[Inject] private DebugProcessor _debugProcessor;

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

		public bool IsBusy => _saveRuntimeData.IsBusy;
		public bool HasSaveGame => _saveStorage != null && _saveStorage.SaveExists;
		public string SavePath => _saveStorage?.SavePath;

		public event Action<SaveOperationState, float, string> OperationChanged
		{
			add => _saveRuntimeData.OperationChanged += value;
			remove => _saveRuntimeData.OperationChanged -= value;
		}

		public void SetAutosaveTime(float time)
		{
			_saveRuntimeData.AutosaveTime = Mathf.Max(0f, time);
			_saveRuntimeData.Autosave = _saveRuntimeData.AutosaveTime > 0f;
			_saveRuntimeData.TimeElapsed = 0f;
		}

		/// <summary>
		/// UnityEvent-compatible save entry point.
		/// </summary>
		public async void SaveGame()
		{
			try
			{
				await SaveGameAsync();
			}
			catch (Exception exception)
			{
				_debugProcessor.LogError(DebugLogCategory.GameIO, $"Save failed: {exception}");
			}
		}

		public async Task SaveGameAsync(CancellationToken cancellationToken = default)
		{
			if (_saveRuntimeData.IsBusy)
				throw new InvalidOperationException("A save or load operation is already in progress.");

			_saveRuntimeData.TimeElapsed = 0f;
			_saveRuntimeData.Begin(SaveOperationState.Saving, "Capturing world state...");
			try
			{
				System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();
				SaveFileData saveData = CaptureSaveFile();
				long captureMilliseconds = stopwatch.ElapsedMilliseconds;
				_saveRuntimeData.Report(0.7f, "Writing save file...");
				await _saveStorage.WriteAsync(saveData, cancellationToken);
				long totalMilliseconds = stopwatch.ElapsedMilliseconds;
				_saveRuntimeData.Complete("Game saved");
				_debugProcessor.Log(
					DebugLogCategory.GameIO,
					$"Saved schema {saveData.SchemaVersion} to {_saveStorage.SavePath} " +
					$"(capture {captureMilliseconds} ms, write {totalMilliseconds - captureMilliseconds} ms, total {totalMilliseconds} ms)");
			}
			catch (Exception exception)
			{
				_saveRuntimeData.Fail(exception.Message);
				throw;
			}
		}

		/// <summary>
		/// UnityEvent-compatible load entry point. Scene-driven loads should be
		/// requested by the menu; Coordinator invokes LoadGameAsync during world
		/// bootstrap.
		/// </summary>
		public async void LoadGame()
		{
			try
			{
				await LoadGameAsync(LoadingProgressReporter.Report);
			}
			catch (Exception exception)
			{
				_debugProcessor.LogError(DebugLogCategory.GameIO, $"Load failed: {exception}");
			}
		}

		public async Task LoadGameAsync(
			Action<float, string> progressReporter = null,
			CancellationToken cancellationToken = default)
		{
			if (_saveRuntimeData.IsBusy)
				throw new InvalidOperationException("A save or load operation is already in progress.");

			_saveRuntimeData.Begin(SaveOperationState.Loading, "Preparing object pools...");
			try
			{
				await _poolingProcessor.InitializePooling(
					(progress, status) => ReportLoadProgress(progress * 0.03f, status, progressReporter),
					cancellationToken);
				await _poolingProcessor.PrewarmPoolsAsync(
					(progress, status) => ReportLoadProgress(0.03f + progress * 0.04f, status, progressReporter),
					cancellationToken);

				ReportLoadProgress(0.08f, "Reading save file...", progressReporter);
				SaveFileData saveFile = await _saveStorage.ReadAsync(cancellationToken);
				Normalize(saveFile);
				_debugProcessor.Log(
					DebugLogCategory.GameIO,
					$"Snapshot contains {saveFile.Players.PlayerSaveDatas.Count} players, " +
					$"{saveFile.Game.BuildingSaveData.Count} buildings, {saveFile.Game.EnemySaveData.Count} enemies, " +
					$"and tech vote={saveFile.Game.WorldSaveData.TechTree.TechVote.Exists}.");
				PrepareRuntimeForLoad();

				SaveGameData gameData = saveFile.Game;
				WorldGenSaveData generationData = gameData.WorldGenData;
				ReportLoadProgress(0.12f, "Restoring terrain...", progressReporter);
				if (generationData.HasTerrainSeed)
					_worldGenProcessor.RestoreTerrainFromSeed(generationData.TerrainSeed, generationData.TerrainGeneratorVersion);
				else
					_worldGenProcessor.SetMesh(_mapper.RestoreMesh(generationData.MapMesh));

				ReportLoadProgress(0.18f, "Restoring resources and foliage...", progressReporter);
				RestoreResources(generationData.Resources, saveFile.SchemaVersion);
				RestoreFoliage(generationData.Foliage, saveFile.SchemaVersion);
				await Task.Yield();

				ReportLoadProgress(0.28f, "Restoring enemy camps...", progressReporter);
				await RestoreEnemyCampsAsync(generationData.EnemyCamps, cancellationToken);

				ReportLoadProgress(0.36f, "Restoring buildings...", progressReporter);
				List<(BuildingBase building, BuildingSaveData data)> loadedBuildings =
					await RestoreBuildingsAsync(gameData.BuildingSaveData, progressReporter, cancellationToken);

				ReportLoadProgress(0.52f, "Restoring world state...", progressReporter);
				RestoreWorldState(gameData.WorldSaveData);
				RestoreBuildingLevels(loadedBuildings);
				RestoreTownResources(gameData.WorldSaveData);

				// Navigation must exist before any AIPath/Seeker component is activated.
				ReportLoadProgress(0.6f, "Rebuilding navigation...", progressReporter);
				_worldGenProcessor.BuildNavigationForLoadedWorld();

				ReportLoadProgress(0.66f, "Restoring enemies...", progressReporter);
				List<(Enemy enemy, EnemySaveData data)> loadedEnemies =
					await RestoreEnemiesAsync(gameData.EnemySaveData, progressReporter, cancellationToken);

				ReportLoadProgress(0.74f, "Restoring players...", progressReporter);
				List<(Player player, PlayerSaveData data)> loadedPlayers =
					await RestorePlayersAsync(saveFile.Players.PlayerSaveDatas, progressReporter, cancellationToken);
				RestoreEnemyHealth(loadedEnemies);

				ReportLoadProgress(0.9f, "Restoring object references and events...", progressReporter);
				RestoreRuler(gameData.WorldSaveData);
				RestoreObjectReferences(loadedPlayers, loadedEnemies);
				RestoreTechVote(gameData.WorldSaveData.TechTree.TechVote, loadedPlayers);

				_worldGenProcessor.CompleteLoadedWorld();

				ReportLoadProgress(1f, "Save load complete", progressReporter);
				_saveRuntimeData.Complete("Save load complete");
				_debugProcessor.Log(DebugLogCategory.GameIO, $"Loaded schema {saveFile.SchemaVersion} from {_saveStorage.SavePath}");
			}
			catch (Exception exception)
			{
				_saveRuntimeData.Fail(exception.Message);
				throw;
			}
		}

		private SaveFileData CaptureSaveFile()
		{
			WorldGenSaveData worldGeneration = CaptureWorldGeneration();
			List<BuildingSaveData> buildings = CaptureBuildings();
			List<EnemySaveData> enemies = CaptureEnemies();
			WorldSaveData world = CaptureWorldState();
			List<PlayerSaveData> players = CapturePlayers();

			return new SaveFileData(
				new SaveGameData(worldGeneration, buildings, enemies, world),
				new SavePlayersData(players));
		}

		private WorldGenSaveData CaptureWorldGeneration()
		{
			List<EnemyCampSaveData> camps = new List<EnemyCampSaveData>();
			List<PoolableObject> campObjects = GetActivePool(SaveItem.EnemyCamp_Goblin.ToString());
			for (int i = 0; i < campObjects.Count; i++)
			{
				if (!(campObjects[i].SaveableObject is SaveableEnemyCamp saveableCamp) || saveableCamp.HealthHandler == null)
					continue;

				camps.Add(new EnemyCampSaveData
				{
					Transform = _mapper.CaptureTransform(saveableCamp.HealthHandler.transform),
					Health = saveableCamp.HealthHandler.Health,
					GUID = _guidProcessor.CreateGUIDandAddToDictionary(campObjects[i])
				});
			}

			bool hasTerrainSeed = _worldGenProcessor.TryGetTerrainSeed(out int terrainSeed);
			MeshSaveData legacyMesh = hasTerrainSeed
				? default
				: _mapper.CaptureMesh(_worldGenProcessor.GeneratedMesh);

			return new WorldGenSaveData(
				hasTerrainSeed,
				terrainSeed,
				legacyMesh,
				CaptureResources(),
				CaptureFoliage(),
				camps);
		}

		private ResourceProcessorSaveData CaptureResources()
		{
			List<ResourceGroupSaveData> groups = new List<ResourceGroupSaveData>();
			foreach (Resource resourceType in Enum.GetValues(typeof(Resource)))
			{
				if (resourceType == Resource.None || resourceType == Resource.Count)
					continue;

				Dictionary<(int meshIndex, int materialIndex), ResourceData[]> resources =
					_resourceProcessor.GetResources(resourceType);
				if (resources == null)
					continue;

				groups.Add(new ResourceGroupSaveData
				{
					ResourceType = resourceType.ToString(),
					Instances = CaptureResourceGroup(resources)
				});
			}

			return new ResourceProcessorSaveData { Groups = groups };
		}

		private List<ResourceDataSaveData> CaptureResourceGroup(
			Dictionary<(int meshIndex, int materialIndex), ResourceData[]> groupedResources)
		{
			if (groupedResources == null)
				return new List<ResourceDataSaveData>();

			List<ResourceDataSaveData> result = new List<ResourceDataSaveData>();
			foreach (ResourceData[] group in groupedResources.Values)
			{
				if (group == null)
					continue;

				for (int i = 0; i < group.Length; i++)
				{
					ResourceData resource = group[i];
					ResourceTarget? current = _resourceProcessor.GetResourceTarget(resource.GUID);
					if (current.HasValue)
						resource.CurrentAmount = current.Value.CurrentAmount;

					result.Add(_mapper.CaptureResource(resource));
				}
			}

			result.Sort((left, right) =>
			{
				int x = left.PositionX.CompareTo(right.PositionX);
				if (x != 0) return x;
				int z = left.PositionZ.CompareTo(right.PositionZ);
				return z != 0 ? z : left.PositionY.CompareTo(right.PositionY);
			});
			return result;
		}

		private FoliageProcessorSaveData CaptureFoliage()
		{
			return new FoliageProcessorSaveData
			{
				OnLandGroups = CaptureFoliageGroups(
					_foliageProcessor.GetOnLandFoliageCache(),
					_foliageGenerationSettings?.FoliageGenerationSettings),
				UnderWaterGroups = CaptureFoliageGroups(
					_foliageProcessor.GetUnderWaterFoliageCache(),
					_waterFoliageGenerationSettings?.WaterFoliageGenerationSettings)
			};
		}

		private List<FoliageGroupSaveData> CaptureFoliageGroups(
			Dictionary<(Mesh mesh, Material material), FoliageData[]> groupedFoliage,
			List<FoliageGenerationSettings> settings)
		{
			Dictionary<string, List<Vector3SaveData>> positionsBySettings =
				new Dictionary<string, List<Vector3SaveData>>(StringComparer.Ordinal);
			if (groupedFoliage == null)
				return new List<FoliageGroupSaveData>();

			foreach (FoliageData[] group in groupedFoliage.Values)
			{
				if (group == null)
					continue;

				for (int i = 0; i < group.Length; i++)
				{
					FoliageData foliage = group[i];
					if (!TryFindFoliageSettings(foliage, settings, out string settingsId, out _))
						throw new InvalidOperationException($"No foliage settings match mesh '{foliage.Mesh?.name}'.");

					if (!positionsBySettings.TryGetValue(settingsId, out List<Vector3SaveData> positions))
					{
						positions = new List<Vector3SaveData>();
						positionsBySettings.Add(settingsId, positions);
					}

					positions.Add(_mapper.CaptureVector3(foliage.Position));
				}
			}

			List<FoliageGroupSaveData> result = new List<FoliageGroupSaveData>(positionsBySettings.Count);
			foreach (KeyValuePair<string, List<Vector3SaveData>> pair in positionsBySettings)
			{
				pair.Value.Sort(CompareSavedPositions);
				result.Add(new FoliageGroupSaveData { SettingsId = pair.Key, Positions = pair.Value });
			}

			result.Sort((left, right) => string.CompareOrdinal(left.SettingsId, right.SettingsId));
			return result;
		}

		private static int CompareSavedPositions(Vector3SaveData left, Vector3SaveData right)
		{
			int x = left.X.CompareTo(right.X);
			if (x != 0) return x;
			int z = left.Z.CompareTo(right.Z);
			return z != 0 ? z : left.Y.CompareTo(right.Y);
		}

		private static bool TryFindFoliageSettings(
			FoliageData foliage,
			List<FoliageGenerationSettings> settings,
			out string settingsId,
			out int meshIndex)
		{
			if (settings != null)
			{
				for (int i = 0; i < settings.Count; i++)
				{
					FoliageGenerationSettings candidate = settings[i];
					if (candidate == null || candidate.Material != foliage.Material || candidate.MeshSettings == null)
						continue;

					for (int mesh = 0; mesh < candidate.MeshSettings.Count; mesh++)
					{
						if (candidate.MeshSettings[mesh]?.Mesh != foliage.Mesh)
							continue;

						settingsId = candidate.StableId;
						meshIndex = mesh;
						return true;
					}
				}
			}

			settingsId = null;
			meshIndex = -1;
			return false;
		}

		private List<BuildingSaveData> CaptureBuildings()
		{
			List<BuildingSaveData> result = new List<BuildingSaveData>();
			for (int type = 0; type < (int)BuildingType.Count; type++)
			{
				List<PoolableObject> objects = GetActivePool(((BuildingType)type).ToString());
				for (int i = 0; i < objects.Count; i++)
				{
					if (!(objects[i].SaveableObject is SaveableBuilding saveable) || saveable.BuildingBase == null)
						continue;

					BuildingBase building = saveable.BuildingBase;
					result.Add(new BuildingSaveData
					{
						BuildingTranform = _mapper.CaptureTransform(building.transform),
						BuildingType = objects[i].PoolName,
						BuildingHealth = building.HealthHandler.Health,
						GUID = _guidProcessor.CreateGUIDandAddToDictionary(objects[i]),
						BuildingState = building.BuildingState,
						Level = building.LevelHandler != null ? building.LevelHandler.Level : 1,
						// Remaining GPU foliage is persisted centrally. The old per-building
						// pooled-foliage list was competing state and is intentionally empty.
						DestroyedFoliage = new List<FoliageSaveData>()
					});
				}
			}

			return result;
		}

		private List<EnemySaveData> CaptureEnemies()
		{
			List<EnemySaveData> result = new List<EnemySaveData>();
			for (int type = 0; type < (int)EnemyType.Count; type++)
			{
				List<PoolableObject> objects = GetActivePool(((EnemyType)type).ToString());
				for (int i = 0; i < objects.Count; i++)
				{
					if (!(objects[i].SaveableObject is SaveableEnemy saveable) || saveable.Enemy == null)
						continue;

					Enemy enemy = saveable.Enemy;
					EnemySaveData data = new EnemySaveData
					{
						Transform = _mapper.CaptureTransform(enemy.transform),
						EnemyType = objects[i].PoolName,
						Health = enemy.HealthHandler.Health,
						GUID = _guidProcessor.CreateGUIDandAddToDictionary(objects[i])
					};

					CaptureReference(enemy.TargetSensor?.CurrentTarget, out data.TargetGUID, out data.TargetPoolType);
					CaptureReference(enemy.StationSensor?.CurrentStation, out data.CampGUID, out data.CampPoolType);
					result.Add(data);
				}
			}

			return result;
		}

		private List<PlayerSaveData> CapturePlayers()
		{
			List<PlayerSaveData> result = new List<PlayerSaveData>();
			List<Player> logicalPlayers = new List<Player>();
			if (_playerProcessor.Players != null)
				logicalPlayers.AddRange(_playerProcessor.Players);
			if (_playerProcessor.Recruits != null)
				logicalPlayers.AddRange(_playerProcessor.Recruits);

			HashSet<Player> captured = new HashSet<Player>();
			for (int i = 0; i < logicalPlayers.Count; i++)
			{
				Player player = logicalPlayers[i];
				if (player == null || !captured.Add(player) || player.TwitchUser == null ||
					player.Character == null || player.RoleHandler == null)
					continue;

				PoolableObject pooledPlayer = player.PoolableObject != null
					? player.PoolableObject
					: player.Character.GetComponent<PoolableObject>();
				if (pooledPlayer == null)
				{
					_debugProcessor.LogWarning(DebugLogCategory.GameIO,
						$"Skipping player '{player.TwitchUser.Username}' because its pooled character is unavailable.");
					continue;
				}

				PlayerSaveData data = new PlayerSaveData
				{
					TwitchID = player.TwitchUser.UserID,
					TwitchName = player.TwitchUser.Username,
					TwitchUserType = player.TwitchUser.TwitchUserType,
					GameUserType = player.TwitchUser.GameUserType,
					IsBroadcaster = player.TwitchUser.IsBroadcaster,
					IsUserPlayer = ReferenceEquals(player, _playerProcessor.UserPlayer),
					GUID = _guidProcessor.CreateGUIDandAddToDictionary(pooledPlayer),
					PetActive = player.Pet != null && player.Pet.IsActive,
					CurrentPet = player.Pet != null ? player.Pet.ActivePetType : PetType.None,
					UnlockedPets = player.GetUnlockedPetTypes(),
					Transform = _mapper.CaptureTransform(player.Character.transform),
					CurrentRole = player.RoleHandler.CurrentRole,
					PreviousRole = player.RoleHandler.PreviousRole,
					Roles = _mapper.CaptureRoles(player.RoleHandler.PlayerRolesData),
					Inventory = _mapper.CaptureInventory(player.RoleHandler.Inventory.Resources),
					Customization = _mapper.CaptureCustomization(player.EquipmentHandler),
					Health = player.HealthHandler.Health,
					RegenRequiresFood = player.HealthHandler.RegenRequiresFood
				};

				CaptureReference(player.TargetSensor?.CurrentTarget, out data.TargetGUID, out data.TargetPoolType);
				CaptureReference(player.StationSensor?.CurrentStation, out data.StationGUID, out data.StationPoolType);
				result.Add(data);
			}

			return result;
		}

		private WorldSaveData CaptureWorldState()
		{
			TechTreeSaveData techTree = new TechTreeSaveData
			{
				UnlockedTechIds = _techTreeProcessor.GetUnlockedTechIds(),
				UnlockedTechs = new List<bool>(),
				CurrentTechData = new List<ObjectiveSaveData>(),
				TechVote = CaptureTechVote()
			};

			if (_techTreeProcessor.CurrentTech != null)
			{
				techTree.CurrentTechName = _techTreeProcessor.CurrentTech.TechName;
				techTree.TechAvailable = true;
				if (_townGoalProcessor.CurrentGoals != null && _townGoalProcessor.CurrentGoals.Count > 0)
				{
					foreach (KeyValuePair<Objective, bool> objective in _townGoalProcessor.CurrentGoals[0].ObjectivesStatuses)
					{
						Objective current = objective.Key;
						techTree.CurrentTechData.Add(new ObjectiveSaveData
						{
							ObjectiveType = current.ObjectiveType.ToString(),
							ResourceType = current.Data.ResourceType.ToString(),
							BuildingType = current.Data.BuildingType.ToString(),
							EnemyType = current.Data.EnemyType.ToString(),
							Amount = current.Amount,
							RequiredAmount = current.RequiredAmount
						});
					}
				}
			}

			List<ResourceAmountSaveData> townResources = new List<ResourceAmountSaveData>();
			foreach (Resource resourceType in Enum.GetValues(typeof(Resource)))
			{
				if (_townResourceProcessor.TryGetResourceAmount(resourceType, out int amount))
				{
					townResources.Add(new ResourceAmountSaveData
					{
						ResourceType = resourceType.ToString(),
						Amount = amount
					});
				}
			}

			Player ruler = _playerProcessor.GetRuler();
			return new WorldSaveData
			{
				TownResources = townResources,
				WoodResourceAmount = _townResourceProcessor.GetResourceAmount(Resource.Wood),
				OreResourceAmount = _townResourceProcessor.GetResourceAmount(Resource.Ore),
				GoldResourceAmount = _townResourceProcessor.GetResourceAmount(Resource.Gold),
				FoodResourceAmount = _townResourceProcessor.GetResourceAmount(Resource.Food),
				WorldAgeInSeconds = _timeProcessor.WorldTimePassed,
				TechTree = techTree,
				IsCurrentRuler = ruler != null,
				RulerName = ruler?.TwitchUser.Username,
				TimeUntillNextRulerVote = _gameEventProcessor.TimeTillRulerVote
			};
		}

		private TechVoteSaveData CaptureTechVote()
		{
			TechVote techVote = _gameEventProcessor.CurrentEvent as TechVote;
			if (techVote == null)
			{
				foreach (GameEvent queuedEvent in _gameEventProcessor.EventQueue)
				{
					if (queuedEvent is TechVote queuedTechVote)
					{
						techVote = queuedTechVote;
						break;
					}
				}
			}

			if (techVote == null)
				return default;

			List<string> techNames = new List<string>();
			foreach (KeyValuePair<string, VoteOption> option in techVote.Options)
			{
				if (option.Value?.OptionData is TechTree.Data.TechNodeData techData &&
					!string.IsNullOrWhiteSpace(techData.TechName))
					techNames.Add(techData.TechName);
			}

			List<TechVotePlayerSaveData> playerVotes = new List<TechVotePlayerSaveData>();
			foreach (KeyValuePair<Player, PlayerVote> vote in techVote.PlayerVotes)
			{
				if (vote.Key?.TwitchUser == null || vote.Value?.VoteOption == null)
					continue;

				playerVotes.Add(new TechVotePlayerSaveData
				{
					TwitchId = vote.Key.TwitchUser.UserID,
					OptionName = vote.Value.VoteOption.OptionName
				});
			}

			float worldTime = _timeProcessor.WorldTimePassed;
			return new TechVoteSaveData
			{
				Exists = techNames.Count > 0,
				SecondsUntilStart = (float)techVote.SecondsUntilStart(worldTime),
				RemainingDuration = techVote.IsActive
					? Mathf.Max(0.1f, (float)techVote.RemainingDuration(worldTime))
					: Mathf.Max(0.1f, (float)techVote.EventDuration),
				TechNames = techNames,
				PlayerVotes = playerVotes
			};
		}

		private void RestoreResources(ResourceProcessorSaveData data, int schemaVersion)
		{
			if (data.Groups == null)
				return;

			for (int groupIndex = 0; groupIndex < data.Groups.Count; groupIndex++)
			{
				ResourceGroupSaveData group = data.Groups[groupIndex];
				if (!Enum.TryParse(group.ResourceType, true, out Resource resourceType) ||
					resourceType == Resource.None || resourceType == Resource.Count)
				{
					_debugProcessor.LogWarning(DebugLogCategory.GameIO, $"Skipping unknown saved resource group '{group.ResourceType}'.");
					continue;
				}

				(List<Mesh> meshes, List<Material> materials) = GetResourceAssets(resourceType);
				List<ResourceData> resources = new List<ResourceData>();
				if (group.Instances != null)
				{
					for (int i = 0; i < group.Instances.Count; i++)
					{
						resources.Add(_mapper.RestoreResource(
							group.Instances[i],
							resourceType,
							meshes.Count,
							materials.Count,
							schemaVersion < 2));
					}
				}

				RestoreResourceType(resourceType, resources, meshes, materials);
			}
		}

		private void RestoreResourceType(
			Resource resourceType,
			List<ResourceData> resources,
			List<Mesh> meshes = null,
			List<Material> materials = null)
		{
			if (meshes == null || materials == null)
				(meshes, materials) = GetResourceAssets(resourceType);
			_resourceProcessor.SetGeneratedResources(
				resourceType,
				resources ?? new List<ResourceData>(),
				meshes,
				materials);
		}

		private (List<Mesh> meshes, List<Material> materials) GetResourceAssets(Resource resourceType)
		{
			List<Mesh> meshes = new List<Mesh>();
			List<Material> materials = new List<Material>();

			void Collect(List<ResourceGenerationSettings> settings)
			{
				if (settings == null)
					return;

				for (int i = 0; i < settings.Count; i++)
				{
					ResourceGenerationSettings item = settings[i];
					if (item == null || MapTargetMaskToResource(item.TargetType) != resourceType)
						continue;

					if (item.Meshes != null)
						meshes.AddRange(item.Meshes);
					if (item.Materials != null)
						materials.AddRange(item.Materials);
				}
			}

			Collect(_resourceGenerationSettings?.ResourceGenerationSettings);
			Collect(_waterResourceGenerationSettings?.WaterResourceGenerationSettings);
			return (meshes, materials);
		}

		private static Resource MapTargetMaskToResource(TargetMask targetType)
		{
			if (targetType.HasFlag(TargetMask.Tree)) return Resource.Wood;
			if (targetType.HasFlag(TargetMask.Ore)) return Resource.Ore;
			if (targetType.HasFlag(TargetMask.Bush) || targetType.HasFlag(TargetMask.Fish)) return Resource.Food;
			return Resource.None;
		}

		private void RestoreFoliage(FoliageProcessorSaveData data, int schemaVersion)
		{
			if (schemaVersion >= 2)
			{
				_foliageProcessor.SetGeneratedFoliage(
					RestoreFoliageGroups(data.OnLandGroups, _foliageGenerationSettings?.FoliageGenerationSettings),
					RestoreFoliageGroups(data.UnderWaterGroups, _waterFoliageGenerationSettings?.WaterFoliageGenerationSettings));
				return;
			}

			_foliageProcessor.SetGeneratedFoliage(
				RestoreLegacyFoliageGroup(data.OnLand, _foliageGenerationSettings?.FoliageGenerationSettings),
				RestoreLegacyFoliageGroup(data.UnderWater, _waterFoliageGenerationSettings?.WaterFoliageGenerationSettings));
		}

		private List<FoliageData> RestoreFoliageGroups(
			List<FoliageGroupSaveData> savedGroups,
			List<FoliageGenerationSettings> settings)
		{
			List<FoliageData> result = new List<FoliageData>();
			if (savedGroups == null)
				return result;

			for (int groupIndex = 0; groupIndex < savedGroups.Count; groupIndex++)
			{
				FoliageGroupSaveData group = savedGroups[groupIndex];
				FoliageGenerationSettings selected = settings?.Find(
					candidate => candidate != null && candidate.StableId == group.SettingsId);
				if (selected == null || selected.MeshSettings == null || selected.MeshSettings.Count == 0)
					throw new InvalidOperationException($"Foliage settings '{group.SettingsId}' no longer exists or has no meshes.");

				if (group.Positions == null)
					continue;

				for (int i = 0; i < group.Positions.Count; i++)
				{
					Vector3 position = _mapper.RestoreVector3(group.Positions[i]);
					int meshIndex = WorldInstanceDeterminism.SelectFoliageMesh(
						position,
						group.SettingsId,
						selected.MeshSettings.Count);
					FoliageMeshSettings meshSettings = selected.MeshSettings[meshIndex];
					if (meshSettings?.Mesh == null)
						throw new InvalidOperationException($"Foliage settings '{group.SettingsId}' contains an invalid mesh at {meshIndex}.");

					result.Add(new FoliageData(
						position,
						WorldInstanceDeterminism.SelectFoliageRotation(position, group.SettingsId),
						meshSettings.BaseScale,
						meshSettings.Mesh,
						selected.Material));
				}
			}

			return result;
		}

		private List<FoliageData> RestoreLegacyFoliageGroup(
			List<FoliageInstanceSaveData> savedFoliage,
			List<FoliageGenerationSettings> settings)
		{
			List<FoliageData> result = new List<FoliageData>();
			if (savedFoliage == null)
				return result;

			for (int i = 0; i < savedFoliage.Count; i++)
			{
				FoliageInstanceSaveData saved = savedFoliage[i];
				FoliageGenerationSettings selected = settings?.Find(candidate => candidate != null && candidate.PoolName == saved.SettingsId);
				if (selected == null || selected.MeshSettings == null || saved.MeshIndex < 0 || saved.MeshIndex >= selected.MeshSettings.Count)
					throw new InvalidOperationException($"Foliage settings '{saved.SettingsId}' or mesh {saved.MeshIndex} no longer exists.");

				result.Add(new FoliageData(
					_mapper.RestoreVector3(saved.Transform.Position),
					Quaternion.Euler(_mapper.RestoreVector3(saved.Transform.Rotation)),
					_mapper.RestoreVector3(saved.Transform.LossyScale),
					selected.MeshSettings[saved.MeshIndex].Mesh,
					selected.Material));
			}

			return result;
		}

		private async Task RestoreEnemyCampsAsync(List<EnemyCampSaveData> camps, CancellationToken cancellationToken)
		{
			float frameStart = Time.realtimeSinceStartup;
			for (int i = 0; i < camps.Count; i++)
			{
				cancellationToken.ThrowIfCancellationRequested();
				PoolableObject pooled = RequirePooledObject(SaveItem.EnemyCamp_Goblin.ToString(), camps[i].Transform);
				if (!(pooled.SaveableObject is SaveableEnemyCamp saveable) || saveable.HealthHandler == null)
					throw new InvalidOperationException("Enemy camp pool item is not saveable.");

				_mapper.ApplyTransform(saveable.HealthHandler.transform, camps[i].Transform);
				saveable.HealthHandler.SetHealth(camps[i].Health);
				_guidProcessor.RegisterLoadedGUID(pooled, camps[i].GUID);
				if (ShouldYieldFrame(ref frameStart)) await Task.Yield();
			}

			_worldGenProcessor.RefreshEnemyCampSpawners();
		}

		private async Task<List<(BuildingBase building, BuildingSaveData data)>> RestoreBuildingsAsync(
			List<BuildingSaveData> buildings,
			Action<float, string> progressReporter,
			CancellationToken cancellationToken)
		{
			List<(BuildingBase building, BuildingSaveData data)> result =
				new List<(BuildingBase, BuildingSaveData)>();
			float frameStart = Time.realtimeSinceStartup;
			for (int i = 0; i < buildings.Count; i++)
			{
				cancellationToken.ThrowIfCancellationRequested();
				BuildingSaveData data = buildings[i];
				PoolableObject pooled = RequirePooledObject(data.BuildingType, data.BuildingTranform);
				if (!(pooled.SaveableObject is SaveableBuilding saveable) || saveable.BuildingBase == null)
					throw new InvalidOperationException($"Building pool '{data.BuildingType}' is not saveable.");

				BuildingBase building = saveable.BuildingBase;
				_mapper.ApplyTransform(building.transform, data.BuildingTranform);
				building.BuildingState = data.BuildingState;
				building.HealthHandler.SetHealth(data.BuildingHealth);
				building.FoliageRemoved = new List<PoolableObject>();
				_guidProcessor.RegisterLoadedGUID(pooled, data.GUID);
				_buildingProcessor.AddLoadedBuilding(building);
				result.Add((building, data));

				if (building.BuildingState == BuildingState.Building)
					building.OnLoadedBuiltBuilding();
				ReportCollectionProgress(0.36f, 0.19f, i, buildings.Count, "buildings", progressReporter);
				if (ShouldYieldFrame(ref frameStart)) await Task.Yield();
			}

			return result;
		}

		private async Task<List<(Enemy enemy, EnemySaveData data)>> RestoreEnemiesAsync(
			List<EnemySaveData> enemies,
			Action<float, string> progressReporter,
			CancellationToken cancellationToken)
		{
			List<(Enemy enemy, EnemySaveData data)> result = new List<(Enemy, EnemySaveData)>();
			float frameStart = Time.realtimeSinceStartup;
			for (int i = 0; i < enemies.Count; i++)
			{
				cancellationToken.ThrowIfCancellationRequested();
				EnemySaveData data = enemies[i];
				PoolableObject pooled = RequirePooledObject(data.EnemyType, data.Transform);
				Enemy enemy = (pooled.SaveableObject as SaveableEnemy)?.Enemy;
				if (enemy == null)
					throw new InvalidOperationException($"Enemy pool '{data.EnemyType}' is not saveable.");

				_mapper.ApplyTransform(enemy.transform, data.Transform);
				enemy.HealthHandler.SetHealth(data.Health);
				_guidProcessor.RegisterLoadedGUID(pooled, data.GUID);
				result.Add((enemy, data));
				ReportCollectionProgress(0.55f, 0.1f, i, enemies.Count, "enemies", progressReporter);
				if (ShouldYieldFrame(ref frameStart)) await Task.Yield();
			}

			return result;
		}

		private async Task<List<(Player player, PlayerSaveData data)>> RestorePlayersAsync(
			List<PlayerSaveData> players,
			Action<float, string> progressReporter,
			CancellationToken cancellationToken)
		{
			List<(Player player, PlayerSaveData data)> result = new List<(Player, PlayerSaveData)>();
			float frameStart = Time.realtimeSinceStartup;
			for (int i = 0; i < players.Count; i++)
			{
				cancellationToken.ThrowIfCancellationRequested();
				PlayerSaveData data = players[i];
				Player player = RestorePlayer(data);
				result.Add((player, data));
				ReportCollectionProgress(0.7f, 0.16f, i, players.Count, "players", progressReporter);
				if (ShouldYieldFrame(ref frameStart)) await Task.Yield();
			}

			return result;
		}

		private Player RestorePlayer(PlayerSaveData data)
		{
			Player player = new Player(new TwitchUser(data.TwitchID, data.TwitchName));
			if (_gameSettings.GM_IDs != null && _gameSettings.GM_IDs.Contains(player.TwitchUser.UserID))
				data.GameUserType = GameUserType.GameMaster;

			player.TwitchUser.TwitchUserType = data.TwitchUserType;
			player.TwitchUser.GameUserType = data.GameUserType;
			player.TwitchUser.IsBroadcaster = data.IsBroadcaster;

			PoolableObject pooledPlayer = RequirePooledObject("Player", data.Transform);
			player.Character = pooledPlayer.gameObject;
			_mapper.ApplyTransform(player.Character.transform, data.Transform);

			if (_playerProcessor.AddExistingPlayer(player, data.CurrentRole) == null)
				throw new InvalidOperationException($"Could not restore player '{data.TwitchName}'.");

			// Schema 3 stores this relationship explicitly. The broadcaster/debugger
			// fallback migrates older saves which had no user-player field.
			if (data.IsUserPlayer ||
				(_playerProcessor.UserPlayer == null &&
				 (data.IsBroadcaster || string.Equals(data.TwitchName, "Debugger", StringComparison.Ordinal))))
			{
				_playerProcessor.SetUserPlayer(player);
			}

			player.RoleHandler.Inventory.SetResources(
				_mapper.RestoreInventory(data.Inventory, player.RoleHandler.Inventory.Resources));
			_mapper.RestoreRoles(data.Roles, player.RoleHandler.PlayerRolesData);
			player.RoleHandler.RecalculateRoles();
			player.RoleHandler.RestorePreviousRole(data.PreviousRole);
			_mapper.RestoreCustomization(player.EquipmentHandler, data.Customization);
			player.HealthHandler.SetRegenRequiresFood(data.RegenRequiresFood);
			player.HealthHandler.SetHealth(data.Health);

			Dictionary<PetType, bool> unlockedPets = new Dictionary<PetType, bool>();
			for (int i = 0; i < (int)PetType.Count; i++)
				unlockedPets[(PetType)i] = data.UnlockedPets != null && data.UnlockedPets.Contains((PetType)i);
			player.PetsUnlocked = unlockedPets;

			PoolableObject pooledPet = _poolingProcessor.GetPooledObject("Pet", player.Character.transform.position, Quaternion.identity, false);
			if (pooledPet != null && pooledPet.TryGetComponent(out Pet pet))
			{
				player.Pet = pet;
				pet.SetOwner(player.Character.transform, player);
				pet.TrySetActivePet(data.PetActive ? data.CurrentPet : PetType.None);
				pet.IsActive = data.PetActive;
			}

			_guidProcessor.RegisterLoadedGUID(pooledPlayer, data.GUID);
			return player;
		}

		private void RestoreWorldState(WorldSaveData world)
		{
			_timeProcessor.WorldTimePassed = world.WorldAgeInSeconds;
			_timeProcessor.CalculateDayCount(world.WorldAgeInSeconds);
			_seasonProcessor.SetSeasonByTimePassed(world.WorldAgeInSeconds);

			TechTreeSaveData techTree = world.TechTree;
			if (techTree.UnlockedTechIds != null)
				_techTreeProcessor.RestoreUnlockedTechIds(techTree.UnlockedTechIds);
			else
				_techTreeProcessor.SetUnlockedTechStates(techTree.UnlockedTechs ?? new List<bool>());
			if (techTree.TechAvailable && !string.IsNullOrWhiteSpace(techTree.CurrentTechName))
			{
				_techTreeProcessor.StartGoalFromNodeName(techTree.CurrentTechName);
				if (_townGoalProcessor.CurrentGoals != null && _townGoalProcessor.CurrentGoals.Count > 0)
					RestoreObjectiveProgress(
						_townGoalProcessor.CurrentGoals[0],
						techTree.CurrentTechData ?? new List<ObjectiveSaveData>());
			}
			else if (techTree.TechVote.Exists)
			{
				// The exact pending/active vote is restored after players so its votes
				// can be rebound to the newly-created runtime Player instances.
				_techTreeProcessor.RequestStartTechVote = false;
			}
			else
			{
				_techTreeProcessor.RequestedTechVoteDelay = 20f;
				_techTreeProcessor.RequestStartTechVote = true;
			}

			_gameEventProcessor.TimeTillRulerVote = world.TimeUntillNextRulerVote;
		}

		private void RestoreTechVote(
			TechVoteSaveData data,
			List<(Player player, PlayerSaveData data)> loadedPlayers)
		{
			if (!data.Exists)
				return;

			TechVote restoredVote = _techTreeProcessor.RestoreTechVote(
				data.TechNames,
				Mathf.Max(0f, data.SecondsUntilStart),
				Mathf.Max(0.1f, data.RemainingDuration));
			if (restoredVote == null)
			{
				_debugProcessor.LogWarning(
					DebugLogCategory.GameIO,
					"The saved technology vote had no valid technology options; a replacement vote will be requested.");
				_techTreeProcessor.RequestedTechVoteDelay = 20f;
				_techTreeProcessor.RequestStartTechVote = true;
				return;
			}

			if (data.PlayerVotes == null || loadedPlayers == null)
				return;

			for (int voteIndex = 0; voteIndex < data.PlayerVotes.Count; voteIndex++)
			{
				TechVotePlayerSaveData savedVote = data.PlayerVotes[voteIndex];
				if (string.IsNullOrWhiteSpace(savedVote.TwitchId) ||
					string.IsNullOrWhiteSpace(savedVote.OptionName) ||
					!restoredVote.Options.TryGetValue(savedVote.OptionName, out VoteOption option))
				{
					continue;
				}

				Player voter = null;
				for (int playerIndex = 0; playerIndex < loadedPlayers.Count; playerIndex++)
				{
					Player candidate = loadedPlayers[playerIndex].player;
					if (candidate?.TwitchUser != null &&
						string.Equals(candidate.TwitchUser.UserID, savedVote.TwitchId, StringComparison.Ordinal))
					{
						voter = candidate;
						break;
					}
				}

				if (voter != null)
					restoredVote.Action(new PlayerVote(voter, option));
			}
		}

		private static void RestoreObjectiveProgress(Goal goal, List<ObjectiveSaveData> savedObjectives)
		{
			if (goal == null || savedObjectives == null || savedObjectives.Count == 0)
				return;

			List<Objective> objectives = new List<Objective>(goal.ObjectivesStatuses.Keys);
			bool hasStableIdentity = savedObjectives.Exists(saved => !string.IsNullOrWhiteSpace(saved.ObjectiveType));
			if (!hasStableIdentity)
			{
				int count = Mathf.Min(objectives.Count, savedObjectives.Count);
				for (int i = 0; i < count; i++)
					objectives[i].SetValues(savedObjectives[i].Amount, savedObjectives[i].RequiredAmount);
				return;
			}

			bool[] used = new bool[savedObjectives.Count];
			for (int objectiveIndex = 0; objectiveIndex < objectives.Count; objectiveIndex++)
			{
				Objective objective = objectives[objectiveIndex];
				for (int savedIndex = 0; savedIndex < savedObjectives.Count; savedIndex++)
				{
					if (used[savedIndex] || !ObjectiveIdentityMatches(objective, savedObjectives[savedIndex]))
						continue;

					ObjectiveSaveData saved = savedObjectives[savedIndex];
					objective.SetValues(saved.Amount, saved.RequiredAmount);
					used[savedIndex] = true;
					break;
				}
			}
		}

		private static bool ObjectiveIdentityMatches(Objective objective, ObjectiveSaveData saved)
		{
			return string.Equals(saved.ObjectiveType, objective.ObjectiveType.ToString(), StringComparison.Ordinal) &&
				string.Equals(saved.ResourceType, objective.Data.ResourceType.ToString(), StringComparison.Ordinal) &&
				string.Equals(saved.BuildingType, objective.Data.BuildingType.ToString(), StringComparison.Ordinal) &&
				string.Equals(saved.EnemyType, objective.Data.EnemyType.ToString(), StringComparison.Ordinal);
		}

		/// <summary>
		/// Applies saved balances only after tech and building levels have rebuilt
		/// the derived storage capacities that bound those balances.
		/// </summary>
		private void RestoreTownResources(WorldSaveData world)
		{
			if (world.TownResources != null)
			{
				for (int i = 0; i < world.TownResources.Count; i++)
				{
					ResourceAmountSaveData savedResource = world.TownResources[i];
					if (Enum.TryParse(savedResource.ResourceType, true, out Resource resourceType) &&
						_townResourceProcessor.TryGetResourceAmount(resourceType, out _))
					{
						_townResourceProcessor.SetResourceAmount(resourceType, savedResource.Amount);
					}
				}
			}
			else
			{
				// Positional fields retained as a read-only fallback for the previous format.
				_townResourceProcessor.SetResourceAmount(Resource.Wood, world.WoodResourceAmount);
				_townResourceProcessor.SetResourceAmount(Resource.Ore, world.OreResourceAmount);
				_townResourceProcessor.SetResourceAmount(Resource.Food, world.FoodResourceAmount);
				_townResourceProcessor.SetResourceAmount(Resource.Gold, world.GoldResourceAmount);
			}

		}

		private static void RestoreBuildingLevels(List<(BuildingBase building, BuildingSaveData data)> buildings)
		{
			for (int i = 0; i < buildings.Count; i++)
			{
				if (buildings[i].building.LevelHandler != null)
					buildings[i].building.LevelHandler.RestoreLevel(Mathf.Max(1, buildings[i].data.Level));

				buildings[i].building.HealthHandler.SetHealth(buildings[i].data.BuildingHealth);
				if (buildings[i].building.DamageHandler != null)
					buildings[i].building.DamageHandler.OnHealthChanged(buildings[i].building.HealthHandler);
			}
		}

		private static void RestoreEnemyHealth(List<(Enemy enemy, EnemySaveData data)> enemies)
		{
			for (int i = 0; i < enemies.Count; i++)
				enemies[i].enemy.RestoreHealth(enemies[i].data.Health);
		}

		private void RestoreRuler(WorldSaveData world)
		{
			if (world.IsCurrentRuler && _playerProcessor.PlayerExistsByNameToLower(world.RulerName, out int rulerIndex))
			{
				_playerProcessor.SetRuler(_playerProcessor.GetPlayer(rulerIndex));
			}
			else
			{
				_gameEventProcessor.CanStartNewRulerVote = true;
				_gameEventProcessor.TimeTillRulerVote = _gameEventProcessor.RulerVoteMinTime;
			}
		}

		private void RestoreObjectReferences(
			List<(Player player, PlayerSaveData data)> players,
			List<(Enemy enemy, EnemySaveData data)> enemies)
		{
			for (int i = 0; i < players.Count; i++)
			{
				Player player = players[i].player;
				PlayerSaveData data = players[i].data;
				PoolableObject stationObject = ResolveReference(data.StationGUID, data.StationPoolType, PoolType.Building);
				if (stationObject != null && stationObject.TryGetComponent(out Station station))
					player.StationSensor.TrySetStation(station);

				PoolableObject targetObject = ResolveReference(data.TargetGUID, data.TargetPoolType, null);
				if (targetObject != null && targetObject.TryGetComponent(out Targetable target))
					player.TargetSensor.TrySetTarget(target);
			}

			for (int i = 0; i < enemies.Count; i++)
			{
				Enemy enemy = enemies[i].enemy;
				EnemySaveData data = enemies[i].data;
				PoolableObject campObject = ResolveReference(data.CampGUID, data.CampPoolType, PoolType.Other);
				if (campObject != null && campObject.TryGetComponent(out Station station))
				{
					enemy.StationSensor?.TrySetStation(station);
					station.GetComponent<EnemySpawner>()?.AddEnemySpawn(enemy);
				}

				PoolableObject targetObject = ResolveReference(data.TargetGUID, data.TargetPoolType, null);
				if (targetObject != null && targetObject.TryGetComponent(out Targetable target))
					enemy.TargetSensor?.TrySetTarget(target);
			}
		}

		private static void CaptureReference(Component component, out uint guid, out string poolType)
		{
			guid = 0;
			poolType = null;
			if (component == null)
				return;

			PoolableObject pooled = component.GetComponent<PoolableObject>();
			GUIDSystem.GUIDComponent guidComponent = component.GetComponent<GUIDSystem.GUIDComponent>();
			if (pooled == null || guidComponent == null)
				return;

			guid = guidComponent.GUID;
			poolType = pooled.PoolType.ToString();
		}

		private PoolableObject ResolveReference(uint guid, string poolType, PoolType? fallbackType)
		{
			if (guid == 0)
				return null;

			if (!string.IsNullOrWhiteSpace(poolType) &&
				_guidProcessor.TryGetComponentFromID(guid, poolType, out PoolableObject exact))
				return exact;

			if (fallbackType.HasValue &&
				_guidProcessor.TryGetComponentFromID(guid, fallbackType.Value.ToString(), out PoolableObject fallback))
				return fallback;

			for (int type = 0; type < (int)PoolType.Count; type++)
			{
				if (_guidProcessor.TryGetComponentFromID(guid, ((PoolType)type).ToString(), out PoolableObject found))
					return found;
			}

			_debugProcessor.LogWarning(DebugLogCategory.GameIO, $"Could not restore reference to GUID {guid} ({poolType}).");
			return null;
		}

		private PoolableObject RequirePooledObject(string poolName, TransformSaveData transform)
		{
			Vector3 position = _mapper.RestoreVector3(transform.Position);
			Quaternion rotation = Quaternion.Euler(_mapper.RestoreVector3(transform.Rotation));
			PoolableObject pooled = _poolingProcessor.GetPooledObject(poolName, position, rotation, false);
			if (pooled == null)
				throw new InvalidOperationException($"Save references missing pool '{poolName}'.");

			return pooled;
		}

		private void PrepareRuntimeForLoad()
		{
			_poolingProcessor.ReturnAllActiveObjectsToPools();
			_gameEventProcessor.ResetWorldState();
			_guidProcessor.ResetWorldState();
			_resourceProcessor.ResetWorldState();
			_buildingProcessor.ResetWorldState();
			_roleProcessor.ResetWorldState();
			_playerProcessor.ResetWorldState();
			_townGoalProcessor.ResetWorldState();
			_townResourceProcessor.ResetWorldState();
		}

		private List<PoolableObject> GetActivePool(string poolName)
		{
			_poolingProcessor.TryGetAllActivePooledObjectsOfType(poolName, out List<PoolableObject> activeObjects);
			return activeObjects;
		}

		private void ReportLoadProgress(float progress, string status, Action<float, string> externalReporter)
		{
			_saveRuntimeData.Report(progress, status);
			externalReporter?.Invoke(progress, status);
		}

		private void ReportCollectionProgress(
			float start,
			float range,
			int index,
			int count,
			string label,
			Action<float, string> reporter)
		{
			float step = count == 0 ? 1f : (index + 1f) / count;
			ReportLoadProgress(start + step * range, $"Restoring {label} ({index + 1}/{count})...", reporter);
		}

		private static void Normalize(SaveFileData save)
		{
			save.Game.BuildingSaveData ??= new List<BuildingSaveData>();
			save.Game.EnemySaveData ??= new List<EnemySaveData>();

			WorldGenSaveData worldGeneration = save.Game.WorldGenData;
			ResourceProcessorSaveData resources = worldGeneration.Resources;
			resources.Groups ??= new List<ResourceGroupSaveData>();
			worldGeneration.Resources = resources;
			worldGeneration.EnemyCamps ??= new List<EnemyCampSaveData>();
			FoliageProcessorSaveData foliage = worldGeneration.Foliage;
			if (save.SchemaVersion >= 2)
			{
				foliage.OnLandGroups ??= new List<FoliageGroupSaveData>();
				foliage.UnderWaterGroups ??= new List<FoliageGroupSaveData>();
			}
			else
			{
				foliage.OnLand ??= new List<FoliageInstanceSaveData>();
				foliage.UnderWater ??= new List<FoliageInstanceSaveData>();
			}
			worldGeneration.Foliage = foliage;
			save.Game.WorldGenData = worldGeneration;

			WorldSaveData world = save.Game.WorldSaveData;
			TechTreeSaveData techTree = world.TechTree;
			techTree.UnlockedTechs ??= new List<bool>();
			techTree.CurrentTechData ??= new List<ObjectiveSaveData>();
			if (save.SchemaVersion >= 3 && techTree.TechVote.Exists)
			{
				TechVoteSaveData techVote = techTree.TechVote;
				techVote.TechNames ??= new List<string>();
				techVote.PlayerVotes ??= new List<TechVotePlayerSaveData>();
				techTree.TechVote = techVote;
			}
			world.TechTree = techTree;
			save.Game.WorldSaveData = world;

			save.Players.PlayerSaveDatas ??= new List<PlayerSaveData>();
		}

		private static bool ShouldYieldFrame(ref float frameStart)
		{
			if (Time.realtimeSinceStartup - frameStart < FrameBudgetSeconds)
				return false;

			frameStart = Time.realtimeSinceStartup;
			return true;
		}

		public void Initialize()
		{
			if (_saveRuntimeData == null || _saveStorage == null || _mapper == null)
				throw new InvalidOperationException("SaveProcessor dependencies were not installed.");
		}

		public void Process()
		{
			if (!_saveRuntimeData.Autosave || _saveRuntimeData.IsBusy || _saveRuntimeData.AutosaveTime <= 0f)
				return;

			_saveRuntimeData.TimeElapsed += Time.deltaTime;
			if (_saveRuntimeData.TimeElapsed < _saveRuntimeData.AutosaveTime)
				return;

			_saveRuntimeData.TimeElapsed = 0f;
			SaveGame();
		}

		public void RefreshSceneData(Container sceneContainer)
		{
			if (sceneContainer == null)
				return;

			_resourceGenerationSettings = sceneContainer.Resolve<ResourceGenSettings>();
			_waterResourceGenerationSettings = sceneContainer.Resolve<WaterResourceGenSettings>();
			_foliageGenerationSettings = sceneContainer.Resolve<FoliageGenSettings>();
			_waterFoliageGenerationSettings = sceneContainer.Resolve<WaterFoliageGenSettings>();
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			containerBuilder.AddSingleton<BinarySaveStorage>(_ => new BinarySaveStorage(), typeof(ISaveStorage));
			containerBuilder.AddSingleton<SaveDataMapper>(_ => new SaveDataMapper());
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_saveRuntimeData != null)
				throw new InvalidOperationException("SaveProcessor: SaveRuntimeData has already been installed.");

			_saveRuntimeData = new SaveRuntimeData();
			containerBuilder.AddSingleton(_saveRuntimeData);
		}
	}
}
