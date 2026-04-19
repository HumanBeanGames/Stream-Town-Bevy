using ScriptablesProcessorInfrastructure;
using UnityEngine;
using Utils;

namespace Data.Containers
{
	/// <summary>
	/// ScriptableObject container for season configuration data including season definitions and timing.
	/// Registered in ProjectScope - available immediately on scene load.
	/// </summary>
	[CreateAssetMenu(fileName = "SeasonDataContainer", menuName = "ScriptableObjects/SeasonDataContainer", order = 1)]
	public class SeasonDataContainer : ScriptableObject, IDataScriptable
	{
        /// <summary>
        /// The all seasons scriptable data.
        /// </summary>
		[SerializeField]
		private AllSeasonsSettings _allSeasonsData;

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
        /// Gets the all seasons scriptable data.
        /// </summary>
		public AllSeasonsSettings AllSeasonsData => _allSeasonsData;

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
        /// Gets the season scriptable data for a specific season.
        /// </summary>
        /// <param name="season">The season.</param>
        /// <returns>The season scriptable data.</returns>
		public SeasonDataSettings GetSeasonData(Season season)
		{
			for (int i = 0; i < _allSeasonsData.SeasonSettingsArray.Length; i++)
			{
				if (_allSeasonsData.SeasonSettingsArray[i].Season == season)
					return _allSeasonsData.SeasonSettingsArray[i];
			}
			UnityEngine.Debug.LogError($"Tried to return a season that hasn't been setup: {season}");
			return null;
		}
	}
}
