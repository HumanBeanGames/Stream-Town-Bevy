using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores all building data configurations for the game.
	/// Acts as a central registry for building type data.
	/// </summary>
	[CreateAssetMenu(fileName = "AllBuildingDataSettings", menuName = "ScriptableObjects/AllBuildingDataSettings", order = 1)]
	public class AllBuildingDataSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Array of all building data configurations indexed by building type.
		/// </summary>
		public BuildingData[] BuildingData;
	}
}
