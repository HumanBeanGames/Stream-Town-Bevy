using System.Collections.Generic;
using ScriptablesProcessorInfrastructure;
using Utils;

namespace Data.Containers
{
	/// <summary>
	/// Container for static building data including building definitions.
	/// Registered in ProjectScope - available immediately on scene load.
	/// </summary>
	public class BuildingDataContainer
	{
        /// <summary>
        /// Dictionary mapping building types to their scriptable data.
        /// </summary>
		private Dictionary<BuildingType, BuildingData> _buildingDataDictionary;

        /// <summary>
        /// Gets the building data dictionary.
        /// </summary>
		public Dictionary<BuildingType, BuildingData> BuildingDataDictionary => _buildingDataDictionary;

        /// <summary>
        /// Gets the building data dictionary.
        /// </summary>
		public Dictionary<BuildingType, BuildingData> BuildingData => _buildingDataDictionary;

        /// <summary>
        /// Initializes a new building data container.
        /// </summary>
        /// <param name="allBuildingData">The all BuildingData object.</param>
		public BuildingDataContainer(AllBuildingDataSettings allBuildingData)
		{
			InitializeBuildingData(allBuildingData);
		}

        /// <summary>
        /// Initializes the building data dictionary from the all BuildingData.
        /// </summary>
        /// <param name="allBuildingData">The all BuildingData object.</param>
		private void InitializeBuildingData(AllBuildingDataSettings allBuildingData)
		{
			_buildingDataDictionary = new Dictionary<BuildingType, BuildingData>();

			for (int i = 0; i < allBuildingData.BuildingData.Length; i++)
			{
				var buildingType = (BuildingType)i;
				if (_buildingDataDictionary.ContainsKey(buildingType))
				{
					UnityEngine.Debug.LogError($"Attempted to add the same building type multiple times {buildingType}.");
					continue;
				}
				_buildingDataDictionary.Add(buildingType, allBuildingData.BuildingData[i]);
			}
		}
	}
}
