using Buildings;
using Character;
using System.Collections.Generic;
using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime building state for the game.
	/// Manages building placers, building instances, counts, and unlock status.
	/// </summary>
	public class BuildingRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Dictionary mapping players to their building placer components.
		/// Tracks which player is currently placing buildings.
		/// </summary>
		[SerializeField]
		private Dictionary<Player, BuildingPlacer> _placers = new Dictionary<Player, BuildingPlacer>();

		/// <summary>
		/// Dictionary mapping building types to lists of building instances.
		/// Tracks all buildings currently placed in the world by type.
		/// </summary>
		[SerializeField]
		private Dictionary<BuildingType, List<BuildingBase>> _buildings = new Dictionary<BuildingType, List<BuildingBase>>();

		/// <summary>
		/// Total number of buildings currently placed in the world.
		/// Used for tracking overall building count.
		/// </summary>
		[SerializeField]
		private int _numOfBuildings = 1;

		/// <summary>
		/// Dictionary mapping building types to their current count.
		/// Tracks how many buildings of each type exist.
		/// </summary>
		[SerializeField]
		private Dictionary<BuildingType, int> _buildingCounts = new Dictionary<BuildingType, int>();

		/// <summary>
		/// Dictionary mapping building types to their unlock status.
		/// Tracks which building types are available for construction.
		/// </summary>
		[SerializeField]
		private Dictionary<BuildingType, bool> _buildingsUnlocked = new Dictionary<BuildingType, bool>();

		/// <summary>
		/// The last building type selected by the user player.
		/// Used to remember the user's last building selection.
		/// </summary>
		[SerializeField]
		private BuildingType _lastBuildingType;

		/// <summary>
		/// Gets or sets the dictionary of building placers per player.
		/// </summary>
		public Dictionary<Player, BuildingPlacer> Placers
		{
			get => _placers;
			set => _placers = value;
		}

		/// <summary>
		/// Gets or sets the dictionary of buildings by type.
		/// </summary>
		public Dictionary<BuildingType, List<BuildingBase>> Buildings
		{
			get => _buildings;
			set => _buildings = value;
		}

		/// <summary>
		/// Gets or sets the total number of buildings.
		/// </summary>
		public int NumberOfBuildings
		{
			get => _numOfBuildings;
			set => _numOfBuildings = value;
		}

		/// <summary>
		/// Gets or sets the dictionary of building counts by type.
		/// </summary>
		public Dictionary<BuildingType, int> BuildingCounts
		{
			get => _buildingCounts;
			set => _buildingCounts = value;
		}

		/// <summary>
		/// Gets or sets the dictionary of building unlock status by type.
		/// </summary>
		public Dictionary<BuildingType, bool> BuildingsUnlocked
		{
			get => _buildingsUnlocked;
			set => _buildingsUnlocked = value;
		}

		/// <summary>
		/// Gets or sets the last building type selected by the user player.
		/// </summary>
		public BuildingType LastBuildingType
		{
			get => _lastBuildingType;
			set => _lastBuildingType = value;
		}

		/// <summary>
		/// Initializes the building runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
