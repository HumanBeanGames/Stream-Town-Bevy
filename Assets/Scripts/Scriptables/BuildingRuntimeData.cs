using Buildings;
using ScriptablesProcessorInfrastructure;
using Character;
using System.Collections.Generic;
using Utils;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores building state for the game.
	/// Manages building placers, building instances, counts, and unlock status.
	/// </summary>
	public class BuildingRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Dictionary mapping players to their building placer components.
		/// Tracks which player is currently placing buildings.
		/// </summary>
		private Dictionary<Player, BuildingPlacer> _placers;

		/// <summary>
		/// Dictionary mapping building types to lists of building instances.
		/// Tracks all buildings currently placed in the world by type.
		/// </summary>
		private Dictionary<BuildingType, List<BuildingBase>> _buildings;

		/// <summary>
		/// Total number of buildings currently placed in the world.
		/// Used for tracking overall building count.
		/// </summary>
		private int _numOfBuildings;

		/// <summary>
		/// Dictionary mapping building types to their current count.
		/// Tracks how many buildings of each type exist.
		/// </summary>
		private Dictionary<BuildingType, int> _buildingCounts;

		/// <summary>
		/// Dictionary mapping building types to their unlock status.
		/// Tracks which building types are available for construction.
		/// </summary>
		private Dictionary<BuildingType, bool> _buildingsUnlocked;

		/// <summary>
		/// The last building type selected by the user player.
		/// Used to remember the user's last building selection.
		/// </summary>
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
		public BuildingRuntimeData()
		{
			_placers = new Dictionary<Player, BuildingPlacer>();
			_buildings = new Dictionary<BuildingType, List<BuildingBase>>();
			_numOfBuildings = 0;
			_buildingCounts = new Dictionary<BuildingType, int>();
			_buildingsUnlocked = new Dictionary<BuildingType, bool>();
			_lastBuildingType = BuildingType.Townhall;
		}
	}
}
