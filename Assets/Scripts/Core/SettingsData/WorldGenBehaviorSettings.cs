using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject for world generator behavior settings.
	/// 
	/// Existing values to copy:
	/// - GenerateOnStart: true
	/// - RandomizeSeed: true
	/// </summary>
	[CreateAssetMenu(fileName = "WorldGenBehaviorSettings", menuName = "Scriptables/World Generation/World Generator Behavior Settings")]
	public class WorldGenBehaviorSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private bool _generateOnStart = true;
		[SerializeField]
		private bool _randomizeSeed = true;

		public bool GenerateOnStart => _generateOnStart;
		public bool RandomizeSeed => _randomizeSeed;
	}
}
