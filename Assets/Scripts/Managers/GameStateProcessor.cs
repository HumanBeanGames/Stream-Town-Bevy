using GameEventSystem;
using System;
using UnityEngine;
using Utils;
using Reflex.Core;
using Reflex.Attributes;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
	/// <summary>
	/// Handles all current Game State Logic.
	/// Manages game state flags for player readiness, world generation, and object pooling.
	/// </summary>
	public partial class GameStateProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// Runtime data for game state.
        /// Assigned in InjectRuntimeData.
        /// </summary>
        private GameStateRuntimeData _gameStateRuntimeData;

        /// <summary>
        /// The Twitch chat processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TwitchChatProcessor _twitchChatProcessor;

		/// <summary>
		/// Player control is active and ready.
		/// </summary>
		public bool PlayerReady
		{
			get => _gameStateRuntimeData.PlayerReady;
			private set
			{
                // Only update and invoke event if value actually changes
				if (_gameStateRuntimeData.PlayerReady == value) return;
				_gameStateRuntimeData.PlayerReady = value;
				_gameStateRuntimeData.InvokeReadiedPlayer();
				_twitchChatProcessor.SetPlayerReady(value);
			}
		}

        /// <summary>
        /// Event fired when the player becomes ready.
        /// </summary>
        public event Action ReadiedPlayer { add => _gameStateRuntimeData.ReadiedPlayer += value; remove => _gameStateRuntimeData.ReadiedPlayer -= value; }

		/// <summary>
		/// The world has been loaded.
		/// </summary>
		public bool WorldGenerated
		{
			get => _gameStateRuntimeData.WorldGenerated;
			private set
			{
                // Only update and invoke event if value actually changes
				if (_gameStateRuntimeData.WorldGenerated == value) return;
				_gameStateRuntimeData.WorldGenerated = value;
				GeneratedWorld?.Invoke();
			}

		}

        /// <summary>
        /// Event fired when the world has finished generating.
        /// </summary>
        public event Action GeneratedWorld;

		/// <summary>
		/// All objects have been pooled.
		/// </summary>
		public bool ObjectsPooled
		{
			get => _gameStateRuntimeData.ObjectsPooled;
			private set
			{
                // Only update and invoke event if value actually changes
				if (_gameStateRuntimeData.ObjectsPooled == value) return;
				_gameStateRuntimeData.ObjectsPooled = value;
				PooledObjects?.Invoke();
			}
		}

        /// <summary>
        /// Event fired when all objects have been pooled.
        /// </summary>
        public event Action PooledObjects;

        /// <summary>
        /// Controls whether game state changes are logged to the console.
        /// </summary>
        public bool EventLogging
		{
			get => _gameStateRuntimeData.EventLogging;
			set => _gameStateRuntimeData.EventLogging = value;
		}

        /// <summary>
        /// Initializes the game state processor.
        /// Creates RuntimeData after all processors are confirmed ready.
        /// No initialization logic required for this processor.
        /// </summary>
        		public void Initialize()
		{
			if (_gameStateRuntimeData == null)
				throw new InvalidOperationException("GameStateProcessor: GameStateRuntimeData has not been installed.");
		}

        /// <summary>
        /// Registers this processor as a singleton in the dependency injection container.
        /// Called by Reflex during container initialization.
        /// </summary>
        /// <param name="containerBuilder">The container builder to register bindings with.</param>
        public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		/// <summary>
		/// Injects the GameStateRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_gameStateRuntimeData != null)
				throw new InvalidOperationException("GameStateProcessor: GameStateRuntimeData has already been installed.");

			_gameStateRuntimeData = new GameStateRuntimeData();
			containerBuilder.AddSingleton(_gameStateRuntimeData);
		}

        /// <summary>
        /// Processes game state logic every frame.
        /// Called every frame by the Coordinator.
        /// GameStateProcessor does not require per-frame updates.
        /// </summary>
        public void Process()
        {
            // GameStateProcessor does not require per-frame updates
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // GameStateProcessor does not have scene-specific settings to refresh
        }

		/// <summary>
		/// Notifies the State Processor that the Player is ready.
		/// </summary>
        public void NotifyPlayerReady()
		{
			PlayerReady = true;

			if (_gameStateRuntimeData.EventLogging)
				Debug.Log("Player Ready");
		}

		/// <summary>
		/// Notifies the State Processor that the world has finished loading.
		/// </summary>
		public void NotifyWorldLoaded()
		{
			WorldGenerated = true;

			if (_gameStateRuntimeData.EventLogging)
				Debug.Log("World Loaded");
		}

		/// <summary>
		/// Notifies the State Processor that all objects have been pooled.
		/// </summary>
		public void NotifyObjectsPooled()
		{
			ObjectsPooled = true;

			if (_gameStateRuntimeData.EventLogging)
				Debug.Log("Pooling Finished");
		}

		/// <summary>
		/// Notifies the State Processor that a new world is being Loaded.
		/// </summary>
		public void NotifyLoadingWorld()
		{
			ResetStateFlags();

			if (_gameStateRuntimeData.EventLogging)
				Debug.Log("Loading New World");
		}

		/// <summary>
		/// Resets all State Processor data to default.
		/// </summary>
		public void ResetStateFlags()
		{
			_gameStateRuntimeData.PlayerReady = false;
			_gameStateRuntimeData.WorldGenerated = false;
			_gameStateRuntimeData.ObjectsPooled = false;
		}
	}
}
