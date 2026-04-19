using Target;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "TargetConfigSettings", menuName = "Scriptables/TargetConfigSettings")]
	public class TargetConfigSettings : ScriptableObject, IDataScriptable
	{
		[Header("Targetable Data")]
		public TargetableData[] TargetableData;
	}
}
