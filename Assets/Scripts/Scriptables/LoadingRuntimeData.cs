using UnityEngine;
using System;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime loading state for the game.
	/// Manages load progress tracking and scene load request events.
	/// </summary>
	public class LoadingRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Current loading progress from 0.0 to 1.0.
		/// Used to display loading bar or progress indicator.
		/// </summary>
		[SerializeField]
		private float _loadProgress;

		/// <summary>
		/// Event fired when a scene load is requested.
		/// Passes the scene index to load.
		/// </summary>
		public event Action<int> OnSceneLoadRequested;

		/// <summary>
		/// Gets or sets the current loading progress.
		/// Value should be between 0.0 (not started) and 1.0 (complete).
		/// </summary>
		public float LoadProgress
		{
			get => _loadProgress;
			set => _loadProgress = value;
		}

		/// <summary>
		/// Initializes the loading runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
