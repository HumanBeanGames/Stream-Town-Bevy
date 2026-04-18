using System;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime time state for the game.
	/// Manages day count, world time passed, and seconds per day.
	/// </summary>
	public class TimeRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		[SerializeField]
		private int _dayCount = 0;
		[SerializeField]
		private float _worldTimePassed = 0;
		[SerializeField]
		private float _secondsPerDay = 120f;

		public int DayCount
		{
			get => _dayCount;
			set => _dayCount = value;
		}
		public float WorldTimePassed
		{
			get => _worldTimePassed;
			set => _worldTimePassed = value;
		}
		public float SecondsPerDay
		{
			get => _secondsPerDay;
			set => _secondsPerDay = value;
		}

		public event Action DayPassed;

		/// <summary>
		/// Initializes the time runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}

		public int CalculateDayCount(float worldTimePassed)
		{
			return Mathf.FloorToInt(worldTimePassed / _secondsPerDay);
		}

		public void InvokeDayPassed()
		{
			DayPassed?.Invoke();
		}
	}
}
