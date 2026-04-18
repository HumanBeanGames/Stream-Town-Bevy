using System.Collections.Generic;
using UnityEngine;
using Audio;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime audio state for the game.
	/// Manages the queue of audio handlers for audio processing.
	/// </summary>
	public class AudioRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Queue of audio handlers waiting to be processed.
		/// </summary>
		[SerializeField]
		private Queue<AudioHandler> _audioHandlers = new Queue<AudioHandler>();

		/// <summary>
		/// Gets the queue of audio handlers.
		/// </summary>
		public Queue<AudioHandler> AudioHandlers => _audioHandlers;

		/// <summary>
		/// Initializes the audio runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
