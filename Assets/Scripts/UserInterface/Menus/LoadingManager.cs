using Processors;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;
using TMPro;
using UserInterface.MainMenu;
using UserInterface.Menus;
using Reflex.Attributes;

namespace UserInterface.MainMenu
{
	// TODO(Architecture): Excluded from strict processor-template field conformance because this class contains serialized scene/UI references. This MAY need to be migrated to a non-processor pattern.
	public class LoadingManager : MonoBehaviour
	{
		[Inject]
		private GameStateProcessor _gameStateProcessor;
		[Inject] private TimeProcessor _timeProcessor;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

        /// <summary>
        /// Speed at which the loading bar progresses.
        /// Higher values make the loading bar fill faster.
        /// </summary>
        [SerializeField]
        private float _loadingSpeed = 0.5f;

        /// <summary>
        /// Time to wait after loading completes before transitioning.
        /// Used to ensure the loading screen is visible for a minimum duration.
        /// </summary>
        [SerializeField]
        private float _waitTime = 0.5f;

        /// <summary>
        /// Array of tooltip strings to display during loading.
        /// Randomly selected tips are shown to engage players while waiting.
        /// </summary>
        [SerializeField]
        private string[] _toolTips;

        /// <summary>
        /// Current loading progress from 0.0 to 1.0.
        /// </summary>
        private float _loadProgress;

        /// <summary>
        /// Event fired when a scene load is requested.
        /// Passes the scene index to load.
        /// </summary>
        public event Action<int> OnSceneLoadRequested;

		[SerializeField]
		private GameObject _loadingUI;

		[SerializeField]
		private TextMeshProUGUI _tooltipText;

		[SerializeField]
		private Image _progressFillImage;

		[SerializeField]
		private TextMeshProUGUI _progressPercentText;

		[SerializeField]
		private TextMeshProUGUI _statusText;

		[SerializeField]
		private GameObject _statusBarPrefab;

		[SerializeField]
		private Transform _statusBarContainer;

		private readonly Dictionary<string, StatusBar> _activeStatusBars = new Dictionary<string, StatusBar>();
		private GameObject _persistentLoadingUiRoot;

		private void DisableUI()
		{
			_loadingUI.SetActive(false);
			LoadingProgressReporter.End("Ready");
			_gameStateProcessor.GeneratedWorld -= DisableUI;
		}

		public async void LoadNonWorldScenes(int sceneIndex)
		{
			_debugProcessor.Log(DebugLogCategory.LoadingManager, "Loading scene " + sceneIndex);
			await LoadSceneAsync(sceneIndex, false);
		}

		private async Task LoadSceneAsync(int sceneIndex, bool loadingWorld)
		{
			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();

			RandomizeTooltip();
			_loadingUI.SetActive(true);
			_loadProgress = 0f;
			LoadingProgressReporter.Begin("Preparing loading screen...");
			if (loadingWorld)
			{
				if (_gameStateProcessor == null)
				{
					_debugProcessor.LogError(DebugLogCategory.LoadingManager, "LoadingManager: GameStateProcessor was not injected before world loading initialization.");
					return;
				}

				_gameStateProcessor.NotifyLoadingWorld();
				_gameStateProcessor.GeneratedWorld -= DisableUI;
				_gameStateProcessor.GeneratedWorld += DisableUI;
			}

			stopwatch.Restart();
			AsyncOperation asyncLoad = SceneManager.LoadSceneAsync(sceneIndex);
			asyncLoad.allowSceneActivation = false;

			float currentProgress = 0.0f;
			float targetProgress = new float();

			while (currentProgress < 1)
			{
				targetProgress = asyncLoad.progress / 0.9f;
				currentProgress = Mathf.MoveTowards(_loadProgress, targetProgress, _loadingSpeed * Time.deltaTime);
				_loadProgress = currentProgress;

				float sceneLoadProgress = loadingWorld ? _loadProgress * 0.4f : _loadProgress;
				LoadingProgressReporter.Report(sceneLoadProgress, $"Loading scene {sceneIndex}...");
				await Task.Yield();
			}
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.LoadingManager, $"[LOAD TIME] Scene async load: {stopwatch.ElapsedMilliseconds}ms");

			Scene scene = new Scene();

			stopwatch.Restart();
			asyncLoad.allowSceneActivation = true;

			while (scene != SceneManager.GetSceneByBuildIndex(sceneIndex))
			{
				scene = SceneManager.GetActiveScene();
				float activationProgress = loadingWorld ? 0.45f : 0.95f;
				LoadingProgressReporter.Report(activationProgress, "Activating scene...");
				await Task.Yield();
			}
			stopwatch.Stop();
			_debugProcessor.Log(DebugLogCategory.LoadingManager, $"[LOAD TIME] Scene activation: {stopwatch.ElapsedMilliseconds}ms");

			await Task.Delay((int)(_waitTime * 1000));

			if (loadingWorld)
			{
				LoadingProgressReporter.Report(0.5f, "Initializing world systems...");
			}
			else
			{
				LoadingProgressReporter.End("Scene ready");
				_loadingUI.SetActive(false);
			}
		}

		public async void LoadWorldScene(int sceneIndex)
		{
			await LoadSceneAsync(sceneIndex, true);
		}

		private void RandomizeTooltip()
		{
			_tooltipText.text = _toolTips[UnityEngine.Random.Range(0, _toolTips.Length)];
		}

		private void HandleProgressUpdated(float progress01, string status)
		{
			if (_progressFillImage != null)
				_progressFillImage.fillAmount = progress01;

			if (_progressPercentText != null)
				_progressPercentText.text = $"{Mathf.RoundToInt(progress01 * 100f)}%";

			if (_statusText != null)
				_statusText.text = status;
		}

		public void Initialize()
		{
			LoadingProgressReporter.OnProgressUpdated += HandleProgressUpdated;
			ParallelProgressReporter.OnOverallProgressUpdated += HandleParallelProgressUpdated;
			ParallelProgressReporter.OnTrackProgressUpdated += HandleTrackProgressUpdated;
			ParallelProgressReporter.OnTrackRegistered += HandleTrackRegistered;
			OnSceneLoadRequested += HandleSceneLoadRequested;
		}

		private void HandleSceneLoadRequested(int sceneIndex)
		{
			LoadNonWorldScenes(sceneIndex);
		}

		private void Awake()
		{
			if (transform.parent != null)
				transform.SetParent(null, false);

			DontDestroyOnLoad(gameObject);
			if (_loadingUI != null)
			{
				_persistentLoadingUiRoot = _loadingUI.transform.root.gameObject;
				if (_persistentLoadingUiRoot != gameObject)
					DontDestroyOnLoad(_persistentLoadingUiRoot);
			}

			Initialize();
		}

		private void HandleParallelProgressUpdated(float progress01, string status)
		{
			// Update main progress bar with overall parallel progress
			HandleProgressUpdated(progress01, status);
		}

		private void HandleTrackProgressUpdated(Dictionary<string, (float progress, string status)> tracks)
		{
			foreach (var track in tracks)
			{
				if (_activeStatusBars.TryGetValue(track.Key, out StatusBar statusBar))
				{
					statusBar.SetProgress(track.Value.progress, track.Value.status);

					if (track.Value.progress >= 1f)
					{
						DestroyStatusBar(track.Key);
					}
				}
			}
		}

		private void HandleTrackRegistered(string trackName)
		{
			if (_statusBarPrefab == null || _statusBarContainer == null)
				return;

			if (_activeStatusBars.ContainsKey(trackName))
				return;

			GameObject statusBarObj = Instantiate(_statusBarPrefab, _statusBarContainer);
			StatusBar statusBar = statusBarObj.GetComponent<StatusBar>();
			statusBar.SetProgress(0f, "Waiting...");
			_activeStatusBars[trackName] = statusBar;
		}

		private void DestroyStatusBar(string trackName)
		{
			if (_activeStatusBars.TryGetValue(trackName, out StatusBar statusBar))
			{
				if (statusBar != null)
					Destroy(statusBar.gameObject);

				_activeStatusBars.Remove(trackName);
			}
		}

		private void OnDestroy()
		{
			LoadingProgressReporter.OnProgressUpdated -= HandleProgressUpdated;
			ParallelProgressReporter.OnOverallProgressUpdated -= HandleParallelProgressUpdated;
			ParallelProgressReporter.OnTrackProgressUpdated -= HandleTrackProgressUpdated;
			ParallelProgressReporter.OnTrackRegistered -= HandleTrackRegistered;
			OnSceneLoadRequested -= HandleSceneLoadRequested;
			if (_gameStateProcessor != null)
				_gameStateProcessor.GeneratedWorld -= DisableUI;

			foreach (var statusBar in _activeStatusBars.Values)
			{
				if (statusBar != null)
					Destroy(statusBar.gameObject);
			}
			_activeStatusBars.Clear();
		}
	}
}

