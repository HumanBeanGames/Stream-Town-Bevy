using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject container for season configuration data including season definitions and timing.
	/// Registered in ProjectScope - available immediately on scene load.
	/// </summary>
	[CreateAssetMenu(fileName = "AllSeasonSettings", menuName = "ScriptableObjects/AllSeasonSettings", order = 1)]
	public class AllSeasonSettings : ScriptableObject, IDataScriptable
	{
        /// <summary>
        /// Array of all season data configurations indexed by season type.
        /// </summary>
		[SerializeField]
		private SeasonDataSettings[] _seasonSettingsArray;

        /// <summary>
        /// The starting season.
        /// </summary>
		[SerializeField]
		private Season _startingSeason = Season.Summer;

        /// <summary>
        /// The number of days per season.
        /// </summary>
		[SerializeField]
		private int _daysPerSeason = 3;

        /// <summary>
        /// The season transition time in seconds.
        /// </summary>
		[SerializeField]
		private float _seasonTransitionTime = 10f;

        /// <summary>
        /// The grass material.
        /// </summary>
		[SerializeField]
		private Material _grassMaterial;

        /// <summary>
        /// The terrain material.
        /// </summary>
		[SerializeField]
		private Material _terrainMaterial;

        /// <summary>
        /// The tree material.
        /// </summary>
		[SerializeField]
		private Material _treeMaterial;

        /// <summary>
        /// The building material.
        /// </summary>
		[SerializeField]
		private Material _buildingMaterial;

        /// <summary>
        /// The water material.
        /// </summary>
		[SerializeField]
		private Material _waterMaterial;

        /// <summary>
        /// Tint value for winter season materials.
        /// </summary>
		[SerializeField]
		private float _winterTint = 0.42f;

        /// <summary>
        /// Tint value for non-winter season materials.
        /// </summary>
		[SerializeField]
		private float _restTint = -0.08f;

        /// <summary>
        /// Gets the array of all season data configurations.
        /// </summary>
		public SeasonDataSettings[] SeasonSettingsArray => _seasonSettingsArray;

        /// <summary>
        /// Gets the starting season.
        /// </summary>
		public Season StartingSeason => _startingSeason;

        /// <summary>
        /// Gets the number of days per season.
        /// </summary>
		public int DaysPerSeason => _daysPerSeason;

        /// <summary>
        /// Gets the season transition time in seconds.
        /// </summary>
		public float SeasonTransitionTime => _seasonTransitionTime;

        /// <summary>
        /// Gets the grass material.
        /// </summary>
		public Material GrassMaterial => _grassMaterial;

        /// <summary>
        /// Gets the terrain material.
        /// </summary>
		public Material TerrainMaterial => _terrainMaterial;

        /// <summary>
        /// Gets the tree material.
        /// </summary>
		public Material TreeMaterial => _treeMaterial;

        /// <summary>
        /// Gets the building material.
        /// </summary>
		public Material BuildingMaterial => _buildingMaterial;

        /// <summary>
        /// Gets the water material.
        /// </summary>
		public Material WaterMaterial => _waterMaterial;

        /// <summary>
        /// Gets the winter tint value.
        /// </summary>
		public float WinterTint => _winterTint;

        /// <summary>
        /// Gets the rest tint value.
        /// </summary>
		public float RestTint => _restTint;

        /// <summary>
        /// Gets the season scriptable data for a specific season.
        /// </summary>
        /// <param name="season">The season.</param>
        /// <returns>The season scriptable data.</returns>
		public SeasonDataSettings GetSeasonData(Season season)
		{
			for (int i = 0; i < _seasonSettingsArray.Length; i++)
			{
				if (_seasonSettingsArray[i].Season == season)
					return _seasonSettingsArray[i];
			}
			UnityEngine.Debug.LogError($"Tried to return a season that hasn't been setup: {season}");
			return null;
		}
	}
}
