using UnityEngine;
using Settings;
using SavingAndLoading;
using UnityEngine.UI;
using UnityEngine.InputSystem;
using UnityEngine.SceneManagement;
using PlayerControls;
using UserInterface.MainMenu;
using Processors;
using Core;
using Reflex.Attributes;

namespace UserInterface 
{
    public class UserInterface_GameMenu : MonoBehaviour 
	{
		[SerializeField]
		private GameObject _gameMenu;

		[SerializeField]
		private GameObject _savePanel;

		[SerializeField]
		private GameObject _loadPanel;

		[SerializeField]
		private GameObject _mainMenuPanel;
		
		private GameObject _settingsPanel;
		[Inject] SettingsProcessor _settingsProcessor;
		[Inject] private Coordinator _gameProcessor;
		[Inject] private SaveProcessor _saveProcessor;

		private LoadingProcessor _loadingProcessor;
		private bool _savedGame;

		public bool SavedGame
        {
            get { return _savedGame; }
			set { _savedGame = value; }
		}
		public void ToggleGameMenu()
		{
			_gameMenu.SetActive(!_gameMenu.activeSelf);
		}

		public void ToggleSettingsPanel()
		{
			_settingsPanel.SetActive(!_settingsPanel.activeSelf);
		}

		public void ToggleSavePanel()
		{
			_savePanel.SetActive(!_savePanel.activeSelf);
		}

		public void ToggleLoadPanel()
		{
			_loadPanel.SetActive(!_loadPanel.activeSelf);			
		}

		public void ToggleMainMenuPanel()
		{
			_mainMenuPanel.SetActive(!_mainMenuPanel.activeSelf);
		}

		public void QuitToMainMenu()
        {
			_settingsProcessor.TogglingConnectionTab(true);
			_loadingProcessor.LoadNonWorldScenes(1);
		}


		[Inject] private Autosave Autosave;
        [Inject] SettingsData CurrentSettings;
        private void Start()
		{
			_loadingProcessor = FindAnyObjectByType<LoadingProcessor>();

			//REVISIT
			if (_gameProcessor != null)
				_saveProcessor.SetAutosaveTime(Autosave.Intervals[CurrentSettings.autosaveTime] * 60.0f);
		}

		private void Update()
		{
			if(Keyboard.current.escapeKey.wasPressedThisFrame)
			{
				if (!_mainMenuPanel.activeSelf && !_settingsPanel.activeSelf)
				{
					ToggleGameMenu();
				}
				if (_settingsPanel.activeSelf)
				{
					_settingsProcessor.ToggleSettingsPanel();
				}
				if (_mainMenuPanel.activeSelf)
				{
					ToggleMainMenuPanel();
				}
			}
		}
	}
}
