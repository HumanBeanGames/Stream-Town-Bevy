using Buildings;
using Character;
using UnityEngine;
using TMPro;
using Utils;
using Level;
using System.Collections;
using System.Collections.Generic;
using Processors;
using Utils.Pooling;
using Reflex.Attributes;
using Twitch;
using Sensors;

namespace UserInterface
{
	/// <summary>
	/// Handles the User Interace for the Debug Menus
	/// </summary>
	public class UserInterface_Debug : MonoBehaviour
	{
		[Header("Contexts")]
		[SerializeField]
		private GameObject _characterContext;
		[SerializeField]
		private GameObject _debugContext;
		[SerializeField]
		private GameObject _buildingContext;

		[Header("Debug Context Data")]
		[SerializeField]
		private TMP_Dropdown _roleDropdownDebug;
		[SerializeField]
		private TMP_InputField _commandInputField;
		[SerializeField]
		private TMP_Dropdown _playerDropdown;

		[Header("Character Context Data")]
		[SerializeField]
		private TextMeshProUGUI _characterRole;
		[SerializeField]
		private TextMeshProUGUI _roleLevel;
		[SerializeField]
		private TMP_Dropdown _roleDropdownCharacter;

		[Header("Building Context Data")]
		[SerializeField]
		private TextMeshProUGUI _buildingName;
		[SerializeField]
		private TextMeshProUGUI _buildingLevel;

		[SerializeField]
		private GameObject _selectionOutline;

		private object _data;
		[Inject] private ObjectPoolingProcessor _poolingProcessor;
		[Inject] private TownResourceProcessor _townResourceProcessor;
		[Inject] private PlayerProcessor _playerProcessor;
		[Inject] private PlayerInputProcessor _playerInputProcessor;
		[Inject] private Processors.TwitchChatProcessor _twitchChatProcessor;
		[Inject] private Processors.BuildingProcessor _buildingProcessor;
		[Inject] private Processors.WorldGenProcessor _worldGenProcessor;
		[Inject] private Processors.GameStateProcessor _gameStateProcessor;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		private Player _debugPlayer;

		/// <summary>
		/// Enables the Character Debug Menu.
		/// </summary>
		public void ShowCharacterContext()
		{
			_characterContext.SetActive(true);
		}

		/// <summary>
		/// Disables the Character Debug Menu.
		/// </summary>
		public void HideCharacterContext()
		{
			_selectionOutline.SetActive(false);
			_characterContext.SetActive(false);
		}

		/// <summary>
		/// Enables the Main Debug Context.
		/// </summary>
		public void ShowDebugContext()
		{
			_debugContext.SetActive(true);
		}

		/// <summary>
		/// Disables the Main Debug Context.
		/// </summary>
		public void HideDebugContext()
		{
			_debugContext.SetActive(false);
		}

		/// <summary>
		/// Enables the Building Debug Menu.
		/// </summary>
		public void ShowBuildingContext()
		{
			_buildingContext.SetActive(true);
		}

		/// <summary>
		/// Disables the Building Debug Menu.
		/// </summary>
		public void HideBuildingContext()
		{
			_selectionOutline.SetActive(false);
			_buildingContext.SetActive(false);
		}

		/// <summary>
		/// Removes a selected building from the game.
		/// </summary>
		public void RemoveBuilding()
		{
			if (((BuildingBase)_data).gameObject.activeInHierarchy)
				((BuildingBase)_data).gameObject.SetActive(false);
			else
				HideBuildingContext();
		}

		/// <summary>
		/// Levels up the selected building by 1.
		/// </summary>
		public void LevelBuilding()
		{
			if (((BuildingBase)_data).gameObject.activeInHierarchy)
			{
				((BuildingBase)_data).GetComponent<LevelHandler>().TryLevel();
				_buildingLevel.text = "Level: " + ((BuildingBase)_data).GetComponent<LevelHandler>().Level.ToString();
			}
			else
				HideBuildingContext();
		}

		/// <summary>
		/// Levels up the selected character's currently active role by 1.
		/// </summary>
		public void LevelCharacter()
		{
			if (((RoleHandler)_data).gameObject.activeInHierarchy)
			{
				((RoleHandler)_data).PlayerRoleData.LevelUp();
				_roleLevel.text = "Level: " + ((RoleHandler)_data).PlayerRoleData.CurrentLevel;
			}
			else
				HideCharacterContext();
		}

		/// <summary>
		/// Attempts to spawn a new character as the selected role. If the role is unavailable, it will spawn a builder.
		/// </summary>
		public void SpawnCharacter()
		{
			PlayerRole role = (PlayerRole)_roleDropdownDebug.value;
			Player recruit = new Player(new TwitchUser(UnityEngine.Random.Range(int.MinValue, 0).ToString(), ""), true);
			_playerProcessor.AddNewPlayer(recruit, role);
		}

		/// <summary>
		/// Gets a random spawn position within 5 units of the townhall, avoiding overlap with existing players.
		/// </summary>
		private Vector3 GetRandomSpawnPositionNearTownhall()
		{
			// Find townhall specifically by BuildingType
			Buildings.BuildingBase[] allBuildings = GameObject.FindObjectsByType<Buildings.BuildingBase>();
			Buildings.BuildingBase townhall = null;
			
			foreach (var building in allBuildings)
			{
				if (building.BuildingType == Utils.BuildingType.Townhall)
				{
					townhall = building;
					break;
				}
			}

			if (townhall == null)
			{
				// Fallback to PlayerSpawnPosition if no townhall found
				return _playerProcessor.PlayerSpawnPosition != null 
					? _playerProcessor.PlayerSpawnPosition.position 
					: Vector3.zero;
			}

			Vector3 center = townhall.transform.position;
			float spawnRadius = 5f;
			float overlapRadius = 1f; // Minimum distance between players
			int maxAttempts = 10;

			for (int attempt = 0; attempt < maxAttempts; attempt++)
			{
				// Generate random position within spawnRadius
				Vector2 randomOffset = UnityEngine.Random.insideUnitCircle * spawnRadius;
				Vector3 candidatePosition = center + new Vector3(randomOffset.x, 0, randomOffset.y);

				// Check for overlap with existing players
				bool hasOverlap = false;
				foreach (var player in _playerProcessor.Players)
				{
					if (player.Character != null && player.Character.activeInHierarchy)
					{
						float distance = Vector3.Distance(candidatePosition, player.Character.transform.position);
						if (distance < overlapRadius)
						{
							hasOverlap = true;
							break;
						}
					}
				}

				if (!hasOverlap)
				{
					return candidatePosition;
				}
			}

			// If we couldn't find a non-overlapping position, return the center
			return center;
		}

		/// <summary>
		/// Attempts to switch the selected character's active role.
		/// </summary>
		public void SetCharacterRole()
		{
			((RoleHandler)_data).TrySetRole((PlayerRole)_roleDropdownCharacter.value);
			_characterRole.text = "Role: " + ((RoleHandler)_data).CurrentRole.ToString();
			_roleLevel.text = "Level: " + ((RoleHandler)_data).PlayerRoleData.CurrentLevel;
		}

		/// <summary>
		/// Called when the Character Debug Menu is enabled and updates the displayed data.
		/// </summary>
		/// <param name="character"></param>
		public void OnCharacterContext(RoleHandler character)
		{
			_data = character;
			HideBuildingContext();
			_characterRole.text = "Role: " + character.CurrentRole.ToString();
			_roleLevel.text = "Level: 1";
			_roleDropdownCharacter.value = (int)character.CurrentRole;
			ShowCharacterContext();
			_selectionOutline.transform.position = new Vector3(character.transform.position.x, 0.15f, character.transform.position.z);
			_selectionOutline.transform.rotation = character.transform.rotation;
			_selectionOutline.transform.parent = character.transform;
			_selectionOutline.transform.localScale = Vector3.one * 1.25f;
			_selectionOutline.SetActive(true);
		}


		/// <summary>
		/// Called when the Building Debug Menu is enabled, and updates the displayed data.
		/// </summary>
		/// <param name="building"></param>
		public void OnBuildingContext(BuildingBase building)
		{
			_data = building;
			HideCharacterContext();
			_buildingName.text = building.BuildingData.BuildingName;
			_buildingLevel.text = "Level: " + building.GetComponent<LevelHandler>().Level.ToString();
			ShowBuildingContext();
			_selectionOutline.transform.position = new Vector3(building.transform.position.x, 0.15f, building.transform.position.z);
			_selectionOutline.transform.rotation = building.transform.rotation;
			_selectionOutline.transform.parent = building.transform;
			BoxCollider bc = building.GetComponent<BoxCollider>();
			_selectionOutline.transform.localScale = new Vector3(bc.size.x * 1.25f, 1, bc.size.z * 1.25f);
			_selectionOutline.SetActive(true);
		}

		/// <summary>
		/// Changes the time scale of the game.
		/// </summary>
		/// <param name="scale"></param>
		public void SetTimeScale(float scale)
		{
			Time.timeScale = scale;
		}

		/// <summary>
		/// Adds the specified amount of wood to the town's resources.
		/// </summary>
		/// <param name="value"></param>
		public void AddWood(int value)
		{
			_townResourceProcessor.AddResource(Utils.Resource.Wood, value);
		}

		/// <summary>
		/// Adds the specified amount of ore to the town's resources.
		/// </summary>
		/// <param name="value"></param>
		public void AddOre(int value)
		{
			_townResourceProcessor.AddResource(Utils.Resource.Ore, value);
		}

		/// <summary>
		/// Adds the specified amount of food to the town's resources.
		/// </summary>
		/// <param name="value"></param>
		public void AddFood(int value)
		{
			_townResourceProcessor.AddResource(Utils.Resource.Food, value);
		}

		/// <summary>
		/// Adds the specified amount of gold to the town's resources.
		/// </summary>
		/// <param name="value"></param>
		public void AddGold(int value)
		{
			_townResourceProcessor.AddResource(Utils.Resource.Gold, value);
		}

		/// <summary>
		/// Initializes the debug user interface.
		/// </summary>
		private IEnumerator InitializeInterface()
		{
			HideBuildingContext();
			HideCharacterContext();

			if (_roleDropdownDebug == null)
			{
				_debugProcessor.LogError(DebugLogCategory.DebugUI, "_roleDropdownDebug is null");
			}
			else
			{
				_roleDropdownDebug.ClearOptions();
			}

			if (_roleDropdownCharacter == null)
			{
				_debugProcessor.LogError(DebugLogCategory.DebugUI, "_roleDropdownCharacter is null");
			}
			else
			{
				_roleDropdownCharacter.ClearOptions();
			}

			List<string> options = new List<string>();

			for (int i = 0; i < (int)PlayerRole.Count; i++)
			{
				options.Add(((PlayerRole)i).ToString());
			}

			_debugProcessor.Log(DebugLogCategory.DebugUI, $"Populating role dropdowns with {options.Count} options");

			List<TMP_Dropdown.OptionData> roleOptions = new List<TMP_Dropdown.OptionData>();
			foreach (string option in options)
			{
				roleOptions.Add(new TMP_Dropdown.OptionData(option));
			}

			if (_roleDropdownDebug != null)
			{
				_roleDropdownDebug.options = roleOptions;
				_roleDropdownDebug.value = 0;
				if (_roleDropdownDebug.captionText != null)
					_roleDropdownDebug.captionText.text = roleOptions.Count > 0 ? roleOptions[0].text : "";
				else
					_debugProcessor.LogError(DebugLogCategory.DebugUI, "_roleDropdownDebug.captionText is null");
			}

			if (_roleDropdownCharacter != null)
			{
				_roleDropdownCharacter.options = roleOptions;
				_roleDropdownCharacter.value = 0;
				if (_roleDropdownCharacter.captionText != null)
					_roleDropdownCharacter.captionText.text = roleOptions.Count > 0 ? roleOptions[0].text : "";
				else
					_debugProcessor.LogError(DebugLogCategory.DebugUI, "_roleDropdownCharacter.captionText is null");
			}

			// Initialize command input field
			if (_commandInputField != null)
			{
				_commandInputField.onEndEdit.AddListener(OnCommandSubmitted);
				_commandInputField.onSelect.AddListener(OnCommandInputSelected);
				_commandInputField.onDeselect.AddListener(OnCommandInputDeselected);
			}

			// Initialize player dropdown
			PopulatePlayerDropdown();

			// Unlock all buildings for debug purposes
			// _buildingProcessor.UnlockAllBuildings();

			// Debug player spawning will be handled later when systems are ready
			// Don't spawn during Awake as pooling may not be initialized yet
			_debugProcessor.Log(DebugLogCategory.DebugUI, "Debug UI initialized. Debug player will spawn when systems are ready.");

			yield break;
		}

		/// <summary>
		/// Spawns the debug player when systems are ready.
		/// Should be called after object pooling is initialized.
		/// </summary>
		public void SpawnDebugPlayerWhenReady()
		{
			StartCoroutine(SpawnDebugPlayerCoroutine());
		}

		private IEnumerator SpawnDebugPlayerCoroutine()
		{
			_debugProcessor.Log(DebugLogCategory.DebugUI, "Navigation graphs ready, spawning debug player");

			// Check if debug player already exists in player list
			foreach (var player in _playerProcessor.Players)
			{
				if (player != null && player.TwitchUser != null && player.TwitchUser.Username == "Debugger")
				{
						_debugProcessor.Log(DebugLogCategory.DebugUI, "Debug player already exists");
					_debugPlayer = player;
					_playerProcessor.SetUserPlayer(_debugPlayer);
					PopulatePlayerDropdown();
					yield break;
				}
			}

			_debugProcessor.Log(DebugLogCategory.DebugUI, "Debug player not found, spawning new one");

			// Create the Player data object
			TwitchUser debugUser = new TwitchUser("debug_id", "Debugger");
			_debugPlayer = new Player(debugUser, true);
			_debugProcessor.Log(DebugLogCategory.DebugUI, "Created Player data object");

			// Spawn a new debug player character
			PoolableObject obj = _poolingProcessor.GetPooledObject("Player");
			if (obj != null)
			{
				obj.gameObject.SetActive(true);

				// Find townhall and spawn randomly within 5 units
				Vector3 spawnPosition = GetRandomSpawnPositionNearTownhall();
				obj.transform.position = spawnPosition;

					_debugProcessor.Log(DebugLogCategory.DebugUI, "Spawned player object at " + spawnPosition);

				// Set up as debug player
				RoleHandler roleHandler = obj.GetComponent<RoleHandler>();
				// Link the Player data to the character
				_debugPlayer.Character = obj.gameObject;
				_debugPlayer.RoleHandler = roleHandler;
				_debugPlayer.StationSensor = obj.GetComponentInChildren<StationSensor>();

					_debugProcessor.Log(DebugLogCategory.DebugUI, "Linked Player data to character");

				// Add the player to the PlayerProcessor
				Player addedPlayer = _playerProcessor.AddExistingPlayer(_debugPlayer, PlayerRole.Builder);
				if (addedPlayer != null)
				{
						_debugProcessor.Log(DebugLogCategory.DebugUI, "Added debug player to PlayerProcessor");
					_debugPlayer = addedPlayer;
					_playerProcessor.SetUserPlayer(_debugPlayer);
				}
				else
				{
					_debugProcessor.LogError(DebugLogCategory.DebugUI, "Failed to add debug player to PlayerProcessor");
				}

					_debugProcessor.Log(DebugLogCategory.DebugUI, "Player count after spawn: " + _playerProcessor.Players.Count);

				// Refresh dropdown to show the new debug player
				PopulatePlayerDropdown();
			}
			else
			{
				_debugProcessor.LogError(DebugLogCategory.DebugUI, "Failed to get Player from object pool");
			}
		}
		private void PopulatePlayerDropdown()
		{
			if (_playerDropdown == null)
				return;

			_playerDropdown.ClearOptions();

			List<TMP_Dropdown.OptionData> playerOptions = new List<TMP_Dropdown.OptionData>();

			_debugProcessor.Log(DebugLogCategory.DebugUI, $"Populating player dropdown. Total players: {_playerProcessor.Players.Count}");

			foreach (var player in _playerProcessor.Players)
			{
				if (player != null && player.TwitchUser != null)
				{
					playerOptions.Add(new TMP_Dropdown.OptionData(player.TwitchUser.Username));
						_debugProcessor.Log(DebugLogCategory.DebugUI, $"Added player to dropdown: {player.TwitchUser.Username}");
				}
			}

			if (playerOptions.Count > 0)
			{
				_playerDropdown.options = playerOptions;
				_playerDropdown.value = 0;
				if (_playerDropdown.captionText != null)
					_playerDropdown.captionText.text = playerOptions[0].text;
					_debugProcessor.Log(DebugLogCategory.DebugUI, $"Player dropdown populated with {playerOptions.Count} options");
			}
			else
			{
				_playerDropdown.options = new List<TMP_Dropdown.OptionData> { new TMP_Dropdown.OptionData("No Players") };
				_playerDropdown.value = 0;
				_playerDropdown.captionText.text = "No Players";
					_debugProcessor.LogWarning(DebugLogCategory.DebugUI, "No players found for dropdown");
			}
		}

		/// <summary>
		/// Called when the user submits a command in the debug input field.
		/// </summary>
		/// <param name="commandText">The command text entered.</param>
		private void OnCommandSubmitted(string commandText)
		{
			OnCommandInputDeselected(commandText);

			if (string.IsNullOrWhiteSpace(commandText))
				return;

			// Get selected player
			Player selectedPlayer = null;
			if (_playerDropdown != null)
			{
				int selectedIndex = _playerDropdown.value;
				if (selectedIndex >= 0 && selectedIndex < _playerDropdown.options.Count)
				{
					string selectedName = _playerDropdown.options[selectedIndex].text;

					// Find the player by name
					foreach (var player in _playerProcessor.Players)
					{
						if (player != null && player.TwitchUser != null && player.TwitchUser.Username == selectedName)
						{
							selectedPlayer = player;
							break;
						}
					}
				}
			}

			// Process the command
			_twitchChatProcessor.ProcessDebugCommand(commandText, selectedPlayer);

			// Clear the input field
			_commandInputField.text = string.Empty;
		}

		private void OnCommandInputSelected(string _)
		{
			_playerInputProcessor.SuppressGameplayInput = true;
		}

		private void OnCommandInputDeselected(string _)
		{
			_playerInputProcessor.SuppressGameplayInput = false;
		}

		private bool _debugPlayerSpawnAttempted = false;

		private void Awake()
		{
			StartCoroutine(InitializeInterface());
		}

		private void OnEnable()
		{
			_gameStateProcessor.GeneratedWorld += OnWorldGenerated;
		}

		private void OnDisable()
		{
			_gameStateProcessor.GeneratedWorld -= OnWorldGenerated;
		}

		private void OnWorldGenerated()
		{
			if (!_debugPlayerSpawnAttempted)
			{
				_debugPlayerSpawnAttempted = true;
				SpawnDebugPlayerWhenReady();
			}
		}
	}
}
