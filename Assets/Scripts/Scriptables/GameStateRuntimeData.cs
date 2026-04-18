using System;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for GameStateProcessor.
	/// Manages initialization state tracking and event logging configuration.
	/// </summary>
	public class GameStateRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Whether the player has finished initialization and is ready to play.
		/// Set to true after player setup completes.
		/// </summary>
		[SerializeField]
		private bool _playerReady;

		/// <summary>
		/// Whether the game world has been generated.
		/// Set to true after procedural generation completes.
		/// </summary>
		[SerializeField]
		private bool _worldGenerated;

		/// <summary>
		/// Whether game objects have been pooled.
		/// Set to true after object pooling system initializes.
		/// </summary>
		[SerializeField]
		private bool _objectsPooled;

		/// <summary>
		/// Whether event logging is enabled.
		/// If true, game events are logged to console for debugging.
		/// </summary>
		[SerializeField]
		private bool _eventLogging = true;

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
		public void Initialize()
		{
			// Initialize with default values if needed
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
