using System;

using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores time state for the game.
	/// Manages day count, world time passed, and seconds per day.
	/// </summary>
	public class TimeRuntimeData : IRuntimeDataScriptable
	{
		private int _dayCount;
		private float _worldTimePassed;
		private float _secondsPerDay;

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
		public TimeRuntimeData()
		{
			_dayCount = 0;
			_worldTimePassed = 0;
			_secondsPerDay = 120f;
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
