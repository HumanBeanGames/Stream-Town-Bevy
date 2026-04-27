using System;

using ScriptablesProcessorInfrastructure;
using Utils;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores season state for the game.
	/// Manages season transitions, visual effects, and season-related events.
	/// </summary>
	public class SeasonRuntimeData : IRuntimeDataScriptable
	{
		private const float _winterTint = 0.42f;
		private const float _restTint = -0.08f;

		private Season _currentSeason;
		private bool _seasonChanging;
		private int _daysPerSeason;
		private Season _transitionFromSeason;
		private Season _transitionToSeason;
		private float _transitionElapsed;
		private float _transitionDuration;
		private bool _transitionTriggerEvent;

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
		public SeasonRuntimeData()
		{
			_currentSeason = Season.Spring;
			_seasonChanging = false;
			_daysPerSeason = 30;
			_transitionFromSeason = Season.Spring;
			_transitionToSeason = Season.Spring;
			_transitionElapsed = 0f;
			_transitionDuration = 0f;
			_transitionTriggerEvent = true;
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
