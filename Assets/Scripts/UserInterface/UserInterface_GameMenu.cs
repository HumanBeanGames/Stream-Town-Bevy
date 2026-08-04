using MetaData;
using PlayerControls;
using Processors;
using Reflex.Attributes;
using Settings;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.SceneManagement;
using UserInterface.MainMenu;

namespace UserInterface
{
	/// <summary>
	/// Thin UI adapter for the game menu. It owns no save state: commands are
	/// forwarded to SaveProcessor and scene transitions to LoadingManager.
	/// </summary>
	public class UserInterface_GameMenu : MonoBehaviour
	{
		[SerializeField] private GameObject _gameMenu;

		[Inject] private SettingsProcessor _settingsProcessor;
		[Inject] private SaveProcessor _saveProcessor;
		[Inject] private MetaData.MetaData _metaData;
		[Inject] private SettingsPanel _settingsPanel;

		private LoadingManager _loadingManager;

		public bool SavedGame => _saveProcessor != null && _saveProcessor.HasSaveGame;

		public void ToggleGameMenu()
		{
			if (_gameMenu == null)
			{
				Debug.LogError("The game menu visual panel has not been assigned.", this);
				return;
			}

			_gameMenu.SetActive(!_gameMenu.activeSelf);
		}

		public void ToggleSettingsPanel()
		{
			_settingsProcessor?.ToggleSettingsPanel();
		}

		// Kept as UnityEvent entry points for the existing menu prefab.
		public void ToggleSavePanel() => SaveGame();
		public void ToggleLoadPanel() => LoadGame();

		public void ToggleMainMenuPanel()
		{
			QuitToMainMenu();
		}

		public void SaveGame()
		{
			_saveProcessor?.SaveGame();
		}

		public void LoadGame()
		{
			if (_saveProcessor == null || !_saveProcessor.HasSaveGame || _saveProcessor.IsBusy)
				return;

			if (_metaData == null)
				_metaData = FindAnyObjectByType<MetaData.MetaData>();

			if (_loadingManager == null)
				_loadingManager = FindAnyObjectByType<LoadingManager>();

			if (_metaData == null || _loadingManager == null)
			{
				Debug.LogError("Cannot load the save: MetaData or LoadingManager is unavailable.");
				return;
			}

			_metaData.LoadType = LoadType.Load;
			_loadingManager.LoadWorldScene(SceneManager.GetActiveScene().buildIndex, LoadType.Load);
		}

		public void QuitToMainMenu()
		{
			_settingsProcessor?.TogglingConnectionTab(true);
			if (_loadingManager == null)
				_loadingManager = FindAnyObjectByType<LoadingManager>();
			_loadingManager?.LoadNonWorldScenes(1);
		}

		public void ToggleIdleMode(bool idle)
		{
			CameraController cameraController = FindAnyObjectByType<CameraController>();
			if (cameraController != null)
				cameraController.IsIdle = idle;
		}

		private void Awake()
		{
			if (_gameMenu == null)
				Debug.LogError("The game menu visual panel has not been assigned.", this);
		}

		private void Start()
		{
			_loadingManager = FindAnyObjectByType<LoadingManager>();
			_settingsProcessor?.ApplyAutosaveIntervalFromCurrentSettings();
		}

		private void Update()
		{
			if (Keyboard.current == null || !Keyboard.current.escapeKey.wasPressedThisFrame)
				return;

			if (_settingsPanel != null && _settingsPanel.Enabled)
			{
				_settingsProcessor.ToggleSettingsPanel();
				return;
			}

			ToggleGameMenu();
		}
	}
}
