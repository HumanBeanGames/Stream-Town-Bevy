using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for world generator debug settings.
	/// </summary>
	public class WorldGeneratorDebugSettings : MonoBehaviour
	{
		[SerializeField]
		private bool _debugGenerationTiming = false;
		[SerializeField]
		private float _frameBudgetSeconds = 0.01f;

		public bool DebugGenerationTiming => _debugGenerationTiming;
		public float FrameBudgetSeconds => _frameBudgetSeconds;
	}
}
