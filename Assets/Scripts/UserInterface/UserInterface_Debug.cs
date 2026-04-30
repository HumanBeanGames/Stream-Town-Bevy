using Buildings;
using Character;
using UnityEngine;
using TMPro;
using Utils;
using Level;
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
		[Inject] private Processors.TwitchChatProcessor _twitchChatProcessor;
		[Inject] private Processors.BuildingProcessor _buildingProcessor;
		[Inject] private Processors.WorldGenProcessor _worldGenProcessor;

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
			PoolableObject obj = _poolingProcessor.GetPooledObject("Player");
			obj.gameObject.SetActive(true);
			
			// Find townhall and spawn randomly within 5 units
			Vector3 spawnPosition = GetRandomSpawnPositionNearTownhall();
			obj.transform.position = spawnPosition;
			
			obj.GetComponent<RoleHandler>().SetStarterRole((PlayerRole)_roleDropdownDebug.value);
		}

		/// <summary>
		/// Gets a random spawn position within 5 units of the townhall, avoiding overlap with existing players.
		/// </summary>
		private Vector3 GetRandomSpawnPositionNearTownhall()
		{
			// Find townhall specifically by BuildingType
			Buildings.BuildingBase[] allBuildings = GameObject.FindObjectsOfType<Buildings.BuildingBase>();
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
		private void InitializeInterface()
		{
			HideBuildingContext();
			HideCharacterContext();

			_roleDropdownDebug.ClearOptions();
			_roleDropdownCharacter.ClearOptions();

			List<string> options = new List<string>();

			for (int i = 0; i < (int)PlayerRole.Count; i++)
			{
				options.Add(((PlayerRole)i).ToString());
			}

			_roleDropdownDebug.AddOptions(options);
			_roleDropdownCharacter.AddOptions(options);

			// Initialize command input field
			if (_commandInputField != null)
			{
				_commandInputField.onEndEdit.AddListener(OnCommandSubmitted);
			}

			// Initialize player dropdown
			PopulatePlayerDropdown();

			// Unlock all buildings for debug testing
			if (_buildingProcessor != null)
			{
				_buildingProcessor.UnlockAllBuildingsForDebug();
				Debug.Log("[Debug] All buildings unlocked for testing");
			}

			// Spawn debug player if it doesn't exist (waits for WorldGen completion)
			StartCoroutine(EnsureDebugPlayerExistsCoroutine());
		}

		/// <summary>
		/// Coroutine that waits for WorldGen to complete before spawning the debug player.
		/// </summary>
		private System.Collections.IEnumerator EnsureDebugPlayerExistsCoroutine()
		{
			// Wait until WorldGen is complete
			if (_worldGenProcessor != null)
			{
				yield return new WaitUntil(() => _worldGenProcessor.IsWorldGenerated);
			}

			Debug.Log("[Debug] WorldGen complete, waiting for navigation graphs");

			// Wait for navigation graphs to be scanned and ready
			yield return new WaitUntil(() => AstarPath.active != null
				&& !AstarPath.active.isScanning
				&& AstarPath.active.data.graphs != null
				&& AstarPath.active.data.graphs.Length > 0);

			Debug.Log("[Debug] Navigation graphs ready, spawning debug player");

			// Check if debug player already exists in player list
			foreach (var player in _playerProcessor.Players)
			{
				if (player != null && player.TwitchUser != null && player.TwitchUser.Username == "Debugger")
				{
					Debug.Log("[Debug] Debug player already exists");
					_debugPlayer = player;
					yield break;
				}
			}

			Debug.Log("[Debug] Debug player not found, spawning new one");

			// Create the Player data object
			TwitchUser debugUser = new TwitchUser("debug_id", "Debugger");
			_debugPlayer = new Player(debugUser, true);
			Debug.Log("[Debug] Created Player data object");

			// Spawn a new debug player character
			PoolableObject obj = _poolingProcessor.GetPooledObject("Player");
			if (obj != null)
			{
				obj.gameObject.SetActive(true);

				// Find townhall and spawn randomly within 5 units
				Vector3 spawnPosition = GetRandomSpawnPositionNearTownhall();
				obj.transform.position = spawnPosition;

				Debug.Log("[Debug] Spawned player object at " + spawnPosition);

				// Set up as debug player
				RoleHandler roleHandler = obj.GetComponent<RoleHandler>();
				roleHandler.SetStarterRole(PlayerRole.Builder);

				// Link the Player data to the character
				_debugPlayer.Character = obj.gameObject;
				_debugPlayer.RoleHandler = roleHandler;
				_debugPlayer.StationSensor = obj.GetComponentInChildren<StationSensor>();

				Debug.Log("[Debug] Linked Player data to character");

				// Add the player to the PlayerProcessor
				Player addedPlayer = _playerProcessor.AddExistingPlayer(_debugPlayer, PlayerRole.Builder);
				if (addedPlayer != null)
				{
					Debug.Log("[Debug] Added debug player to PlayerProcessor");
					_debugPlayer = addedPlayer;
				}
				else
				{
					Debug.LogError("[Debug] Failed to add debug player to PlayerProcessor");
				}

				Debug.Log("[Debug] Player count after spawn: " + _playerProcessor.Players.Count);

				// Refresh dropdown to show the new debug player
				PopulatePlayerDropdown();
			}
			else
			{
				Debug.LogError("[Debug] Failed to get Player from object pool");
			}
		}

		/// <summary>
		/// Populates the player dropdown with active players.
		/// </summary>
		private void PopulatePlayerDropdown()
		{
			if (_playerDropdown == null || _playerProcessor == null)
				return;

			_playerDropdown.ClearOptions();

			List<string> playerNames = new List<string>();

			foreach (var player in _playerProcessor.Players)
			{
				if (player != null && player.TwitchUser != null)
				{
					playerNames.Add(player.TwitchUser.Username);
				}
			}

			if (playerNames.Count > 0)
			{
				_playerDropdown.AddOptions(playerNames);
			}
			else
			{
				_playerDropdown.AddOptions(new List<string> { "No Players" });
			}
		}

		/// <summary>
		/// Called when the user submits a command in the debug input field.
		/// </summary>
		/// <param name="commandText">The command text entered.</param>
		private void OnCommandSubmitted(string commandText)
		{
			if (string.IsNullOrWhiteSpace(commandText))
				return;

			// Get selected player
			Player selectedPlayer = null;
			if (_playerDropdown != null && _playerProcessor != null)
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

		private void Awake()
		{
			InitializeInterface();
		}
	}
}
