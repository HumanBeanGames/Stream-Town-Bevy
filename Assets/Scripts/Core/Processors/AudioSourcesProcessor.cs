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
		/// Created and bound in InjectRuntimeData().
		/// </summary>
		private AudioSourcesRuntimeData _audioSourcesRuntimeData;

		/// <summary>
		/// Initializes the audio processor by clearing the audio handler queue.
		/// Creates RuntimeData after all processors are confirmed ready.
		/// Called during game initialization to ensure clean state.
		/// </summary>
		public void Initialize()
		{
			if (_audioSourcesRuntimeData == null)
				throw new InvalidOperationException("AudioSourcesProcessor: AudioSourcesRuntimeData has not been installed.");

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
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// AudioSourcesProcessor does not have scene-specific settings to refresh
		}

		/// <summary>
		/// Processes a batch of audio handlers for the current frame.
		/// Updates audio handlers in rotation to distribute processing load.
		/// </summary>
		internal void ProcessSources()
		{
			if (_audioSourcesRuntimeData == null || _audioSettingsScriptable == null)
				return;

			Queue<AudioHandler> audioHandlers = _audioSourcesRuntimeData.AudioHandlers;
			int updatesPerTick = _audioSettingsScriptable.UpdatesPerTick;
			int handlersToProcess = Mathf.Min(updatesPerTick, audioHandlers.Count);

			// Process up to the configured number of handlers per tick
			// Limited by both the updatesPerTick setting and the total number of handlers
			for (int i = 0; i < handlersToProcess; i++)
			{
				// Dequeue the next handler to process
				AudioHandler audioHandler = audioHandlers.Dequeue();
				if (audioHandler == null)
					continue;
				
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
			if (_audioSourcesRuntimeData == null || handler == null || handler.Tracked)
				return;

			_audioSourcesRuntimeData.AudioHandlers.Enqueue(handler);
			handler.Tracked = true;
		}

		/// <summary>
		/// Removes a scene-owned handler when it is disabled or destroyed.
		/// </summary>
		public void RemoveSourceFromQueue(AudioHandler handler)
		{
			if (_audioSourcesRuntimeData == null || handler == null)
				return;

			handler.Tracked = false;
			Queue<AudioHandler> audioHandlers = _audioSourcesRuntimeData.AudioHandlers;
			int handlerCount = audioHandlers.Count;
			for (int i = 0; i < handlerCount; i++)
			{
				AudioHandler candidate = audioHandlers.Dequeue();
				if (candidate != null && candidate != handler)
					audioHandlers.Enqueue(candidate);
			}
		}
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_audioSourcesRuntimeData != null)
				throw new InvalidOperationException("AudioSourcesProcessor: AudioSourcesRuntimeData has already been installed.");

			_audioSourcesRuntimeData = new AudioSourcesRuntimeData();
			containerBuilder.AddSingleton(_audioSourcesRuntimeData);
		}
	}
}
