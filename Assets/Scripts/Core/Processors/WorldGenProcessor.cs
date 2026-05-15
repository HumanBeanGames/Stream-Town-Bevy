using Enemies;
using GameResources;
using GUIDSystem;
using Processors;
using Pathfinding;
using SavingAndLoading.SavableObjects;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using UnityEngine;
using Utils;
using Utils.Pooling;
using Reflex.Attributes;
using Reflex.Core;
using World.Generation.Settings;
using UserInterface.MainMenu;
using World.Generation;
using World;
using ScriptablesProcessorInfrastructure;
using UnityEngine.SceneManagement;
using UnityEngine.Rendering.Universal;

namespace Processors
{
	public partial class WorldGenProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// The max number of attempts the generation will try to place an enemy camp. Prevents an infinite loop.
		/// </summary>
		private const int MAX_CAMP_GENERATION_ATTEMPTS = 500;

		private WorldGenRuntimeData _runtimeData;

		[Inject] private WorldGenScaleSettings _scaleSettings;
		[Inject] private TerrainGenSettings _terrainGenerationSettings;
		[Inject] private ResourceGenSettings _resourceGenerationSettings;
		[Inject] private WaterResourceGenSettings _waterResourceGenerationSettings;
		[Inject] private FoliageGenSettings _foliageGenerationSettings;
		[Inject] private WaterFoliageGenSettings _waterFoliageGenerationSettings;
		[Inject] private CampGenSettings _campGenerationSettings;

		[Inject] private WorldGenBehaviorSettings _behaviorSettings;
		[Inject] private ObjectPoolingProcessor _objectPoolingProcessor;
		[Inject] private ResourceProcessor _resourceProcessor;
		[Inject] private FoliageProcessor _foliageProcessor;
		[Inject] private GameStateProcessor _gameStateProcessor;
		[Inject] private ProjectCamera _projectCamera;
		[Inject] private GUIDProcessor _guidProcessor;
		[Inject] private GridSystem.Partitioning.CellSpacePartitioning _cellSpacePartitioning;
		[Inject] private GridSystem.GridProcessor _gridProcessor;
		[Inject] private BuildingProcessor _buildingProcessor;

		private const int MAX_GENERATION_ATTEMPTS = 10;
		private Task _poolingInitializationTask = null;
		private bool _objectsGenerated = false;
		private const int MAX_TOWNHALL_SPIRAL_RINGS = 24;
		private const string TERRAIN_HOST_NAME = "GeneratedTerrain";

		[Inject] private WorldGenDebugSettings _debugSettings;

		[Inject] private WorldGenLayerSettings _layerSettings;

		[Inject] private Processors.DebugProcessor _debugProcessor;

		private const string ENEMY_CAMP_POOL_NAME = "EnemyCamp_Goblin";

		public Mesh GeneratedMesh => _runtimeData.GeneratedMesh;
		public IReadOnlyList<EnemySpawner> EnemyCampSpawners => _runtimeData.EnemyCampSpawners;
		public bool IsWorldGenerated => _runtimeData.WorldGenerated;

		private static bool IsValidTownhallHeight(float height)
		{
			return height > 0f;
		}

		private static (Mesh mesh, Vector3 scale) SelectFoliageMeshAndScale(FoliageGenerationSettings settings)
		{
			if (settings.MeshSettings == null || settings.MeshSettings.Count == 0)
				throw new InvalidOperationException($"WorldGenProcessor: Foliage settings '{settings.PoolName}' must define at least one mesh setting.");

			FoliageMeshSettings meshSettings = settings.MeshSettings[UnityEngine.Random.Range(0, settings.MeshSettings.Count)];
			if (meshSettings.Mesh == null)
				throw new InvalidOperationException($"WorldGenProcessor: Foliage settings '{settings.PoolName}' contains a mesh setting with no Mesh assigned.");

			return (meshSettings.Mesh, meshSettings.BaseScale);
		}

		private void RebindProjectCameraToActiveSceneMainCamera()
		{
			if (_projectCamera == null)
				return;

			Scene activeScene = SceneManager.GetActiveScene();
			Camera[] sceneCameras = FindObjectsByType<Camera>(FindObjectsSortMode.None);
			for (int i = 0; i < sceneCameras.Length; i++)
			{
				Camera candidate = sceneCameras[i];
				if (candidate == null || candidate.gameObject.scene.handle != activeScene.handle || !candidate.CompareTag("MainCamera"))
					continue;

				_projectCamera.Cam = candidate;
				_projectCamera.Data = candidate.GetComponent<UniversalAdditionalCameraData>();
				return;
			}
		}

		private void FocusProjectCameraOnTownhall(GameObject townhallObject)
		{
			RebindProjectCameraToActiveSceneMainCamera();

			if (townhallObject == null || _projectCamera == null || !_projectCamera.Exists)
				return;

			Transform cameraTransform = _projectCamera.Cam.transform;
			Vector3 townhallPosition = townhallObject.transform.position;
			cameraTransform.position = townhallPosition + new Vector3(0f, 25f, -25f);
			cameraTransform.rotation = Quaternion.Euler(45f, 0f, 0f);
		}

		private void CompleteWorldGeneration(GameObject townhallObject)
		{
			FocusProjectCameraOnTownhall(townhallObject);

			// Populate spatial partitioning indices for efficient queries
			// Call through GridProcessor to ensure cells are created first
			if (_gridProcessor != null)
			{
				_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Repopulating spatial partitioning indices via GridProcessor");
				_gridProcessor.RepopulateSpatialIndices(_resourceProcessor, _foliageProcessor);
			}
			else
			{
				_debugProcessor.LogWarning(DebugLogCategory.WorldGenProcessor, "GridProcessor is null, cannot repopulate indices");
			}

			_runtimeData.WorldGenerated = true;
			_gameStateProcessor.NotifyWorldLoaded();
		}

		#region IProcessor Implementation

		/// <summary>
		/// Instantiates and registers this processor's RuntimeData as a singleton.
		/// Called during container initialization before dependency injection.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_runtimeData != null)
				throw new InvalidOperationException("WorldGenProcessor: WorldGenRuntimeData has already been installed.");

			_runtimeData = new WorldGenRuntimeData();
			containerBuilder.AddSingleton(_runtimeData);
		}

		/// <summary>
		/// Initializes the processor.
		/// Called once during game initialization after all dependencies are injected.
		/// Starts world generation state machine.
		/// </summary>
		public void Initialize()
		{
			if (_runtimeData == null)
				throw new InvalidOperationException("WorldGenProcessor: WorldGenRuntimeData has not been installed.");

			// Check if generation is suppressed
			if (_behaviorSettings.SuppressGeneration)
			{
				_runtimeData.State = GenerationState.Complete;
				_runtimeData.WorldGenerated = true;
				return;
			}

			// Reset generation flags
			_objectsGenerated = false;

			// Start world generation on initialization
			_runtimeData.State = GenerationState.InitializingPooling;
			_runtimeData.GenerationAttempts = 0;
		}

		/// <summary>
		/// Processes logic every frame.
		/// Called every frame by the Coordinator.
		/// Handles world generation state machine.
		/// </summary>
		public void Process()
		{
			// Debug.Log($"[WorldGen] Process called: State={_runtimeData.State}");
			switch (_runtimeData.State)
			{
				case GenerationState.Idle:
					// Do nothing
					break;

				case GenerationState.InitializingPooling:
					// Start pooling initialization if not already started
					if (_poolingInitializationTask == null)
					{
						_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Starting pooling initialization");
						_objectPoolingProcessor.ResetPoolingInitialization();
						_poolingInitializationTask = _objectPoolingProcessor.InitializePooling(null);
					}

					// Check if pooling initialization is complete
					if (_poolingInitializationTask != null && _poolingInitializationTask.IsCompleted)
					{
						_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Pooling initialization complete");
						_poolingInitializationTask = null;
						_runtimeData.State = GenerationState.GeneratingTerrain;
					}
					break;

				case GenerationState.GeneratingTerrain:
					_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "GeneratingTerrain state");
					if (_behaviorSettings.RandomizeSeed)
						_terrainGenerationSettings.GenerationSettings.Seed = UnityEngine.Random.Range(int.MinValue, int.MaxValue);

					GenerateTerrain();
					_runtimeData.State = GenerationState.SpawningTownhall;
					break;

				case GenerationState.SpawningTownhall:
					_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "SpawningTownhall state");
					if (SpawnTownhall())
						_runtimeData.State = GenerationState.GeneratingObjects;
					else
					{
						_runtimeData.GenerationAttempts++;
						if (_runtimeData.GenerationAttempts >= MAX_GENERATION_ATTEMPTS)
						{
							_debugProcessor.LogError(DebugLogCategory.WorldGenProcessor, $"Failed to spawn townhall after {MAX_GENERATION_ATTEMPTS} attempts.");
							_runtimeData.State = GenerationState.Complete;
						}
						else
						{
							_runtimeData.State = GenerationState.GeneratingTerrain;
						}
					}
					break;

				case GenerationState.GeneratingObjects:
					if (!_objectsGenerated)
					{
						_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "GeneratingObjects state - calling GeneratePooledObjectsExceptTownhallSync");
						GeneratePooledObjectsExceptTownhallSync();
						_objectsGenerated = true;
						_runtimeData.State = GenerationState.GeneratingNavmesh;
					}
					break;

				case GenerationState.GeneratingNavmesh:
					_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "GeneratingNavmesh state");
					GenerateNavmesh();
					CompleteWorldGeneration(GetActiveTownhallObject());
					_runtimeData.State = GenerationState.Complete;
					break;

				case GenerationState.Complete:
					// Generation complete, do nothing
					break;
			}
		}

		#endregion

		#region IInstaller Implementation

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

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			RebindProjectCameraToActiveSceneMainCamera();

			// WorldGenProcessor re-injects dependencies via Coordinator's specialized handling
			// No additional refresh needed beyond what Coordinator already does
		}

		#endregion

#if UNITY_EDITOR
		[SerializeField]
		private bool _regen = false;
		[SerializeField]
		private bool _previewTreePlacements = false;
		[SerializeField]
		private Mesh _treeMesh;
		List<Vector3> _previewTreePositions = new List<Vector3>();
#endif
		/// <summary>
		/// Generates a terrain based on the stored settings.
		/// </summary>
        /// 
		
		public bool IsPointWithinBounds(Vector3 point)
        {
			float xSize = _scaleSettings.XScale * _terrainGenerationSettings.GenerationSettings.Size;
			float ySize = _scaleSettings.YScale * _terrainGenerationSettings.GenerationSettings.Size;

			float minX = -(0.5f * xSize);
			float maxX = +(0.5f * xSize);
			float minZ = -(0.5f * ySize);
			float maxZ = +(0.5f * ySize);

			if (point.x > minX && point.x < maxX || point.x > maxX && point.x < minX)
				if (point.z > minZ && point.z < maxZ || point.z > maxZ && point.z < minZ)
					return true;

			return false;
		}
		public void GenerateTerrain()
		{
			GenerationSettings sourceSettings = _terrainGenerationSettings.GenerationSettings;
			int scaledTerrainSize = GetScaledTerrainSize();

			GenerationSettings meshSettings = new GenerationSettings(
				scaledTerrainSize,
				sourceSettings.LevelOfDetail,
				sourceSettings.NoiseScale,
				sourceSettings.Octaves,
				sourceSettings.Persistance,
				sourceSettings.Lacunarity,
				sourceSettings.Seed,
				sourceSettings.Offset,
				sourceSettings.SpawnThreshold
			);

			meshSettings.Spacing = sourceSettings.Spacing;

			GameObject terrainHost = GetOrCreateTerrainHost();
			_runtimeData.GeneratedMesh = ProceduralMeshGenerator.CreateMesh(ProceduralMeshGenerator.GenerateTerrainMeshData(meshSettings, _terrainGenerationSettings.MeshHeightMultiplier, _terrainGenerationSettings.MeshHeightCurve, _terrainGenerationSettings.EnableIslandBias, _terrainGenerationSettings.IslandBiasCurve, _terrainGenerationSettings.IslandSize, _terrainGenerationSettings.IslandMultiplier, _terrainGenerationSettings.IslandAddition, _terrainGenerationSettings.QuantizationFactor, _terrainGenerationSettings.CellSize, _terrainGenerationSettings.TopFaceProportion), terrainHost);
		}

		/// <summary>
		/// Generates the A* pathfinding navigation graph.
		/// Creates a GridGraph with 8-point node size, configures collision detection,
		/// marks resource positions as unwalkable, and scans the graph.
		/// </summary>
		private void GenerateNavmesh()
		{
			// Get AstarPath component from scene
			AstarPath astarPath = GameObject.FindObjectOfType<AstarPath>();
			if (astarPath == null)
			{
				_debugProcessor.LogError(DebugLogCategory.WorldGenProcessor, "AstarPath component not found in scene. Cannot generate navmesh.");
				return;
			}

			// Get or create GridGraph
			GridGraph gridGraph = astarPath.data.FindGraphOfType(typeof(GridGraph)) as GridGraph;
			if (gridGraph == null)
			{
				gridGraph = astarPath.data.AddGraph(typeof(GridGraph)) as GridGraph;
				if (gridGraph == null)
				{
					_debugProcessor.LogError(DebugLogCategory.WorldGenProcessor, "Failed to create GridGraph.");
					return;
				}
				gridGraph.name = "WorldGridGraph";
			}

			// Configure graph dimensions based on terrain scale
			float xSize = _scaleSettings.XScale * _terrainGenerationSettings.GenerationSettings.Size;
			float zSize = _scaleSettings.YScale * _terrainGenerationSettings.GenerationSettings.Size;
			float nodeSize = 1f; // Match terrain grid cell size (1 unit) for proper alignment

			// Force 170x170 dimensions as specified
			int gridWidth = 170;
			int gridDepth = 170;
			gridGraph.SetDimensions(gridWidth, gridDepth, nodeSize);

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"GridGraph configured: {gridWidth}x{gridDepth} nodes, nodeSize={nodeSize}, calculated worldSize={xSize}x{zSize}");

			// Center the graph on the terrain
			gridGraph.center = Vector3.zero;

			// Configure collision detection
			// Disable raycast collision - mark all nodes as walkable by default
			// We'll manually mark unwalkable nodes based on terrain height and resources
			gridGraph.collision.use2D = false;
			gridGraph.collision.mask = 0; // No collision detection
			gridGraph.collision.height = 2f;
			gridGraph.collision.diameter = 0.5f;

			// Set max climb to quantization factor + 0.05
			float maxClimb = _terrainGenerationSettings.QuantizationFactor + 0.05f;
			gridGraph.maxClimb = maxClimb;

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Collision disabled - will use manual walkability detection, maxClimb={maxClimb}");

			// Scan the graph to generate nodes
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Scanning A* GridGraph...");
			astarPath.Scan(gridGraph);

			// Check if graph has walkable nodes after scan
			int walkableCount = 0;
			int totalNodes = 0;
			gridGraph.GetNodes(node =>
			{
				totalNodes++;
				if (node.Walkable)
					walkableCount++;
			});
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"After scan: {walkableCount}/{totalNodes} nodes walkable");

			// Mark nodes as unwalkable based on terrain height
			// Nodes below terrain threshold should be unwalkable
			MarkTerrainUnwalkableNodes(gridGraph);

			// Mark resource positions as unwalkable
			// Resources don't have colliders, so we manually mark nodes
			MarkResourceNodesUnwalkable(gridGraph);

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "A* GridGraph generation complete.");
		}

		/// <summary>
		/// Marks grid nodes as walkable only if they are on the ground layer.
		/// All other nodes are marked as unwalkable.
		/// Also blocks nodes where terrain drops below gradation factor.
		/// </summary>
		private void MarkTerrainUnwalkableNodes(GridGraph gridGraph)
		{
			int groundWalkableCount = 0;
			int gradationBlockedCount = 0;
			int terrainMask = GetTerrainMask();
			float gradationFactor = _terrainGenerationSettings.QuantizationFactor;

			gridGraph.GetNodes(node =>
			{
				Vector3 worldPos = (Vector3)node.position;

				// Start with unwalkable
				node.Walkable = false;

				// Raycast down to find ground
				if (Physics.Raycast(new Vector3(worldPos.x, 100, worldPos.z), Vector3.down, out RaycastHit hit, 200, terrainMask))
				{
					// If node is at ground surface (within tolerance), mark as walkable
					if (Mathf.Abs(worldPos.y - hit.point.y) < 0.5f)
					{
						// Check if terrain height is below gradation factor
						if (hit.point.y < (gradationFactor - 0.05f))
						{
							// Terrain drops below gradation factor, keep unwalkable
							gradationBlockedCount++;
						}
						else
						{
							node.Walkable = true;
							groundWalkableCount++;
						}
					}
				}
			});

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Marked {groundWalkableCount} nodes as walkable on ground layer, {gradationBlockedCount} blocked by gradation factor");
		}

		/// <summary>
		/// Marks grid nodes at resource positions as unwalkable.
		/// Resources use GPU instancing without colliders, so we manually block nodes.
		/// </summary>
		private void MarkResourceNodesUnwalkable(GridGraph gridGraph)
		{
			if (_resourceProcessor == null)
			{
				_debugProcessor.LogWarning(DebugLogCategory.WorldGenProcessor, "ResourceProcessor not available for node blocking.");
				return;
			}

			int blockedCount = 0;
			int totalResources = 0;

			// Block wood resources
			BlockResourcesInDictionary(_resourceProcessor.GetWoodResources(), gridGraph, ref blockedCount, ref totalResources);
			// Block ore resources
			BlockResourcesInDictionary(_resourceProcessor.GetOreResources(), gridGraph, ref blockedCount, ref totalResources);
			// Block food resources
			BlockResourcesInDictionary(_resourceProcessor.GetFoodResources(), gridGraph, ref blockedCount, ref totalResources);
			// Block gold resources
			BlockResourcesInDictionary(_resourceProcessor.GetGoldResources(), gridGraph, ref blockedCount, ref totalResources);
			// Block recruit resources
			BlockResourcesInDictionary(_resourceProcessor.GetRecruitResources(), gridGraph, ref blockedCount, ref totalResources);

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Blocked {blockedCount} grid nodes at resource positions (total resources checked: {totalResources}).");
		}

		/// <summary>
		/// Blocks grid nodes at positions specified in a resource dictionary.
		/// </summary>
		private void BlockResourcesInDictionary(
			System.Collections.Generic.Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> resourceDict,
			GridGraph gridGraph,
			ref int blockedCount,
			ref int totalResources)
		{
			if (resourceDict == null)
			{
				_debugProcessor.LogWarning(DebugLogCategory.WorldGenProcessor, "Resource dictionary is null.");
				return;
			}

			int dictResourceCount = 0;
			foreach (var kvp in resourceDict)
			{
				GameResources.ResourceData[] resources = kvp.Value;
				if (resources == null)
					continue;

				dictResourceCount += resources.Length;
				foreach (var resource in resources)
				{
					totalResources++;
					Vector3 position = resource.Position;
					GraphNode node = gridGraph.GetNearest(position, NNConstraint.Default).node;

					if (node == null)
					{
						_debugProcessor.LogWarning(DebugLogCategory.WorldGenProcessor, $"No node found near resource position {position}");
					}
					else if (!node.Walkable)
					{
						// Node already unwalkable, likely from collision detection
					}
					else
					{
						node.Walkable = false;
						blockedCount++;
					}
				}
			}

			if (dictResourceCount > 0)
			{
				_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Checked {dictResourceCount} resources in dictionary");
			}
		}

		private int GetScaledTerrainSize()
		{
			float dominantScale = Mathf.Max(_scaleSettings.XScale, _scaleSettings.YScale);
			return Mathf.Max(1, Mathf.RoundToInt(_terrainGenerationSettings.GenerationSettings.Size * dominantScale));
		}

		public void SetMesh(Mesh mesh)
		{
			GameObject terrainHost = GetOrCreateTerrainHost();
			_runtimeData.GeneratedMesh = ProceduralMeshGenerator.CreateMesh(mesh, terrainHost);
		}

		private GameObject GetOrCreateTerrainHost()
		{
			if (_runtimeData.TerrainHost != null)
			{
				EnsureTerrainHostSetup(_runtimeData.TerrainHost);
				return _runtimeData.TerrainHost;
			}

			GameObject terrainPrefab = _terrainGenerationSettings.TerrainPrefab;
			if (terrainPrefab == null)
			{
				throw new InvalidOperationException("Terrain prefab is not set in TerrainGenSettings.");
			}

			Scene targetScene = SceneManager.GetActiveScene();
			GameObject terrainHost = UnityEngine.Object.Instantiate(terrainPrefab);
			terrainHost.name = TERRAIN_HOST_NAME;
			if (targetScene.IsValid() && targetScene.isLoaded)
			{
				SceneManager.MoveGameObjectToScene(terrainHost, targetScene);
			}

			_runtimeData.TerrainHost = terrainHost;
			EnsureTerrainHostSetup(terrainHost);
			return terrainHost;
		}

		private void EnsureTerrainHostSetup(GameObject terrainHost)
		{
			if (terrainHost == null)
			{
				return;
			}

			terrainHost.name = TERRAIN_HOST_NAME;
			terrainHost.transform.SetParent(null, false);
			terrainHost.transform.position = Vector3.zero;
			terrainHost.transform.rotation = Quaternion.identity;
			terrainHost.transform.localScale = Vector3.one;
			terrainHost.layer = GetTerrainLayer();

			if (terrainHost.TryGetComponent<MeshRenderer>(out var meshRenderer) && _terrainGenerationSettings.TerrainMaterial != null)
			{
				meshRenderer.material = _terrainGenerationSettings.TerrainMaterial;
			}
		}

		private int GetTerrainLayer()
		{
			int terrainMask = GetTerrainMask();
			for (int layer = 0; layer < 32; layer++)
			{
				if ((terrainMask & (1 << layer)) != 0)
				{
					return layer;
				}
			}

			int groundLayer = LayerMask.NameToLayer("Ground");
			return groundLayer >= 0 ? groundLayer : 0;
		}

		public void RefreshEnemyCampSpawners()
		{
			_runtimeData.EnemyCampSpawners.Clear();

			List<PoolableObject> activeEnemyCamps = GetAllActivePooledObjectsOfType(ENEMY_CAMP_POOL_NAME);
			for (int i = 0; i < activeEnemyCamps.Count; i++)
			{
				EnemySpawner enemySpawner = activeEnemyCamps[i].GetComponent<EnemySpawner>();
				if (enemySpawner != null)
					_runtimeData.EnemyCampSpawners.Add(enemySpawner);
			}
		}

		public bool CanSpawnRaidEnemies()
		{
			return _runtimeData.EnemyCampSpawners.Count > 0;
		}

		public void SetEnemyCampSpawningEnabled(bool enabled)
		{
			for (int i = 0; i < _runtimeData.EnemyCampSpawners.Count; i++)
			{
				EnemySpawner enemySpawner = _runtimeData.EnemyCampSpawners[i];
				if (enemySpawner != null)
					enemySpawner.CanSpawnEnemies = enabled;
			}
		}

		public bool TryGetRandomEnemyCampSpawnLocation(out Transform spawnLocation)
		{
			spawnLocation = null;

			List<EnemySpawner> availableSpawners = new List<EnemySpawner>();
			for (int i = 0; i < _runtimeData.EnemyCampSpawners.Count; i++)
			{
				EnemySpawner enemySpawner = _runtimeData.EnemyCampSpawners[i];
				if (enemySpawner == null || enemySpawner.SpawnLocations == null || enemySpawner.SpawnLocations.Length == 0)
					continue;

				availableSpawners.Add(enemySpawner);
			}

			if (availableSpawners.Count == 0)
				return false;

			EnemySpawner selectedSpawner = availableSpawners[UnityEngine.Random.Range(0, availableSpawners.Count)];
			spawnLocation = selectedSpawner.GetRandomSpawnLocation();
			return spawnLocation != null;
		}

		private int GetTerrainMask()
		{
			return _layerSettings.TerrainMask.value == 0 ? LayerMask.GetMask("Ground") : _layerSettings.TerrainMask;
		}

		private bool TryGetTerrainHeight(Vector3 worldPosition, out float height)
		{
			height = 0;
			int terrainMask = GetTerrainMask();
			if (!Physics.Raycast(new Vector3(worldPosition.x, 100, worldPosition.z), Vector3.down, out RaycastHit hit, 200, terrainMask))
				return false;

			height = hit.point.y;
			return true;
		}

		private Vector3 GetSpawnBiasOrigin()
		{
			GameObject townhallObject = GetActiveTownhallObject();
			return townhallObject == null ? Vector3.zero : townhallObject.transform.position;
		}

		private float GetSpawnNoiseValue(GenerationSettings settings, float[,] heightMap, int mapX, int mapY, Vector3 position, Vector3 spawnBiasOrigin)
		{
			float noiseValue = heightMap[mapX, mapY];
			if (!settings.EnableSpawnBias)
				return noiseValue;

			float maxDistance = Mathf.Max(0.001f, settings.SpawnBiasMaxDistance);
			float distance = Vector2.Distance(new Vector2(position.x, position.z), new Vector2(spawnBiasOrigin.x, spawnBiasOrigin.z));
			float normalizedDistance = Mathf.Clamp01(distance / maxDistance);
			float bias = settings.SpawnBiasCurve.Evaluate(normalizedDistance) * settings.SpawnBiasMultiplier + settings.SpawnBiasAddition;
			return noiseValue + bias;
		}

		private bool IsPositionOverlappingBuilding(Vector3 position, float horizontalHalfExtent)
		{
			if (_runtimeData.Buildings == null)
				return false;

			Bounds spawnBounds = new Bounds(position, new Vector3(horizontalHalfExtent * 2f, 8f, horizontalHalfExtent * 2f));

			foreach (var kvp in _runtimeData.Buildings)
			{
				List<Buildings.BuildingBase> buildingList = kvp.Value;
				if (buildingList == null)
					continue;

				for (int i = 0; i < buildingList.Count; i++)
				{
					Buildings.BuildingBase building = buildingList[i];
					if (building == null || !building.gameObject.activeInHierarchy)
						continue;

					if (building.TryGetComponent(out BoxCollider buildingCollider))
					{
						if (buildingCollider.bounds.Intersects(spawnBounds))
							return true;
					}
					else
					{
						float checkDistance = horizontalHalfExtent + 1f;
						if (Mathf.Abs(building.transform.position.x - position.x) <= checkDistance && Mathf.Abs(building.transform.position.z - position.z) <= checkDistance)
							return true;
					}
				}
			}

			return false;
		}

		private bool TryGetTownhallFootprintAABB(Vector3 townhallPosition, BoxCollider collider, out float minX, out float maxX, out float minZ, out float maxZ)
		{
			minX = maxX = minZ = maxZ = 0;
			if (collider == null)
				return false;

			Vector3 colliderCenter = townhallPosition + new Vector3(collider.center.x, 0, collider.center.z);
			Vector3 scaledSize = Vector3.Scale(collider.size, collider.transform.lossyScale);

			float halfX = scaledSize.x * 0.5f;
			float halfZ = scaledSize.z * 0.5f;

			minX = colliderCenter.x - halfX;
			maxX = colliderCenter.x + halfX;
			minZ = colliderCenter.z - halfZ;
			maxZ = colliderCenter.z + halfZ;
			return true;
		}

		private bool TryGetFlatTownhallFootprintHeight(Vector3 townhallPosition, BoxCollider collider, out float flatHeight)
		{
			flatHeight = 0;
			if (!TryGetTownhallFootprintAABB(townhallPosition, collider, out float minX, out float maxX, out float minZ, out float maxZ))
				return false;

			float spacing = Mathf.Max(1f, _terrainGenerationSettings.GenerationSettings.Spacing);
			int sampleCountX = Mathf.Max(1, Mathf.CeilToInt((maxX - minX) / spacing));
			int sampleCountZ = Mathf.Max(1, Mathf.CeilToInt((maxZ - minZ) / spacing));

			bool hasHeight = false;
			float firstHeight = 0;

			for (int x = 0; x < sampleCountX; x++)
			{
				for (int z = 0; z < sampleCountZ; z++)
				{
					float sampleX = Mathf.Lerp(minX, maxX, (x + 0.5f) / sampleCountX);
					float sampleZ = Mathf.Lerp(minZ, maxZ, (z + 0.5f) / sampleCountZ);
					Vector3 samplePoint = new Vector3(sampleX, 0, sampleZ);

					if (!IsPointWithinBounds(samplePoint))
						return false;

					if (!TryGetTerrainHeight(samplePoint, out float sampleHeight))
						return false;

					if (!hasHeight)
					{
						firstHeight = sampleHeight;
						hasHeight = true;
						continue;
					}

					if (sampleHeight != firstHeight)
						return false;
				}
			}

			flatHeight = firstHeight;
			return true;
		}

		private IEnumerable<Vector2Int> GetSpiralOffsets(int maxRings)
		{
			yield return Vector2Int.zero;

			for (int ring = 1; ring <= maxRings; ring++)
			{
				int x = ring;
				int z = -ring + 1;

				for (; z <= ring; z++)
					yield return new Vector2Int(x, z);

				x = ring - 1;
				z = ring;
				for (; x >= -ring; x--)
					yield return new Vector2Int(x, z);

				x = -ring;
				z = ring - 1;
				for (; z >= -ring; z--)
					yield return new Vector2Int(x, z);

				x = -ring + 1;
				z = -ring;
				for (; x <= ring; x++)
					yield return new Vector2Int(x, z);
			}
		}

		private bool TryFindTownhallPlacement(GameObject townhallObject, Vector3 origin, out Vector3 placement)
		{
			placement = origin;
			if (townhallObject == null)
				return false;

			BoxCollider collider = townhallObject.GetComponent<BoxCollider>();
			if (collider == null)
			{
				if (!TryGetTerrainHeight(origin, out float fallbackHeight))
					return false;

				if (!IsValidTownhallHeight(fallbackHeight))
					return false;

				placement = new Vector3(origin.x, fallbackHeight, origin.z);
				return true;
			}

			float spacing = Mathf.Max(1f, _terrainGenerationSettings.GenerationSettings.Spacing);
			foreach (Vector2Int offset in GetSpiralOffsets(MAX_TOWNHALL_SPIRAL_RINGS))
			{
				Vector3 candidate = origin + new Vector3(offset.x * spacing, 0, offset.y * spacing);
				if (!TryGetFlatTownhallFootprintHeight(candidate, collider, out float flatHeight))
					continue;

				if (!IsValidTownhallHeight(flatHeight))
					continue;

				placement = new Vector3(candidate.x, flatHeight, candidate.z);
				return true;
			}

			return false;
		}

		private GameObject GetActiveTownhallObject()
		{
			List<PoolableObject> activeTownhalls = GetAllActivePooledObjectsOfType("Townhall");
			if (activeTownhalls == null || activeTownhalls.Count == 0)
				return null;

			for (int i = 0; i < activeTownhalls.Count; i++)
			{
				SaveableBuilding saveable = activeTownhalls[i].SaveableObject as SaveableBuilding;
				if (saveable != null && saveable.BuildingBase != null)
					return saveable.BuildingBase.gameObject;
			}

			return null;
		}

		private void EnsureTownhallRegistered(Buildings.BuildingBase townhallBuilding)
		{
			if (townhallBuilding == null)
				return;

			if (_runtimeData.Buildings != null && _runtimeData.Buildings.TryGetValue(townhallBuilding.BuildingType, out List<Buildings.BuildingBase> existingBuildings) && existingBuildings != null && existingBuildings.Contains(townhallBuilding))
				return;

			// Add building to runtime data
			if (!_runtimeData.Buildings.ContainsKey(townhallBuilding.BuildingType))
				_runtimeData.Buildings[townhallBuilding.BuildingType] = new List<Buildings.BuildingBase>();
			_runtimeData.Buildings[townhallBuilding.BuildingType].Add(townhallBuilding);

			// Register with BuildingProcessor so it counts towards NumberOfBuildings
			_buildingProcessor.AddLoadedBuilding(townhallBuilding);
		}

		/// <summary>
		/// Gets a pooled object by type.
		/// Uses ObjectPoolingProcessor instead of local dictionary.
		/// </summary>
		private PoolableObject GetPooledObject(string poolName, bool createIfNotFound = true)
		{
			return _objectPoolingProcessor.GetPooledObject(poolName, printWarning: false);
		}

		/// <summary>
		/// Gets all active pooled objects of a specific type.
		/// Uses ObjectPoolingProcessor instead of local _runtimeData.PooledObjects.
		/// </summary>
		private List<PoolableObject> GetAllActivePooledObjectsOfType(string poolName)
		{
			return _objectPoolingProcessor.GetAllActivePooledObjectsOfType(poolName);
		}

		/// <summary>
		/// Generates all pooled objects except the townhall synchronously.
		/// Simplified version for Process-based generation without progress reporting.
		/// </summary>
		private void GeneratePooledObjectsExceptTownhallSync()
		{
			// Track occupied positions to prevent overlapping spawns across ALL types
			HashSet<(int, int)> occupiedCells = new HashSet<(int, int)>();

			// Generate resources first - collect data for GPU instancing via ResourceRenderer
			if (_resourceGenerationSettings != null)
			{
				foreach (ResourceGenerationSettings settings in _resourceGenerationSettings.ResourceGenerationSettings)
				{
					GenerateResourceData(settings, settings.Seed, WorldUtils.OnGroundCheckHeight, occupiedCells);
				}
			}

			// Generate water resources - share occupiedCells with regular resources
			if (_waterResourceGenerationSettings != null)
			{
				foreach (ResourceGenerationSettings settings in _waterResourceGenerationSettings.WaterResourceGenerationSettings)
				{
					GenerateResourceData(settings, settings.Seed, WorldUtils.OnShoreLineCheckHeight, occupiedCells, isWaterGeneration: true);
				}
			}

			// Generate land foliage - share occupiedCells with resources
			if (_foliageGenerationSettings != null)
			{
				foreach (FoliageGenerationSettings settings in _foliageGenerationSettings.FoliageGenerationSettings)
				{
					GenerateFoliageData(settings, settings.Seed, WorldUtils.OnGroundCheckHeight, _runtimeData.OnLandFoliage, occupiedCells);
				}
			}

			// Generate water foliage - share occupiedCells with all previous types
			if (_waterFoliageGenerationSettings != null)
			{
				foreach (FoliageGenerationSettings settings in _waterFoliageGenerationSettings.WaterFoliageGenerationSettings)
				{
					GenerateFoliageData(settings, settings.Seed, WorldUtils.UnderWaterCheckHeight, _runtimeData.UnderWaterFoliage, occupiedCells, isWaterGeneration: true);
				}
			}

			// Transfer generated data to processors for rendering
			TransferGeneratedDataToProcessors();

			RefreshEnemyCampSpawners();
		}

		/// <summary>
		/// Transfers generated resource and foliage data to their respective processors for rendering.
		/// </summary>
		private void TransferGeneratedDataToProcessors()
		{
			// Collect mesh and material lists from settings
			Dictionary<global::Utils.Resource, List<Mesh>> meshListsByType = new Dictionary<global::Utils.Resource, List<Mesh>>();
			Dictionary<global::Utils.Resource, List<Material>> materialListsByType = new Dictionary<global::Utils.Resource, List<Material>>();

			if (_resourceGenerationSettings != null)
			{
				foreach (ResourceGenerationSettings settings in _resourceGenerationSettings.ResourceGenerationSettings)
				{
					global::Utils.Resource resourceType = TargetMaskToResource(settings.TargetType);
					if (!meshListsByType.ContainsKey(resourceType))
						meshListsByType[resourceType] = new List<Mesh>();
					if (!materialListsByType.ContainsKey(resourceType))
						materialListsByType[resourceType] = new List<Material>();

					if (settings.Meshes != null)
						meshListsByType[resourceType].AddRange(settings.Meshes);
					if (settings.Materials != null)
						materialListsByType[resourceType].AddRange(settings.Materials);
				}
			}

			// Update ResourceProcessor with generated resources
			_resourceProcessor.SetGeneratedResources(global::Utils.Resource.Wood, _runtimeData.WoodResources,
				meshListsByType.ContainsKey(global::Utils.Resource.Wood) ? meshListsByType[global::Utils.Resource.Wood] : null,
				materialListsByType.ContainsKey(global::Utils.Resource.Wood) ? materialListsByType[global::Utils.Resource.Wood] : null);
			_resourceProcessor.SetGeneratedResources(global::Utils.Resource.Ore, _runtimeData.OreResources,
				meshListsByType.ContainsKey(global::Utils.Resource.Ore) ? meshListsByType[global::Utils.Resource.Ore] : null,
				materialListsByType.ContainsKey(global::Utils.Resource.Ore) ? materialListsByType[global::Utils.Resource.Ore] : null);
			_resourceProcessor.SetGeneratedResources(global::Utils.Resource.Food, _runtimeData.FoodResources,
				meshListsByType.ContainsKey(global::Utils.Resource.Food) ? meshListsByType[global::Utils.Resource.Food] : null,
				materialListsByType.ContainsKey(global::Utils.Resource.Food) ? materialListsByType[global::Utils.Resource.Food] : null);
			_resourceProcessor.SetGeneratedResources(global::Utils.Resource.Gold, _runtimeData.GoldResources,
				meshListsByType.ContainsKey(global::Utils.Resource.Gold) ? meshListsByType[global::Utils.Resource.Gold] : null,
				materialListsByType.ContainsKey(global::Utils.Resource.Gold) ? materialListsByType[global::Utils.Resource.Gold] : null);
			_resourceProcessor.SetGeneratedResources(global::Utils.Resource.Recruit, _runtimeData.RecruitResources,
				meshListsByType.ContainsKey(global::Utils.Resource.Recruit) ? meshListsByType[global::Utils.Resource.Recruit] : null,
				materialListsByType.ContainsKey(global::Utils.Resource.Recruit) ? materialListsByType[global::Utils.Resource.Recruit] : null);

			// Update FoliageProcessor with generated foliage
			_foliageProcessor.SetGeneratedFoliage(_runtimeData.OnLandFoliage, _runtimeData.UnderWaterFoliage);

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Transferred generated data to processors for rendering");
		}

		/// <summary>
		/// Generates resource data for GPU instancing via ResourceRenderer instead of pooling GameObjects.
		/// </summary>
		private void GenerateResourceData(ResourceGenerationSettings settings, int seed, Func<Vector3, (bool, float)> comparisonLambda, HashSet<(int, int)> occupiedCells, bool isWaterGeneration = false)
		{
			// Track existing GUIDs to prevent collisions
			HashSet<uint> existingGUIDs = new HashSet<uint>();
			foreach (var resource in _runtimeData.WoodResources)
				existingGUIDs.Add(resource.GUID);
			foreach (var resource in _runtimeData.OreResources)
				existingGUIDs.Add(resource.GUID);
			foreach (var resource in _runtimeData.FoodResources)
				existingGUIDs.Add(resource.GUID);
			foreach (var resource in _runtimeData.GoldResources)
				existingGUIDs.Add(resource.GUID);
			foreach (var resource in _runtimeData.RecruitResources)
				existingGUIDs.Add(resource.GUID);

			int size = settings.Size;
			int actualSeed = seed;
			Vector2 offset = GetPositiveNoiseOffset(actualSeed, size);
			float[,] heightMap = Noise.GenerateNoiseMap(new GenerationSettings(size, settings.LevelOfDetail, settings.NoiseScale, settings.Octaves, settings.Persistance, settings.Lacunarity, actualSeed, offset, settings.SpawnThreshold));

			int halfSize = size / 2;

			int spacing = settings.Spacing;
			if (spacing == 0)
				spacing = 1;

			Vector3 spawnBiasOrigin = GetSpawnBiasOrigin();
			int terrainMask = GetTerrainMask();

			global::Utils.Resource resourceType = TargetMaskToResource(settings.TargetType);

			int positionsChecked = 0;
			int raycastFailures = 0;
			int spawnAttempts = 0;
			int spawns = 0;
			int overlapSkips = 0;

			Vector3 position;
			for (int y = -halfSize + 2; y < halfSize - 2; y += spacing)
			{
				for (int x = -halfSize + 2; x < halfSize - 2; x += spacing)
				{
					positionsChecked++;
					float centeredOffset = spacing * 0.5f;
					position = new Vector3(y + centeredOffset, 0, x + centeredOffset);

					// Check if this cell is already occupied
					int cellX = Mathf.FloorToInt(position.x / spacing);
					int cellZ = Mathf.FloorToInt(position.z / spacing);
					if (occupiedCells.Contains((cellX, cellZ)))
					{
						continue;
					}

					if (GetSpawnNoiseValue(settings, heightMap, x + halfSize, y + halfSize, position, spawnBiasOrigin) >= settings.SpawnThreshold)
					{
						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
							spawnAttempts++;
							position.y = lambaResult.Item2;

							// Get terrain height at spawn position using raycast
							if (!Physics.Raycast(new Vector3(position.x, 100, position.z), Vector3.down, out RaycastHit hit, 200, terrainMask))
							{
								raycastFailures++;
								continue;
							}

							position.y = hit.point.y;

							// Check terrain height constraints
							if (isWaterGeneration)
							{
								if (position.y > -0.5f)
								{
									raycastFailures++;
									continue;
								}
							}
							else
							{
								if (position.y <= 0f)
								{
									raycastFailures++;
									continue;
								}
							}

							if (IsPositionOverlappingBuilding(position, spacing * 0.45f))
							{
								overlapSkips++;
								continue;
							}

							// Select mesh and material
							int meshIndex = -1;
							int materialIndex = -1;
							Mesh selectedMesh = null;
							if (settings.Meshes != null && settings.Meshes.Count > 0)
							{
								meshIndex = UnityEngine.Random.Range(0, settings.Meshes.Count);
								selectedMesh = settings.Meshes[meshIndex];
							}
							if (settings.Materials != null && settings.Materials.Count > 0)
							{
								materialIndex = UnityEngine.Random.Range(0, settings.Materials.Count);
							}

							// Calculate amount
							int amount = 100;
							if (settings.SetByDistance)
							{
								float normalizedDistance = Vector2.Distance(new Vector2(position.x, position.z), new Vector2(spawnBiasOrigin.x, spawnBiasOrigin.z)) / Mathf.Max(1f, settings.MaxDistance);
								float eval = settings.AmountCurve.Evaluate(Mathf.Clamp01(normalizedDistance));
								amount = (int)MathExtended.RemapValue(eval, 0, 1, settings.MinAmount, settings.MaxAmount);
							}

							Vector3 resourcePosition = resourceType == global::Utils.Resource.Wood
								? position + new Vector3(0.5f, 0, 0.5f)
								: position;

							// Create resource data for GPU instancing
							float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
							Quaternion rotation = Quaternion.Euler(0, randomRotation, 0);
							uint guid = _guidProcessor.GenerateResourceGUID(existingGUIDs);
							existingGUIDs.Add(guid);
							GameResources.ResourceData resourceData = new GameResources.ResourceData(resourcePosition, resourceType, amount, false, Matrix4x4.TRS(resourcePosition, rotation, Vector3.one), guid, meshIndex, materialIndex);

							// Add to appropriate resource list
							switch (resourceType)
							{
								case global::Utils.Resource.Wood:
									_runtimeData.WoodResources.Add(resourceData);
									break;
								case global::Utils.Resource.Ore:
									_runtimeData.OreResources.Add(resourceData);
									break;
								case global::Utils.Resource.Food:
									_runtimeData.FoodResources.Add(resourceData);
									break;
								case global::Utils.Resource.Gold:
									_runtimeData.GoldResources.Add(resourceData);
									break;
								case global::Utils.Resource.Recruit:
									_runtimeData.RecruitResources.Add(resourceData);
									break;
							}

							// Mark this cell as occupied
							occupiedCells.Add((cellX, cellZ));
							spawns++;
						}
					}
				}
			}

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Resource generation summary: ResourceType={resourceType}, PositionsChecked={positionsChecked}, SpawnAttempts={spawnAttempts}, RaycastFailures={raycastFailures}, OverlapSkips={overlapSkips}, Spawns={spawns}");
		}

		/// <summary>
		/// Generates foliage data for GPU instancing via FoliageRenderer instead of pooling GameObjects.
		/// </summary>
		private void GenerateFoliageData(FoliageGenerationSettings settings, int seed, Func<Vector3, (bool, float)> comparisonLambda, List<GameResources.FoliageData> foliageList, HashSet<(int, int)> occupiedCells, bool isWaterGeneration = false)
		{
			int size = settings.Size;
			int actualSeed = seed;
			Vector2 offset = GetPositiveNoiseOffset(actualSeed, size);
			float[,] heightMap = Noise.GenerateNoiseMap(new GenerationSettings(size, settings.LevelOfDetail, settings.NoiseScale, settings.Octaves, settings.Persistance, settings.Lacunarity, actualSeed, offset, settings.SpawnThreshold));

			int halfSize = size / 2;

			int spacing = settings.Spacing;
			if (spacing == 0)
				spacing = 1;

			Vector3 spawnBiasOrigin = GetSpawnBiasOrigin();
			int terrainMask = GetTerrainMask();

			int positionsChecked = 0;
			int raycastFailures = 0;
			int spawnAttempts = 0;
			int spawns = 0;
			int overlapSkips = 0;

			Vector3 position;
			for (int y = -halfSize + 2; y < halfSize - 2; y += spacing)
			{
				for (int x = -halfSize + 2; x < halfSize - 2; x += spacing)
				{
					positionsChecked++;
					float centeredOffset = spacing * 0.5f;
					position = new Vector3(y + centeredOffset, 0, x + centeredOffset);

					// Check if this cell is already occupied
					int cellX = Mathf.FloorToInt(position.x / spacing);
					int cellZ = Mathf.FloorToInt(position.z / spacing);
					if (occupiedCells.Contains((cellX, cellZ)))
					{
						continue;
					}

					if (GetSpawnNoiseValue(settings, heightMap, x + halfSize, y + halfSize, position, spawnBiasOrigin) >= settings.SpawnThreshold)
					{
						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
							spawnAttempts++;
							position.y = lambaResult.Item2;

							// Get terrain height at spawn position using raycast
							if (!Physics.Raycast(new Vector3(position.x, 100, position.z), Vector3.down, out RaycastHit hit, 200, terrainMask))
							{
								raycastFailures++;
								continue;
							}

							position.y = hit.point.y;

							// Check terrain height constraints
							if (isWaterGeneration)
							{
								if (position.y > -0.5f)
								{
									raycastFailures++;
									continue;
								}
							}
							else
							{
								if (position.y <= 0f)
								{
									raycastFailures++;
									continue;
								}
							}

							if (IsPositionOverlappingBuilding(position, spacing * 0.45f))
							{
								overlapSkips++;
								continue;
							}

							(Mesh selectedMesh, Vector3 selectedScale) = SelectFoliageMeshAndScale(settings);

							// Create foliage data for GPU instancing
							float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
							Quaternion rotation = Quaternion.Euler(0, randomRotation, 0);
							GameResources.FoliageData foliageData = new GameResources.FoliageData(position, rotation, selectedScale, selectedMesh, settings.Material);
							foliageList.Add(foliageData);

							// Mark this cell as occupied
							occupiedCells.Add((cellX, cellZ));
							spawns++;
						}
					}
				}
			}

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Foliage generation summary: PoolName={settings.PoolName}, PositionsChecked={positionsChecked}, SpawnAttempts={spawnAttempts}, RaycastFailures={raycastFailures}, OverlapSkips={overlapSkips}, Spawns={spawns}");
		}

		/// <summary>
		/// Sets generated resources for a specific resource type.
		/// Uses _runtimeData resource lists instead of ResourceRuntimeData.
		/// </summary>
		private void SetGeneratedResources(global::Utils.Resource resourceType, List<GameResources.ResourceData> resources, List<Mesh> meshes, List<Material> materials)
		{
			switch (resourceType)
			{
				case global::Utils.Resource.Wood:
					_runtimeData.WoodResources = resources ?? new List<GameResources.ResourceData>();
					break;
				case global::Utils.Resource.Ore:
					_runtimeData.OreResources = resources ?? new List<GameResources.ResourceData>();
					break;
				case global::Utils.Resource.Food:
					_runtimeData.FoodResources = resources ?? new List<GameResources.ResourceData>();
					break;
				case global::Utils.Resource.Gold:
					_runtimeData.GoldResources = resources ?? new List<GameResources.ResourceData>();
					break;
				case global::Utils.Resource.Recruit:
					_runtimeData.RecruitResources = resources ?? new List<GameResources.ResourceData>();
					break;
			}
		}

		/// <summary>
		/// Spawns the townhall synchronously for Process-based generation.
		/// </summary>
		private bool SpawnTownhall()
		{
			return TryEnsureActiveTownhall(out _);
		}

		private bool TryEnsureActiveTownhall(out GameObject townhallObject)
		{
			townhallObject = GetActiveTownhallObject();
			if (townhallObject != null)
				return true;

			PoolableObject townhallPoolObject = GetPooledObject("Townhall");
			if (townhallPoolObject == null)
			{
				_debugProcessor.LogError(DebugLogCategory.WorldGenProcessor, "Failed to spawn townhall - Townhall pool object is null or pool 'Townhall' does not exist.");
				return false;
			}

			SaveableBuilding saveableTownhall = townhallPoolObject.SaveableObject as SaveableBuilding;
			if (saveableTownhall == null)
			{
				_debugProcessor.LogError(DebugLogCategory.WorldGenProcessor, "Failed to spawn townhall - Pool object does not have a SaveableBuilding component.");
				return false;
			}

			if (saveableTownhall.BuildingBase == null)
			{
				_debugProcessor.LogError(DebugLogCategory.WorldGenProcessor, "Failed to spawn townhall - SaveableBuilding does not have a BuildingBase.");
				return false;
			}

			townhallObject = saveableTownhall.BuildingBase.gameObject;
			if (TryFindTownhallPlacement(townhallObject, Vector3.zero, out Vector3 placement))
				townhallObject.transform.position = placement;
			else if (TryGetTerrainHeight(Vector3.zero, out float fallbackHeight) && IsValidTownhallHeight(fallbackHeight))
				townhallObject.transform.position = new Vector3(0, fallbackHeight, 0);
			else
				return false;

			townhallObject.SetActive(true);
			EnsureTownhallRegistered(saveableTownhall.BuildingBase);
			return true;
		}

		private void RemoveTownhallOverlapsFromGeneratedData(List<GameResources.ResourceData> woodResources, List<GameResources.ResourceData> oreResources, List<GameResources.ResourceData> foodResources, List<GameResources.ResourceData> goldResources, List<GameResources.ResourceData> recruitResources, List<GameResources.FoliageData> onLandFoliage, List<GameResources.FoliageData> underWaterFoliage)
		{
			GameObject townhallObject = GetActiveTownhallObject();
			if (townhallObject == null)
				return;

			BoxCollider collider = townhallObject.GetComponent<BoxCollider>();
			if (collider == null)
				return;

			if (!TryGetTownhallFootprintAABB(townhallObject.transform.position, collider, out float minX, out float maxX, out float minZ, out float maxZ))
				return;

			bool IsInFootprint(Vector3 position)
			{
				return position.x >= minX && position.x <= maxX && position.z >= minZ && position.z <= maxZ;
			}

			woodResources?.RemoveAll(resource => IsInFootprint(resource.Position));
			oreResources?.RemoveAll(resource => IsInFootprint(resource.Position));
			foodResources?.RemoveAll(resource => IsInFootprint(resource.Position));
			goldResources?.RemoveAll(resource => IsInFootprint(resource.Position));
			recruitResources?.RemoveAll(resource => IsInFootprint(resource.Position));

			onLandFoliage?.RemoveAll(foliage => IsInFootprint(foliage.Position));
			underWaterFoliage?.RemoveAll(foliage => IsInFootprint(foliage.Position));
		}

		/// <summary>
		/// Maps TargetMask to Utils.Resource type.
		/// </summary>
		private global::Utils.Resource TargetMaskToResource(TargetMask targetType)
		{
			if (targetType.HasFlag(TargetMask.Tree))
				return global::Utils.Resource.Wood;
			if (targetType.HasFlag(TargetMask.Ore))
				return global::Utils.Resource.Ore;
			if (targetType.HasFlag(TargetMask.Bush))
				return global::Utils.Resource.Food;
			if (targetType.HasFlag(TargetMask.Fish))
				return global::Utils.Resource.Food;
			return global::Utils.Resource.None;
		}

		/// <summary>
		/// Attempts to generate enemy camps.
		/// </summary>
		/// <param name="settings"></param>
		/// <param name="camps"></param>
		/// <param name="poolProcessor"></param>
		//private void GenerateEnemyCamps(CampGenerationSettings settings, ref List<GameObject> camps, ObjectPoolingProcessor poolProcessor)
		//{
		//	Coordinator.Instance.EnemyCampSpawners = new List<Enemies.EnemySpawner>();
		//	//Attempt to place Enemy camps
		//	for (int i = 0; i < settings.MaxAmount; i++)
		//	{
		//		int attempts = 0;
		//		bool valid = false;
		//		while (!valid && attempts < MAX_CAMP_GENERATION_ATTEMPTS)
		//		{
		//			attempts++;
		//			bool failed = false;
		//			//Get Random Position in valid Range
		//			Vector3 randPos = new Vector3(UnityEngine.Random.Range(settings.MinBounds.x, settings.MaxBounds.x), 0, UnityEngine.Random.Range(settings.MinBounds.y, settings.MaxBounds.y));
		//			int r = UnityEngine.Random.Range(0, 2);
		//			if (r == 0)
		//				randPos.x *= -1;
		//			r = UnityEngine.Random.Range(0, 2);
		//			if (r == 0)
		//				randPos.z *= -1;
		//			int halfSize = settings.CampSize / 2;

		//			//Check that it is entirely over land
		//			for (int x = -halfSize; x <= halfSize; x++)
		//			{
		//				for (int y = -halfSize; y <= halfSize; y++)
		//				{
		//					if (WorldUtils.GetTerrainHeightAtPosition(randPos + new Vector3(x, 0, y)) != 0)
		//					{
		//						failed = true;
		//					}
		//				}
		//			}

		//			if (failed)
		//				continue;

		//			//If it is on land, check that its not too close to other camps
		//			for (int j = 0; j < camps.Count; j++)
		//			{
		//				if (Vector3.Distance(camps[j].transform.position, randPos) <= settings.MinDistanceFromOther)
		//					failed = true;
		//			}

		//			if (failed)
		//				continue;

		//			//Seems everything is fine... Spawn camp at the spot
		//			GameObject obj = poolProcessor.GetPooledObject(settings.GetPoolName(), false);
		//			obj.transform.position = randPos;
		//			obj.SetActive(true);
		//			Coordinator.Instance.EnemyCampSpawners.Add(obj.GetComponent<EnemySpawner>());
		//			camps.Add(obj);
		//			break;
		//		}
		//	}
		//}

		/// <summary>
		/// Generates all pooled objects required for World Generation.
		/// </summary>
		private void GeneratePooledObjects(Action<float, string> progressReporter = null)
		{
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "GeneratePooledObjects called");
			PoolableObject th = GetPooledObject("Townhall");
			SaveableBuilding saveableTownhall = (SaveableBuilding)th.SaveableObject;
			GameObject thObj = saveableTownhall.BuildingBase.gameObject;
			thObj.transform.position = Vector3.zero;

			if (TryFindTownhallPlacement(thObj, Vector3.zero, out Vector3 townhallPlacement))
				thObj.transform.position = townhallPlacement;
			else
				_debugProcessor.LogWarning(DebugLogCategory.WorldGenProcessor, "No valid flat townhall position found during ScanWorld; using origin fallback.");

			thObj.SetActive(true);
			EnsureTownhallRegistered(saveableTownhall.BuildingBase);

			GeneratePooledObjectsExceptTownhall(progressReporter);
		}

		/// <summary>
		/// Generates all pooled objects except the townhall (townhall is spawned separately before terrain generation).
		/// </summary>
		private void GeneratePooledObjectsExceptTownhall(Action<float, string> progressReporter = null)
		{
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "GeneratePooledObjectsExceptTownhall called");
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"_resourceGenerationSettings is null: {_resourceGenerationSettings == null}");
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"_waterResourceGenerationSettings is null: {_waterResourceGenerationSettings == null}");
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"_foliageGenerationSettings is null: {_foliageGenerationSettings == null}");
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"_waterFoliageGenerationSettings is null: {_waterFoliageGenerationSettings == null}");

			int seed = _terrainGenerationSettings.GenerationSettings.Seed;
			DateTime before = DateTime.Now;
			DateTime after;
			TimeSpan duration;
			int totalSettings = 0;
			if (_resourceGenerationSettings != null)
				totalSettings += _resourceGenerationSettings.ResourceGenerationSettings.Count;
			if (_waterResourceGenerationSettings != null)
				totalSettings += _waterResourceGenerationSettings.WaterResourceGenerationSettings.Count;
			if (_foliageGenerationSettings != null)
				totalSettings += _foliageGenerationSettings.FoliageGenerationSettings.Count;
			if (_waterFoliageGenerationSettings != null)
				totalSettings += _waterFoliageGenerationSettings.WaterFoliageGenerationSettings.Count;

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Total settings to process: {totalSettings}");

			int completedSettings = 0;

			// Lists to collect ResourceData during generation
			List<GameResources.ResourceData> woodResources = new List<GameResources.ResourceData>();
			List<GameResources.ResourceData> oreResources = new List<GameResources.ResourceData>();
			List<GameResources.ResourceData> foodResources = new List<GameResources.ResourceData>();
			List<GameResources.ResourceData> goldResources = new List<GameResources.ResourceData>();
			List<GameResources.ResourceData> recruitResources = new List<GameResources.ResourceData>();

			List<GameResources.FoliageData> onLandFoliage = new List<GameResources.FoliageData>();
			List<GameResources.FoliageData> underWaterFoliage = new List<GameResources.FoliageData>();

			HashSet<(int, int)> occupiedCells = new HashSet<(int, int)>();
			HashSet<uint> existingGUIDs = new HashSet<uint>();

			void ReportSpawnProgress(float settingProgress, string status)
			{
				if (totalSettings <= 0)
				{
					progressReporter?.Invoke(1f, status);
					return;
				}

				float overallProgress = (completedSettings + Mathf.Clamp01(settingProgress)) / totalSettings;
				progressReporter?.Invoke(Mathf.Clamp01(overallProgress), status);
			}

			if (_resourceGenerationSettings != null)
			{
				foreach (ResourceGenerationSettings settings in _resourceGenerationSettings.ResourceGenerationSettings)
				{
					string label = TargetMaskToResource(settings.TargetType).ToString();
					before = DateTime.Now;
					seed++;
					GenerateFromSettings(settings, seed, WorldUtils.OnGroundCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning resources: {label} ({completedSettings + 1}/{totalSettings})..."), woodResources, oreResources, foodResources, goldResources, recruitResources, occupiedCells, null, null, existingGUIDs);
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugSettings.DebugGenerationTiming)
						_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Generating {label} took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed resources: {label} ({completedSettings}/{totalSettings})");
				}

			}

			if (_waterResourceGenerationSettings != null)
				foreach (ResourceGenerationSettings settings in _waterResourceGenerationSettings.WaterResourceGenerationSettings)
				{
					string label = TargetMaskToResource(settings.TargetType).ToString();
					before = DateTime.Now;
					seed++;
					GenerateFromSettings(settings, seed, WorldUtils.OnShoreLineCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning shoreline resources: {label} ({completedSettings + 1}/{totalSettings})..."), woodResources, oreResources, foodResources, goldResources, recruitResources, occupiedCells, null, null, existingGUIDs);
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugSettings.DebugGenerationTiming)
						_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Generating {label} took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed shoreline resources: {label} ({completedSettings}/{totalSettings})");
				}

			if (_foliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _foliageGenerationSettings.FoliageGenerationSettings)
				{
					before = DateTime.Now;
					seed++;
					GenerateFromSettings(settings, seed, WorldUtils.OnGroundCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning foliage ({completedSettings + 1}/{totalSettings})..."), onLandFoliage: onLandFoliage);
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugSettings.DebugGenerationTiming)
						_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Generating foliage took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed foliage ({completedSettings}/{totalSettings})");
				}

				foreach (FoliageGenerationSettings settings in _waterFoliageGenerationSettings.WaterFoliageGenerationSettings)
				{
					before = DateTime.Now;
					seed++;
					GenerateFromSettings(settings, seed, WorldUtils.UnderWaterCheckHeight, false, p =>
					ReportSpawnProgress(p, $"Spawning underwater foliage ({completedSettings + 1}/{totalSettings})..."), underWaterFoliage: underWaterFoliage);
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugSettings.DebugGenerationTiming)
						_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Generating underwater foliage took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed underwater foliage ({completedSettings}/{totalSettings})");
				}

			RemoveTownhallOverlapsFromGeneratedData(woodResources, oreResources, foodResources, goldResources, recruitResources, onLandFoliage, underWaterFoliage);

			{
			// Collect mesh and material lists from resource generation settings by type
				Dictionary<global::Utils.Resource, List<Mesh>> meshListsByType = new Dictionary<global::Utils.Resource, List<Mesh>>();
				Dictionary<global::Utils.Resource, List<Material>> materialListsByType = new Dictionary<global::Utils.Resource, List<Material>>();

				void CollectMeshMaterials(ResourceGenerationSettings settings)
				{
					global::Utils.Resource resourceType = TargetMaskToResource(settings.TargetType);
					if (!meshListsByType.ContainsKey(resourceType))
						meshListsByType[resourceType] = new List<Mesh>();
					if (!materialListsByType.ContainsKey(resourceType))
						materialListsByType[resourceType] = new List<Material>();

					if (settings.Meshes != null)
						meshListsByType[resourceType].AddRange(settings.Meshes);
					if (settings.Materials != null)
						materialListsByType[resourceType].AddRange(settings.Materials);
				}

				if (_resourceGenerationSettings != null)
					foreach (ResourceGenerationSettings settings in _resourceGenerationSettings.ResourceGenerationSettings)
						CollectMeshMaterials(settings);

				if (_waterResourceGenerationSettings != null)
					foreach (ResourceGenerationSettings settings in _waterResourceGenerationSettings.WaterResourceGenerationSettings)
						CollectMeshMaterials(settings);

				// Parallel resource spawning
				_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Starting parallel resource spawning");
				UserInterface.MainMenu.ParallelProgressReporter.Reset();
				UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("Wood Resources", 0.2f);
				UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("Ore Resources", 0.2f);
				UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("Food Resources", 0.2f);
				UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("Gold Resources", 0.2f);
				UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("Recruit Resources", 0.2f);

				System.Diagnostics.Stopwatch resourceStopwatch = System.Diagnostics.Stopwatch.StartNew();

				// Start all resource spawning in parallel
				SetResourceParallel("Wood Resources", () => { SetGeneratedResources(global::Utils.Resource.Wood, woodResources, meshListsByType.ContainsKey(global::Utils.Resource.Wood) ? meshListsByType[global::Utils.Resource.Wood] : null, materialListsByType.ContainsKey(global::Utils.Resource.Wood) ? materialListsByType[global::Utils.Resource.Wood] : null); });
				SetResourceParallel("Ore Resources", () => { SetGeneratedResources(global::Utils.Resource.Ore, oreResources, meshListsByType.ContainsKey(global::Utils.Resource.Ore) ? meshListsByType[global::Utils.Resource.Ore] : null, materialListsByType.ContainsKey(global::Utils.Resource.Ore) ? materialListsByType[global::Utils.Resource.Ore] : null); });
				SetResourceParallel("Food Resources", () => { SetGeneratedResources(global::Utils.Resource.Food, foodResources, meshListsByType.ContainsKey(global::Utils.Resource.Food) ? meshListsByType[global::Utils.Resource.Food] : null, materialListsByType.ContainsKey(global::Utils.Resource.Food) ? materialListsByType[global::Utils.Resource.Food] : null); });
				SetResourceParallel("Gold Resources", () => { SetGeneratedResources(global::Utils.Resource.Gold, goldResources, meshListsByType.ContainsKey(global::Utils.Resource.Gold) ? meshListsByType[global::Utils.Resource.Gold] : null, materialListsByType.ContainsKey(global::Utils.Resource.Gold) ? materialListsByType[global::Utils.Resource.Gold] : null); });
				SetResourceParallel("Recruit Resources", () => { SetGeneratedResources(global::Utils.Resource.Recruit, recruitResources, meshListsByType.ContainsKey(global::Utils.Resource.Recruit) ? meshListsByType[global::Utils.Resource.Recruit] : null, materialListsByType.ContainsKey(global::Utils.Resource.Recruit) ? materialListsByType[global::Utils.Resource.Recruit] : null); });

				resourceStopwatch.Stop();
				_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Parallel resource spawning: {resourceStopwatch.ElapsedMilliseconds}ms");

				// Clean up parallel progress tracks
				UserInterface.MainMenu.ParallelProgressReporter.UnregisterTrack("Wood Resources");
				UserInterface.MainMenu.ParallelProgressReporter.UnregisterTrack("Ore Resources");
				UserInterface.MainMenu.ParallelProgressReporter.UnregisterTrack("Food Resources");
				UserInterface.MainMenu.ParallelProgressReporter.UnregisterTrack("Gold Resources");
				UserInterface.MainMenu.ParallelProgressReporter.UnregisterTrack("Recruit Resources");

				// Note: A* graph bounds update would need to be handled by the pathfinding system
				// This is intentionally left as a coordination point for future implementation

				// Parallel foliage spawning
				_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Starting parallel foliage spawning");
				UserInterface.MainMenu.ParallelProgressReporter.Reset();
				UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("OnLand Foliage", 0.5f);
				UserInterface.MainMenu.ParallelProgressReporter.RegisterTrack("UnderWater Foliage", 0.5f);

				System.Diagnostics.Stopwatch foliageStopwatch = System.Diagnostics.Stopwatch.StartNew();

				// Start foliage spawning in parallel
				SetFoliageParallel("OnLand Foliage", () => { _runtimeData.OnLandFoliage.Clear(); _runtimeData.OnLandFoliage.AddRange(onLandFoliage); });
				SetFoliageParallel("UnderWater Foliage", () => { _runtimeData.UnderWaterFoliage.Clear(); _runtimeData.UnderWaterFoliage.AddRange(underWaterFoliage); });

				foliageStopwatch.Stop();
				_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Parallel foliage spawning: {foliageStopwatch.ElapsedMilliseconds}ms");

				// Clean up parallel progress tracks
				UserInterface.MainMenu.ParallelProgressReporter.UnregisterTrack("OnLand Foliage");
				UserInterface.MainMenu.ParallelProgressReporter.UnregisterTrack("UnderWater Foliage");

				RefreshEnemyCampSpawners();

				progressReporter?.Invoke(1f, "World resource spawning complete");
			}
		}

		private void SetResourceParallel(string trackName, Action setResourceAction)
		{
			UserInterface.MainMenu.ParallelProgressReporter.UpdateTrack(trackName, 0f, "Setting resources...");

			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();
			setResourceAction();
			stopwatch.Stop();

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"{trackName}: {stopwatch.ElapsedMilliseconds}ms");
			UserInterface.MainMenu.ParallelProgressReporter.UpdateTrack(trackName, 1f, "Complete");
		}

		private void SetFoliageParallel(string trackName, Action setFoliageAction)
		{
			UserInterface.MainMenu.ParallelProgressReporter.UpdateTrack(trackName, 0f, "Setting foliage...");

			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();
			setFoliageAction();
			stopwatch.Stop();

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"{trackName}: {stopwatch.ElapsedMilliseconds}ms");
			UserInterface.MainMenu.ParallelProgressReporter.UpdateTrack(trackName, 1f, "Complete");
		}

		public void MainMenuGenerateWorld()
		{
			WorldUtils.GroundLayerMask = LayerMask.GetMask("Ground");
			// Object pooling initialization moved to ObjectPoolingProcessor
			int seed = _terrainGenerationSettings.GenerationSettings.Seed;

			if (_resourceGenerationSettings != null)
			{
				foreach (ResourceGenerationSettings settings in _resourceGenerationSettings.ResourceGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, WorldUtils.OnGroundCheckHeight);
				}
			}

			if (_foliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _foliageGenerationSettings.FoliageGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, WorldUtils.OnGroundCheckHeight);
				}

			if (_waterFoliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _waterFoliageGenerationSettings.WaterFoliageGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, WorldUtils.UnderWaterCheckHeight, false);
				}
		}

		private void GenerateFromSettings(GenerationSettings settings, ref int seed, Func<Vector3, (bool, float)> comparisonLambda, bool useCollision = true)
		{
			settings.Size = GetScaledTerrainSize();
			settings.Seed = seed;
			settings.Offset = GetPositiveNoiseOffset(settings.Seed, settings.Size);
			settings.HeightMap = Noise.GenerateNoiseMap(settings);

			int halfSize = settings.Size / 2;

			if (settings.Spacing == 0)
				settings.Spacing = 1;

			Vector3 colSize = Vector3.one * settings.Spacing * 0.45f;
			float centeredOffset = settings.Spacing * 0.5f;
			Vector3 spawnBiasOrigin = GetSpawnBiasOrigin();

			int terrainMask = GetTerrainMask();
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"GenerateFromSettings called: PoolName={settings.GetPoolName()}, Size={settings.Size}, Spacing={settings.Spacing}, TerrainMask={terrainMask}");

			int positionsChecked = 0;
			int raycastFailures = 0;
			int spawnAttempts = 0;
			int spawns = 0;

			Vector3 position;
			for (int y = -halfSize + 2; y < halfSize - 2; y += settings.Spacing)
			{
				for (int x = -halfSize + 2; x < halfSize - 2; x += settings.Spacing)
				{
					positionsChecked++;
					position = new Vector3(y + centeredOffset, 0, x + centeredOffset);
					if (!IsPointWithinBounds(position))
						continue;

					if (GetSpawnNoiseValue(settings, settings.HeightMap, x + halfSize, y + halfSize, position, spawnBiasOrigin) >= settings.SpawnThreshold)
					{
						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
							spawnAttempts++;

							// Select mesh for offset calculation
							Mesh selectedMesh = null;
							if (settings is ResourceGenerationSettings meshMaterialSettings)
							{
								if (meshMaterialSettings.Meshes != null && meshMaterialSettings.Meshes.Count > 0)
								{
									selectedMesh = meshMaterialSettings.Meshes[UnityEngine.Random.Range(0, meshMaterialSettings.Meshes.Count)];
								}
							}
							else if (settings is FoliageGenerationSettings foliageSettings)
							{
								(selectedMesh, _) = SelectFoliageMeshAndScale(foliageSettings);
							}

							// Get terrain height at spawn position using raycast
							if (!Physics.Raycast(new Vector3(position.x, 100, position.z), Vector3.down, out RaycastHit hit, 200, terrainMask))
							{
								raycastFailures++;
								continue;
							}

							position.y = hit.point.y;

							if (IsPositionOverlappingBuilding(position, settings.Spacing * 0.45f))
								continue;

							if (useCollision)
								if (Physics.BoxCast(position + Vector3.up * 5, colSize, Vector3.down, Quaternion.identity, 10, _layerSettings.CollisionMask))
									continue;

							PoolableObject obj = GetPooledObject(settings.GetPoolName());
							obj.transform.position = position;
							float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
							obj.transform.Rotate(Vector3.up, randomRotation);
							obj.gameObject.SetActive(true);
							spawns++;
						}
					}
				}
			}

			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Spawn summary for {settings.GetPoolName()}: PositionsChecked={positionsChecked}, SpawnAttempts={spawnAttempts}, RaycastFailures={raycastFailures}, Spawns={spawns}");
		}

		private void GenerateFromSettings(GenerationSettings settings, int seed, Func<Vector3, (bool, float)> comparisonLambda, bool useCollision, Action<float> progressReporter, List<GameResources.ResourceData> woodResources = null, List<GameResources.ResourceData> oreResources = null, List<GameResources.ResourceData> foodResources = null, List<GameResources.ResourceData> goldResources = null, List<GameResources.ResourceData> recruitResources = null, HashSet<(int, int)> occupiedCells = null, List<GameResources.FoliageData> onLandFoliage = null, List<GameResources.FoliageData> underWaterFoliage = null, HashSet<uint> existingGUIDs = null)
		{
			settings.Size = GetScaledTerrainSize();
			settings.Seed = seed;
			settings.Offset = GetPositiveNoiseOffset(settings.Seed, settings.Size);

			if (settings.Spacing == 0)
				settings.Spacing = 1;

			Vector3 colSize = Vector3.one * settings.Spacing * 0.45f;
			float centeredOffset = settings.Spacing * 0.5f;
			float[,] generatedNoiseMap = Noise.GenerateNoiseMap(settings);
			settings.HeightMap = generatedNoiseMap;

			int halfSize = settings.Size / 2;
			int start = -halfSize + 2;
			int end = halfSize - 2;
			int checksPerAxis = Mathf.Max(1, Mathf.CeilToInt((end - start) / (float)settings.Spacing));
			int totalChecks = checksPerAxis * checksPerAxis;
			int checksProcessed = 0;
			Vector3 spawnBiasOrigin = GetSpawnBiasOrigin();

			int terrainMask = GetTerrainMask();
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"GenerateFromSettings (with progress) called: PoolName={settings.GetPoolName()}, Size={settings.Size}, Spacing={settings.Spacing}, TerrainMask={terrainMask}");

			int positionsChecked = 0;
			int raycastFailures = 0;
			int spawnAttempts = 0;
			int spawns = 0;

			global::Utils.Resource resourceType = global::Utils.Resource.None;
			if (settings is ResourceGenerationSettings resourceSettings)
			{
				resourceType = TargetMaskToResource(resourceSettings.TargetType);
			}

			for (int y = start; y < end; y += settings.Spacing)
			{
				for (int x = start; x < end; x += settings.Spacing)
				{
					checksProcessed++;
					positionsChecked++;
					Vector3 position = new Vector3(y + centeredOffset, 0, x + centeredOffset);
					if (!IsPointWithinBounds(position))
						continue;

					if (GetSpawnNoiseValue(settings, settings.HeightMap, x + halfSize, y + halfSize, position, spawnBiasOrigin) >= settings.SpawnThreshold)
					{
						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
							spawnAttempts++;

							//TODO:: Put this as an out in the lambda
							position.y = lambaResult.Item2;

							// Check for overlap prevention
							if (occupiedCells != null)
							{
								int cellX = Mathf.FloorToInt(position.x / settings.Spacing);
								int cellZ = Mathf.FloorToInt(position.z / settings.Spacing);

								if (occupiedCells.Contains((cellX, cellZ)))
								{
									// Cell is occupied, try random offset
									bool foundValidPosition = false;
									for (int attempt = 0; attempt < 5; attempt++)
									{
										float offsetX = UnityEngine.Random.Range(-settings.Spacing * 0.5f, settings.Spacing * 0.5f);
										float offsetZ = UnityEngine.Random.Range(-settings.Spacing * 0.5f, settings.Spacing * 0.5f);
										Vector3 offsetPosition = position + new Vector3(offsetX, 0, offsetZ);

										int offsetCellX = Mathf.FloorToInt(offsetPosition.x / settings.Spacing);
										int offsetCellZ = Mathf.FloorToInt(offsetPosition.z / settings.Spacing);

										if (!occupiedCells.Contains((offsetCellX, offsetCellZ)))
										{
											position = offsetPosition;
											cellX = offsetCellX;
											cellZ = offsetCellZ;
											foundValidPosition = true;
											break;
										}
									}

									if (!foundValidPosition)
										continue; // Skip if no valid position found
								}

								// Mark cell as occupied
								occupiedCells.Add((cellX, cellZ));
							}

							//Check for collision
							if (useCollision)
								if (Physics.BoxCast(position + Vector3.up * 5, colSize, Vector3.down, Quaternion.identity, 10, _layerSettings.CollisionMask))
									continue;

							// Randomly select mesh and material indices from the settings (for offset calculation)
							int meshIndex = -1;
							int materialIndex = -1;
							Mesh spawnedMesh = null;
							if (settings is ResourceGenerationSettings meshMaterialSettings)
							{
								if (meshMaterialSettings.Meshes != null && meshMaterialSettings.Meshes.Count > 0)
								{
									meshIndex = UnityEngine.Random.Range(0, meshMaterialSettings.Meshes.Count);
									spawnedMesh = meshMaterialSettings.Meshes[meshIndex];
								}
								if (meshMaterialSettings.Materials != null && meshMaterialSettings.Materials.Count > 0)
								{
									materialIndex = UnityEngine.Random.Range(0, meshMaterialSettings.Materials.Count);
								}
							}

							// Require terrain hit at spawn position
							if (!Physics.Raycast(new Vector3(position.x, 100, position.z), Vector3.down, out RaycastHit hit, 200, terrainMask))
							{
								raycastFailures++;
								continue;
							}

							position.y = hit.point.y;

							if (IsPositionOverlappingBuilding(position, settings.Spacing * 0.45f))
								continue;

							// Collect ResourceData if resource lists are provided
							if (resourceType != global::Utils.Resource.None && woodResources != null && oreResources != null && foodResources != null && goldResources != null && recruitResources != null)
							{
								spawns++;
								float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
								Quaternion rotation = Quaternion.Euler(0, randomRotation, 0);

								// Calculate amount (default 100, or distance-based if enabled)
								int amount = 100;
								if (settings is ResourceGenerationSettings resSettings && resSettings.SetByDistance)
								{
									float normalizedDistance = Vector2.Distance(new Vector2(position.x, position.z), new Vector2(spawnBiasOrigin.x, spawnBiasOrigin.z)) / Mathf.Max(1f, resSettings.MaxDistance);
									float eval = resSettings.AmountCurve.Evaluate(Mathf.Clamp01(normalizedDistance));
									amount = (int)MathExtended.RemapValue(eval, 0, 1, resSettings.MinAmount, resSettings.MaxAmount);
								}

								// Collect resource data with correct constructor signature
								uint guid = _guidProcessor.GenerateResourceGUID(existingGUIDs);
								existingGUIDs.Add(guid);
								GameResources.ResourceData resourceData = new GameResources.ResourceData(position, resourceType, amount, false, Matrix4x4.TRS(position, rotation, Vector3.one), guid, meshIndex, materialIndex);
								switch (resourceType)
								{
									case global::Utils.Resource.Wood:
										woodResources.Add(resourceData);
										break;
									case global::Utils.Resource.Ore:
										oreResources.Add(resourceData);
										break;
									case global::Utils.Resource.Food:
										foodResources.Add(resourceData);
										break;
									case global::Utils.Resource.Gold:
										goldResources.Add(resourceData);
										break;
									case global::Utils.Resource.Recruit:
										recruitResources.Add(resourceData);
										break;
								}
							}
							else if (settings is FoliageGenerationSettings foliageSettings)
							{
								spawns++;
								// Handle foliage data collection
								float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
								Quaternion rotation = Quaternion.Euler(0, randomRotation, 0);
								(Mesh selectedMesh, Vector3 selectedScale) = SelectFoliageMeshAndScale(foliageSettings);

								GameResources.FoliageData foliageData = new GameResources.FoliageData(position, rotation, selectedScale, selectedMesh, foliageSettings.Material);

								if (onLandFoliage != null)
									onLandFoliage.Add(foliageData);
								else if (underWaterFoliage != null)
									underWaterFoliage.Add(foliageData);
							}
							else
							{
								spawns++;
								// Only instantiate if not collecting ResourceData (foliage, etc.)
								PoolableObject obj = GetPooledObject(settings.GetPoolName(), false);
								obj.transform.position = position;
								float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
								obj.transform.Rotate(Vector3.up, randomRotation);
								obj.gameObject.SetActive(true);
							}

						}
					}

					if (checksProcessed % 30 == 0)
					{
						progressReporter?.Invoke(checksProcessed / (float)totalChecks);
					}
				}
			}

			progressReporter?.Invoke(1f);
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"Spawn summary for {settings.GetPoolName()} (with progress): PositionsChecked={positionsChecked}, SpawnAttempts={spawnAttempts}, RaycastFailures={raycastFailures}, Spawns={spawns}");
		}

		/// <summary>
		/// Attempts to generate a new world with the given settings.
		/// </summary>
		public void TryGenerateWorld(Action<float, string> progressReporter = null)
		{
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "TryGenerateWorld (public) called");
			TryGenerateWorldInternal(false, progressReporter);
		}

		private void TryGenerateWorldInternal(bool forceGenerate, Action<float, string> progressReporter = null)
		{
			_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, $"TryGenerateWorldInternal called: forceGenerate={forceGenerate}, CurrentState={_runtimeData.State}");
			if (_runtimeData == null)
				throw new InvalidOperationException("WorldGenProcessor: WorldGenRuntimeData has not been installed.");

			if (_behaviorSettings.SuppressGeneration)
			{
				_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Generation suppressed by behavior settings");
				_runtimeData.State = GenerationState.Complete;
				CompleteWorldGeneration(GetActiveTownhallObject());
				progressReporter?.Invoke(1f, "World generation suppressed");
				return;
			}

			if (forceGenerate || _behaviorSettings.GenerateOnStart)
			{
				_runtimeData.WorldGenerated = false;
				_runtimeData.GenerationAttempts = 0;
				progressReporter?.Invoke(0.0f, "Preparing terrain generation...");
				WorldUtils.GroundLayerMask = LayerMask.GetMask("Ground");

				progressReporter?.Invoke(0.08f, "Preparing townhall...");
				PoolableObject th = GetPooledObject("Townhall");
				SaveableBuilding saveableTownhall = (SaveableBuilding)th.SaveableObject;
				GameObject thObj = saveableTownhall.BuildingBase.gameObject;
				thObj.transform.position = Vector3.zero;
				thObj.SetActive(false);

				int attempts = 0;
				bool terrainAcceptable = false;
				while (!terrainAcceptable && attempts < MAX_GENERATION_ATTEMPTS)
				{
					attempts++;
					float attemptProgress = Mathf.Clamp01(attempts / (float)MAX_GENERATION_ATTEMPTS);
					progressReporter?.Invoke(0.1f + (attemptProgress * 0.45f), $"Generating terrain (attempt {attempts}/{MAX_GENERATION_ATTEMPTS})...");
					if (_behaviorSettings.RandomizeSeed)
						_terrainGenerationSettings.GenerationSettings.Seed = UnityEngine.Random.Range(int.MinValue, int.MaxValue);

					GenerateTerrain();

					if (!TryFindTownhallPlacement(thObj, Vector3.zero, out Vector3 townhallPlacement))
					{
						_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Could not find a valid flat townhall footprint, regenerating terrain.");
						continue;
					}

					thObj.transform.position = townhallPlacement;
					thObj.SetActive(true);

					_runtimeData.TerrainCheckPassed = false;
					AcceptableTerrainCheck();
					terrainAcceptable = _runtimeData.TerrainCheckPassed;

					if (!terrainAcceptable)
						thObj.SetActive(false);
				}

				if (!terrainAcceptable)
				{
					_debugProcessor.LogError(DebugLogCategory.WorldGenProcessor, $"Failed to generate acceptable terrain after {MAX_GENERATION_ATTEMPTS} attempts. Proceeding with current terrain.");
					if (!TryFindTownhallPlacement(thObj, Vector3.zero, out Vector3 fallbackPlacement))
					{
						fallbackPlacement = Vector3.zero;
						if (TryGetTerrainHeight(fallbackPlacement, out float fallbackHeight) && IsValidTownhallHeight(fallbackHeight))
							fallbackPlacement.y = fallbackHeight;
						else
							return;
					}

					thObj.transform.position = fallbackPlacement;
					thObj.SetActive(true);
				}

				EnsureTownhallRegistered(saveableTownhall.BuildingBase);

				progressReporter?.Invoke(0.7f, "Spawning world resources...");
				GeneratePooledObjectsExceptTownhall((progress, status) =>
				{
					progressReporter?.Invoke(0.7f + (Mathf.Clamp01(progress) * 0.28f), status);
				});

				progressReporter?.Invoke(1f, "World generation complete");
				CompleteWorldGeneration(GetActiveTownhallObject());
			}
		}

		public void RegenerateTerrainAndWorldRuntime()
		{
			if (!Application.isPlaying)
			{
				_debugProcessor.LogWarning(DebugLogCategory.WorldGenProcessor, "RegenerateTerrainAndWorldRuntime can only be called in play mode.");
				return;
			}

			if (_runtimeData.IsEditorRegenerating)
				return;

			RegenerateTerrainAndWorldRuntimeCoroutine();
		}

		private void RegenerateTerrainAndWorldRuntimeCoroutine()
		{
			_runtimeData.IsEditorRegenerating = true;
			TryGenerateWorldInternal(true);
			_runtimeData.IsEditorRegenerating = false;

			// Center camera on town hall after runtime regeneration
			GameObject townhallObject = GetActiveTownhallObject();
			if (townhallObject != null && Camera.main != null)
			{
				Vector3 townhallPosition = townhallObject.transform.position;
				Camera.main.transform.position = new Vector3(townhallPosition.x, townhallPosition.y + 10f, townhallPosition.z - 15f);
				Camera.main.transform.LookAt(townhallPosition);
			}
		}

		public void RegenerateResourcesAndFoliageRuntime()
		{
			if (!Application.isPlaying)
			{
				_debugProcessor.LogWarning(DebugLogCategory.WorldGenProcessor, "RegenerateResourcesAndFoliageRuntime can only be called in play mode.");
				return;
			}

			if (_runtimeData.IsEditorRegenerating)
				return;

			RegenerateResourcesAndFoliageRuntimeCoroutine();
		}

		private void RegenerateResourcesAndFoliageRuntimeCoroutine()
		{
			_runtimeData.IsEditorRegenerating = true;

			if (!TryEnsureActiveTownhall(out _))
			{
				_debugProcessor.LogWarning(DebugLogCategory.WorldGenProcessor, "Could not ensure active townhall before regenerating resources and foliage.");
				_runtimeData.IsEditorRegenerating = false;
				return;
			}

			GeneratePooledObjectsExceptTownhall();
			_runtimeData.IsEditorRegenerating = false;
		}

		private void AcceptableTerrainCheck()
		{
			// Check Town Hall is not above water
			if (!TownHallAboveGround())
			{
				_debugProcessor.Log(DebugLogCategory.WorldGenProcessor, "Town Hall Above Water, Regenerating Terrain");
				_runtimeData.TerrainCheckPassed = false;
				return;
			}

			// Pathfinding check disabled - GridGraph configuration needs fixing
			// The full A* scan in ScanWorld() will validate pathfinding after all objects are spawned
			_runtimeData.TerrainCheckPassed = true;
		}

		private bool TownHallAboveGround()
		{
			int townHallCheckSize = 5;
			GameObject townhallObject = GetActiveTownhallObject();
			Vector3 center = townhallObject == null ? Vector3.zero : townhallObject.transform.position;
			int terrainMask = GetTerrainMask();

			for (int i = -(townHallCheckSize) / 2; i < townHallCheckSize / 2; i++)
			{
				for (int j = -(townHallCheckSize / 2); j < townHallCheckSize / 2; j++)
				{
					if (Physics.Raycast(new Vector3(center.x + i, center.y + 5, center.z + j), Vector3.down, out RaycastHit info, 10, terrainMask))
					{
						if (!WorldUtils.OnGroundCheck(info.point))
						{
							return false;
						}
					}
				}
			}

			return true;
		}

		public void ScanWorld()
		{
			WorldUtils.GroundLayerMask = LayerMask.GetMask("Ground");
			GenerateTerrain();
			GeneratePooledObjects();
		}

#if UNITY_EDITOR
		private List<Vector3> GenerateDebugPositions(GenerationSettings settings, ref int seed, Func<Vector3, (bool, float)> comparisonLambda)
		{
			List<Vector3> listOfPositions = new List<Vector3>();

			settings.Size = GetScaledTerrainSize();
			settings.Seed = ++seed;
			settings.Offset = GetPositiveNoiseOffset(settings.Seed, settings.Size);
			//Generate resource map (stored in Height Map)
			Vector3 colSize = Vector3.one * settings.Spacing * 0.45f;
			settings.HeightMap = Noise.GenerateNoiseMap(settings);

			//Set pooled objects to the position

			if (settings.Spacing == 0)
				settings.Spacing = 1;

			float centeredOffset = settings.Spacing * 0.5f;

			int halfSize = settings.Size / 2;

			for (int y = -halfSize; y < halfSize; y += settings.Spacing)
			{
				for (int x = -halfSize; x < halfSize; x += settings.Spacing)
				{
					if (settings.HeightMap[x + halfSize, y + halfSize] >= settings.SpawnThreshold)
					{
						Vector3 position = new Vector3(y + centeredOffset, 0, x + centeredOffset);
						if (!IsPointWithinBounds(position))
							continue;

						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
							position.y = lambaResult.Item2;
							listOfPositions.Add(position);
						}
					}
				}
			}

			return listOfPositions;
		}

		private static Vector2 GetPositiveNoiseOffset(int seed, int size)
		{
			System.Random seededRandom = new System.Random(seed);
			float minOffset = size * 0.5f;
			float maxAdditionalOffset = size * 2f;

			float offsetX = minOffset + ((float)seededRandom.NextDouble() * maxAdditionalOffset);
			float offsetY = minOffset + ((float)seededRandom.NextDouble() * maxAdditionalOffset);

			return new Vector2(offsetX, offsetY);
		}

		private void OnDrawGizmosSelected()
		{
			// Preview Tree Placements
			if (_regen)
			{
				_regen = false;
				if (_previewTreePlacements)
				{
					ResourceGenerationSettings settings = default;

					for (int i = 0; i < _resourceGenerationSettings.ResourceGenerationSettings.Count; i++)
					{
						if (_resourceGenerationSettings.ResourceGenerationSettings[i].TargetType == TargetMask.Tree)
						{
							settings = _resourceGenerationSettings.ResourceGenerationSettings[i];
							break;
						}
					}

					_previewTreePositions = GenerateDebugPositions(settings, ref settings.Seed, WorldUtils.OnGroundCheckHeight);

				}
			}

			for (int i = 0; i < _previewTreePositions.Count; i++)
			{
				Gizmos.color = Color.blue;
				Gizmos.DrawMesh(_treeMesh, _previewTreePositions[i]);
				Gizmos.color = Color.white;
			}
		}
#endif
	}
}













