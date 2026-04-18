using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "TownGoalSettings", menuName = "Scriptables/Town Goal Settings")]
	public class TownGoalSettingsScriptable : ScriptableObject, IDataScriptable
	{
		public int MaxGoals = 2;
	}
}
