using System.Collections;
using MetaData;
using Reflex.Attributes;
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
        private MainMenuRuntimeData _mainMenuRuntimeData;

		[SerializeField]
		private MetaData.MetaData _metaData;

		[Inject]
		private MetaData.MetaData _injectedMetaData;

		[Inject]
		private SettingsProcessor _settingsProcessor;

		[Inject]
		private Twitch.TwitchClientProcessor _twitchClientProcessor;

		private SaveProcessor _saveProcessor;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		[SerializeField]
		LoadingManager _loadingManager = null;

		[SerializeField]
		private Button _loadButton;

		[Inject]
		private SettingsData _currentSettings;

		[SerializeField]
		private GameObject _channelNameUI;

		[Inject]
		private Access_ChannelNameInput _channelNameInput;

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
				_settingsProcessor.SaveChannelName(_mainMenuRuntimeData.ChannelName);
				_twitchClientProcessor?.EnsureConnectionForConfiguredChannel();

				_mainMenuRuntimeData.Loading = true;
				_metaData.LoadType = _mainMenuRuntimeData.LoadType;
				_settingsProcessor.TogglingConnectionTab(false);
				_loadingManager.LoadWorldScene(_sceneIndex, _mainMenuRuntimeData.LoadType);
			}
			else
			{
				_debugProcessor.LogError(DebugLogCategory.MainMenuManager, "Channel name is null or empty. Cannot confirm.");
			}
		}

		public void SetChannelName(string name)
		{
			_mainMenuRuntimeData.ChannelName = name?.Trim().TrimStart('#').ToLowerInvariant() ?? string.Empty;
		}

		public void ToggleChannelName()
		{
			_channelNameUI.SetActive(!_channelNameUI.activeSelf);
		}

		public void GenerateWorld()
		{
			if (!_mainMenuRuntimeData.Loading)
			{
				if (HasConfiguredChannel())
				{
					_mainMenuRuntimeData.Loading = true;
					_metaData.LoadType = LoadType.Generate;
					_debugProcessor.Log(DebugLogCategory.MainMenuManager, "Generating World");
					_settingsProcessor.TogglingConnectionTab(false);
					_loadingManager.LoadWorldScene(_sceneIndex, LoadType.Generate);
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
				if (HasConfiguredChannel())
				{
					_mainMenuRuntimeData.Loading = true;
					_metaData.LoadType = LoadType.Load;
					_debugProcessor.Log(DebugLogCategory.MainMenuManager, "Loading World");
					_settingsProcessor.TogglingConnectionTab(false);
					_loadingManager.LoadWorldScene(_sceneIndex, LoadType.Load);
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
			_settingsProcessor.ToggleSettingsPanel();
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
			_currentSettings ??= SettingsIO.LoadOrCreate();
			string savedChannel = _currentSettings?.channelName?.Trim() ?? string.Empty;
			_mainMenuRuntimeData.ChannelName = savedChannel;
			if (_channelNameInput != null && string.IsNullOrWhiteSpace(_channelNameInput.text))
				_channelNameInput.text = savedChannel;

			if (_metaData == null)
				_metaData = _injectedMetaData;
			if (_loadingManager == null)
				_loadingManager = FindAnyObjectByType<LoadingManager>();
			if (_metaData == null)
				_metaData = FindAnyObjectByType<MetaData.MetaData>();
			RefreshLoadButtonAvailability();
			StartCoroutine(RefreshLoadButtonWhenSaveStorageIsReady());
		}

		private bool HasConfiguredChannel()
		{
			_currentSettings ??= SettingsIO.LoadOrCreate();
			return _currentSettings != null && !string.IsNullOrWhiteSpace(_currentSettings.channelName);
		}

		private IEnumerator RefreshLoadButtonWhenSaveStorageIsReady()
		{
			// Scene Start can run before Reflex has finished injecting the persistent
			// SaveProcessor's storage. SavePath being populated means availability can
			// now be evaluated conclusively, including the valid "no save" case.
			while (_saveProcessor == null || string.IsNullOrEmpty(_saveProcessor.SavePath))
			{
				Container projectContainer = Container.ProjectContainer;
				if (projectContainer != null && projectContainer.HasBinding(typeof(SaveProcessor)))
					_saveProcessor = projectContainer.Resolve<SaveProcessor>();

				yield return null;
			}

			RefreshLoadButtonAvailability();
		}

		[Inject]
		private void InjectSaveProcessor(SaveProcessor saveProcessor)
		{
			_saveProcessor = saveProcessor;
			RefreshLoadButtonAvailability();
		}

		private void RefreshLoadButtonAvailability()
		{
			if (_loadButton != null)
				_loadButton.interactable = _saveProcessor != null && _saveProcessor.HasSaveGame;
		}
	}
}
