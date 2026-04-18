using UnityEngine;
using UnityEngine.VFX;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "WeatherVFX", menuName = "Scriptables/WeatherVFX")]
	public class WeatherVFX : ScriptableObject, IDataScriptable
	{
		[Header("Visual Effects")]
		public VisualEffect AutumnVFX;
		public VisualEffect WinterVFX;
		public VisualEffect SummerVFX;
		public VisualEffect SpringVFX;
	}
}
