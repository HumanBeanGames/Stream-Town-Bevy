using System;

using ScriptablesProcessorInfrastructure;

namespace Processors
{
	/// <summary>
	/// Runtime data for GameStateProcessor.
	/// Manages initialization state tracking and event logging configuration.
	/// </summary>
	public class GameStateRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Whether the player has finished initialization and is ready to play.
		/// Set to true after player setup completes.
		/// </summary>
		private bool _playerReady;

		/// <summary>
		/// Whether the game world has been generated.
		/// Set to true after procedural generation completes.
		/// </summary>
		private bool _worldGenerated;

		/// <summary>
		/// Whether game objects have been pooled.
		/// Set to true after object pooling system initializes.
		/// </summary>
		private bool _objectsPooled;

		/// <summary>
		/// Whether event logging is enabled.
		/// If true, game events are logged to console for debugging.
		/// </summary>
		private bool _eventLogging;

		/// <summary>
		/// Event fired when the player becomes ready.
		/// Invoked when PlayerReady is set to true.
		/// </summary>
		public event Action ReadiedPlayer;

		/// <summary>
		/// Gets or sets whether the player is ready.
		/// </summary>
		public bool PlayerReady
		{
			get => _playerReady;
			set => _playerReady = value;
		}

		/// <summary>
		/// Gets or sets whether the world has been generated.
		/// </summary>
		public bool WorldGenerated
		{
			get => _worldGenerated;
			set => _worldGenerated = value;
		}

		/// <summary>
		/// Gets or sets whether objects have been pooled.
		/// </summary>
		public bool ObjectsPooled
		{
			get => _objectsPooled;
			set => _objectsPooled = value;
		}

		/// <summary>
		/// Gets or sets whether event logging is enabled.
		/// </summary>
		public bool EventLogging
		{
			get => _eventLogging;
			set => _eventLogging = value;
		}

		/// <summary>
		/// Initializes the game state runtime data with default values.
		/// </summary>
		public GameStateRuntimeData()
		{
			_playerReady = false;
			_worldGenerated = false;
			_objectsPooled = false;
			_eventLogging = true;
		}

		/// <summary>
		/// Invokes the ReadiedPlayer event.
		/// Called when the player becomes ready.
		/// </summary>
		public void InvokeReadiedPlayer()
		{
			ReadiedPlayer?.Invoke();
		}
	}
}
