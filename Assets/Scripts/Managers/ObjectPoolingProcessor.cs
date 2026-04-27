using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using UnityEngine;
using UnityEngine.SceneManagement;
using Utils.Pooling;
using Utils;
using Reflex.Attributes;
using Reflex.Core;
using Reflex.Injectors;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
    /// <summary>
    /// Processor that manages object pooling for performance optimization.
    /// Handles pooling of game objects to reduce instantiation overhead.
    /// </summary>
    public partial class ObjectPoolingProcessor : MonoBehaviour, IInstaller, IProcessor, IAsyncInitializableProcessor, IMainThreadInitializableProcessor
    {
        /// <summary>
        /// Reflex DI container for dependency injection.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Container _container;

        /// <summary>
        /// Scene-specific Reflex DI container.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Container _sceneContainer;

        /// <summary>
        /// Object pooling settings containing configuration for pooled objects.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private ObjectPoolingSettings _objectPoolingSettings;

        /// <summary>
        /// Runtime data for object pooling state.
        /// Installed via InjectRuntimeData during container initialization.
        /// </summary>
        private ObjectPoolingRuntimeData _objectPoolingRuntimeData;

        /// <summary>
        /// Game state processor for notifying when objects are pooled.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private GameStateProcessor _gameStateProcessor;

        /// <summary>
        /// Tracks whether pooling has been initialized for the current scene.
        /// </summary>
        private bool _poolingInitializedForScene = false;

        /// <summary>
        /// Processor containing game event runtime data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private GameEventProcessor _gameEventProcessor;

        /// <summary>
        /// Adds a pooled object back to its pool for reuse.
        /// </summary>
        /// <param name="poolName">The name of the pool to add the object to.</param>
        /// <param name="go">The pooled object to return to the pool.</param>
        public void AddToPool(string poolName, PoolableObject go)
        {
            if (string.IsNullOrEmpty(poolName))
                throw new InvalidOperationException("ObjectPoolingProcessor: Tried to return a pooled object before its pool name was initialized.");

            if (!_objectPoolingRuntimeData.PooledObjects.ContainsKey(poolName))
                throw new KeyNotFoundException($"ObjectPoolingProcessor: Tried to return object to unregistered pool '{poolName}'.");

			_objectPoolingRuntimeData.PooledObjects[poolName].Enqueue(go);
		}

		/// <summary>
		/// Initializes the object pooling system as a coroutine.
		/// Pools all configured objects and reports progress.
		/// </summary>
		/// <param name="progressReporter">Optional callback for reporting progress (progress, message).</param>
		public async Task InitializePooling(Action<float, string> progressReporter = null)
		{
			Debug.Log($"[ObjectPoolingProcessor] InitializePooling entry - _poolingInitializedForScene: {_poolingInitializedForScene}, _objectPoolingSettings: {_objectPoolingSettings != null}, _container: {_container != null}, _sceneContainer: {_sceneContainer != null}");

			if (_poolingInitializedForScene)
			{
				Debug.Log($"[ObjectPoolingProcessor] Pooling already initialized for current scene, skipping");
				return;
			}

			if (_objectPoolingSettings == null)
			{
				Debug.LogError("[ObjectPoolingProcessor] ObjectPoolingSettings is null, cannot initialize pooling");
				return;
			}

			Debug.Log($"[ObjectPoolingProcessor] Settings not null, ObjectsToPool: {_objectPoolingSettings.ObjectsToPool != null}, Count: {_objectPoolingSettings.ObjectsToPool?.Count ?? 0}");

			if (_objectPoolingSettings.ObjectsToPool == null || _objectPoolingSettings.ObjectsToPool.Count == 0)
			{
				Debug.LogWarning("[ObjectPoolingProcessor] No objects to pool configured, skipping initialization");
				_poolingInitializedForScene = true;
				return;
			}

			Debug.Log($"[ObjectPoolingProcessor] InitializePooling called, activeSelf: {gameObject.activeSelf}, activeInHierarchy: {gameObject.activeInHierarchy}, has parent: {transform.parent != null}");

			if (_objectPoolingRuntimeData.PoolParents != null)
			{
				foreach (var poolParent in _objectPoolingRuntimeData.PoolParents.Values)
				{
					if (poolParent != null)
					{
						Debug.Log($"[ObjectPoolingProcessor] Destroying old pool parent: {poolParent.name}");
						UnityEngine.Object.Destroy(poolParent);
					}
				}
			}

			await PoolObjectsAsync(_objectPoolingSettings.ObjectsToPool, _objectPoolingSettings.DebugPooling, progressReporter);
			_gameStateProcessor.NotifyObjectsPooled();
			_poolingInitializedForScene = true;
		}

		/// <summary>
		/// Gets a pooled object from the specified pool.
		/// If no inactive objects are available, instantiates a new one.
		/// </summary>
		/// <param name="name">The name of the pool to get an object from.</param>
		/// <param name="printWarning">Whether to print a warning when exceeding pool size.</param>
		/// <returns>The pooled object, or null if the pool doesn't exist.</returns>
		public PoolableObject GetPooledObject(string name, bool printWarning = true)
		{
			if (!_objectPoolingRuntimeData.PooledObjects.ContainsKey(name))
			{
				Debug.LogError($"Tried to grab a pooled object of {name} but it didnt exist. Perhaps try pooling it you dingus!");
				return null;
			}

			if (_objectPoolingRuntimeData.PooledObjects[name].Count > 0)
			{
				PoolableObject go = _objectPoolingRuntimeData.PooledObjects[name].Peek();

				if (!go.gameObject.activeInHierarchy)
				{
					_objectPoolingRuntimeData.PooledObjects[name].Dequeue();
					go.gameObject.SetActive(true);

					if (go is Utils.Pooling.IPooledObjectReset resettable)
						resettable.OnReset();

					return go;
				}
			}

			// If we got to this point we ran out of pooled objects, perhaps need more
			if (printWarning)
				Debug.LogWarning($"Exceeded Pool amount and Instantiating a new object of type {name}. Current Count is {_objectPoolingRuntimeData.PooledObjects[name].Count + 1}");

			return InstantiateNewObjectToPool(name, _objectPoolingSettings.ObjectsToPool);
		}

		/// <summary>
		/// Gets all active pooled objects of a specific type.
		/// </summary>
		/// <param name="name">The name of the pool type.</param>
		/// <returns>List of all active objects of the specified type, or null if pool doesn't exist.</returns>
		public List<PoolableObject> GetAllActivePooledObjectsOfType(string name)
		{
			if (!_objectPoolingRuntimeData.AllObjects.ContainsKey(name))
			{
				Debug.LogError($"Tried to get active pooled objects of {name} but they don't exist.");
				return null;
			}

			List<PoolableObject> activeObjects = new List<PoolableObject>();
			for (int i = _objectPoolingRuntimeData.AllObjects[name].Count - 1; i >= 0; i--)
			{
				PoolableObject go = _objectPoolingRuntimeData.AllObjects[name][i];

				if (go.transform.gameObject.activeInHierarchy)
				{
					activeObjects.Add(go);
				}
			}
			return activeObjects;
		}

		/// <summary>
		/// Gets all active objects of a type within a box collider's bounds.
		/// </summary>
		/// <param name="collider">The box collider defining the bounds.</param>
		/// <param name="center">The center position of the search area.</param>
		/// <param name="type">The type of objects to search for.</param>
		/// <returns>List of active objects within the collider bounds.</returns>
		public List<PoolableObject> GetAllActiveObjectsOfTypeWithinBoxCollider(BoxCollider collider, Vector3 center, string type)
		{
			Vector3 startPosition = center + new Vector3(collider.size.x * 0.5f + 1, 0, collider.size.z * 0.5f + 1);
			Vector3 endPosition = center - new Vector3(collider.size.x * 0.5f + 1, 0, collider.size.z * 0.5f + 1);
			return GetAllActiveObjectsOfTypeWithinAABB(startPosition, endPosition, type);
		}

		/// <summary>
		/// Gets all active objects of a type within an axis-aligned bounding box.
		/// </summary>
		/// <param name="startPosition">The start position of the AABB.</param>
		/// <param name="endPosition">The end position of the AABB.</param>
		/// <param name="type">The type of objects to search for.</param>
		/// <returns>List of active objects within the AABB.</returns>
		public List<PoolableObject> GetAllActiveObjectsOfTypeWithinAABB(Vector3 startPosition, Vector3 endPosition, string type)
		{
			List<PoolableObject> activeObjects = new List<PoolableObject>();

			List<PoolableObject> activeObjectsOfType = GetAllActivePooledObjectsOfType(type);

			for (int i = 0; i < activeObjectsOfType.Count; i++)
			{
				if (activeObjectsOfType[i].transform.position.x > startPosition.x && activeObjectsOfType[i].transform.position.x < endPosition.x || activeObjectsOfType[i].transform.position.x > endPosition.x && activeObjectsOfType[i].transform.position.x < startPosition.x)
					if (activeObjectsOfType[i].transform.position.z > startPosition.z && activeObjectsOfType[i].transform.position.z < endPosition.z || activeObjectsOfType[i].transform.position.z > endPosition.z && activeObjectsOfType[i].transform.position.z < startPosition.z)
						activeObjects.Add(activeObjectsOfType[i]);
			}

			return activeObjects;
		}

		/// <summary>
		/// Gets all active pooled objects of a save item type.
		/// </summary>
		/// <param name="item">The save item type.</param>
		/// <returns>List of active objects of the specified save item type.</returns>
		public List<PoolableObject> GetAllActivePooledObjectsOfType(SaveItem item)
		{
			return GetAllActivePooledObjectsOfType(item.ToString());
		}

		/// <summary>
		/// Simple pooling method that instantiates all pooled objects immediately without frame budgeting.
		/// Used for quick initialization or testing purposes.
		/// </summary>
		public void SimplePoolObjects()
		{
			Dictionary<string, Queue<PoolableObject>> pooledObjects = new Dictionary<string, Queue<PoolableObject>>();
			Dictionary<string, GameObject> poolParents = new Dictionary<string, GameObject>();
			Dictionary<string, List<PoolableObject>> allObjects = new Dictionary<string, List<PoolableObject>>();
			Container injectionContainer = GetRequiredInjectionContainer();
			_objectPoolingRuntimeData.InitializePooling(pooledObjects, allObjects, poolParents, TimeSpan.Zero);

			GameObject poolParent = new GameObject("Pooled Objects");
			Scene targetScene = SceneManager.GetActiveScene();
			if (targetScene.IsValid() && targetScene.isLoaded)
			{
				SceneManager.MoveGameObjectToScene(poolParent, targetScene);
			}

			for (int i = 0; i < _objectPoolingSettings.ObjectsToPool.Count; i++)
			{
				string objName = _objectPoolingSettings.ObjectsToPool[i].Name;
				GameObject parentObject = new GameObject(objName + " Pool");

				parentObject.transform.parent = poolParent.transform;
				pooledObjects[objName] = new Queue<PoolableObject>(_objectPoolingSettings.ObjectsToPool[i].PoolAmount);
				allObjects[objName] = new List<PoolableObject>(_objectPoolingSettings.ObjectsToPool[i].PoolAmount);
				poolParents.Add(objName, parentObject);

				for (int j = 0; j < _objectPoolingSettings.ObjectsToPool[i].PoolAmount; j++)
				{
					GameObject obj = UnityEngine.Object.Instantiate(_objectPoolingSettings.ObjectsToPool[i].Prefab, new Vector3(-500, 0, -500), Quaternion.identity, parentObject.transform);
					GameObjectInjector.InjectRecursive(obj, injectionContainer);
					PoolableObject poolObj = obj.GetComponent<PoolableObject>();
					if (poolObj == null)
						poolObj = obj.AddComponent<PoolableObject>();
					obj.name = objName + j;
					allObjects[objName].Add(poolObj);
					poolObj.Initialize(objName);
					obj.SetActive(false);
				}
			}

			_objectPoolingRuntimeData.InitializePooling(pooledObjects, allObjects, poolParents, TimeSpan.Zero);
		}

		/// <summary>
		/// Disables all active objects in a specific pool.
		/// </summary>
		/// <param name="name">The name of the pool to disable objects from.</param>
		public void DisableObjectsInPool(string name)
		{
			List<PoolableObject> objects = GetAllActivePooledObjectsOfType(name);

			for(int i =0; i < objects.Count; i++)
			{
				objects[i].gameObject.SetActive(false);
			}
		}

		/// <summary>
		/// Injects dependencies into all pooled objects.
		/// Injection is now handled automatically by the Reflex DI container.
		/// </summary>
		public void InjectAllPooledObjects()
		{
			// Injection handled by Reflex DI container automatically
			Debug.Log($"[ObjectPoolingProcessor] Pooled objects injection handled by DI container");
		}

		/// <summary>
		/// Initializes the object pooling processor.
		/// No initialization logic required here; initialization happens via InitializeAsync.
		/// </summary>
		public void Initialize()
		{
			if (_objectPoolingRuntimeData == null)
				throw new InvalidOperationException("ObjectPoolingProcessor: ObjectPoolingRuntimeData has not been installed.");

			SceneManager.sceneLoaded += OnSceneLoaded;
			Debug.Log($"[ObjectPoolingProcessor] Scene loaded event handler registered");
		}

		        private void OnSceneLoaded(Scene scene, LoadSceneMode mode)
        {
            Debug.Log($"[ObjectPoolingProcessor] OnSceneLoaded called - Scene: {scene.name}, mode: {mode}");
            
            // Reset the flag so pooling can be re-initialized for the new scene
            _poolingInitializedForScene = false;

            Debug.Log($"[ObjectPoolingProcessor] Waiting for Coordinator to refresh scene bindings before re-initializing pooling for scene: {scene.name}");
        }

        private void OnDestroy()
        {
            SceneManager.sceneLoaded -= OnSceneLoaded;
		}

		/// <summary>
		/// Asynchronously initializes the object pooling processor.
		/// Called by the Coordinator during startup.
		/// </summary>
		public async Task InitializeAsync(ProcessorStartupContext startupContext, CancellationToken cancellationToken)
		{
			await InitializePooling((progress, status) => startupContext.Report(progress, status));
		}

		/// <summary>
		/// Processes object pooling logic every frame.
		/// Called every frame by the Coordinator.
		/// ObjectPoolingProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			// ObjectPoolingProcessor does not require per-frame updates
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			_sceneContainer = sceneContainer;
			if (sceneContainer != null)
			{
				var newSettings = sceneContainer.Resolve<ObjectPoolingSettings>();
				if (newSettings != null)
				{
					_objectPoolingSettings = newSettings;
					Debug.Log($"[ObjectPoolingProcessor] Refreshed settings from scene container, ObjectsToPool count: {newSettings.ObjectsToPool?.Count ?? 0}");
				}
			}
		}

		/// <summary>
		/// Ensures all pool parents are in the active scene.
		/// Should be called before world generation to ensure pooled objects are accessible.
		/// </summary>
		public void EnsurePoolParentsInActiveScene()
		{
			if (_objectPoolingRuntimeData.PoolParents == null)
				return;

			Scene targetScene = SceneManager.GetActiveScene();
			Debug.Log($"[ObjectPoolingProcessor] Ensuring pool parents in active scene: {targetScene.name}");

			foreach (var poolParent in _objectPoolingRuntimeData.PoolParents.Values)
			{
				if (poolParent != null)
				{
					SceneManager.MoveGameObjectToScene(poolParent, targetScene);
					Debug.Log($"[ObjectPoolingProcessor] Moved pool parent '{poolParent.name}' to scene: {targetScene.name}");
				}
			}
		}

		/// <summary>
		/// Resets the pooling initialization flag to allow re-initialization.
		/// Should be called when loading a new scene.
		/// </summary>
		public void ResetPoolingInitialization()
		{
			_poolingInitializedForScene = false;
			Debug.Log($"[ObjectPoolingProcessor] Pooling initialization flag reset");
		}

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
		/// Injects the ObjectPoolingRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_objectPoolingRuntimeData != null)
				throw new InvalidOperationException("ObjectPoolingProcessor: ObjectPoolingRuntimeData has already been installed.");

			_objectPoolingRuntimeData = new ObjectPoolingRuntimeData();
			containerBuilder.AddSingleton(_objectPoolingRuntimeData);
		}

		private Container GetRequiredInjectionContainer()
		{
			if (_sceneContainer == null)
				throw new InvalidOperationException("ObjectPoolingProcessor: Scene container has not been refreshed before pooled object injection.");

			return _sceneContainer;
		}

		// Instantiates a new object to add to a pool when the pool is exhausted.
		private PoolableObject InstantiateNewObjectToPool(string name, List<Utils.Pooling.PooledObjectData> objectsToPool)
		{
			Container injectionContainer = GetRequiredInjectionContainer();
			for (int i = 0; i < objectsToPool.Count; i++)
			{
				if (objectsToPool[i].Name == name)
				{
					GameObject obj = UnityEngine.Object.Instantiate(objectsToPool[i].Prefab);
					GameObjectInjector.InjectRecursive(obj, injectionContainer);
					PoolableObject poolObj = null;
					if (!obj.TryGetComponent(out poolObj))
						poolObj = obj.AddComponent<PoolableObject>();

					poolObj.Initialize(name);
					if (obj.TryGetComponent<RectTransform>(out RectTransform rt))
					{
						rt.SetParent(_objectPoolingRuntimeData.PoolParents[name].transform, false);
					}
					else
						obj.transform.parent = _objectPoolingRuntimeData.PoolParents[name].transform;
					_objectPoolingRuntimeData.AllObjects[name].Add(poolObj);
					obj.name = name + _objectPoolingRuntimeData.AllObjects[name].Count;
					obj.SetActive(true);

					if (poolObj is Utils.Pooling.IPooledObjectReset resettable)
						resettable.OnReset();

					return poolObj;
				}
			}

			Debug.LogError($"Something really went wrong with trying to instantiate a new object of type {name}");
			return null;
		}

		private async Task PoolObjectsAsync(List<Utils.Pooling.PooledObjectData> objectsToPool, bool debugPooling, Action<float, string> progressReporter = null)
		{
			Dictionary<string, Queue<PoolableObject>> pooledObjects = new Dictionary<string, Queue<PoolableObject>>();
			Dictionary<string, GameObject> poolParents = new Dictionary<string, GameObject>();
			Dictionary<string, List<PoolableObject>> allObjects = new Dictionary<string, List<PoolableObject>>();
			Container injectionContainer = GetRequiredInjectionContainer();
			_objectPoolingRuntimeData.InitializePooling(pooledObjects, allObjects, poolParents, TimeSpan.Zero);
			float frameStartTime = Time.realtimeSinceStartup;

			GameObject poolParent = new GameObject("Pooled Objects");
			Scene targetScene = SceneManager.GetActiveScene();
			Debug.Log($"[ObjectPoolingProcessor] Creating pool parent. Active scene: {targetScene.name}, isValid: {targetScene.IsValid()}, isLoaded: {targetScene.isLoaded}");
			
			if (targetScene.IsValid() && targetScene.isLoaded)
			{
				SceneManager.MoveGameObjectToScene(poolParent, targetScene);
				Debug.Log($"[ObjectPoolingProcessor] Moved pool parent to scene: {targetScene.name}");
			}
			else
			{
				Debug.LogWarning($"[ObjectPoolingProcessor] Active scene not loaded, pool parent may not be in correct scene");
			}

			int objectTypeCount = objectsToPool.Count;

			if (objectTypeCount == 0)
			{
				progressReporter?.Invoke(1f, "Pooling complete");
				return;
			}

			for (int i = 0; i < objectsToPool.Count; i++)
			{
				DateTime before = DateTime.Now;
				string objName = objectsToPool[i].Name;
				int poolAmount = objectsToPool[i].PoolAmount;
				GameObject parentObject = new GameObject(objName + " Pool");

				parentObject.transform.parent = poolParent.transform;
				pooledObjects[objName] = new Queue<PoolableObject>(objectsToPool[i].PoolAmount);
				allObjects[objName] = new List<PoolableObject>(objectsToPool[i].PoolAmount);
				poolParents.Add(objName, parentObject);

				for (int j = 0; j < objectsToPool[i].PoolAmount; j++)
				{
					GameObject obj = UnityEngine.Object.Instantiate(objectsToPool[i].Prefab, new Vector3(-500, 0, -500), Quaternion.identity, parentObject.transform);
					GameObjectInjector.InjectRecursive(obj, injectionContainer);

					PoolableObject poolObj = obj.GetComponent<PoolableObject>();
					if (poolObj == null)
						poolObj = obj.AddComponent<PoolableObject>();
					obj.name = objName + j;
					allObjects[objName].Add(poolObj);
					poolObj.Initialize(objName);

					obj.SetActive(false);

					float typeProgress = poolAmount > 0 ? (j + 1f) / poolAmount : 1f;
					float overallProgress = (i + typeProgress) / objectTypeCount;
					progressReporter?.Invoke(overallProgress, $"Pooling {objName} ({j + 1}/{poolAmount})");

					if ((j + 1) % 30 == 0 && Time.realtimeSinceStartup - frameStartTime >= 0.01f)
					{
						frameStartTime = Time.realtimeSinceStartup;
						await Task.Yield();
					}
				}

				if (poolAmount == 0)
				{
					float overallProgress = (i + 1f) / objectTypeCount;
					progressReporter?.Invoke(overallProgress, $"Pooling {objName}");
				}

				DateTime after = DateTime.Now;
				TimeSpan duration = after.Subtract(before);
				if (debugPooling)
					Debug.Log($"Pooling {objName} took {duration.TotalMilliseconds}ms");
				frameStartTime = Time.realtimeSinceStartup;
				await Task.Yield();
			}

			_objectPoolingRuntimeData.InitializePooling(pooledObjects, allObjects, poolParents, TimeSpan.Zero);
			progressReporter?.Invoke(1f, "Pooling complete");
		}
	}
}
