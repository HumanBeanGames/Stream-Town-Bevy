using Buildings;
using Character;
using GameEventSystem;
using GridSystem.Partitioning;
using TownGoal;
using System.Collections.Generic;
using TechTree;
using UnityEngine;
using UnityEngine.InputSystem;
using Utils;
using World;
using World.Generation;
using SavingAndLoading;
using GUIDSystem;
using Enemies;
using PlayerControls;
using Reflex.Core;
using Reflex.Extensions;
using UnityEngine.EventSystems;
using UserInterface.MainMenu;
using ScriptablesProcessorInfrastructure;
using System;
using System.Collections;
using System.Linq;
using Environment;
using Utils.Pooling;
using Twitch;
using Audio;
using Data.Containers;
using GameResources;
using GridSystem;
using Sensors;
using Settings;
using Processors;
using System.Threading;
using System.Threading.Tasks;
using UnityEngine.SceneManagement;

namespace Core
{
	[DefaultExecutionOrder(-1000)]
	public class Coordinator : MonoBehaviour
	{
		private static Coordinator _instance;

		public enum StartupState
		{
			NotStarted,
			WaitingForDependencies,
			Initializing,
			Activating,
			Ready,
			Failed
		}

		private List<IProcessor> _processors;
		private List<IDataScriptable> _dataScriptables;
		private readonly Dictionary<string, ProcessorStartupReport> _startupReports = new Dictionary<string, ProcessorStartupReport>();
		private readonly List<Task> _processorStartupTasks = new List<Task>();
		private int _frameCounter = 0;
		private const int WARNING_FRAME_INTERVAL = 120;
		private bool _initializationComplete = false;
		private CancellationTokenSource _startupCancellationTokenSource;
		private Task _startupTask;
		private int _loadedStartupSceneBuildIndex = -1;
		private GameObject _loadingScreen;
		private bool _worldSceneBootstrapInProgress;

		public StartupState CurrentStartupState { get; private set; } = StartupState.NotStarted;

		public IReadOnlyDictionary<string, ProcessorStartupReport> StartupReports => _startupReports;

		public IReadOnlyList<Task> ProcessorStartupTasks => _processorStartupTasks;

		private void Awake()
		{
			if (_instance != null && _instance != this)
			{
				throw new System.InvalidOperationException("Duplicate Coordinator detected. Ensure only one Coordinator exists across the loader scene and persistent prefabs.");
			}

			_instance = this;
			_processors = new List<IProcessor>();
			_dataScriptables = new List<IDataScriptable>();
			_startupCancellationTokenSource = new CancellationTokenSource();
			DontDestroyOnLoad(gameObject);

			_loadingScreen = GameObject.Find("UI_LoadingScreen");
		}

		[RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
		private static void RegisterSceneLoadHook()
		{
			SceneManager.sceneLoaded -= HandleSceneLoadedStatic;
			SceneManager.sceneLoaded += HandleSceneLoadedStatic;
		}

		private static void HandleSceneLoadedStatic(Scene scene, LoadSceneMode mode)
		{
			_instance?.HandleSceneLoaded(scene, mode);
		}

		private void Start()
		{
			Debug.Log("[COORDINATOR] Startup sequence started");
			StartCoroutine(StartupSequence());
		}

		private IEnumerator StartupSequence()
		{
			CurrentStartupState = StartupState.WaitingForDependencies;
			Debug.Log("[COORDINATOR] Stage: Waiting for dependencies");

			while (!AllProcessorsAvailable())
			{
				CacheProcessorsFromBindings();
				_frameCounter++;
				if (_frameCounter % WARNING_FRAME_INTERVAL == 0)
				{
					Debug.LogWarning($"[COORDINATOR] Waiting for processor bindings to become available... Missing: {string.Join(", ", GetMissingProcessorDependencies())}");
				}
				yield return null;
			}

			CacheProcessorsFromBindings();
			Debug.Log("[COORDINATOR] All processors available");
			_frameCounter = 0;

			int nextSceneBuildIndex = GetNextSceneBuildIndex();
			if (!IsValidBuildIndex(nextSceneBuildIndex))
			{
				CurrentStartupState = StartupState.Failed;
				Debug.LogError($"[COORDINATOR] Could not load next scene. Build index {nextSceneBuildIndex} is invalid.");
				yield break;
			}

			yield return LoadSceneAdditively(nextSceneBuildIndex);
			_loadedStartupSceneBuildIndex = nextSceneBuildIndex;
			_frameCounter = 0;

			while (!AllDataScriptablesAvailable())
			{
				CacheDataScriptablesFromLoadedSceneBindings();
				_frameCounter++;
				if (_frameCounter % WARNING_FRAME_INTERVAL == 0)
				{
					Debug.LogWarning($"[COORDINATOR] Waiting for loaded scene data scriptables to become available... Missing: {string.Join(", ", GetMissingDataScriptableDependencies())}");
				}
				yield return null;
			}

			CacheDataScriptablesFromLoadedSceneBindings();
			Debug.Log($"[COORDINATOR] Found {_dataScriptables.Count} bound data scriptables in loaded scene");

			// Inject data types into processors before initialization
			InjectProcessorsWithDataTypes();

			CurrentStartupState = StartupState.Initializing;
			Debug.Log("[COORDINATOR] Stage: Initializing processors");
			_startupTask = InitializeAndActivateProcessorsAsync(_startupCancellationTokenSource.Token);
			while (!_startupTask.IsCompleted)
			{
				yield return null;
			}

			if (_startupTask.IsFaulted)
			{
				CurrentStartupState = StartupState.Failed;
				Debug.LogError("[COORDINATOR] Startup task failed");
				Debug.LogException(_startupTask.Exception);
				yield break;
			}

			// Mark initialization as complete so Process() can run
			_initializationComplete = true;
			CurrentStartupState = StartupState.Ready;
			Debug.Log("[COORDINATOR] Stage: Ready - All processors initialized and activated");

			if (_loadingScreen != null)
			{
				_loadingScreen.SetActive(false);
				Debug.Log("[COORDINATOR] Deactivated UI loading screen");
			}
		}

		private IEnumerable<Container> GetAvailableContainers()
		{
			var sceneScopes = FindObjectsByType<SceneScope>(FindObjectsSortMode.None);
			var processedScenes = new HashSet<int>();
			for (int i = 0; i < sceneScopes.Length; i++)
			{
				Scene scene = sceneScopes[i].gameObject.scene;
				if (!scene.isLoaded || !processedScenes.Add(scene.handle))
				{
					continue;
				}

				yield return scene.GetSceneContainer();
			}

			if (Container.ProjectContainer != null)
			{
				yield return Container.ProjectContainer;
			}
		}

		private static bool IsValidBuildIndex(int buildIndex)
		{
			return buildIndex >= 0 && buildIndex < SceneManager.sceneCountInBuildSettings;
		}

		private int GetNextSceneBuildIndex()
		{
			return SceneManager.GetActiveScene().buildIndex + 1;
		}

		private void HandleSceneLoaded(Scene scene, LoadSceneMode mode)
		{
			if (!_initializationComplete)
			{
				return;
			}

			if (_worldSceneBootstrapInProgress)
			{
				return;
			}

			Debug.Log($"[COORDINATOR] Scene loaded callback received for {scene.name} ({scene.buildIndex})");
			StartCoroutine(RefreshSceneBindingsAndTryGenerate(scene));
		}

		private IEnumerator RefreshSceneBindingsAndTryGenerate(Scene scene)
		{
			_worldSceneBootstrapInProgress = true;
			_loadedStartupSceneBuildIndex = scene.buildIndex;
			Debug.Log($"[COORDINATOR] Refreshing scene data for {scene.name} ({scene.buildIndex})");

			Container loadedSceneContainer = null;
			for (int i = 0; i < WARNING_FRAME_INTERVAL; i++)
			{
				loadedSceneContainer = GetLoadedSceneContainer();
				if (loadedSceneContainer != null)
				{
					break;
				}

				yield return null;
			}

			if (loadedSceneContainer == null)
			{
				Debug.LogWarning($"[COORDINATOR] No scene container became available for {scene.name} ({scene.buildIndex}).");
				_worldSceneBootstrapInProgress = false;
				yield break;
			}

			// Refresh scene-specific data for all processors
			Debug.Log($"[COORDINATOR] Refreshing scene data for {_processors.Count} processors");
			foreach (var processor in _processors)
			{
				try
				{
					processor.RefreshSceneData(loadedSceneContainer);
					Debug.Log($"[COORDINATOR] Refreshed scene data for {GetProcessorName(processor)}");
				}
				catch (Exception ex)
				{
					Debug.LogError($"[COORDINATOR] Failed to refresh scene data for {GetProcessorName(processor)}: {ex.Message}");
				}
			}

			if (!TryResolveProcessor<WorldGenProcessor>(out var worldGenProcessor))
			{
				Debug.LogError("[COORDINATOR] Could not resolve WorldGenProcessor for scene bootstrap.");
				_worldSceneBootstrapInProgress = false;
				yield break;
			}

			try
			{
				Reflex.Injectors.AttributeInjector.Inject(worldGenProcessor, loadedSceneContainer);
				Debug.Log($"[COORDINATOR] Refreshed WorldGenProcessor scene data from {scene.name}.");

				LoadingProgressReporter.Report(0.55f, "Checking world generation...");
				worldGenProcessor.Initialize();
				Debug.Log($"[COORDINATOR] WorldGenProcessor reinitialized for {scene.name}.");
			}
			catch (Exception ex)
			{
				Debug.LogError($"[COORDINATOR] WorldGenProcessor scene-load initialize failed: {ex.Message}");
				_worldSceneBootstrapInProgress = false;
				yield break;
			}

			_worldSceneBootstrapInProgress = false;
		}

		private IEnumerator LoadSceneAdditively(int sceneBuildIndex)
		{
			Scene scene = SceneManager.GetSceneByBuildIndex(sceneBuildIndex);
			if (!scene.IsValid() || !scene.isLoaded)
			{
				Debug.Log($"[COORDINATOR] Loading next scene additively: {sceneBuildIndex}");
				AsyncOperation loadOperation = SceneManager.LoadSceneAsync(sceneBuildIndex, LoadSceneMode.Additive);
				if (loadOperation == null)
				{
					CurrentStartupState = StartupState.Failed;
					Debug.LogError($"[COORDINATOR] Failed to start additive load for scene {sceneBuildIndex}.");
					yield break;
				}

				while (!loadOperation.isDone)
				{
					yield return null;
				}
			}

			scene = SceneManager.GetSceneByBuildIndex(sceneBuildIndex);
			while (!scene.IsValid() || !scene.isLoaded)
			{
				yield return null;
				scene = SceneManager.GetSceneByBuildIndex(sceneBuildIndex);
			}

			if (!SceneManager.SetActiveScene(scene))
			{
				Debug.LogWarning($"[COORDINATOR] Loaded scene {sceneBuildIndex} additively but could not set it active.");
			}

			Debug.Log($"[COORDINATOR] Loaded next scene additively: {scene.name} ({sceneBuildIndex})");
		}

		private Container GetLoadedSceneContainer()
		{
			if (!IsValidBuildIndex(_loadedStartupSceneBuildIndex))
			{
				return null;
			}

			Scene scene = SceneManager.GetSceneByBuildIndex(_loadedStartupSceneBuildIndex);
			if (!scene.IsValid() || !scene.isLoaded)
			{
				return null;
			}

			var sceneScopes = FindObjectsByType<SceneScope>(FindObjectsSortMode.None);
			for (int i = 0; i < sceneScopes.Length; i++)
			{
				if (sceneScopes[i].gameObject.scene.handle != scene.handle)
				{
					continue;
				}

				try
				{
					return scene.GetSceneContainer();
				}
				catch
				{
					return null;
				}
			}

			return null;
		}

		private bool HasBindingInAvailableContainers(Type contractType)
		{
			foreach (var container in GetAvailableContainers())
			{
				if (container.HasBinding(contractType))
				{
					return true;
				}
			}

			return false;
		}

		private bool TryResolveFromAvailableContainers(Type contractType, out object resolved)
		{
			foreach (var container in GetAvailableContainers())
			{
				if (!container.HasBinding(contractType))
				{
					continue;
				}

				resolved = container.Resolve(contractType);
				return true;
			}

			resolved = null;
			return false;
		}

		private bool TryResolveProcessor<TProcessor>(out TProcessor processor) where TProcessor : class
		{
			if (TryResolveFromAvailableContainers(typeof(TProcessor), out var resolved) && resolved is TProcessor typedProcessor)
			{
				processor = typedProcessor;
				return true;
			}

			processor = null;
			return false;
		}

		private void CacheProcessorsFromBindings()
		{
			_processors.Clear();
			foreach (var processorType in GetRequiredProcessorTypes().OrderBy(type => type.Name))
			{
				if (!TryResolveFromAvailableContainers(processorType, out var resolved) || resolved is not IProcessor processor)
				{
					continue;
				}

				_processors.Add(processor);
			}
		}

		private IEnumerable<Type> GetRequiredProcessorTypes()
		{
			yield return typeof(AudioSourcesProcessor);
			yield return typeof(BuildingProcessor);
			yield return typeof(DayAndNightProcessor);
			yield return typeof(EventProcessor);
			yield return typeof(FoliageProcessor);
			yield return typeof(GameEventProcessor);
			yield return typeof(GameStateProcessor);
			yield return typeof(GUIDProcessor);
			yield return typeof(GridProcessor);
			yield return typeof(ObjectPoolingProcessor);
			yield return typeof(PlayerInputProcessor);
			yield return typeof(PlayerProcessor);
			yield return typeof(ResourceProcessor);
			yield return typeof(RoleProcessor);
			yield return typeof(SaveProcessor);
			yield return typeof(SeasonProcessor);
			yield return typeof(SensorProcessor);
			yield return typeof(StationProcessor);
			yield return typeof(TargetProcessor);
			yield return typeof(TechTreeProcessor);
			yield return typeof(TimeProcessor);
			yield return typeof(TownGoalProcessor);
			yield return typeof(TownResourceProcessor);
			yield return typeof(TradeProcessor);
			yield return typeof(TwitchChatProcessor);
			yield return typeof(LabelDisplayProcessor);
			yield return typeof(UIProcessor);
			yield return typeof(WeatherProcessor);
			yield return typeof(WorldGenProcessor);
		}

		private IEnumerable<string> GetMissingProcessorDependencies()
		{
			foreach (var processorType in GetRequiredProcessorTypes())
			{
				if (!HasBindingInAvailableContainers(processorType))
				{
					yield return processorType.Name;
				}
			}
		}

		private void CacheDataScriptablesFromLoadedSceneBindings()
		{
			_dataScriptables.Clear();
			Container loadedSceneContainer = GetLoadedSceneContainer();
			if (loadedSceneContainer == null)
			{
				return;
			}

			foreach (var dataScriptableType in GetRequiredDataScriptableTypes().OrderBy(type => type.Name))
			{
				if (!loadedSceneContainer.HasBinding(dataScriptableType))
				{
					continue;
				}

				object resolved = loadedSceneContainer.Resolve(dataScriptableType);
				if (resolved is not IDataScriptable dataScriptable)
				{
					continue;
				}

				_dataScriptables.Add(dataScriptable);
			}
		}

		private IEnumerable<Type> GetRequiredDataScriptableTypes()
		{
			yield return typeof(AllBuildingDataSettings);
			yield return typeof(AllRoleDataSettings);
			yield return typeof(ScriptablesProcessorInfrastructure.AllSeasonSettings);
			yield return typeof(ScriptablesProcessorInfrastructure.AudioSettings);
			yield return typeof(BuildingConfigSettings);
			yield return typeof(BuildingSettings);
			yield return typeof(CampGenSettings);
			yield return typeof(DayAndNightSettings);
			yield return typeof(DebugSettings);
			yield return typeof(FoliageGenSettings);
			yield return typeof(GameEventConfigSettings);
			yield return typeof(GameEventSettings);
			yield return typeof(GameSettings);
			yield return typeof(GridSettings);
			yield return typeof(ObjectPoolingSettings);
			yield return typeof(ObjectSelectionSettings);
			yield return typeof(PlayerInputSettings);
			yield return typeof(ResourceDataSettings);
			yield return typeof(ResourceGenSettings);
			yield return typeof(SaveSettings);
			yield return typeof(SensorSettings);
			yield return typeof(TargetSettings);
			yield return typeof(TechTreeSettings);
			yield return typeof(TerrainGenSettings);
			yield return typeof(TimeSettings);
			yield return typeof(TownGoalSettings);
			yield return typeof(TradeSettings);
			yield return typeof(UISettings);
			yield return typeof(WaterFoliageGenSettings);
			yield return typeof(WaterResourceGenSettings);
			yield return typeof(WeatherSettings);
			yield return typeof(WorldGenBehaviorSettings);
			yield return typeof(WorldGenDebugSettings);
			yield return typeof(WorldGenLayerSettings);
			yield return typeof(WorldGenScaleSettings);
		}

		private IEnumerable<string> GetMissingDataScriptableDependencies()
		{
			Container loadedSceneContainer = GetLoadedSceneContainer();
			if (loadedSceneContainer == null)
			{
				yield return "LoadedSceneContainer";
				yield break;
			}

			foreach (var dataScriptableType in GetRequiredDataScriptableTypes())
			{
				if (!loadedSceneContainer.HasBinding(dataScriptableType))
				{
					yield return dataScriptableType.Name;
					continue;
				}

				object resolved = loadedSceneContainer.Resolve(dataScriptableType);
				if (resolved is not IDataScriptable)
				{
					yield return dataScriptableType.Name;
				}
			}
		}

		private bool AllProcessorsAvailable()
		{
			return !GetMissingProcessorDependencies().Any();
		}

		private bool AllDataScriptablesAvailable()
		{
			return !GetMissingDataScriptableDependencies().Any();
		}

		private async Task InitializeAndActivateProcessorsAsync(CancellationToken cancellationToken)
		{
			Debug.Log($"[COORDINATOR] Starting initialization of {_processors.Count} processors");
			ParallelProgressReporter.Reset();
			InitializeStartupReports();

			_processorStartupTasks.Clear();
			foreach (var processor in _processors)
			{
				_processorStartupTasks.Add(InitializeProcessorAsync(processor, cancellationToken));
			}

			await Task.WhenAll(_processorStartupTasks);
			Debug.Log("[COORDINATOR] All processors initialized");

			CurrentStartupState = StartupState.Activating;
			Debug.Log("[COORDINATOR] Stage: Activating processors");
			ActivateProcessors();
			Debug.Log("[COORDINATOR] All processors activated");
		}

		private void InitializeStartupReports()
		{
			_startupReports.Clear();
			float processorWeight = _processors.Count > 0 ? 1f / _processors.Count : 1f;

			foreach (var processor in _processors)
			{
				string processorName = GetProcessorName(processor);
				_startupReports[processorName] = new ProcessorStartupReport(processorName);
				ParallelProgressReporter.RegisterTrack(processorName, processorWeight);
				Debug.Log($"[COORDINATOR] Registered track for: {processorName}");
			}
		}

		private async Task InitializeProcessorAsync(IProcessor processor, CancellationToken cancellationToken)
		{
			string processorName = GetProcessorName(processor);
			ProcessorStartupReport report = _startupReports[processorName];
			report.Stage = ProcessorStartupStage.Initializing;
			UpdateProcessorProgress(processorName, report, 0f, "Preparing initialization...");
			Debug.Log($"[COORDINATOR] Initializing: {processorName}");

			try
			{
				ProcessorStartupContext startupContext = new ProcessorStartupContext(
					processorName,
					(progress, status) => UpdateProcessorProgress(processorName, report, progress, status));

				if (processor is IAsyncInitializableProcessor asyncInitializableProcessor)
				{
					UpdateProcessorProgress(processorName, report, 0.1f, "Running async pre-initialize...");
					await asyncInitializableProcessor.InitializeAsync(startupContext, cancellationToken);
				}

				cancellationToken.ThrowIfCancellationRequested();
				UpdateProcessorProgress(processorName, report, 0.8f, processor is IMainThreadInitializableProcessor ? "Running main-thread initialize..." : "Running task initialize...");

				if (processor is IMainThreadInitializableProcessor)
				{
					await Task.Yield();
					cancellationToken.ThrowIfCancellationRequested();
					processor.Initialize();
				}
				else
				{
					await Task.Run(() =>
					{
						cancellationToken.ThrowIfCancellationRequested();
						processor.Initialize();
					}, cancellationToken);
				}

				report.Stage = ProcessorStartupStage.Initialized;
				UpdateProcessorProgress(processorName, report, 1f, processor is IPostInitializeProcessor ? "Initialized" : "Ready");
				Debug.Log($"[COORDINATOR] Initialized: {processorName}");

				if (processor is not IPostInitializeProcessor)
				{
					report.Stage = ProcessorStartupStage.Activated;
					Debug.Log($"[COORDINATOR] Ready: {processorName}");
				}
			}
			catch (OperationCanceledException)
			{
				report.Stage = ProcessorStartupStage.Failed;
				report.Status = "Cancelled";
				Debug.LogError($"[COORDINATOR] Initialization cancelled: {processorName}");
				throw;
			}
			catch (Exception ex)
			{
				report.Stage = ProcessorStartupStage.Failed;
				report.Exception = ex;
				report.Status = ex.Message;
				UpdateProcessorProgress(processorName, report, report.Progress, $"Failed: {ex.Message}");
				Debug.LogError($"[COORDINATOR] Initialization failed: {processorName} - {ex.Message}");
				throw;
			}
		}

		private void InjectProcessorsWithDataTypes()
		{
			Debug.Log("[COORDINATOR] Injecting data types into processors");
			Container injectionContainer = GetLoadedSceneContainer() ?? Container.ProjectContainer;
			if (injectionContainer == null)
			{
				Debug.LogError("[COORDINATOR] No container available for injection");
				return;
			}
			foreach (var processor in _processors)
			{
				try
				{
					Reflex.Injectors.AttributeInjector.Inject(processor, injectionContainer);
					Debug.Log($"[COORDINATOR] Injected: {GetProcessorName(processor)}");
				}
				catch (Exception ex)
				{
					Debug.LogError($"[COORDINATOR] Injection failed: {GetProcessorName(processor)} - {ex.Message}");
					throw;
				}
			}
			Debug.Log("[COORDINATOR] All processors injected with data types");
		}

		private void ActivateProcessors()
		{
			foreach (var processor in _processors)
			{
				if (processor is not IPostInitializeProcessor postInitializeProcessor)
					continue;

				string processorName = GetProcessorName(processor);
				ProcessorStartupReport report = _startupReports[processorName];

				try
				{
					report.Stage = ProcessorStartupStage.Activating;
					UpdateProcessorProgress(processorName, report, 1f, "Activating...");
					Debug.Log($"[COORDINATOR] Activating: {processorName}");

					postInitializeProcessor.Activate();

					report.Stage = ProcessorStartupStage.Activated;
					UpdateProcessorProgress(processorName, report, 1f, "Ready");
					Debug.Log($"[COORDINATOR] Activated: {processorName}");
				}
				catch (Exception ex)
				{
					report.Stage = ProcessorStartupStage.Failed;
					report.Exception = ex;
					UpdateProcessorProgress(processorName, report, report.Progress, $"Failed: {ex.Message}");
					Debug.LogError($"[COORDINATOR] Activation failed: {processorName} - {ex.Message}");
					throw;
				}
			}
		}

		private void UpdateProcessorProgress(string processorName, ProcessorStartupReport report, float progress, string status)
		{
			report.Progress = Mathf.Clamp01(progress);
			report.Status = status;
			ParallelProgressReporter.UpdateTrack(processorName, report.Progress, status);
		}

		private static string GetProcessorName(IProcessor processor)
		{
			string typeName = processor.GetType().Name;
			if (typeName.EndsWith("Processor"))
			{
				string baseName = typeName.Substring(0, typeName.Length - "Processor".Length);
				return $"{baseName} Processor";
			}

			return typeName;
		}

		private void Update()
		{
			if (!_initializationComplete)
				return;

			foreach (var processor in _processors)
			{
				processor.Process();
			}
		}

		private void OnDestroy()
		{
			Debug.Log("[COORDINATOR] Destroying coordinator");
			if (_startupCancellationTokenSource != null)
			{
				_startupCancellationTokenSource.Cancel();
				_startupCancellationTokenSource.Dispose();
				_startupCancellationTokenSource = null;
			}
		}
	}
}
