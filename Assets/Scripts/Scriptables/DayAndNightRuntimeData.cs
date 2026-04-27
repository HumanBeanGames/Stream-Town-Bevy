using System;

using ScriptablesProcessorInfrastructure;

namespace Processors
{
	/// <summary>
	/// Runtime data for DayAndNightProcessor.
	/// Manages the current time of day, day/night transition progress, and related visual effects.
	/// </summary>
	public class DayAndNightRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Current time of day in hours (0-24).
		/// </summary>
		private float _currentTimeOfDay;

		/// <summary>
		/// Current day/night percentage (0 = midnight, 0.5 = noon, 1 = midnight).
		/// </summary>
		private float _dayNightPercentage;

		/// <summary>
		/// Whether it is currently day time.
		/// </summary>
		private bool _isDay;

		/// <summary>
		/// Whether a day/night transition is currently in progress.
		/// </summary>
		private bool _isTransitioning;

		/// <summary>
		/// Whether the current transition is to day time.
		/// </summary>
		private bool _transitionToDay;

		/// <summary>
		/// Time elapsed in the current transition.
		/// </summary>
		private float _transitionTime;

		/// <summary>
		/// Time remaining until the next transition.
		/// </summary>
		private float _timeTillTransition;

		/// <summary>
		/// Length of the day period in seconds.
		/// </summary>
		private float _dayLength;

		/// <summary>
		/// Length of the night period in seconds.
		/// </summary>
		private float _nightLength;

		/// <summary>
		/// Length of the transition period in seconds.
		/// </summary>
		private float _transitionLength;

		/// <summary>
		/// Event fired when the daytime period has begun.
		/// </summary>
		public event Action OnDayStarted;

		/// <summary>
		/// Event fired when the nighttime period has begun.
		/// </summary>
		public event Action OnNightStarted;

		/// <summary>
		/// Event fired when the daytime period is about to start.
		/// </summary>
		public event Action OnDayStarting;

		/// <summary>
		/// Event fired when the nighttime period is about to start.
		/// </summary>
		public event Action OnNightStarting;

		/// <summary>
		/// Gets or sets the current time of day in hours (0-24).
		/// </summary>
		public float CurrentTimeOfDay
		{
			get => _currentTimeOfDay;
			set => _currentTimeOfDay = value;
		}

		/// <summary>
		/// Gets or sets the current day/night percentage (0-1).
		/// </summary>
		public float DayNightPercentage
		{
			get => _dayNightPercentage;
			set => _dayNightPercentage = value;
		}

		/// <summary>
		/// Gets or sets whether it is currently day time.
		/// </summary>
		public bool IsDay
		{
			get => _isDay;
			set => _isDay = value;
		}

		/// <summary>
		/// Gets or sets whether it is currently day time.
		/// </summary>
		public bool IsDayTime
		{
			get => _isDay;
			set => _isDay = value;
		}

		/// <summary>
		/// Gets or sets whether a transition is in progress.
		/// </summary>
		public bool IsTransitioning
		{
			get => _isTransitioning;
			set => _isTransitioning = value;
		}

		/// <summary>
		/// Gets or sets whether transitioning to day.
		/// </summary>
		public bool TransitionToDay
		{
			get => _transitionToDay;
			set => _transitionToDay = value;
		}

		/// <summary>
		/// Gets or sets transition time elapsed.
		/// </summary>
		public float TransitionTime
		{
			get => _transitionTime;
			set => _transitionTime = value;
		}

		/// <summary>
		/// Gets or sets time till next transition.
		/// </summary>
		public float TimeTillTransition
		{
			get => _timeTillTransition;
			set => _timeTillTransition = value;
		}

		/// <summary>
		/// Gets or sets day length.
		/// </summary>
		public float DayLength
		{
			get => _dayLength;
			set => _dayLength = value;
		}

		/// <summary>
		/// Gets or sets night length.
		/// </summary>
		public float NightLength
		{
			get => _nightLength;
			set => _nightLength = value;
		}

		/// <summary>
		/// Gets or sets transition length.
		/// </summary>
		public float TransitionLength
		{
			get => _transitionLength;
			set => _transitionLength = value;
		}

		/// <summary>
		/// Initializes the day/night runtime data with default values.
		/// </summary>
		public DayAndNightRuntimeData()
		{
			_currentTimeOfDay = 0f;
			_dayNightPercentage = 0f;
			_isDay = true;
			_isTransitioning = false;
			_transitionToDay = false;
			_transitionTime = 0f;
			_timeTillTransition = 0f;
			_dayLength = 0f;
			_nightLength = 0f;
			_transitionLength = 0f;
		}

		public void InvokeDayStarted()
		{
			OnDayStarted?.Invoke();
		}

		public void InvokeNightStarted()
		{
			OnNightStarted?.Invoke();
		}

		public void InvokeDayStarting()
		{
			OnDayStarting?.Invoke();
		}

		public void InvokeNightStarting()
		{
			OnNightStarting?.Invoke();
		}
	}
}
