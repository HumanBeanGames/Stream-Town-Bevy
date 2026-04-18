using Processors;
using Reflex.Attributes;
using System;
using UnityEngine;
using World;

namespace GameEventSystem
{
	/// <summary>
	/// Holds data relating to an In Game Event.
	/// </summary>
	public class GameEvent
	{
        /// <summary>
        /// The type of game event.
        /// </summary>
		[Serializable]
		public enum EventType
		{
            /// <summary>No event type.</summary>
			None,
            /// <summary>Fish God event.</summary>
			FishGod,
            /// <summary>Night raid event.</summary>
			NightRaid,
            /// <summary>Blood moon raid event.</summary>
			BloodMoonRaid,
            /// <summary>Adventure Land Necro event.</summary>
			AdventureLandNecro,
            /// <summary>Adventure Land Fish God event.</summary>
			AdventureLandFishGod,
            /// <summary>Dragon Fire event.</summary>
			DragonFire,
            /// <summary>Dragon Forest event.</summary>
			DragonForest,
            /// <summary>Dragon Ice event.</summary>
			DragonIce,
            /// <summary>Dragon Two-Headed event.</summary>
			DragonTwoHeaded,
            /// <summary>Dragon Undead event.</summary>
			DragonUndead,
            /// <summary>Subscription event.</summary>
			Subscription,
            /// <summary>Bits donated event.</summary>
			BitsDonated,
            /// <summary>Vote event.</summary>
			Vote,
            /// <summary>Monster raid event.</summary>
			MonsterRaid,
            /// <summary>New king vote event.</summary>
			NewKingVote,
            /// <summary>Keep king vote event.</summary>
			KeepKingVote,
            /// <summary>Tech vote event.</summary>
			TechVote,
            /// <summary>Total count of event types.</summary>
			Count
		}

        /// <summary>
        /// Whether the event always returns success.
        /// </summary>
		protected bool _alwaysReturnSuccess = false;

        /// <summary>
        /// The return data.
        /// </summary>
		protected object _returnData;

		/// <summary>
		/// When the event will start.
		/// </summary>
		protected double _eventStartTime;

		/// <summary>
		/// How long the event will last for, leave as -1 for undetermined.
		/// </summary>
		protected double _eventDuration;

		/// <summary>
		/// The Type of event.
		/// </summary>
		private EventType _eventType;

		/// <summary>
		/// Any extra data that needs to be passed for this event.
		/// </summary>
		private object _data;

		/// <summary>
		/// If this event can override the currently running event.
		/// </summary>
		private bool _overrideCurrentEvent;

		/// <summary>
		/// How long after the start time can this event still start? -1 for no timeout.
		/// </summary>
		private double _timeout;

        /// <summary>
        /// Event fired when the event starts.
        /// </summary>
		private Action EventStarted;

        /// <summary>
        /// Event fired when the event ends.
        /// </summary>
		public Action<bool, EventType, object> EventEnded;

        /// <summary>
        /// Whether the event was successful.
        /// </summary>
		private bool _success;

        /// <summary>
        /// Whether the event is active.
        /// </summary>
		private bool _active;

        /// <summary>
        /// Game event processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GameEventProcessor _gameEventProcessor;

        /// <summary>
        /// Gets the start time.
        /// </summary>
		public double StartTime => _eventStartTime;

        /// <summary>
        /// Gets the event duration.
        /// </summary>
		public double EventDuration => _eventDuration;

        /// <summary>
        /// Gets the event type.
        /// </summary>
		public EventType Event => _eventType;

        /// <summary>
        /// Gets the event data.
        /// </summary>
		public object Data => _data;

        /// <summary>
        /// Gets whether the event can override the current event.
        /// </summary>
		public bool OverrideCurrentEvent => _overrideCurrentEvent;

        /// <summary>
        /// Gets the timeout.
        /// </summary>
		public double Timeout => _timeout;

        /// <summary>
        /// Gets the remaining duration.
        /// </summary>
        /// <param name="currentTime">The current time.</param>
        /// <returns>The remaining duration.</returns>
		public double RemainingDuration(double currentTime) => _active ? (StartTime + EventDuration) - currentTime : -1;

        /// <summary>
        /// Gets whether the event was successful.
        /// </summary>
		public bool Success => _success;

        /// <summary>
        /// Initializes a new game event instance.
        /// </summary>
        /// <param name="delay">The delay before the event starts.</param>
        /// <param name="eventDuration">The duration of the event.</param>
        /// <param name="eventType">The type of event.</param>
        /// <param name="data">Additional data for the event.</param>
        /// <param name="overrideCurrentEvent">Whether to override the current event.</param>
        /// <param name="timeout">The timeout for the event.</param>
		public GameEvent(double delay, double eventDuration, EventType eventType = EventType.None, object data = null, bool overrideCurrentEvent = false, double timeout = -1)
		{
			_eventStartTime = delay; // Will be set to current time + delay when started
			_eventDuration = eventDuration;
			_eventType = eventType;
			_data = data;
			_overrideCurrentEvent = overrideCurrentEvent;
			_timeout = timeout;
		}

        /// <summary>
        /// Starts the Event.
        /// </summary>
        /// <param name="force">Whether to force start the event.</param>
        /// <param name="currentTime">The current time.</param>
		internal void Start(bool force = false, double currentTime = 0)
		{
			_eventStartTime = currentTime;
			_active = true;
			OnStarted();
			EventStarted?.Invoke();
		}

        /// <summary>
        /// Stops the Event.
        /// </summary>
        /// <param name="completedSuccessfully">Whether the event completed successfully.</param>
		internal void Stop(bool completedSuccessfully = false)
		{
			_success = _alwaysReturnSuccess ? true : completedSuccessfully;
			Debug.Log($"Event Finished: " + (_success ? "successful" : "failed"));
			_active = false;
			OnStopped();
			EventEnded?.Invoke(_success, Event, _returnData);
		}

        /// <summary>
        /// Updates the event.
        /// </summary>
        /// <param name="currentTime">The current time.</param>
		public virtual void Update(double currentTime) { OnUpdate(currentTime); }

        /// <summary>
        /// Processes any action logic.
        /// </summary>
        /// <param name="data">The action data.</param>
		public void Action(object data = null)
		{
			OnActioned(data);
		}

        /// <summary>
        /// Called when the event completes successfully.
        /// </summary>
		protected void OnCompleteEvent()
		{
			Stop(true);
		}

        /// <summary>
        /// Called when the event starts.
        /// </summary>
		protected virtual void OnStarted() { }

        /// <summary>
        /// Called when the event ends.
        /// </summary>
		protected virtual void OnStopped() { }

        /// <summary>
        /// Called when the event is actioned.
        /// </summary>
        /// <param name="data">The action data.</param>
		protected virtual void OnActioned(object data = null) { }

        /// <summary>
        /// Called every frame.
        /// </summary>
        /// <param name="currentTime">The current time.</param>
		protected void OnUpdate(double currentTime)
		{
			if (currentTime >= EventDuration + StartTime)
				Stop(false);
		}

	}
}
