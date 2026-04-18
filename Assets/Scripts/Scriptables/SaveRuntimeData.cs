using UnityEngine;
using Units;
using System.Collections.Generic;
using Character;
using World;
using World.Generation;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for SaveProcessor.
	/// Manages player/enemy lists, autosave configuration, loading progress.
	/// </summary>
	public class SaveRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// List of players to be saved/loaded.
		/// Contains all player data for save operations.
		/// </summary>
		[SerializeField]
		private List<Player> _players = null;

		/// <summary>
		/// List of enemies to be saved/loaded.
		/// Contains all enemy data for save operations.
		/// </summary>
		[SerializeField]
		private List<Enemies.Enemy> _enemies = null;

		/// <summary>
		/// Whether autosave is currently enabled.
		/// If true, the game automatically saves at intervals.
		/// </summary>
		[SerializeField]
		private bool _autosave = false;

		/// <summary>
		/// Time interval between autosaves in seconds.
		/// Determines how frequently autosaves occur.
		/// </summary>
		[SerializeField]
		private float _autosaveTime = 0.0f;

		/// <summary>
		/// Time elapsed since the last autosave.
		/// Used to trigger autosave at the configured interval.
		/// </summary>
		[SerializeField]
		private float _timeElapsed = 0.0f;

		/// <summary>
		/// Current loading progress percentage.
		/// Used to display loading progress to the player.
		/// </summary>
		[SerializeField]
		private int _loadPercent = 0;

		/// <summary>
		/// Gets the list of players for save/load.
		/// </summary>
		public List<Player> Players
		{
			get => _players;
			set => _players = value;
		}

		/// <summary>
		/// Gets the list of enemies for save/load.
		/// </summary>
		public List<Enemies.Enemy> Enemies
		{
			get => _enemies;
			set => _enemies = value;
		}

		/// <summary>
		/// Gets or sets whether autosave is enabled.
		/// </summary>
		public bool Autosave
		{
			get => _autosave;
			set => _autosave = value;
		}

		/// <summary>
		/// Gets or sets the autosave interval in seconds.
		/// </summary>
		public float AutosaveTime
		{
			get => _autosaveTime;
			set => _autosaveTime = value;
		}

		/// <summary>
		/// Gets or sets the time elapsed since last autosave.
		/// </summary>
		public float TimeElapsed
		{
			get => _timeElapsed;
			set => _timeElapsed = value;
		}

		/// <summary>
		/// Gets the loading progress percentage.
		/// </summary>
		public int LoadPercent => _loadPercent;

		/// <summary>
		/// Initializes the save runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
			_players = new List<Player>();
			_enemies = new List<Enemies.Enemy>();
		}

		public void InitializeEnemies(List<Enemies.Enemy> enemies)
		{
			_enemies = enemies ?? new List<Enemies.Enemy>();
		}

		public void InitializePlayers(List<Player> players)
		{
			_players = players ?? new List<Player>();
		}
	}
}
