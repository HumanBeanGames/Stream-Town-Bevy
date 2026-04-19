using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "SensorSettings", menuName = "Scriptables/Sensor Settings")]
	public class SensorSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private float _updateRate = 0.25f;

		public float UpdateRate => _updateRate;
	}
}
