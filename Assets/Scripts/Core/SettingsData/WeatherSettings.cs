using UnityEngine;
using UnityEngine.VFX;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "WeatherSettings", menuName = "Scriptables/Weather Settings")]
	public class WeatherSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private VisualEffect _autumnVFX;
		[SerializeField]
		private VisualEffect _winterVFX;
		[SerializeField]
		private VisualEffect _summerVFX;
		[SerializeField]
		private VisualEffect _springVFX;

		public VisualEffect AutumnVFX => _autumnVFX;
		public VisualEffect WinterVFX => _winterVFX;
		public VisualEffect SummerVFX => _summerVFX;
		public VisualEffect SpringVFX => _springVFX;
	}
}
