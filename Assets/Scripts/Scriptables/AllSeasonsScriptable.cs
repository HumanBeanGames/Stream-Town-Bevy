using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores all season data configurations for the game.
	/// Acts as a central registry for season data.
	/// </summary>
	[CreateAssetMenu(fileName = "AllSeasonsSettings", menuName = "ScriptableObjects/AllSeasonsSettings", order = 1)]
	public class AllSeasonsSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Array of all season data configurations indexed by season type.
		/// </summary>
		public SeasonSettings[] SeasonSettingsArray;
	}
}
