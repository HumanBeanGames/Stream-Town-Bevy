using Buildings;
using System.Collections.Generic;
using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores global building configuration settings for the game.
	/// Contains cost modifiers, max levels, and building ages.
	/// </summary>
	[CreateAssetMenu(fileName = "BuildingConfigSettings", menuName = "Scriptables/BuildingConfigSettings")]
	public class BuildingConfigSettings : ScriptableObject, IDataScriptable
	{
		[Header("Cost Modifiers")]
		/// <summary>
		/// Dictionary mapping building types to their individual cost modifiers.
		/// Positive values increase cost, negative values decrease cost.
		/// </summary>
		[System.NonSerialized] public Dictionary<BuildingType, int> BuildingCostModifiers = new Dictionary<BuildingType, int>();

		[Header("Global Modifier")]
		/// <summary>
		/// Global modifier applied to all building costs.
		/// Positive values increase all building costs, negative values decrease them.
		/// </summary>
		public int GlobalBuildCostModifier = 0;

		[Header("Max Levels")]
		/// <summary>
		/// Dictionary mapping building types to their maximum upgrade levels.
		/// Determines how many times each building type can be upgraded.
		/// </summary>
		[System.NonSerialized] public Dictionary<BuildingType, int> BuildingsMaxLevel = new Dictionary<BuildingType, int>();

		[Header("Building Ages")]
		/// <summary>
		/// Dictionary mapping building types to their required building age.
		/// Determines which age a building belongs to for progression purposes.
		/// </summary>
		[System.NonSerialized] public Dictionary<BuildingType, Age> BuildingAges = new Dictionary<BuildingType, Age>();
	}
}
