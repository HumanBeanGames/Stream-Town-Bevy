using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores audio system settings for the game.
	/// Contains editor-configurable audio processing parameters.
	/// </summary>
	[CreateAssetMenu(fileName = "AudioSettings", menuName = "Scriptables/Audio Settings")]
	public class AudioSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Number of audio updates to process per game tick.
		/// Controls the processing rate of the audio system.
		/// </summary>
		[SerializeField]
		private int _updatesPerTick = 30;

		/// <summary>
		/// Gets the number of audio updates per tick.
		/// </summary>
		public int UpdatesPerTick => _updatesPerTick;
	}
}
