using System;
using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime season state for the game.
	/// Manages season transitions, visual effects, and season-related events.
	/// </summary>
	public class SeasonRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		private const float _winterTint = 0.42f;
		private const float _restTint = -0.08f;

		[SerializeField]
		private Season _currentSeason;
		[SerializeField]
		private bool _seasonChanging = false;
		[SerializeField]
		private int _daysPerSeason = 30;
		[SerializeField]
		private Season _transitionFromSeason;
		[SerializeField]
		private Season _transitionToSeason;
		[SerializeField]
		private float _transitionElapsed;
		[SerializeField]
		private float _transitionDuration;
		[SerializeField]
		private bool _transitionTriggerEvent = true;

		public event Action<Season> OnSeasonChanged;
		public event Action<Season> OnSeasonChanging;

		public bool SeasonChanging
		{
			get => _seasonChanging;
			set => _seasonChanging = value;
		}
		public Season CurrentSeason
		{
			get => _currentSeason;
			set => _currentSeason = value;
		}
		public int DaysPerSeason
		{
			get => _daysPerSeason;
			set => _daysPerSeason = value;
		}
		public Season TransitionFromSeason
		{
			get => _transitionFromSeason;
			set => _transitionFromSeason = value;
		}
		public Season TransitionToSeason
		{
			get => _transitionToSeason;
			set => _transitionToSeason = value;
		}
		public float TransitionElapsed
		{
			get => _transitionElapsed;
			set => _transitionElapsed = value;
		}
		public float TransitionDuration
		{
			get => _transitionDuration;
			set => _transitionDuration = value;
		}
		public bool TransitionTriggerEvent
		{
			get => _transitionTriggerEvent;
			set => _transitionTriggerEvent = value;
		}

		/// <summary>
		/// Initializes the season runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}

		public void InvokeSeasonChanging(Season season)
		{
			OnSeasonChanging?.Invoke(season);
		}

		public void InvokeSeasonChanged(Season season)
		{
			OnSeasonChanged?.Invoke(season);
		}
	}
}
