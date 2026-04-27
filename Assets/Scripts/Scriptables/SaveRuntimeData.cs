using Units;

using ScriptablesProcessorInfrastructure;
using System.Collections.Generic;
using Character;
using World;
using World.Generation;

namespace Processors
{
	/// <summary>
	/// Runtime data for SaveProcessor.
	/// Manages player/enemy lists, autosave configuration, loading progress.
	/// </summary>
	public class SaveRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// List of players to be saved/loaded.
		/// Contains all player data for save operations.
		/// </summary>
		private List<Player> _players;

		/// <summary>
		/// List of enemies to be saved/loaded.
		/// Contains all enemy data for save operations.
		/// </summary>
		private List<Enemies.Enemy> _enemies;

		/// <summary>
		/// Whether autosave is currently enabled.
		/// If true, the game automatically saves at intervals.
		/// </summary>
		private bool _autosave;

		/// <summary>
		/// Time interval between autosaves in seconds.
		/// Determines how frequently autosaves occur.
		/// </summary>
		private float _autosaveTime;

		/// <summary>
		/// Time elapsed since the last autosave.
		/// Used to trigger autosave at the configured interval.
		/// </summary>
		private float _timeElapsed;

		/// <summary>
		/// Current loading progress percentage.
		/// Used to display loading progress to the player.
		/// </summary>
		private int _loadPercent;

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
		public SaveRuntimeData()
		{
			_players = new List<Player>();
			_enemies = new List<Enemies.Enemy>();
			_autosave = false;
			_autosaveTime = 0.0f;
			_timeElapsed = 0.0f;
			_loadPercent = 0;
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
