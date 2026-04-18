using System.Collections.Generic;
using Audio;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime audio sources state for the game.
	/// </summary>
	public class AudioSourcesRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		private Queue<AudioHandler> _audioHandlers = new Queue<AudioHandler>();

		public Queue<AudioHandler> AudioHandlers => _audioHandlers;
	}
}
