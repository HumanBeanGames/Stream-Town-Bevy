using Target;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "TargetConfig", menuName = "Scriptables/TargetConfig")]
	public class TargetConfig : ScriptableObject, IDataScriptable
	{
		[Header("Targetable Data")]
		public TargetableData[] TargetableData;
	}
}
