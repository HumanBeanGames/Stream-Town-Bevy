using UnityEngine;
using UnityEngine.VFX;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "WeatherVFXSettings", menuName = "Scriptables/WeatherVFXSettings")]
	public class WeatherVFXSettings : ScriptableObject, IDataScriptable
	{
		[Header("Visual Effects")]
		public VisualEffect AutumnVFX;
		public VisualEffect WinterVFX;
		public VisualEffect SummerVFX;
		public VisualEffect SpringVFX;
	}
}
