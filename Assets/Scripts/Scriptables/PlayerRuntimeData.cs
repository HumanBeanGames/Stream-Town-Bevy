using Character;
using System.Collections.Generic;
using UnityEngine;
using Units;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime player state for the game.
	/// Manages player lists, ruler, stat modifiers, and update queue.
	/// </summary>
	public class PlayerRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// List of all active players in the game.
		/// Includes all player characters currently spawned.
		/// </summary>
		[SerializeField]
		private List<Player> _players = new List<Player>();

		/// <summary>
		/// List of recruits waiting to be assigned roles.
		/// Players in this list have not yet been assigned a role.
		/// </summary>
		[SerializeField]
		private List<Player> _recruits = new List<Player>();

		/// <summary>
		/// The current ruler player.
		/// The ruler has special privileges and responsibilities.
		/// </summary>
		[SerializeField]
		private Player _ruler;

		/// <summary>
		/// The user-controlled player (the actual human player).
		/// This is the player that the user directly controls.
		/// </summary>
		[SerializeField]
		private Player _userPlayer;

		/// <summary>
		/// Dictionary mapping player roles to their stat modifiers.
		/// Each role has specific stat bonuses/penalties.
		/// </summary>
		[SerializeField]
		private Dictionary<PlayerRole, StatModifiers> _roleStatModifiers;

		/// <summary>
		/// Global stat modifiers applied to all players.
		/// Used for game-wide stat adjustments.
		/// </summary>
		[SerializeField]
		private StatModifiers _globalStatModifier;

		/// <summary>
		/// Queue of players pending stat updates.
		/// Used to batch process player stat calculations.
		/// </summary>
		[SerializeField]
		private Queue<Player> _playerUpdateQueue;

		/// <summary>
		/// Transform defining where players spawn in the world.
		/// Set in the editor and used by the player spawning system.
		/// </summary>
		[SerializeField]
		private Transform _playerSpawnPosition;

		/// <summary>
		/// Gets the list of all active players.
		/// </summary>
		public List<Player> Players => _players;

		/// <summary>
		/// Gets the list of unassigned recruits.
		/// </summary>
		public List<Player> Recruits => _recruits;

		/// <summary>
		/// Gets or sets the current ruler player.
		/// </summary>
		public Player Ruler
		{
			get => _ruler;
			set => _ruler = value;
		}

		/// <summary>
		/// Gets or sets the user-controlled player.
		/// </summary>
		public Player UserPlayer
		{
			get => _userPlayer;
			set => _userPlayer = value;
		}

		/// <summary>
		/// Gets the global stat modifiers.
		/// </summary>
		public StatModifiers GlobalStatModifiers => _globalStatModifier;

		/// <summary>
		/// Gets the dictionary of role-specific stat modifiers.
		/// </summary>
		public Dictionary<PlayerRole, StatModifiers> RoleStatModifiers => _roleStatModifiers;

		/// <summary>
		/// Gets the queue of players pending stat updates.
		/// </summary>
		public Queue<Player> PlayerUpdateQueue => _playerUpdateQueue;

		/// <summary>
		/// Gets the number of players in the game.
		/// </summary>
		public int PlayerCount => _players.Count;

		/// <summary>
		/// Gets the player spawn position transform.
		/// </summary>
		public Transform PlayerSpawnPosition => _playerSpawnPosition;

		/// <summary>
		/// Gets the global stat modifier.
		/// </summary>
		public StatModifiers GlobalStatModifier => _globalStatModifier;

		/// <summary>
		/// Initializes the player runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
			_players = new List<Player>();
			_recruits = new List<Player>();
			_playerUpdateQueue = new Queue<Player>();
		}

		public void InitializePlayerState(Dictionary<PlayerRole, StatModifiers> roleStatModifiers, StatModifiers globalStatModifier, Queue<Player> playerUpdateQueue)
		{
			_roleStatModifiers = roleStatModifiers ?? new Dictionary<PlayerRole, StatModifiers>();
			_globalStatModifier = globalStatModifier;
			_playerUpdateQueue = playerUpdateQueue ?? new Queue<Player>();
		}
	}
}
