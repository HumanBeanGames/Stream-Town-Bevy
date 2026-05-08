using MetaData;
using Reflex.Attributes;
using SavingAndLoading;
using ScriptablesProcessorInfrastructure;
using Settings;
using UnityEngine;
using UnityEngine.InputSystem;
using UnityEngine.UI;
using Reflex.Core;
using Data.Containers;
using UserInterface.MainMenu;
using Processors;

namespace UserInterface.MainMenu
{
	// TODO(Architecture): Excluded from strict processor-template field conformance because this class contains serialized scene/UI references. This MAY need to be migrated to a non-processor pattern.
	public class MainMenuManager : MonoBehaviour
	{
		[SerializeField]
		private int _sceneIndex = 0;

        /// <summary>
        /// Runtime main menu data ScriptableObject.
        /// </summary>
        [SerializeField]
        private MainMenuRuntimeData _mainMenuRuntimeData;

		[SerializeField]
		private MetaData.MetaData _metaData;

		[Inject]
		private MetaData.MetaData _injectedMetaData;

		[Inject]
		private SettingsProcessor _settingsProcessor;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		[SerializeField]
		private SettingsPanel _settingsPanel;

		[SerializeField]
		LoadingManager _loadingManager = null;

		[SerializeField]
		private Button _loadButton;

		[SerializeField]
		SettingsData CurrentSettings;

		[SerializeField]
		private GameObject _channelNameUI;

		[Inject]
		private Access_ChannelNameInput _channelNameInput;

		public System.Action<string> CodeDisplay
		{
			get => _mainMenuRuntimeData.CodeDisplay;
			set => _mainMenuRuntimeData.CodeDisplay = value;
		}

		public GameObject ConnectPanel
		{
			get => _mainMenuRuntimeData.ConnectPanel;
			set => _mainMenuRuntimeData.ConnectPanel = value;
		}

		public LoadType LoadType
		{
			get => _mainMenuRuntimeData.LoadType;
			set => _mainMenuRuntimeData.LoadType = value;
		}

		public void ConfirmChannelName()
		{
			_debugProcessor.Log(DebugLogCategory.MainMenuManager, $"ConfirmChannelName called. Input text: {_channelNameInput?.text}");

			// Get the channel name from the input field
			string inputChannelName = _channelNameInput?.text ?? string.Empty;
			SetChannelName(inputChannelName);

			_debugProcessor.Log(DebugLogCategory.MainMenuManager, $"ChannelName after SetChannelName: {_mainMenuRuntimeData.ChannelName}");

			if (_mainMenuRuntimeData.ChannelName != null && _mainMenuRuntimeData.ChannelName != "")
			{
				_debugProcessor.Log(DebugLogCategory.MainMenuManager, "Channel name is valid, proceeding to save and load");
				CurrentSettings.channelName = _mainMenuRuntimeData.ChannelName;
				_settingsProcessor.SaveSettings();

				_mainMenuRuntimeData.Loading = true;
				_metaData.LoadType = _mainMenuRuntimeData.LoadType;
				_settingsProcessor.TogglingConnectionTab(false);
				_loadingManager.LoadWorldScene(_sceneIndex);
			}
			else
			{
				_debugProcessor.LogError(DebugLogCategory.MainMenuManager, "Channel name is null or empty. Cannot confirm.");
			}
		}

		public void SetChannelName(string name)
		{
			_mainMenuRuntimeData.ChannelName = name.ToLower();	
		}

		public void ToggleChannelName()
		{
			_channelNameUI.SetActive(!_channelNameUI.activeSelf);
		}

		public void GenerateWorld()
		{
			if (!_mainMenuRuntimeData.Loading)
			{
				if (CurrentSettings.channelName != null && CurrentSettings.channelName != "")
				{
					_mainMenuRuntimeData.Loading = true;
					_metaData.LoadType = LoadType.Generate;
					_debugProcessor.Log(DebugLogCategory.MainMenuManager, "Generating World");
					_settingsProcessor.TogglingConnectionTab(false);
					_loadingManager.LoadWorldScene(_sceneIndex);
				}
				else
				{
					_mainMenuRuntimeData.LoadType = LoadType.Generate;
					ToggleChannelName();
				}
			}
		}

		public void LoadWorld()
		{
			if (!_mainMenuRuntimeData.Loading)
			{
				if (CurrentSettings.channelName != null && CurrentSettings.channelName != "")
				{
					_mainMenuRuntimeData.Loading = true;
					_metaData.LoadType = LoadType.Load;
					_debugProcessor.Log(DebugLogCategory.MainMenuManager, "Loading World");
					_settingsProcessor.TogglingConnectionTab(false);
					_loadingManager.LoadWorldScene(_sceneIndex);
				}
				else
				{
					_mainMenuRuntimeData.LoadType = LoadType.Load;
					ToggleChannelName();
				}
			}
		}

		public void LoadCredits(int creditsSceneIndex)
		{
			_loadingManager.LoadNonWorldScenes(creditsSceneIndex);
		}

		public void OptionMenuToggle()
		{
			_settingsPanel.Enabled = !_settingsPanel.Enabled;
		}

		public void QuitGame()
		{
			Application.Quit();
		}

		private void Awake()
		{
			_mainMenuRuntimeData = new MainMenuRuntimeData();
		}

		private void Start()
		{
			if (_metaData == null)
				_metaData = _injectedMetaData;
			if (_loadingManager == null)
				_loadingManager = FindAnyObjectByType<LoadingManager>();
			if (_metaData == null)
				_metaData = FindAnyObjectByType<MetaData.MetaData>();
			if (_settingsPanel == null)
				_settingsPanel = FindAnyObjectByType<SettingsPanel>();

			if (GameIO.DoesSaveFileExist(GameIO.SaveFileType.GameSave))
				return;

			_loadButton.interactable = false;
			_loadButton.image.color = new Color(191, 191, 191, 255);
		}
	}
}
