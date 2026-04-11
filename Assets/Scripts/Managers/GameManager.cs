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
using UnityEngine.EventSystems;
using UserInterface.MainMenu;
using Reflex.Attributes;
using Reflex.Core;
using System.Collections;
using Environment;
using Scriptables;
using Utils.Pooling;
using Twitch;
using Audio;

namespace Managers
{
	[DefaultExecutionOrder(-1000)]
	public class GameManager : MonoBehaviour
	{
		public static string[] GM_IDS = new[] { "43134305", "47817756", "51998688", "652607201", "159586407", "489520238", "56878491", "406879525" };

		public BuildingPlacer _buildingPlacer;

		[SerializeField]
		private Transform _playerSpawnPosition = null;
		[SerializeField]
		private EnemySpawner _enemySpawner;
		[SerializeField]
		private GameObject _connectPanel;

		private ProceduralWorldGenerator _proceduralWorldGen = null;
		[Inject] private MetaData.MetaData _metaData;
		private Player _debugPlayer;
		public TwitchUser _broadcaster;

		[Inject] private GameEventManager _gameEventManager;
		[Inject] private AudioSourcesManager _audioSourcesManager;
		[Inject] private BuildingManager _buildingManager;
		[Inject] private Container _container;
		[Inject] private Container _sceneContainer;
		[Inject] private PlayerManager _playerManager;
		[Inject] private TownGoalManager _townGoalManager;
		[Inject] private RoleManager _roleManager;
		[Inject] private TechTreeManager _techTreeManager;
		[Inject] private GUIDManager _guidManager;
		[Inject] private ObjectPoolingManager _poolingManager;
		[Inject] private SaveManager _saveManager;
		[Inject] private TimeManager _timeManager;
		[Inject] private StationManager _stationManager;
		[Inject] private TownResourceManager _townResourceManager;

		public UIManager UIManager { get; set; }

		private Player _userPlayer;
		public Player UserPlayer => _userPlayer;
		public TwitchUser Broadcaster
		{ get; set; }

		private List<PathProbe> _pathProbes = new List<PathProbe>();
		public CameraController CameraController { get; set; }

		// Debug options
		[SerializeField]
		private bool _debugBuildingControls = true;
		public BuildingType LastBuildingType { get; set; } = BuildingType.Barracks;

		private float _buttonDelay = 0.02f;

		public TMPro.TMP_Text CodeDisplay;

		private string _code;
		public string Code => _code;
		// Debug Properties
		[field: SerializeField, Header("DEBUG OPTIONS")]
		public bool BuildingsCostResources { get; set; }
		[field: SerializeField]
		public bool PlayerRoleLimits { get; set; }
		[field: SerializeField]
		public bool IgnoreTechUnlocks { get; set; }
		public bool DebugBuildingControls => _debugBuildingControls;

		public Vector3 PlayerSpawnPosition => _playerSpawnPosition.position;
		public ProceduralWorldGenerator ProceduralWorldGenerator => _proceduralWorldGen;
		public EnemySpawner EnemySpawner => _enemySpawner;
		public MetaData.MetaData MetaDatas => _metaData;
		public List<PathProbe> PathProbes => _pathProbes;

		public GameObject ConnectPanel
		{
			set { _connectPanel = value; }
			get { return _connectPanel; }
		}

		public void AddPathProbe(PathProbe probe) => _pathProbes.Add(probe);

		public void SetUserPlayer(Player player)
		{
			_userPlayer = player;
		}

		private void Awake()
		{
			Debug.Log("[STARTUP] GameManager.Awake started");
			InitializeNonInjectedComponents();
			
			// Initialize static helpers early
			var _ = Utils.TargetFlagHelper.TargetFlags;
			
			// Initialize critical managers synchronously before other components' Start() methods
			Debug.Log("[STARTUP] Initializing WorldUtils");
			WorldUtils.Initialize(_timeManager);
			Debug.Log("[STARTUP] Initializing BuildingManager");
			_buildingManager.Initialize();
			Debug.Log("[STARTUP] Initializing PlayerManager");
			_playerManager.Initialize();
			Debug.Log("[STARTUP] Initializing TownGoalManager");
			_townGoalManager.Initialize();
			Debug.Log("[STARTUP] Initializing RoleManager");
			_roleManager.Initialize();
			Debug.Log("[STARTUP] Initializing TownResourceManager");
			_townResourceManager.Initialize();
			
			Debug.Log("[STARTUP] GameManager.Awake completed, starting StartupSequence");
			StartGameManager();
		}

		private void ProcessManagers()
		{
			//UpdateManager.Update();
			TileHelper.ProcessQueue();
			_gameEventManager.ProcessEvents();
			_audioSourcesManager.ProcessSources();
		}

		private IEnumerator StartupSequence()
		{
			Debug.Log("[STARTUP] StartupSequence started");
			System.Diagnostics.Stopwatch totalStopwatch = System.Diagnostics.Stopwatch.StartNew();
			System.Diagnostics.Stopwatch stopwatch = System.Diagnostics.Stopwatch.StartNew();

			Debug.Log("[STARTUP] Initializing Twitch managers");
			TwitchChatManager.Initialize(_playerManager, _timeManager);
			Twitch.Commands.ModeratorCommands.Initialize(_playerManager, _gameEventManager);
			Twitch.Commands.PlayerCommands.Initialize(_playerManager, _gameEventManager);
			Twitch.Utils.TwitchUtils.Initialize(_playerManager);
			Twitch.Commands.RoleCommands.Initialize(_playerManager, _stationManager, _roleManager);
			Twitch.Commands.RulerCommands.Initialize(_playerManager, _roleManager, _townResourceManager, CameraController, _gameEventManager);
			Twitch.Commands.BuildingCommands.Initialize(_buildingManager);
			Twitch.Commands.MiscCommands.Initialize(_buildingManager);
			Twitch.Commands.EventCommands.Initialize(_playerManager, _gameEventManager);
			Twitch.Commands.BroadcasterCommands.Initialize(this);
			Twitch.Commands.GameMasterCommands.Initialize(this);
			Debug.Log("[STARTUP] Twitch managers initialized");

			_code = Random.Range(100000, 999999).ToString();
			_connectPanel.SetActive(true);
			CodeDisplay.text = $"!CONNECT {_code}";

			Debug.Log("[STARTUP] Starting TechTree and GUID initialization");
			stopwatch.Restart();
			_techTreeManager.ManualInject(_playerManager, _buildingManager, _townResourceManager, _gameEventManager, _townGoalManager, UIManager, _metaData);
			_techTreeManager.InitializeTree();
			_guidManager.Initialize();   // Must happen before pooling manager
			stopwatch.Stop();
			Debug.Log($"[LOAD TIME] TechTree and GUID initialization: {stopwatch.ElapsedMilliseconds}ms");

			Debug.Log("[STARTUP] Starting pooling initialization");
			yield return StartCoroutine(_poolingManager.InitializePooling());
			stopwatch.Stop();
			Debug.Log($"[LOAD TIME] Pooling initialization: {stopwatch.ElapsedMilliseconds}ms");

			Debug.Log("[STARTUP] Injecting all pooled objects with SceneScope dependencies");
			_poolingManager.InjectAllPooledObjects(_sceneContainer);
			stopwatch.Stop();
			Debug.Log($"[LOAD TIME] Pooled object injection: {stopwatch.ElapsedMilliseconds}ms");

			if (_metaData != null)
			{
				if (_metaData.LoadType == MetaData.LoadType.Generate)
				{
					Debug.Log("Generating new world!");
					stopwatch.Restart();
					yield return StartCoroutine(_proceduralWorldGen.TryGenerateWorld());
					stopwatch.Stop();
					Debug.Log($"[LOAD TIME] World generation: {stopwatch.ElapsedMilliseconds}ms");
				}

				else if (_metaData.LoadType == MetaData.LoadType.Load)
				{
					Debug.Log("Loading World!");
					stopwatch.Restart();
					_saveManager.LoadGame();
					stopwatch.Stop();
					Debug.Log($"[LOAD TIME] Game loading: {stopwatch.ElapsedMilliseconds}ms");
				}
			}
			else
			{
				stopwatch.Restart();
				yield return StartCoroutine(_proceduralWorldGen.TryGenerateWorld());
				stopwatch.Stop();
				Debug.Log($"[LOAD TIME] World generation (fallback): {stopwatch.ElapsedMilliseconds}ms");
			}

			stopwatch.Restart();
			AstarPath.active.Scan();
			stopwatch.Stop();
			Debug.Log($"[LOAD TIME] A* pathfinding scan: {stopwatch.ElapsedMilliseconds}ms");

			totalStopwatch.Stop();
			Debug.Log($"[LOAD TIME] TOTAL StartupSequence: {totalStopwatch.ElapsedMilliseconds}ms");

			GameStateManager.NotifyPlayerReady();
		}

		private void InitializeNonInjectedComponents()
		{
			_proceduralWorldGen = GetComponentInChildren<ProceduralWorldGenerator>();
			if (_proceduralWorldGen == null)
				Debug.LogError("ProceduralWorldGenerator not found in child object");
		}

		private void UpdateDebugBuildingControls()
		{
			if (!_debugBuildingControls)
				return;

			_buttonDelay -= Time.deltaTime;
			if (Keyboard.current.escapeKey.wasReleasedThisFrame || Mouse.current.rightButton.wasReleasedThisFrame)
				_buildingManager.TryCancelBuilding(_userPlayer);


			if (Keyboard.current.eKey.wasReleasedThisFrame)
			{
				_buildingManager.TryRotateBuilding(_userPlayer, 1);
			}

			if (Keyboard.current.qKey.wasReleasedThisFrame)
			{
				_buildingManager.TryRotateBuilding(_userPlayer, -1);
			}

			if (Mouse.current.leftButton.wasReleasedThisFrame && !WorldUtils.IsPointerOverUI(EventSystem.current))
			{
				if (_userPlayer != null)
					if (_buildingManager.GetPlacerBuildingType(_userPlayer, out BuildingType type))
					{
						_buildingManager.TryPlaceBuilding(_userPlayer, out string message);
						_buildingManager.TryStartNewBuildingPlacer(_userPlayer, type, out message);
						Debug.Log(message);
					}
			}
		}

		private void IncrementBuildingType(ref BuildingType type)
		{
			type++;

			if (type >= BuildingType.Count)
				type = 0;
		}

		private void StartGameManager()
		{
			//_userPlayer = new Player(new Twitch.TwitchUser("69", "PLAYER"));
			StartCoroutine(StartupSequence());
		}

		private void Update()
		{
			ProcessManagers();
			UpdateDebugBuildingControls();
		}

		private void OnDisable()
		{
			TL_Client.ForceDisconnect();
		}
	}
}