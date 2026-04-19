using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "TownGoalSettings", menuName = "Scriptables/Town Goal Settings")]
	public class TownGoalSettings : ScriptableObject, IDataScriptable
	{
		public int MaxGoals = 2;
	}
}
