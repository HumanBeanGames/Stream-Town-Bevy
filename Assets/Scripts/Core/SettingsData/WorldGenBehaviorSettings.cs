using UnityEngine;
using Sirenix.OdinInspector;

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
		[Title("Generation Control")]
		[SerializeField]
		private bool _suppressGeneration = false;

		[HideIf("_suppressGeneration", Value = true)]
		[Title("Generation Settings")]
		[SerializeField]
		private bool _generateOnStart = true;

		[HideIf("_suppressGeneration", Value = true)]
		[SerializeField]
		private bool _randomizeSeed = true;

		public bool SuppressGeneration => _suppressGeneration;
		public bool GenerateOnStart => _generateOnStart;
		public bool RandomizeSeed => _randomizeSeed;
	}
}
