using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for world generator behavior settings.
	/// </summary>
	public class WorldGeneratorBehaviorSettings : MonoBehaviour
	{
		[SerializeField]
		private bool _generateOnStart = true;
		[SerializeField]
		private bool _randomizeSeed = true;

		public bool GenerateOnStart => _generateOnStart;
		public bool RandomizeSeed => _randomizeSeed;
	}
}
