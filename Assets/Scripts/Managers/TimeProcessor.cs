using System;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using Data.Containers;
using ScriptablesProcessorInfrastructure;

namespace Processors
{
	/// <summary>
	/// Processor that manages game time and day cycles.
	/// Handles time progression, day counting, and time-related events.
	/// </summary>
	public class TimeProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// ScriptableObject containing time data settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TimeSettings _timeDataScriptable;

        /// <summary>
        /// ScriptableObject containing time runtime data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TimeRuntimeData _timeRuntimeData;

        /// <summary>
        /// Gets or sets the world time passed in seconds.
        /// </summary>
        public float WorldTimePassed
        {
            get => _timeRuntimeData.WorldTimePassed;
            set => _timeRuntimeData.WorldTimePassed = value;
        }

        /// <summary>
        /// Gets or sets the seconds per day.
        /// </summary>
        public float SecondsPerDay
        {
            get => _timeRuntimeData.SecondsPerDay;
            set => _timeRuntimeData.SecondsPerDay = value;
        }

		public float DefaultSecondsPerDay => _timeDataScriptable.SecondsPerDay;

		public float CurrentWorldTime => _timeRuntimeData.WorldTimePassed;

        /// <summary>
        /// Gets the DayPassed event from runtime data.
        /// </summary>
        public event Action DayPassed
        {
            add => _timeRuntimeData.DayPassed += value;
            remove => _timeRuntimeData.DayPassed -= value;
        }

        /// <summary>
        /// Gets the current day count.
        /// </summary>
		public int DayCount => _timeRuntimeData.DayCount;

        /// <summary>
        /// Calculates how many days have passed and invokes the day passed event if a new day has started.
        /// </summary>
		public void CalculateDayCount()
		{
			int prevDayCount = _timeRuntimeData.DayCount;
			_timeRuntimeData.DayCount = _timeRuntimeData.CalculateDayCount(_timeRuntimeData.WorldTimePassed);

			if (prevDayCount < _timeRuntimeData.DayCount)
			{
				Debug.Log("Day Passed");
				_timeRuntimeData.InvokeDayPassed();
			}
		}

		public int CalculateDayCount(float timePassed)
		{
			_timeRuntimeData.DayCount = _timeRuntimeData.CalculateDayCount(timePassed);
			return _timeRuntimeData.DayCount;
		}

        /// <summary>
        /// Initializes the time processor.
        /// No initialization logic required.
        /// </summary>
		public void Initialize()
		{
			_timeRuntimeData.SecondsPerDay = _timeDataScriptable.SecondsPerDay;
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

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register TimeRuntimeData ScriptableObject
			TimeRuntimeData timeRuntimeData = ScriptableObject.CreateInstance<TimeRuntimeData>();
			containerBuilder.AddSingleton(timeRuntimeData);
		}

        /// <summary>
        /// Updates world time passed and calculates day count.
        /// Called every frame by the Coordinator.
        /// </summary>
		public void Process()
		{
			_timeRuntimeData.WorldTimePassed += Time.deltaTime;
			CalculateDayCount();
		}
	}
}
