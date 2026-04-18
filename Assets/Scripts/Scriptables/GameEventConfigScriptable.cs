using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores game event system configuration for the game.
	/// Contains debug settings for event logging.
	/// </summary>
	[CreateAssetMenu(fileName = "GameEventConfig", menuName = "Scriptables/GameEventConfig")]
	public class GameEventConfig : ScriptableObject, IDataScriptable
	{
		[Header("Debug Settings")]
		/// <summary>
		/// Whether to log game events to the console.
		/// If true, events are logged for debugging; if false, events are not logged.
		/// </summary>
		public bool LogEvents = true;
	}
}
