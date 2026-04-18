using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject for world generator debug settings.
	/// 
	/// Existing values to copy:
	/// - DebugGenerationTiming: false
	/// - FrameBudgetSeconds: 0.01f
	/// </summary>
	[CreateAssetMenu(fileName = "WorldGenDebugSettings", menuName = "Scriptables/World Generation/World Generator Debug Settings")]
	public class WorldGenDebugSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private bool _debugGenerationTiming = false;
		[SerializeField]
		private float _frameBudgetSeconds = 0.01f;

		public bool DebugGenerationTiming => _debugGenerationTiming;
		public float FrameBudgetSeconds => _frameBudgetSeconds;
	}
}
