using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "TimeData", menuName = "Scriptables/TimeData")]
	public class TimeSettings : ScriptableObject, IDataScriptable
	{
		[Header("Time Settings")]
		public int SecondsPerDay = 3600;
	}
}
