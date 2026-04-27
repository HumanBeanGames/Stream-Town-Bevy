using System.Collections.Generic;

using ScriptablesProcessorInfrastructure;
using Audio;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores audio state for the game.
	/// Manages the queue of audio handlers for audio processing.
	/// </summary>
	public class AudioRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Queue of audio handlers waiting to be processed.
		/// </summary>
		private Queue<AudioHandler> _audioHandlers;

		/// <summary>
		/// Gets the queue of audio handlers.
		/// </summary>
		public Queue<AudioHandler> AudioHandlers => _audioHandlers;

		/// <summary>
		/// Initializes the audio runtime data with default values.
		/// </summary>
		public AudioRuntimeData()
		{
			_audioHandlers = new Queue<AudioHandler>();
		}
	}
}
