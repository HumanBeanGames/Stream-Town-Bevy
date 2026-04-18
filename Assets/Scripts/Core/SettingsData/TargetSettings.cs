using Target;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "TargetSettings", menuName = "Scriptables/Target Settings")]
	public class TargetSettingsScriptable : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private TargetableData[] _targetableData;

		public TargetableData[] TargetableData => _targetableData;
	}
}
