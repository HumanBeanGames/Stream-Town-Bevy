using GameResources;
using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores configuration data for a specific building type.
	/// Contains building name, sprite, type, unlock status, costs, and multipliers.
	/// </summary>
	public class BuildingData : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Display name of the building shown in UI.
		/// </summary>
		public string BuildingName;

		/// <summary>
		/// Sprite/icon used to represent the building in the UI.
		/// </summary>
		public Sprite BuildingSprite;

		/// <summary>
		/// The type/category of this building.
		/// Used for building identification and lookup.
		/// </summary>
		public BuildingType BuildingType;

		/// <summary>
		/// Whether this building can be leveled up after placement.
		/// If false, building remains at base level permanently.
		/// </summary>
		public bool CanLevel = true;

		/// <summary>
		/// Whether this building can be placed by players.
		/// If false, building is disabled from placement (e.g., special or decorative buildings).
		/// </summary>
		public bool Placeable = true;

		/// <summary>
		/// Whether this building is unlocked and available for construction.
		/// Unlocked status is typically controlled by tech tree progression.
		/// </summary>
		public bool Unlocked = false;

		/// <summary>
		/// The age at which this building becomes available.
		/// Used for progression and age-based building availability.
		/// </summary>
		public Age StartingAge;

		[Header("Base Cost To Build")]
		/// <summary>
		/// Base resource cost to construct this building.
		/// Contains the required resources and their amounts.
		/// </summary>
		public ResourceCostData BuildResourceCost;

		/// <summary>
		/// Multiplier applied to build cost for each existing building of this type.
		/// Used to scale costs based on building count (e.g., 2x cost for second building).
		/// </summary>
		public float CostIncreasePerBuildingMultiplier = 2;

		[Header("Base Cost To Level")]
		/// <summary>
		/// Base resource cost to level up this building.
		/// Contains the required resources and their amounts for upgrade.
		/// </summary>
		public ResourceCostData LevelResourceCost;

		/// <summary>
		/// Multiplier applied to level cost for each level increase.
		/// Used to scale upgrade costs based on current level (e.g., 2x cost for level 2).
		/// </summary>
		public float CostIncreasePerLevelMultiplier = 2;
	}
}
