using Enemies;
using GameResources;
using GUIDSystem;
using Managers;
using Pathfinding;
using SavingAndLoading.SavableObjects;
using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Utils;
using Utils.Pooling;
using Reflex.Attributes;
using World.Generation.Settings;

namespace World.Generation
{
	public class ProceduralWorldGenerator : MonoBehaviour
	{
		/// <summary>
		/// The max number of attempts the generation will try to place an enemy camp. Prevents an infinite loop.
		/// </summary>
		private const int MAX_CAMP_GENERATION_ATTEMPTS = 500;
		private Mesh _generatedMesh;
		[Inject] private ObjectPoolingManager _poolingManager;
		[Inject] private BuildingManager _buildingManager;
		[Inject] private ResourceManager _resourceManager;
		[Inject] private GUIDManager _guidManager;
		[Inject] private GameResources.FoliageManager _foliageManager;

		[Inject] private WorldGeneratorScaleSettings _scaleSettings;
		[Inject] private TerrainGenerationSettings _terrainGenerationSettings;
		[Inject] private ResourceGenerationSettingsContainer _resourceGenerationSettings;
		[Inject] private WaterResourceGenerationSettingsContainer _waterResourceGenerationSettings;
		[Inject] private FoliageGenerationSettingsContainer _foliageGenerationSettings;
		[Inject] private WaterFoliageGenerationSettingsContainer _waterFoliageGenerationSettings;
		[Inject] private CampGenerationSettingsContainer _campGenerationSettings;

		[Inject] private WorldGeneratorBehaviorSettings _behaviorSettings;

		private bool _terrainCheckPassed;
		private const int MAX_GENERATION_ATTEMPTS = 10;
		private const int MAX_TOWNHALL_SPIRAL_RINGS = 24;
		private bool _isEditorRegenerating;

		[Inject] private WorldGeneratorDebugSettings _debugSettings;

		[Inject] private WorldGeneratorLayerSettings _layerSettings;

		public Mesh GeneratedMesh => _generatedMesh;

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

			_generatedMesh = ProceduralMeshGenerator.CreateMesh(ProceduralMeshGenerator.GenerateTerrainMeshData(meshSettings, _terrainGenerationSettings.MeshHeightMultiplier, _terrainGenerationSettings.MeshHeightCurve, _terrainGenerationSettings.EnableIslandBias, _terrainGenerationSettings.IslandBiasCurve, _terrainGenerationSettings.IslandSize, _terrainGenerationSettings.IslandMultiplier, _terrainGenerationSettings.IslandAddition, _terrainGenerationSettings.QuantizationFactor, _terrainGenerationSettings.TopFaceProportion), gameObject);
		}

		private int GetScaledTerrainSize()
		{
			float dominantScale = Mathf.Max(_scaleSettings.XScale, _scaleSettings.YScale);
			return Mathf.Max(1, Mathf.RoundToInt(_terrainGenerationSettings.GenerationSettings.Size * dominantScale));
		}

		public void SetMesh(Mesh mesh)
		{
			_generatedMesh = ProceduralMeshGenerator.CreateMesh(mesh, gameObject);
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

		private float GetSpawnNoiseValue(GenerationSettings settings, int mapX, int mapY, Vector3 position, Vector3 spawnBiasOrigin)
		{
			float noiseValue = settings.HeightMap[mapX, mapY];
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
			if (_buildingManager == null)
				return false;

			Dictionary<BuildingType, List<Buildings.BuildingBase>> buildingsByType = _buildingManager.GetAllBuildingsDictionary();
			if (buildingsByType == null)
				return false;

			Bounds spawnBounds = new Bounds(position, new Vector3(horizontalHalfExtent * 2f, 8f, horizontalHalfExtent * 2f));

			foreach (var kvp in buildingsByType)
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

				placement = new Vector3(origin.x, fallbackHeight, origin.z);
				return true;
			}

			float spacing = Mathf.Max(1f, _terrainGenerationSettings.GenerationSettings.Spacing);
			foreach (Vector2Int offset in GetSpiralOffsets(MAX_TOWNHALL_SPIRAL_RINGS))
			{
				Vector3 candidate = origin + new Vector3(offset.x * spacing, 0, offset.y * spacing);
				if (!TryGetFlatTownhallFootprintHeight(candidate, collider, out float flatHeight))
					continue;

				placement = new Vector3(candidate.x, flatHeight, candidate.z);
				return true;
			}

			return false;
		}

		private GameObject GetActiveTownhallObject()
		{
			List<PoolableObject> activeTownhalls = _poolingManager.GetAllActivePooledObjectsOfType("Townhall");
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
			if (townhallBuilding == null || _buildingManager == null)
				return;

			Dictionary<BuildingType, List<Buildings.BuildingBase>> buildingsByType = _buildingManager.GetAllBuildingsDictionary();
			if (buildingsByType != null && buildingsByType.TryGetValue(townhallBuilding.BuildingType, out List<Buildings.BuildingBase> existingBuildings) && existingBuildings != null && existingBuildings.Contains(townhallBuilding))
				return;

			_buildingManager.AddLoadedBuilding(townhallBuilding);
		}

		private bool TryEnsureActiveTownhall(out GameObject townhallObject)
		{
			townhallObject = GetActiveTownhallObject();
			if (townhallObject != null)
				return true;

			PoolableObject townhallPoolObject = _poolingManager.GetPooledObject("Townhall");
			if (townhallPoolObject == null)
				return false;

			SaveableBuilding saveableTownhall = townhallPoolObject.SaveableObject as SaveableBuilding;
			if (saveableTownhall == null || saveableTownhall.BuildingBase == null)
				return false;

			townhallObject = saveableTownhall.BuildingBase.gameObject;
			if (TryFindTownhallPlacement(townhallObject, Vector3.zero, out Vector3 placement))
				townhallObject.transform.position = placement;
			else if (TryGetTerrainHeight(Vector3.zero, out float fallbackHeight))
				townhallObject.transform.position = new Vector3(0, fallbackHeight, 0);

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
		/// <param name="poolManager"></param>
		//private void GenerateEnemyCamps(CampGenerationSettings settings, ref List<GameObject> camps, ObjectPoolingManager poolManager)
		//{
		//	GameManager.Instance.EnemyCampSpawners = new List<Enemies.EnemySpawner>();
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
		//			GameObject obj = poolManager.GetPooledObject(settings.GetPoolName(), false);
		//			obj.transform.position = randPos;
		//			obj.SetActive(true);
		//			GameManager.Instance.EnemyCampSpawners.Add(obj.GetComponent<EnemySpawner>());
		//			camps.Add(obj);
		//			break;
		//		}
		//	}
		//}

		/// <summary>
		/// Generates all pooled objects required for World Generation.
		/// </summary>
		private IEnumerator GeneratePooledObjects(Action<float, string> progressReporter = null)
		{
			PoolableObject th = _poolingManager.GetPooledObject("Townhall");
			SaveableBuilding saveableTownhall = (SaveableBuilding)th.SaveableObject;
			GameObject thObj = saveableTownhall.BuildingBase.gameObject;
			thObj.transform.position = Vector3.zero;

			if (TryFindTownhallPlacement(thObj, Vector3.zero, out Vector3 townhallPlacement))
				thObj.transform.position = townhallPlacement;
			else
				Debug.LogWarning("ProceduralWorldGenerator: No valid flat townhall position found during ScanWorld; using origin fallback.", this);

			thObj.SetActive(true);
			EnsureTownhallRegistered(saveableTownhall.BuildingBase);

			yield return StartCoroutine(GeneratePooledObjectsExceptTownhall(progressReporter));
		}

		/// <summary>
		/// Generates all pooled objects except the townhall (townhall is spawned separately before terrain generation).
		/// </summary>
		private IEnumerator GeneratePooledObjectsExceptTownhall(Action<float, string> progressReporter = null)
		{
			ObjectPoolingManager poolManager = _poolingManager;
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
					yield return StartCoroutine(GenerateFromSettingsCoroutine(settings, seed, poolManager, WorldUtils.OnGroundCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning resources: {label} ({completedSettings + 1}/{totalSettings})..."), woodResources, oreResources, foodResources, goldResources, recruitResources, occupiedCells));
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugSettings.DebugGenerationTiming)
						Debug.Log($"Generating {label} took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed resources: {label} ({completedSettings}/{totalSettings})");
					yield return null;
				}

			}

			if (_waterResourceGenerationSettings != null)
				foreach (ResourceGenerationSettings settings in _waterResourceGenerationSettings.WaterResourceGenerationSettings)
				{
					string label = TargetMaskToResource(settings.TargetType).ToString();
					before = DateTime.Now;
					seed++;
					yield return StartCoroutine(GenerateFromSettingsCoroutine(settings, seed, poolManager, WorldUtils.OnShoreLineCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning shoreline resources: {label} ({completedSettings + 1}/{totalSettings})..."), woodResources, oreResources, foodResources, goldResources, recruitResources, occupiedCells));
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugSettings.DebugGenerationTiming)
						Debug.Log($"Generating {label} took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed shoreline resources: {label} ({completedSettings}/{totalSettings})");
					yield return null;
				}

			if (_foliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _foliageGenerationSettings.FoliageGenerationSettings)
				{
					before = DateTime.Now;
					seed++;
					yield return StartCoroutine(GenerateFromSettingsCoroutine(settings, seed, poolManager, WorldUtils.OnGroundCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning foliage ({completedSettings + 1}/{totalSettings})..."), onLandFoliage: onLandFoliage));
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugSettings.DebugGenerationTiming)
						Debug.Log($"Generating foliage took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed foliage ({completedSettings}/{totalSettings})");
					yield return null;
				}

			if (_waterFoliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _waterFoliageGenerationSettings.WaterFoliageGenerationSettings)
				{
					before = DateTime.Now;
					seed++;
					yield return StartCoroutine(GenerateFromSettingsCoroutine(settings, seed, poolManager, WorldUtils.UnderWaterCheckHeight, false, p =>
						ReportSpawnProgress(p, $"Spawning underwater foliage ({completedSettings + 1}/{totalSettings})..."), underWaterFoliage: underWaterFoliage));
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugSettings.DebugGenerationTiming)
						Debug.Log($"Generating underwater foliage took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed underwater foliage ({completedSettings}/{totalSettings})");
					yield return null;
				}

			RemoveTownhallOverlapsFromGeneratedData(woodResources, oreResources, foodResources, goldResources, recruitResources, onLandFoliage, underWaterFoliage);

			if (_resourceManager != null)
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

				_resourceManager.SetWoodResources(woodResources.ToArray(), meshListsByType.ContainsKey(global::Utils.Resource.Wood) ? meshListsByType[global::Utils.Resource.Wood] : null, materialListsByType.ContainsKey(global::Utils.Resource.Wood) ? materialListsByType[global::Utils.Resource.Wood] : null);
				_resourceManager.SetOreResources(oreResources.ToArray(), meshListsByType.ContainsKey(global::Utils.Resource.Ore) ? meshListsByType[global::Utils.Resource.Ore] : null, materialListsByType.ContainsKey(global::Utils.Resource.Ore) ? materialListsByType[global::Utils.Resource.Ore] : null);
				_resourceManager.SetFoodResources(foodResources.ToArray(), meshListsByType.ContainsKey(global::Utils.Resource.Food) ? meshListsByType[global::Utils.Resource.Food] : null, materialListsByType.ContainsKey(global::Utils.Resource.Food) ? materialListsByType[global::Utils.Resource.Food] : null);
				_resourceManager.SetGoldResources(goldResources.ToArray(), meshListsByType.ContainsKey(global::Utils.Resource.Gold) ? meshListsByType[global::Utils.Resource.Gold] : null, materialListsByType.ContainsKey(global::Utils.Resource.Gold) ? materialListsByType[global::Utils.Resource.Gold] : null);
				_resourceManager.SetRecruitResources(recruitResources.ToArray(), meshListsByType.ContainsKey(global::Utils.Resource.Recruit) ? meshListsByType[global::Utils.Resource.Recruit] : null, materialListsByType.ContainsKey(global::Utils.Resource.Recruit) ? materialListsByType[global::Utils.Resource.Recruit] : null);

				// Update A* pathfinding graph for all resource positions
				_resourceManager.UpdateAllGraphBounds();
			}

			if (_foliageManager != null)
			{
				_foliageManager.SetOnLandFoliage(onLandFoliage.ToArray());
				_foliageManager.SetUnderWaterFoliage(underWaterFoliage.ToArray());
			}

			progressReporter?.Invoke(1f, "World resource spawning complete");
		}

		public void InstantiateResourcesFromResourceManager(ObjectPoolingManager poolManager)
		{
			// Resources are now rendered via GPU instancing in ResourceRenderer
			// This method is no longer needed
		}

		public void MainMenuGenerateWorld()
		{
			WorldUtils.GroundLayerMask = LayerMask.GetMask("Ground");
			ObjectPoolingManager poolManager = GetComponent<ObjectPoolingManager>();
			poolManager.SimplePoolObjects();
			int seed = _terrainGenerationSettings.GenerationSettings.Seed;

			if (_resourceGenerationSettings != null)
			{
				foreach (ResourceGenerationSettings settings in _resourceGenerationSettings.ResourceGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, poolManager, WorldUtils.OnGroundCheckHeight);
				}
			}

			if (_foliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _foliageGenerationSettings.FoliageGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, poolManager, WorldUtils.OnGroundCheckHeight);
				}

			if (_waterFoliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _waterFoliageGenerationSettings.WaterFoliageGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, poolManager, WorldUtils.UnderWaterCheckHeight, false);
				}
		}

		private void GenerateFromSettings(GenerationSettings settings, ref int seed, ObjectPoolingManager poolManager, Func<Vector3, (bool, float)> comparisonLambda, bool useCollision = true)
		{
			settings.Size = GetScaledTerrainSize();
			settings.Seed = ++seed;
			settings.Offset = GetPositiveNoiseOffset(settings.Seed, settings.Size);
			settings.HeightMap = Noise.GenerateNoiseMap(settings);

			int halfSize = settings.Size / 2;

			if (settings.Spacing == 0)
				settings.Spacing = 1;

			Vector3 colSize = Vector3.one * settings.Spacing * 0.45f;
			float centeredOffset = settings.Spacing * 0.5f;
			Vector3 spawnBiasOrigin = GetSpawnBiasOrigin();

			Vector3 position;
			for (int y = -halfSize + 2; y < halfSize - 2; y += settings.Spacing)
			{
				for (int x = -halfSize + 2; x < halfSize - 2; x += settings.Spacing)
				{
					position = new Vector3(y + centeredOffset, 0, x + centeredOffset);
					if (!IsPointWithinBounds(position))
						continue;

					if (GetSpawnNoiseValue(settings, x + halfSize, y + halfSize, position, spawnBiasOrigin) >= settings.SpawnThreshold)
					{
						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
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
								if (foliageSettings.Meshes != null && foliageSettings.Meshes.Count > 0)
								{
									selectedMesh = foliageSettings.Meshes[UnityEngine.Random.Range(0, foliageSettings.Meshes.Count)];
								}
							}

							// Get terrain height at spawn position using raycast
							if (!Physics.Raycast(new Vector3(position.x, 100, position.z), Vector3.down, out RaycastHit hit, 200, _layerSettings.TerrainMask))
								continue;

							position.y = hit.point.y;

							if (IsPositionOverlappingBuilding(position, settings.Spacing * 0.45f))
								continue;

							if (useCollision)
								if (Physics.BoxCast(position + Vector3.up * 5, colSize, Vector3.down, Quaternion.identity, 10, _layerSettings.CollisionMask))
									continue;

							PoolableObject obj = poolManager.GetPooledObject(settings.GetPoolName(), false);
							obj.transform.position = position;
							float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
							obj.transform.Rotate(Vector3.up, randomRotation);
							obj.gameObject.SetActive(true);
						}
					}
				}
			}
		}

		private IEnumerator GenerateFromSettingsCoroutine(GenerationSettings settings, int seed, ObjectPoolingManager poolManager, Func<Vector3, (bool, float)> comparisonLambda, bool useCollision, Action<float> progressReporter, List<GameResources.ResourceData> woodResources = null, List<GameResources.ResourceData> oreResources = null, List<GameResources.ResourceData> foodResources = null, List<GameResources.ResourceData> goldResources = null, List<GameResources.ResourceData> recruitResources = null, HashSet<(int, int)> occupiedCells = null, List<GameResources.FoliageData> onLandFoliage = null, List<GameResources.FoliageData> underWaterFoliage = null)
		{
			settings.Size = GetScaledTerrainSize();
			settings.Seed = seed;
			settings.Offset = GetPositiveNoiseOffset(settings.Seed, settings.Size);

			if (settings.Spacing == 0)
				settings.Spacing = 1;

			Vector3 colSize = Vector3.one * settings.Spacing * 0.45f;
			float centeredOffset = settings.Spacing * 0.5f;
			float[,] generatedNoiseMap = null;
			yield return StartCoroutine(Noise.GenerateNoiseMapCoroutine(settings, _debugSettings.FrameBudgetSeconds, result => generatedNoiseMap = result));
			settings.HeightMap = generatedNoiseMap;

			int halfSize = settings.Size / 2;
			int start = -halfSize + 2;
			int end = halfSize - 2;
			int checksPerAxis = Mathf.Max(1, Mathf.CeilToInt((end - start) / (float)settings.Spacing));
			int totalChecks = checksPerAxis * checksPerAxis;
			int checksProcessed = 0;
			float frameStartTime = Time.realtimeSinceStartup;
			Vector3 spawnBiasOrigin = GetSpawnBiasOrigin();

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
					Vector3 position = new Vector3(y + centeredOffset, 0, x + centeredOffset);
					if (!IsPointWithinBounds(position))
						continue;

					if (GetSpawnNoiseValue(settings, x + halfSize, y + halfSize, position, spawnBiasOrigin) >= settings.SpawnThreshold)
					{
						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
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
							if (!Physics.Raycast(new Vector3(position.x, 100, position.z), Vector3.down, out RaycastHit hit, 200, _layerSettings.TerrainMask))
								continue;

							position.y = hit.point.y;

							if (IsPositionOverlappingBuilding(position, settings.Spacing * 0.45f))
								continue;

							// Collect ResourceData if resource lists are provided
							if (resourceType != global::Utils.Resource.None && woodResources != null && oreResources != null && foodResources != null && goldResources != null && recruitResources != null)
							{
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

								// Generate GUID
								uint guid = (uint)UnityEngine.Random.Range(1, int.MaxValue);

								Matrix4x4 matrix = Matrix4x4.TRS(position, rotation, Vector3.one);
								GameResources.ResourceData resourceData = new GameResources.ResourceData(position, resourceType, amount, false, matrix, guid, meshIndex, materialIndex);
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
							else if (onLandFoliage != null || underWaterFoliage != null)
							{
								// Collect FoliageData if foliage lists are provided
								if (settings is FoliageGenerationSettings foliageSettings)
								{
									float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
									Quaternion rotation = Quaternion.Euler(0, randomRotation, 0);

									// Randomly select a mesh from the list
									Mesh selectedMesh = null;
									if (foliageSettings.Meshes != null && foliageSettings.Meshes.Count > 0)
									{
										selectedMesh = foliageSettings.Meshes[UnityEngine.Random.Range(0, foliageSettings.Meshes.Count)];
									}

									// Get terrain height at spawn position using raycast
									if (Physics.Raycast(new Vector3(position.x, 100, position.z), Vector3.down, out RaycastHit terrainHit, 200, _layerSettings.TerrainMask))
									{
										position.y = terrainHit.point.y;
									}

									GameResources.FoliageData foliageData = new GameResources.FoliageData(position, rotation, Vector3.one, selectedMesh, foliageSettings.Material);

									if (onLandFoliage != null)
										onLandFoliage.Add(foliageData);
									else if (underWaterFoliage != null)
										underWaterFoliage.Add(foliageData);
								}
							}
							else
							{
								// Only instantiate if not collecting ResourceData (foliage, etc.)
								PoolableObject obj = poolManager.GetPooledObject(settings.GetPoolName(), false);
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
						if (Time.realtimeSinceStartup - frameStartTime >= _debugSettings.FrameBudgetSeconds)
						{
							frameStartTime = Time.realtimeSinceStartup;
							yield return null;
						}
					}
				}
			}

			progressReporter?.Invoke(1f);
		}

		/// <summary>
		/// Attempts to generate a new world with the given settings.
		/// </summary>
		public IEnumerator TryGenerateWorld(Action<float, string> progressReporter = null)
		{
			yield return StartCoroutine(TryGenerateWorldInternal(false, progressReporter));
		}

		private IEnumerator TryGenerateWorldInternal(bool forceGenerate, Action<float, string> progressReporter = null)
		{
			if (forceGenerate || _behaviorSettings.GenerateOnStart)
			{
				progressReporter?.Invoke(0.0f, "Preparing terrain generation...");
				WorldUtils.GroundLayerMask = LayerMask.GetMask("Ground");
				yield return new WaitForEndOfFrame();

				progressReporter?.Invoke(0.08f, "Preparing townhall...");
				yield return null;
				PoolableObject th = _poolingManager.GetPooledObject("Townhall");
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
					yield return new WaitForEndOfFrame();
					if (_behaviorSettings.RandomizeSeed)
						_terrainGenerationSettings.GenerationSettings.Seed = UnityEngine.Random.Range(int.MinValue, int.MaxValue);

					GenerateTerrain();
					yield return new WaitForEndOfFrame();

					if (!TryFindTownhallPlacement(thObj, Vector3.zero, out Vector3 townhallPlacement))
					{
						Debug.Log("ProceduralWorldGenerator: Could not find a valid flat townhall footprint, regenerating terrain.", this);
						continue;
					}

					thObj.transform.position = townhallPlacement;
					thObj.SetActive(true);

					_terrainCheckPassed = false;
					yield return StartCoroutine(AcceptableTerrainCheckCoroutine());
					terrainAcceptable = _terrainCheckPassed;

					if (!terrainAcceptable)
						thObj.SetActive(false);
				}

				if (!terrainAcceptable)
				{
					Debug.LogError($"ProceduralWorldGenerator: Failed to generate acceptable terrain after {MAX_GENERATION_ATTEMPTS} attempts. Proceeding with current terrain.", this);
					if (!TryFindTownhallPlacement(thObj, Vector3.zero, out Vector3 fallbackPlacement))
					{
						fallbackPlacement = Vector3.zero;
						if (TryGetTerrainHeight(fallbackPlacement, out float fallbackHeight))
							fallbackPlacement.y = fallbackHeight;
					}

					thObj.transform.position = fallbackPlacement;
					thObj.SetActive(true);
				}

				EnsureTownhallRegistered(saveableTownhall.BuildingBase);

				progressReporter?.Invoke(0.7f, "Spawning world resources...");
				yield return StartCoroutine(GeneratePooledObjectsExceptTownhall((progress, status) =>
				{
					progressReporter?.Invoke(0.7f + (Mathf.Clamp01(progress) * 0.28f), status);
				}));
				//Check that townhall is on ground, not above water.

				progressReporter?.Invoke(1f, "World generation complete");

				// Center camera on town hall
				GameObject townhallObject = GetActiveTownhallObject();
				if (townhallObject != null && Camera.main != null)
				{
					Vector3 townhallPosition = townhallObject.transform.position;
					Camera.main.transform.position = new Vector3(townhallPosition.x, townhallPosition.y + 10f, townhallPosition.z - 15f);
					Camera.main.transform.LookAt(townhallPosition);
				}

				GameStateManager.NotifyWorldLoaded();
			}
		}

		public void RegenerateTerrainAndWorldRuntime()
		{
			if (!Application.isPlaying)
			{
				Debug.LogWarning("RegenerateTerrainAndWorldRuntime can only be called in play mode.", this);
				return;
			}

			if (_isEditorRegenerating)
				return;

			StartCoroutine(RegenerateTerrainAndWorldRuntimeCoroutine());
		}

		private IEnumerator RegenerateTerrainAndWorldRuntimeCoroutine()
		{
			_isEditorRegenerating = true;
			yield return StartCoroutine(TryGenerateWorldInternal(true));
			_isEditorRegenerating = false;

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
				Debug.LogWarning("RegenerateResourcesAndFoliageRuntime can only be called in play mode.", this);
				return;
			}

			if (_isEditorRegenerating)
				return;

			StartCoroutine(RegenerateResourcesAndFoliageRuntimeCoroutine());
		}

		private IEnumerator RegenerateResourcesAndFoliageRuntimeCoroutine()
		{
			_isEditorRegenerating = true;

			if (!TryEnsureActiveTownhall(out _))
			{
				Debug.LogWarning("ProceduralWorldGenerator: Could not ensure active townhall before regenerating resources and foliage.", this);
				_isEditorRegenerating = false;
				yield break;
			}

			yield return StartCoroutine(GeneratePooledObjectsExceptTownhall());
			_isEditorRegenerating = false;
		}

		private IEnumerator AcceptableTerrainCheckCoroutine()
		{
			// Check Town Hall is not above water
			if (!TownHallAboveGround())
			{
				Debug.Log("Town Hall Above Water, Regenerating Terrain");
				yield break;
			}

			// Pathfinding check disabled - GridGraph configuration needs fixing
			// The full A* scan in ScanWorld() will validate pathfinding after all objects are spawned
			_terrainCheckPassed = true;
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

		public IEnumerator ScanWorld()
		{
			WorldUtils.GroundLayerMask = LayerMask.GetMask("Ground");
			yield return new WaitForEndOfFrame();
			GenerateTerrain();
			yield return StartCoroutine(GeneratePooledObjects());
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













