using System;
using System.Collections.Generic;
using Audio;
using GameResources;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using AudioSettings = ScriptablesProcessorInfrastructure.AudioSettings;

namespace Processors
{
	/// <summary>
	/// Processor that manages audio handler updates and processing queue.
	/// Processes audio handlers in batches to balance performance with audio quality.
	/// </summary>
	public class AudioSourcesProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// ScriptableObject containing audio settings.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private AudioSettings _audioSettingsScriptable;

		/// <summary>
		/// Runtime audio sources data ScriptableObject.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private AudioSourcesRuntimeData _audioSourcesRuntimeData;

		/// <summary>
		/// Initializes the audio processor by clearing the audio handler queue.
		/// Called during game initialization to ensure clean state.
		/// </summary>
		public void Initialize()
		{
			_audioSourcesRuntimeData.AudioHandlers.Clear();
		}

		/// <summary>
		/// Processes audio logic every frame.
		/// Called every frame by the Coordinator.
		/// </summary>
		public void Process()
		{
			ProcessSources();
		}

		/// <summary>
		/// Processes a batch of audio handlers for the current frame.
		/// Updates audio handlers in rotation to distribute processing load.
		/// </summary>
		internal void ProcessSources()
		{
			Queue<AudioHandler> audioHandlers = _audioSourcesRuntimeData.AudioHandlers;
			int updatesPerTick = _audioSettingsScriptable.UpdatesPerTick;

			// Process up to the configured number of handlers per tick
			// Limited by both the updatesPerTick setting and the total number of handlers
			for (int i = 0; i < updatesPerTick && i < audioHandlers.Count; i++)
			{
				// Dequeue the next handler to process
				AudioHandler audioHandler = audioHandlers.Dequeue();
				
				// Only update and requeue if the handler is still enabled
				if (audioHandler.enabled)
				{
					audioHandler.UpdateLogic();
					audioHandlers.Enqueue(audioHandler);
				}
				else
					// Mark disabled handlers as untracked to remove them from the queue
					audioHandler.Tracked = false;
			}
		}

		/// <summary>
		/// Adds an audio handler to the processing queue.
		/// The handler will be processed in future ProcessSources calls.
		/// </summary>
		/// <param name="handler">The audio handler to add to the queue.</param>
		public void AddSourceToQueue(AudioHandler handler)
		{
			_audioSourcesRuntimeData.AudioHandlers.Enqueue(handler);
			handler.Tracked = true;
		}
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register AudioSourcesRuntimeData ScriptableObject
			AudioSourcesRuntimeData audioSourcesRuntimeData = ScriptableObject.CreateInstance<AudioSourcesRuntimeData>();
			containerBuilder.AddSingleton(audioSourcesRuntimeData);
		}
	}
}
