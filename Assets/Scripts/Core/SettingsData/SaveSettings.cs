using UnityEngine;
using World.Generation;
using Processors;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores save/load system settings for the game.
	/// Contains references to world generation components.
	/// </summary>
	[CreateAssetMenu(fileName = "SaveSettings", menuName = "Scriptables/Save Settings")]
	public class SaveSettingsScriptable : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Reference to the procedural world generator component.
		/// Used to regenerate the world when loading saved games.
		/// </summary>
		[SerializeField]
		private WorldGenProcessor _generationObject = null;

		/// <summary>
		/// Gets the procedural world generator component.
		/// </summary>
		public WorldGenProcessor GenerationObject => _generationObject;
	}
}
