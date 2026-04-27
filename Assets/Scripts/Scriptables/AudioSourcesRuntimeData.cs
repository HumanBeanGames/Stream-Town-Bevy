using System.Collections.Generic;

using Audio;
using ScriptablesProcessorInfrastructure;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores audio sources state for the game.
	/// </summary>
	public class AudioSourcesRuntimeData : IRuntimeDataScriptable
	{
		private Queue<AudioHandler> _audioHandlers;

		public Queue<AudioHandler> AudioHandlers => _audioHandlers;

		public AudioSourcesRuntimeData()
		{
			_audioHandlers = new Queue<AudioHandler>();
		}
	}
}
