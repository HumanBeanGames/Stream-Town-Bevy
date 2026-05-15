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
        /// Created and bound in InjectRuntimeData().
        /// </summary>
        private TimeRuntimeData _timeRuntimeData;

        /// <summary>
        /// The Twitch chat processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TwitchChatProcessor _twitchChatProcessor;

        /// <summary>
        /// The debug processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Processors.DebugProcessor _debugProcessor;

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
				_debugProcessor.Log(DebugLogCategory.General, "Day Passed");
				_timeRuntimeData.InvokeDayPassed();
			}
		}

		public int CalculateDayCount(float timePassed)
		{
			_timeRuntimeData.DayCount = _timeRuntimeData.CalculateDayCount(timePassed);
			return _timeRuntimeData.DayCount;
		}

		public void ResetWorldTime()
		{
			_timeRuntimeData.WorldTimePassed = 0f;
			_timeRuntimeData.DayCount = 0;
			_twitchChatProcessor.SetCurrentWorldTime(0f);
		}

        /// <summary>
        /// Initializes the time processor.
        /// Creates RuntimeData after all processors are confirmed ready.
        /// </summary>
		public void Initialize()
		{
			if (_timeRuntimeData == null)
				throw new InvalidOperationException("TimeProcessor runtime data has not been installed.");

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
			if (_timeRuntimeData != null)
				throw new InvalidOperationException("TimeProcessor runtime data has already been installed.");

			_timeRuntimeData = new TimeRuntimeData();
			containerBuilder.AddSingleton(_timeRuntimeData);
		}

        /// <summary>
        /// Updates world time passed and calculates day count.
        /// Called every frame by the Coordinator.
        /// </summary>
		public void Process()
		{
			_timeRuntimeData.WorldTimePassed += Time.deltaTime;
			_twitchChatProcessor.SetCurrentWorldTime(_timeRuntimeData.WorldTimePassed);
			CalculateDayCount();
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// TimeProcessor does not have scene-specific settings to refresh
		}
	}
}
