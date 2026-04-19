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

namespace Processors
{
	// TODO(Architecture): Excluded from strict processor-template field conformance because this class contains serialized scene/UI references. This MAY need to be migrated to a non-processor pattern.
	public class MainMenuProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		[Inject] private MainMenuSettings _mainMenuSettings;

        /// <summary>
        /// Runtime main menu data ScriptableObject.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private MainMenuRuntimeData _mainMenuRuntimeData;

		[Inject] private MetaData.MetaData _metaData;
		[Inject] private SettingsProcessor _settingsProcessor;
		[Inject] private SettingsPanel _settingsPanel;

		[SerializeField]
		LoadingProcessor _loadingProcessor = null;

		[SerializeField]
		private Button _loadButton;

		[SerializeField]
		[Inject] SettingsData CurrentSettings;

		[SerializeField]
		private GameObject _channelNameUI;

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

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register MainMenuRuntimeData ScriptableObject
			MainMenuRuntimeData mainMenuRuntimeData = ScriptableObject.CreateInstance<MainMenuRuntimeData>();
			containerBuilder.AddSingleton(mainMenuRuntimeData);
		}

		public void ConfirmChannelName()
		{
			if (_mainMenuRuntimeData.ChannelName != null && _mainMenuRuntimeData.ChannelName != "")
			{
				CurrentSettings.channelName = _mainMenuRuntimeData.ChannelName;
				_settingsProcessor.SaveSettings();

				_mainMenuRuntimeData.Loading = true;
				_metaData.LoadType = _mainMenuRuntimeData.LoadType;
				_settingsProcessor.TogglingConnectionTab(false);
				_loadingProcessor.LoadWorldScene(_mainMenuSettings.SceneIndex);
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
					Debug.Log("Generating World");
					_settingsProcessor.TogglingConnectionTab(false);
					_loadingProcessor.LoadWorldScene(_mainMenuSettings.SceneIndex);
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
					Debug.Log("Loading World");
					_settingsProcessor.TogglingConnectionTab(false);
					_loadingProcessor.LoadWorldScene(_mainMenuSettings.SceneIndex);
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
			_loadingProcessor.LoadNonWorldScenes(creditsSceneIndex);
		}

		public void OptionMenuToggle()
		{
			_settingsPanel.Enabled = !_settingsPanel.Enabled;
		}

		public void QuitGame()
		{
			Application.Quit();
		}

		public void Initialize()
		{
			_loadingProcessor = FindAnyObjectByType<LoadingProcessor>();

			if (GameIO.DoesSaveFileExist(GameIO.SaveFileType.GameSave))
				return;

			_loadButton.interactable = false;
			_loadButton.image.color = new Color(191, 191, 191, 255);
		}

		/// <summary>
		/// Processes main menu logic every frame.
		/// Called every frame by the Coordinator.
		/// </summary>
		public void Process()
		{
			if (Keyboard.current.escapeKey.wasPressedThisFrame)
			{
				if (_settingsPanel.Enabled)
				{
					_settingsProcessor.ToggleSettingsPanel();
				}
			}
		}
	}
}
