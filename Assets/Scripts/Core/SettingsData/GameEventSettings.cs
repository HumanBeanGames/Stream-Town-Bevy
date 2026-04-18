using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores game event system settings for the game.
	/// Contains references to visual effects and debug logging configuration.
	/// </summary>
	[CreateAssetMenu(fileName = "GameEventSettings", menuName = "Scriptables/Game Event Settings")]
	public class GameEventSettingsScriptable : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Transform for the fish god spawn location.
		/// Used during specific game events to spawn special entities.
		/// </summary>
		[SerializeField]
		private Transform _fishGodSpawn;

		/// <summary>
		/// Particle system for the falling fish visual effect.
		/// Used during specific game events to create visual flair.
		/// </summary>
		[SerializeField]
		private ParticleSystem _fallingFishVFX;

		/// <summary>
		/// Whether to log game events to the console.
		/// If true, events are logged for debugging; if false, events are not logged.
		/// </summary>
		[SerializeField]
		private bool _logEvents = true;

		/// <summary>
		/// Gets the fish god spawn transform.
		/// </summary>
		public Transform FishGodSpawn => _fishGodSpawn;

		/// <summary>
		/// Gets the falling fish particle system.
		/// </summary>
		public ParticleSystem FallingFishVFX => _fallingFishVFX;

		/// <summary>
		/// Gets whether game events should be logged.
		/// </summary>
		public bool LogEvents => _logEvents;
	}
}
