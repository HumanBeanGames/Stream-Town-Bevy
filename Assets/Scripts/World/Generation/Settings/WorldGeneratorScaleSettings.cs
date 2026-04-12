using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for world generator scale settings.
	/// </summary>
	public class WorldGeneratorScaleSettings : MonoBehaviour
	{
		[SerializeField]
		private float _xScale = 4;
		[SerializeField]
		private float _yScale = 4;

		public float XScale => _xScale;
		public float YScale => _yScale;
	}
}
