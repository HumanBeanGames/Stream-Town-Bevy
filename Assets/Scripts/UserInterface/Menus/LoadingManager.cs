using UnityEngine;
using UnityEngine.SceneManagement;
using System.Collections;
using UnityEngine.UI;
using TMPro;
using Managers;
using System;

namespace UserInterface.MainMenu
{
	public class LoadingManager : MonoBehaviour
	{
		[SerializeField]
		private float _loadingSpeed = 0.5f;

		[SerializeField]
		private GameObject _loadingUI;

		[SerializeField]
		private float _waitTime = 0.5f;

		[SerializeField]
		private TextMeshProUGUI _tooltipText;

		[SerializeField]
		private Image _progressFillImage;

		[SerializeField]
		private TextMeshProUGUI _progressPercentText;

		[SerializeField]
		private TextMeshProUGUI _statusText;

		[SerializeField, TextArea]
		private string[] _toolTips;

		private float _loadProgress;

		private void DisableUI()
		{
			_loadingUI.SetActive(false);
			LoadingProgressReporter.End("Ready");
			GameStateManager.ReadiedPlayer -= DisableUI;
		}

		public void LoadNonWorldScenes(int sceneIndex)
		{
			Debug.Log("Loading scene " + sceneIndex);
			StartCoroutine(LoadAsyncScene(sceneIndex, false));
		}

		IEnumerator LoadAsyncScene(int sceneIndex, bool loadingWorld)
		{
			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();

			RandomizeTooltip();
			_loadingUI.SetActive(true);
			_loadProgress = 0f;
			LoadingProgressReporter.Begin("Preparing loading screen...");

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
				yield return null;
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
				yield return null;
			}
			stopwatch.Stop();
			Debug.Log($"[LOAD TIME] Scene activation: {stopwatch.ElapsedMilliseconds}ms");

			yield return new WaitForSeconds(_waitTime);

			if (loadingWorld)
			{
				LoadingProgressReporter.Report(0.5f, "Initializing world systems...");
				GameStateManager.ReadiedPlayer += DisableUI;
			}
			else
			{
				LoadingProgressReporter.End("Scene ready");
				_loadingUI.SetActive(false);
			}
		}

		public void LoadWorldScene(int sceneIndex)
		{
			StartCoroutine(LoadAsyncScene(sceneIndex, true));
		}

		private void RandomizeTooltip()
		{
			_tooltipText.text = _toolTips[UnityEngine.Random.Range(0,_toolTips.Length)];
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

		private void Awake()
		{
			DontDestroyOnLoad(gameObject);
			LoadingProgressReporter.Reset();
		}

		private void OnEnable()
		{
			LoadingProgressReporter.OnProgressUpdated += HandleProgressUpdated;
		}

		private void OnDisable()
		{
			LoadingProgressReporter.OnProgressUpdated -= HandleProgressUpdated;
		}
	}
}

