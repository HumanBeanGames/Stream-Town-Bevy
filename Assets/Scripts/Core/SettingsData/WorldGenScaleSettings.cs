using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject for world generator scale settings.
	/// 
	/// Existing values to copy:
	/// - XScale: 4
	/// - YScale: 4
	/// </summary>
	[CreateAssetMenu(fileName = "WorldGenScaleSettings", menuName = "Scriptables/World Generation/World Generator Scale Settings")]
	public class WorldGenScaleSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private float _xScale = 4;
		[SerializeField]
		private float _yScale = 4;

		public float XScale => _xScale;
		public float YScale => _yScale;
	}
}
