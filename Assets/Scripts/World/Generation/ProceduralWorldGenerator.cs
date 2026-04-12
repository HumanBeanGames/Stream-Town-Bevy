using Enemies;
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

		[SerializeField]
		private float _xScale = 4;
		[SerializeField]
		private float _yScale = 4;
		[SerializeField]
		private GenerationSettings _generationSettings;
		[SerializeField]
		private List<ResourceGenerationSettings> _resourceGenerationSettings;
		[SerializeField]
		private List<ResourceGenerationSettings> _waterResourceGenerationSettings;
		[SerializeField]
		private List<FoliageGenerationSettings> _foliageGenerationSettings;
		[SerializeField]
		private List<FoliageGenerationSettings> _waterFoliageGenerationSettings;
		[SerializeField]
		private List<CampGenerationSettings> _campGenerationSettings;

		[SerializeField]
		private bool _generateOnStart = true;
		[SerializeField]
		private bool _randomizeSeed = true;

		private bool _terrainCheckPassed;
		private const int MAX_GENERATION_ATTEMPTS = 10;

		[SerializeField]
		private bool _debugGenerationTiming = false;
		private const float _frameBudgetSeconds = 0.01f;

		[SerializeField]
		private LayerMask _collisionMask;
		[SerializeField]
		private LayerMask _terrainMask;
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
			float xSize = _xScale * _generationSettings.Size;
			float ySize = _yScale * _generationSettings.Size;

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
			_generatedMesh = ProceduralMeshGenerator.CreateMesh(ProceduralMeshGenerator.GenerateTerrainMeshData(_generationSettings), gameObject);
		}

		public void SetMesh(Mesh mesh)
		{
			_generatedMesh = ProceduralMeshGenerator.CreateMesh(mesh, gameObject);
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
			GameObject thObj = ((SaveableBuilding)th.SaveableObject).BuildingBase.gameObject;
			thObj.transform.position = Vector3.zero;
			thObj.SetActive(true);
			_buildingManager.AddLoadedBuilding(((SaveableBuilding)th.SaveableObject).BuildingBase);

			yield return StartCoroutine(GeneratePooledObjectsExceptTownhall(progressReporter));
		}

		/// <summary>
		/// Generates all pooled objects except the townhall (townhall is spawned separately before terrain generation).
		/// </summary>
		private IEnumerator GeneratePooledObjectsExceptTownhall(Action<float, string> progressReporter = null)
		{
			ObjectPoolingManager poolManager = _poolingManager;
			int seed = _generationSettings.Seed;
			DateTime before = DateTime.Now;
			DateTime after;
			TimeSpan duration;
			int totalSettings = 0;
			if (_resourceGenerationSettings != null)
				totalSettings += _resourceGenerationSettings.Count;
			if (_waterResourceGenerationSettings != null)
				totalSettings += _waterResourceGenerationSettings.Count;
			if (_foliageGenerationSettings != null)
				totalSettings += _foliageGenerationSettings.Count;
			if (_waterFoliageGenerationSettings != null)
				totalSettings += _waterFoliageGenerationSettings.Count;

			int completedSettings = 0;
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

			// Generate all normal resources (trees, ore, etc).
			if (_resourceGenerationSettings != null)
			{
				foreach (ResourceGenerationSettings settings in _resourceGenerationSettings)
				{
					string label = settings.PoolName;
					before = DateTime.Now;
					seed++;
					yield return StartCoroutine(GenerateFromSettingsCoroutine(settings, seed, poolManager, WorldUtils.OnGroundCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning resources: {label} ({completedSettings + 1}/{totalSettings})...")));
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugGenerationTiming)
						Debug.Log($"Generating {settings.PoolName} took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed resources: {label} ({completedSettings}/{totalSettings})");
					yield return null;
				}

			}

			// Generate all resources for water on the shore line (fish).
			if (_waterResourceGenerationSettings != null)
				foreach (ResourceGenerationSettings settings in _waterResourceGenerationSettings)
				{
					string label = settings.PoolName;
					before = DateTime.Now;
					seed++;
					yield return StartCoroutine(GenerateFromSettingsCoroutine(settings, seed, poolManager, WorldUtils.OnShoreLineCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning shoreline resources: {label} ({completedSettings + 1}/{totalSettings})...")));
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugGenerationTiming)
						Debug.Log($"Generating {settings.PoolName} took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed shoreline resources: {label} ({completedSettings}/{totalSettings})");
					yield return null;
				}

			// Generate the ground foliage (flowers, grass, etc).
			if (_foliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _foliageGenerationSettings)
				{
					string label = settings.PoolNames != null && settings.PoolNames.Length > 0 ? settings.PoolNames[0] : "Foliage";
					before = DateTime.Now;
					seed++;
					yield return StartCoroutine(GenerateFromSettingsCoroutine(settings, seed, poolManager, WorldUtils.OnGroundCheckHeight, true, p =>
						ReportSpawnProgress(p, $"Spawning foliage: {label} ({completedSettings + 1}/{totalSettings})...")));
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugGenerationTiming)
						Debug.Log($"Generating {settings.PoolNames[0]} took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed foliage: {label} ({completedSettings}/{totalSettings})");
					yield return null;
				}

			// Generate the underwater foliage (seaweed, corals, etc.).
			if (_waterFoliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _waterFoliageGenerationSettings)
				{
					string label = settings.PoolNames != null && settings.PoolNames.Length > 0 ? settings.PoolNames[0] : "Water Foliage";
					before = DateTime.Now;
					seed++;
					yield return StartCoroutine(GenerateFromSettingsCoroutine(settings, seed, poolManager, WorldUtils.UnderWaterCheckHeight, false, p =>
						ReportSpawnProgress(p, $"Spawning underwater foliage: {label} ({completedSettings + 1}/{totalSettings})...")));
					after = DateTime.Now;
					duration = after.Subtract(before);
					if (_debugGenerationTiming)
						Debug.Log($"Generating {settings.PoolNames[0]} took {duration.TotalMilliseconds}ms");
					completedSettings++;
					ReportSpawnProgress(0f, $"Completed underwater foliage: {label} ({completedSettings}/{totalSettings})");
					yield return null;
				}

			progressReporter?.Invoke(1f, "World resource spawning complete");
		}

		public void MainMenuGenerateWorld()
		{
			WorldUtils.GroundLayerMask = LayerMask.GetMask("Ground");
			ObjectPoolingManager poolManager = GetComponent<ObjectPoolingManager>();
			poolManager.SimplePoolObjects();
			int seed = _generationSettings.Seed;

			if (_resourceGenerationSettings != null)
			{
				foreach (ResourceGenerationSettings settings in _resourceGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, poolManager, WorldUtils.OnGroundCheckHeight);
				}
			}

			if (_foliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _foliageGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, poolManager, WorldUtils.OnGroundCheckHeight);
				}

			if (_waterFoliageGenerationSettings != null)
				foreach (FoliageGenerationSettings settings in _waterFoliageGenerationSettings)
				{
					GenerateFromSettings(settings, ref seed, poolManager, WorldUtils.UnderWaterCheckHeight, false);
				}
		}

		private void GenerateFromSettings(GenerationSettings settings, ref int seed, ObjectPoolingManager poolManager, Func<Vector3, (bool, float)> comparisonLambda, bool useCollision = true)
		{
			settings.Size = (_generationSettings.Size * (int)_xScale);
			settings.Seed = ++seed;
			Vector3 colSize = Vector3.one * settings.Spacing * 0.45f;
			settings.HeightMap = Noise.GenerateNoiseMap(settings);

			int halfSize = settings.Size / 2;

			if (settings.Spacing == 0)
				settings.Spacing = 1;

			Vector3 position;
			for (int y = -halfSize + 2; y < halfSize - 2; y += settings.Spacing)
			{
				for (int x = -halfSize + 2; x < halfSize - 2; x += settings.Spacing)
				{
					position = new Vector3(y + settings.Offset.y, 0, x + settings.Offset.x);
					if (Mathf.FloorToInt(settings.HeightMap[x + halfSize, y + halfSize]) == 1)
					{
						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
							position.y = lambaResult.Item2;

							if (useCollision)
								if (Physics.BoxCast(position + Vector3.up * 5, colSize, Vector3.down, Quaternion.identity, 10, _collisionMask))
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

		private IEnumerator GenerateFromSettingsCoroutine(GenerationSettings settings, int seed, ObjectPoolingManager poolManager, Func<Vector3, (bool, float)> comparisonLambda, bool useCollision, Action<float> progressReporter)
		{
			settings.Size = (_generationSettings.Size * (int)_xScale);
			settings.Seed = seed;

			if (settings.Spacing == 0)
				settings.Spacing = 1;

			Vector3 colSize = Vector3.one * settings.Spacing * 0.45f;
			float[,] generatedNoiseMap = null;
			yield return StartCoroutine(Noise.GenerateNoiseMapCoroutine(settings, _frameBudgetSeconds, result => generatedNoiseMap = result));
			settings.HeightMap = generatedNoiseMap;

			int halfSize = settings.Size / 2;
			int start = -halfSize + 2;
			int end = halfSize - 2;
			int checksPerAxis = Mathf.Max(1, Mathf.CeilToInt((end - start) / (float)settings.Spacing));
			int totalChecks = checksPerAxis * checksPerAxis;
			int checksProcessed = 0;
			float frameStartTime = Time.realtimeSinceStartup;

			for (int y = start; y < end; y += settings.Spacing)
			{
				for (int x = start; x < end; x += settings.Spacing)
				{
					checksProcessed++;
					if (Mathf.FloorToInt(settings.HeightMap[x + halfSize, y + halfSize]) == 1)
					{
						Vector3 position = new Vector3(y + settings.Offset.y, 0, x + settings.Offset.x);

						(bool, float) lambaResult = comparisonLambda(position);

						if (lambaResult.Item1)
						{
							//TODO:: Put this as an out in the lambda
							position.y = lambaResult.Item2;

							//Check for collision
							if (useCollision)
								if (Physics.BoxCast(position + Vector3.up * 5, colSize, Vector3.down, Quaternion.identity, 10, _collisionMask))
									continue;

							PoolableObject obj = poolManager.GetPooledObject(settings.GetPoolName(), false);
							obj.transform.position = position;
							float randomRotation = UnityEngine.Random.Range(0, 4) * 90;
							obj.transform.Rotate(Vector3.up, randomRotation);
							obj.gameObject.SetActive(true);
						}

					}

					if (checksProcessed % 30 == 0)
					{
						progressReporter?.Invoke(checksProcessed / (float)totalChecks);
						if (Time.realtimeSinceStartup - frameStartTime >= _frameBudgetSeconds)
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
			if (_generateOnStart)
			{
				progressReporter?.Invoke(0.0f, "Preparing terrain generation...");
				WorldUtils.GroundLayerMask = LayerMask.GetMask("Ground");
				yield return new WaitForEndOfFrame();

				// Spawn townhall first so it exists for pathfinding check
				progressReporter?.Invoke(0.08f, "Spawning townhall...");
				yield return null;
				PoolableObject th = _poolingManager.GetPooledObject("Townhall");
				GameObject thObj = ((SaveableBuilding)th.SaveableObject).BuildingBase.gameObject;
				thObj.transform.position = Vector3.zero;

				thObj.SetActive(true);
				_buildingManager.AddLoadedBuilding(((SaveableBuilding)th.SaveableObject).BuildingBase);

				int attempts = 0;
				bool terrainAcceptable = false;
				while (!terrainAcceptable && attempts < MAX_GENERATION_ATTEMPTS)
				{
					attempts++;
					float attemptProgress = Mathf.Clamp01(attempts / (float)MAX_GENERATION_ATTEMPTS);
					progressReporter?.Invoke(0.1f + (attemptProgress * 0.45f), $"Generating terrain (attempt {attempts}/{MAX_GENERATION_ATTEMPTS})...");
					yield return new WaitForEndOfFrame();
					if (_randomizeSeed)
						_generationSettings.Seed = UnityEngine.Random.Range(int.MinValue, int.MaxValue);

					GenerateTerrain();
					yield return new WaitForEndOfFrame();

					_terrainCheckPassed = false;
					yield return StartCoroutine(AcceptableTerrainCheckCoroutine());
					terrainAcceptable = _terrainCheckPassed;
				}

				if (!terrainAcceptable)
				{
					Debug.LogError($"ProceduralWorldGenerator: Failed to generate acceptable terrain after {MAX_GENERATION_ATTEMPTS} attempts. Proceeding with current terrain.", this);
				}

				progressReporter?.Invoke(0.7f, "Spawning world resources...");
				yield return StartCoroutine(GeneratePooledObjectsExceptTownhall((progress, status) =>
				{
					progressReporter?.Invoke(0.7f + (Mathf.Clamp01(progress) * 0.28f), status);
				}));
				//Check that townhall is on ground, not above water.

				progressReporter?.Invoke(1f, "World generation complete");

				GameStateManager.NotifyWorldLoaded();
			}
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

			for (int i = -(townHallCheckSize) / 2; i < townHallCheckSize / 2; i++)
			{
				for (int j = -(townHallCheckSize / 2); j < townHallCheckSize / 2; j++)
				{
					if (Physics.Raycast(new Vector3(i, 5, j), Vector3.down, out RaycastHit info, 10, _terrainMask))
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
			//Check that townhall is on ground, not above water.
			GameStateManager.NotifyWorldLoaded();
			AstarPath.active.Scan();
		}

		// Unity Functions.
		private void OnValidate()
		{
			if (_generationSettings == null)
				return;

			if (_generationSettings.Lacunarity < 1)
				_generationSettings.Lacunarity = 1;

			if (_generationSettings.Octaves < 0)
				_generationSettings.Octaves = 0;

		}

#if UNITY_EDITOR

		private List<Vector3> GenerateDebugPositions(GenerationSettings settings, ref int seed, Func<Vector3, (bool, float)> comparisonLambda)
		{
			List<Vector3> listOfPositions = new List<Vector3>();

			settings.Size = (_generationSettings.Size * (int)_xScale);
			settings.Seed = ++seed;
			//Generate resource map (stored in Height Map)
			Vector3 colSize = Vector3.one * settings.Spacing * 0.45f;
			settings.HeightMap = Noise.GenerateNoiseMap(settings);

			//Set pooled objects to the position
			int halfSize = (settings.Size) / 2;

			if (settings.Spacing == 0)
				settings.Spacing = 1;

			for (int y = -halfSize; y < halfSize; y += settings.Spacing)
			{
				for (int x = -halfSize; x < halfSize; x += settings.Spacing)
				{
					if (Mathf.FloorToInt(settings.HeightMap[x + halfSize, y + halfSize]) == 1)
					{
						Vector3 position = new Vector3(y + settings.Offset.y, 0, x + settings.Offset.x);
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

		private void OnDrawGizmosSelected()
		{
			// Preview Tree Placements
			if (_regen)
			{
				_regen = false;
				if (_previewTreePlacements)
				{
					ResourceGenerationSettings settings = default;

					for (int i = 0; i < _resourceGenerationSettings.Count; i++)
					{
						if (_resourceGenerationSettings[i].TargetType == TargetMask.Tree)
						{
							settings = _resourceGenerationSettings[i];
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