using MetaData;
using Reflex.Attributes;
using SavingAndLoading;
using Scriptables;
using Settings;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.UI;

namespace UserInterface.MainMenu 
{
	public class MainMenuManager : MonoBehaviour
	{
		[SerializeField]
		private int _sceneIndex = 0;

		[SerializeField]
		LoadingManager _loadingManager = null;

		[SerializeField]
		private MetaData.MetaData _metaData;

		[SerializeField]
		private Button _loadButton;

		[SerializeField]
		[Inject] SettingsData CurrentSettings;

		[SerializeField]
		private GameObject _channelNameUI;

		private bool _loading = false;

		[Inject] private SettingsManager _settingsManager;

		private LoadType _loadType;

		private string _channelName;

		public void ConfirmChannelName()
		{
			if (_channelName != null && _channelName != "")
			{
				CurrentSettings.channelName = _channelName;
				_settingsManager.SaveSettings();

				_loading = true;
				_metaData.LoadType = _loadType;
				_settingsManager.TogglingConnectionTab(false);
				_loadingManager.LoadWorldScene(_sceneIndex);
			}
		}

		public void SetChannelName(string name)
		{
			_channelName = name.ToLower();	
		}

		public void ToggleChannelName()
		{
			_channelNameUI.SetActive(!_channelNameUI.activeSelf);
		}

		public void GenerateWorld()
		{
			if (!_loading)
			{
				if (CurrentSettings.channelName != null && CurrentSettings.channelName != "")
				{
					_loading = true;
					_metaData.LoadType = LoadType.Generate;
					Debug.Log("Generating World");
					_settingsManager.TogglingConnectionTab(false);
					_loadingManager.LoadWorldScene(_sceneIndex);
				}
				else
				{
					_loadType = LoadType.Generate;
					ToggleChannelName();
				}
			}
		}

		public void LoadWorld()
		{
			if (!_loading)
			{
				if (CurrentSettings.channelName != null && CurrentSettings.channelName != "")
				{
					_loading = true;
					_metaData.LoadType = LoadType.Load;
					Debug.Log("Loading World");
					_settingsManager.TogglingConnectionTab(false);
					_loadingManager.LoadWorldScene(_sceneIndex);
				}
				else
				{
					_loadType = LoadType.Load;
					ToggleChannelName();
				}
			}
		}

		public void LoadCredits(int creditsSceneIndex)
		{
			_loadingManager.LoadNonWorldScenes(creditsSceneIndex);
		}

		[Inject] private SettingsPanel _settingsPanel;
		public void OptionMenuToggle()
		{
			_settingsPanel.Enabled = !_settingsPanel.Enabled;
		}

		public void QuitGame()
		{
			Application.Quit();
		}

		private void Start()
		{
			_loadingManager = FindObjectOfType<LoadingManager>();
			_metaData = FindObjectOfType<MetaData.MetaData>();
			_settingsManager.LoadedSettingsToUI();

			if (GameIO.DoesSaveFileExist(GameIO.SaveFileType.GameSave))
				return;

			_loadButton.interactable = false;
			_loadButton.image.color = new Color(191, 191, 191, 255);
			
		}

		private void Update()
		{
			if (Keyboard.current.escapeKey.wasPressedThisFrame)
			{
				if (_settingsPanel.Enabled)
				{
					_settingsManager.ToggleSettingsPanel();
				}
			}
		}
	}
}