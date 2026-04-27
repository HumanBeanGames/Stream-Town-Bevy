using Character;

using ScriptablesProcessorInfrastructure;
using System.Collections.Generic;
using Units;
using Utils;
using UnityEngine;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores player state for the game.
	/// Manages player lists, ruler, stat modifiers, and update queue.
	/// </summary>
	public class PlayerRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// List of all active players in the game.
		/// Includes all player characters currently spawned.
		/// </summary>
		private List<Player> _players;

		/// <summary>
		/// List of recruits waiting to be assigned roles.
		/// Players in this list have not yet been assigned a role.
		/// </summary>
		private List<Player> _recruits;

		/// <summary>
		/// The current ruler player.
		/// The ruler has special privileges and responsibilities.
		/// </summary>
		private Player _ruler;

		/// <summary>
		/// The user-controlled player (the actual human player).
		/// This is the player that the user directly controls.
		/// </summary>
		private Player _userPlayer;

		/// <summary>
		/// Dictionary mapping player roles to their stat modifiers.
		/// Each role has specific stat bonuses/penalties.
		/// </summary>
		private Dictionary<PlayerRole, StatModifiers> _roleStatModifiers;

		/// <summary>
		/// Global stat modifiers applied to all players.
		/// Used for game-wide stat adjustments.
		/// </summary>
		private StatModifiers _globalStatModifier;

		/// <summary>
		/// Queue of players pending stat updates.
		/// Used to batch process player stat calculations.
		/// </summary>
		private Queue<Player> _playerUpdateQueue;

		/// <summary>
		/// Transform defining where players spawn in the world.
		/// Set in the editor and used by the player spawning system.
		/// </summary>
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
		public PlayerRuntimeData()
		{
			_players = new List<Player>();
			_recruits = new List<Player>();
			_ruler = null;
			_userPlayer = null;
			_roleStatModifiers = new Dictionary<PlayerRole, StatModifiers>();
			_globalStatModifier = null;
			_playerUpdateQueue = new Queue<Player>();
			_playerSpawnPosition = null;
		}

		public void InitializePlayerState(Dictionary<PlayerRole, StatModifiers> roleStatModifiers, StatModifiers globalStatModifier, Queue<Player> playerUpdateQueue)
		{
			_roleStatModifiers = roleStatModifiers ?? new Dictionary<PlayerRole, StatModifiers>();
			_globalStatModifier = globalStatModifier;
			_playerUpdateQueue = playerUpdateQueue ?? new Queue<Player>();
		}
	}
}
