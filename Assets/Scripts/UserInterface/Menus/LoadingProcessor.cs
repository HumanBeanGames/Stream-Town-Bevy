using Processors;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;
using TMPro;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using UserInterface.MainMenu;

namespace Processors
{
	// TODO(Architecture): Excluded from strict processor-template field conformance because this class contains serialized scene/UI references. This MAY need to be migrated to a non-processor pattern.
	public class LoadingProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		[Inject] private LoadingSettings _loadingSettings;

        /// <summary>
        /// Runtime loading data ScriptableObject.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private LoadingRuntimeData _loadingRuntimeData;

        /// <summary>
        /// Gets the on scene load requested event.
        /// </summary>
        public event Action<int> OnSceneLoadRequested
        {
            add => _loadingRuntimeData.OnSceneLoadRequested += value;
            remove => _loadingRuntimeData.OnSceneLoadRequested -= value;
        }

		[Inject] private GameStateProcessor _gameStateProcessor;

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
		private Image _buildingProcessorProgress;

		[SerializeField]
		private TextMeshProUGUI _buildingProcessorStatusText;

		[SerializeField]
		private TextMeshProUGUI _buildingProcessorPercentText;

		[SerializeField]
		private Image _playerProcessorProgress;

		[SerializeField]
		private TextMeshProUGUI _playerProcessorStatusText;

		[SerializeField]
		private TextMeshProUGUI _playerProcessorPercentText;

		[SerializeField]
		private Image _townGoalProcessorProgress;

		[SerializeField]
		private TextMeshProUGUI _townGoalProcessorStatusText;

		[SerializeField]
		private TextMeshProUGUI _townGoalProcessorPercentText;

		[SerializeField]
		private Image _roleProcessorProgress;

		[SerializeField]
		private TextMeshProUGUI _roleProcessorStatusText;

		[SerializeField]
		private TextMeshProUGUI _roleProcessorPercentText;

		[SerializeField]
		private Image _townResourceProcessorProgress;

		[SerializeField]
		private TextMeshProUGUI _townResourceProcessorStatusText;

		[SerializeField]
		private TextMeshProUGUI _townResourceProcessorPercentText;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register LoadingRuntimeData ScriptableObject
			LoadingRuntimeData loadingRuntimeData = ScriptableObject.CreateInstance<LoadingRuntimeData>();
			containerBuilder.AddSingleton(loadingRuntimeData);
		}

		private void DisableUI()
		{
			_loadingUI.SetActive(false);
			LoadingProgressReporter.End("Ready");
			_gameStateProcessor.ReadiedPlayer -= DisableUI;
		}

		public async void LoadNonWorldScenes(int sceneIndex)
		{
			Debug.Log("Loading scene " + sceneIndex);
			await LoadSceneAsync(sceneIndex, false);
		}

		private async Task LoadSceneAsync(int sceneIndex, bool loadingWorld)
		{
			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();

			RandomizeTooltip();
			_loadingUI.SetActive(true);
			_loadingRuntimeData.LoadProgress = 0f;
			LoadingProgressReporter.Begin("Preparing loading screen...");

			stopwatch.Restart();
			AsyncOperation asyncLoad = SceneManager.LoadSceneAsync(sceneIndex);
			asyncLoad.allowSceneActivation = false;

			float currentProgress = 0.0f;
			float targetProgress = new float();

			while (currentProgress < 1)
			{
				targetProgress = asyncLoad.progress / 0.9f;
				currentProgress = Mathf.MoveTowards(_loadingRuntimeData.LoadProgress, targetProgress, _loadingSettings.LoadingSpeed * Time.deltaTime);
				_loadingRuntimeData.LoadProgress = currentProgress;

				float sceneLoadProgress = loadingWorld ? _loadingRuntimeData.LoadProgress * 0.4f : _loadingRuntimeData.LoadProgress;
				LoadingProgressReporter.Report(sceneLoadProgress, $"Loading scene {sceneIndex}...");
				await Task.Yield();
			}
			stopwatch.Stop();
			Debug.Log($"[LOAD TIME] Scene async load: {stopwatch.ElapsedMilliseconds}ms");

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
			Debug.Log($"[LOAD TIME] Scene activation: {stopwatch.ElapsedMilliseconds}ms");

			await Task.Delay((int)(_loadingSettings.WaitTime * 1000));

			if (loadingWorld)
			{
				LoadingProgressReporter.Report(0.5f, "Initializing world systems...");
				_gameStateProcessor.ReadiedPlayer += DisableUI;
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
			_tooltipText.text = _loadingSettings.ToolTips[UnityEngine.Random.Range(0, _loadingSettings.ToolTips.Length)];
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
			_loadingRuntimeData.OnSceneLoadRequested += HandleSceneLoadRequested;
		}

		public void Process()
		{
			// LoadingProcessor does not require per-frame updates
		}

		private void HandleSceneLoadRequested(int sceneIndex)
		{
			LoadNonWorldScenes(sceneIndex);
		}

		private void HandleParallelProgressUpdated(float progress01, string status)
		{
			// Update main progress bar with overall parallel progress
			HandleProgressUpdated(progress01, status);
		}

		private void HandleTrackProgressUpdated(Dictionary<string, (float progress, string status)> tracks)
		{
			if (tracks.ContainsKey("Building Processor"))
			{
				if (_buildingProcessorProgress != null)
					_buildingProcessorProgress.fillAmount = tracks["Building Processor"].progress;
				if (_buildingProcessorStatusText != null)
					_buildingProcessorStatusText.text = tracks["Building Processor"].status;
				if (_buildingProcessorPercentText != null)
					_buildingProcessorPercentText.text = $"{Mathf.RoundToInt(tracks["Building Processor"].progress * 100f)}%";
			}

			if (tracks.ContainsKey("Player Processor"))
			{
				if (_playerProcessorProgress != null)
					_playerProcessorProgress.fillAmount = tracks["Player Processor"].progress;
				if (_playerProcessorStatusText != null)
					_playerProcessorStatusText.text = tracks["Player Processor"].status;
				if (_playerProcessorPercentText != null)
					_playerProcessorPercentText.text = $"{Mathf.RoundToInt(tracks["Player Processor"].progress * 100f)}%";
			}

			if (tracks.ContainsKey("Town Goal Processor"))
			{
				if (_townGoalProcessorProgress != null)
					_townGoalProcessorProgress.fillAmount = tracks["Town Goal Processor"].progress;
				if (_townGoalProcessorStatusText != null)
					_townGoalProcessorStatusText.text = tracks["Town Goal Processor"].status;
				if (_townGoalProcessorPercentText != null)
					_townGoalProcessorPercentText.text = $"{Mathf.RoundToInt(tracks["Town Goal Processor"].progress * 100f)}%";
			}

			if (tracks.ContainsKey("Role Processor"))
			{
				if (_roleProcessorProgress != null)
					_roleProcessorProgress.fillAmount = tracks["Role Processor"].progress;
				if (_roleProcessorStatusText != null)
					_roleProcessorStatusText.text = tracks["Role Processor"].status;
				if (_roleProcessorPercentText != null)
					_roleProcessorPercentText.text = $"{Mathf.RoundToInt(tracks["Role Processor"].progress * 100f)}%";
			}

			if (tracks.ContainsKey("Town Resource Processor"))
			{
				if (_townResourceProcessorProgress != null)
					_townResourceProcessorProgress.fillAmount = tracks["Town Resource Processor"].progress;
				if (_townResourceProcessorStatusText != null)
					_townResourceProcessorStatusText.text = tracks["Town Resource Processor"].status;
				if (_townResourceProcessorPercentText != null)
					_townResourceProcessorPercentText.text = $"{Mathf.RoundToInt(tracks["Town Resource Processor"].progress * 100f)}%";
			}
		}
	}
}

