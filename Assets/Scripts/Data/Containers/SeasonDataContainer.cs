using ScriptablesProcessorInfrastructure;
using Utils;

namespace Data.Containers
{
	/// <summary>
	/// Container for season configuration data including season definitions and timing.
	/// Registered in ProjectScope - available immediately on scene load.
	/// </summary>
	public class SeasonDataContainer
	{
        /// <summary>
        /// The all seasons scriptable data.
        /// </summary>
		private AllSeasonsSettings _allSeasonsData;

        /// <summary>
        /// The starting season.
        /// </summary>
		private Season _startingSeason;

        /// <summary>
        /// The number of days per season.
        /// </summary>
		private int _daysPerSeason;

        /// <summary>
        /// The season transition time in seconds.
        /// </summary>
		private float _seasonTransitionTime;

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
        /// Initializes a new season data container.
        /// </summary>
        /// <param name="allSeasonsData">The all seasons scriptable data.</param>
        /// <param name="startingSeason">The starting season.</param>
        /// <param name="daysPerSeason">The number of days per season.</param>
        /// <param name="seasonTransitionTime">The season transition time in seconds.</param>
		public SeasonDataContainer(AllSeasonsSettings allSeasonsData, Season startingSeason = Season.Summer, int daysPerSeason = 3, float seasonTransitionTime = 10f)
		{
			_allSeasonsData = allSeasonsData;
			_startingSeason = startingSeason;
			_daysPerSeason = daysPerSeason;
			_seasonTransitionTime = seasonTransitionTime;
		}

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
