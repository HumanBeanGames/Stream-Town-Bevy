using Buildings;
using Character;
using GameEventSystem;
using GameEventSystem.Events.Voting;
using System;
using System.Collections.Generic;
using TechTree;
using TownGoal.Data;
using Twitch;
using UnityEngine;
using UnityEngine.Events;
using World;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Utils;

namespace Processors
{
    /// <summary>
    /// Processor that manages game events and the event queue.
    /// Handles event scheduling, execution, and ruler voting mechanics.
    /// </summary>
    public class GameEventProcessor : MonoBehaviour, IInstaller, IProcessor
    {
        /// <summary>
        /// Player processor for accessing player data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// ScriptableObject containing game event settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private GameEventSettings _gameEventSettings;

        /// <summary>
        /// Time processor for accessing time data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TimeProcessor _timeProcessor;

        /// <summary>
        /// The debug processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Processors.DebugProcessor _debugProcessor;

        /// <summary>
        /// Runtime game event data.
        /// Assigned in InjectRuntimeData.
        /// </summary>
        private GameEventRuntimeData _gameEventRuntimeData;

        /// <summary>
        /// Gets the sorted queue of pending game events.
        /// </summary>
        public SortedSet<GameEvent> EventQueue => _gameEventRuntimeData.EventQueue;

        /// <summary>
        /// Gets or sets the time remaining until a ruler vote can start.
        /// </summary>
        public float TimeTillRulerVote
        {
            get => _gameEventRuntimeData.TimeTillRulerVote;
            set => _gameEventRuntimeData.TimeTillRulerVote = value;
        }

        /// <summary>
        /// Gets the minimum time in seconds before a new ruler vote can start.
        /// </summary>
        public float RulerVoteMinTime => _gameEventRuntimeData.RulerVoteMinTime;

        /// <summary>
        /// Gets or sets whether a new ruler vote can be started.
        /// </summary>
        public bool CanStartNewRulerVote
        {
            get => _gameEventRuntimeData.CanStartNewRulerVote;
            set => _gameEventRuntimeData.CanStartNewRulerVote = value;
        }

        /// <summary>
        /// Gets the currently active game event.
        /// </summary>
        public GameEvent CurrentEvent => _gameEventRuntimeData.CurrentEvent;

        /// <summary>
        /// Gets the falling fish visual effect particle system.
        /// </summary>
        public ParticleSystem FallingFishVFX => _gameEventRuntimeData.FallingFishVFX;

        /// <summary>
        /// Gets the fish god spawn transform.
        /// </summary>
        public Transform FishGodSpawn => _gameEventRuntimeData.FishGodSpawn;

        /// <summary>
        /// Invokes the resource sold event with the specified resource and amount.
        /// </summary>
        /// <param name="resource">The type of resource sold.</param>
        /// <param name="amount">The amount of resource sold.</param>
        public void InvokeResourceSold(Resource resource, int amount)
        {
            _gameEventRuntimeData.InvokeResourceSold(resource, amount);
        }

        /// <summary>
        /// Invokes the resource bought event with the specified resource and amount.
        /// </summary>
        /// <param name="resource">The type of resource bought.</param>
        /// <param name="amount">The amount of resource bought.</param>
        public void InvokeResourceBought(Resource resource, int amount)
        {
            _gameEventRuntimeData.InvokeResourceBought(resource, amount);
        }

        /// <summary>
        /// Invokes the resource gained event with the specified resource and amount.
        /// </summary>
        /// <param name="resource">The type of resource gained.</param>
        /// <param name="amount">The amount of resource gained.</param>
        public void InvokeResourceGained(Resource resource, int amount)
        {
            _gameEventRuntimeData.InvokeResourceGained(resource, amount);
        }

        public void InvokeBuildingBuilt(BuildingType buildingType)
        {
            _gameEventRuntimeData.InvokeBuildingBuilt(buildingType);
        }

        public void InvokeEnemyKilled(EnemyType enemyType)
        {
            _gameEventRuntimeData.InvokeEnemyKilled(enemyType);
        }

        /// <summary>
        /// Gets the resource gained event.
        /// </summary>
        public event Action<Resource, int> ResourceGained
        {
            add => _gameEventRuntimeData.ResourceGained += value;
            remove => _gameEventRuntimeData.ResourceGained -= value;
        }

        /// <summary>
        /// Gets the resource sold event.
        /// </summary>
        public event Action<Resource, int> ResourceSold
        {
            add => _gameEventRuntimeData.ResourceSold += value;
            remove => _gameEventRuntimeData.ResourceSold -= value;
        }

        /// <summary>
        /// Gets the resource bought event.
        /// </summary>
        public event Action<Resource, int> ResourceBought
        {
            add => _gameEventRuntimeData.ResourceBought += value;
            remove => _gameEventRuntimeData.ResourceBought -= value;
        }

        public event Action<BuildingType> BuildingBuilt
        {
            add => _gameEventRuntimeData.BuildingBuilt += value;
            remove => _gameEventRuntimeData.BuildingBuilt -= value;
        }

        public event Action<EnemyType> EnemyKilled
        {
            add => _gameEventRuntimeData.EnemyKilled += value;
            remove => _gameEventRuntimeData.EnemyKilled -= value;
        }

        /// <summary>
        /// Initializes the game event processor with the initial time until ruler vote.
        /// </summary>
        /// <param name="initialTimeUntilRulerVote">Initial time in seconds before first ruler vote.</param>
        public void Initialize(float initialTimeUntilRulerVote)
        {
            _gameEventRuntimeData.TimeTillRulerVote = initialTimeUntilRulerVote;
        }

        /// <summary>
        /// Sets the falling fish visual effect particle system.
        /// </summary>
        /// <param name="fallingFishVFX">The particle system to use.</param>
        public void SetFallingFishVFX(ParticleSystem fallingFishVFX)
        {
            _gameEventRuntimeData.FallingFishVFX = fallingFishVFX;
        }

        /// <summary>
        /// Sets the fish god spawn transform.
        /// </summary>
        /// <param name="fishGodSpawn">The transform to use as spawn point.</param>
        public void SetFishGodSpawn(Transform fishGodSpawn)
        {
            _gameEventRuntimeData.FishGodSpawn = fishGodSpawn;
        }

        /// <summary>
        /// Adds an event to the event queue.
        /// Does not add if an event of the same type already exists in the queue.
        /// </summary>
        /// <param name="gameEvent">The game event to add.</param>
        /// <returns>True if the event was added, false if it already exists in the queue.</returns>
        public bool AddEvent(GameEvent gameEvent)
        {
            // Check if an event of this type already exists in the queue
            if (EventTypeExistsInQueue(gameEvent.Event))
                return false;

            // Add the event to the sorted queue
            _gameEventRuntimeData.EventQueue.Add(gameEvent);

            // Log the event addition if logging is enabled
            if (_gameEventSettings.LogEvents)
                _debugProcessor.Log(DebugLogCategory.General, $"Game Event Added: '{gameEvent.Event}'");

            return true;
        }

        /// <summary>
        /// Creates a new event and stores it in the event queue.
        /// Does not create if an event of the same type already exists in the queue.
        /// </summary>
        /// <param name="delay">How long from the current time should the event start.</param>
        /// <param name="eventDuration">How long the event should last.</param>
        /// <param name="eventType">The type of event to create.</param>
        /// <param name="data">Optional data associated with the event.</param>
        /// <param name="overrideCurrentEvent">Whether this event should override the current event.</param>
        /// <param name="timeout">Optional timeout for the event to be valid.</param>
        /// <returns>True if the event was created, false if it already exists in the queue.</returns>
        public bool CreateEvent(double delay, double eventDuration, GameEvent.EventType eventType, object data = null, bool overrideCurrentEvent = false, double timeout = -1)
        {
            if (EventTypeExistsInQueue(eventType))
                return false;

            _gameEventRuntimeData.EventQueue.Add(new GameEvent(delay, eventDuration, eventType, data, overrideCurrentEvent, timeout));

            if (_gameEventSettings.LogEvents)
                _debugProcessor.Log(DebugLogCategory.General, $"Game Event Added: '{eventType}'");

            return true;
        }

        /// <summary>
        /// Removes all events from the current queue.
        /// Used to clear the queue when loading a new game or resetting state.
        /// </summary>
        public void DisposeEventsQueue()
        {
            _gameEventRuntimeData.EventQueue.Clear();
            // Log the queue disposal if logging is enabled
            if (_gameEventSettings.LogEvents)
                _debugProcessor.Log(DebugLogCategory.General, "Game Event Queue Disposed.");
        }

        /// <summary>
        /// Returns the next event in the queue without removing it.
        /// Used to preview the upcoming event.
        /// </summary>
        /// <returns>The next event in the queue, or null if the queue is empty.</returns>
        public GameEvent PeekNextEvent()
        {
            if (_gameEventRuntimeData.EventQueue.Count == 0)
                return null;

            // Return the minimum (earliest) event from the sorted set
            return _gameEventRuntimeData.EventQueue.Min;
        }

        /// <summary>
        /// Checks if an event of the specified type exists in the queue or is currently active.
        /// </summary>
        /// <param name="type">The event type to check for.</param>
        /// <returns>True if the event type exists in the queue or is current, false otherwise.</returns>
        public bool EventTypeExistsInQueue(GameEvent.EventType type)
        {
            // Check if the current event is of this type
            if (_gameEventRuntimeData.CurrentEvent != null && _gameEventRuntimeData.CurrentEvent.Event == type)
                return true;

            // Check if the queue is empty
            if (_gameEventRuntimeData.EventQueue.Count == 0)
                return false;

            // Iterate through all events in the queue
            IEnumerator<GameEvent> enumerator = _gameEventRuntimeData.EventQueue.GetEnumerator();

            while (enumerator.MoveNext())
            {
                if (enumerator.Current.Event == type)
                    return true;
            }

            return false;
        }

        /// <summary>
        /// Processes the event queue and updates the current event.
        /// Called every frame to check if events should start or end.
        /// </summary>
        public void ProcessEvents()
        {
    // Update the current event if one is active
    if (_gameEventRuntimeData.CurrentEvent != null)
        _gameEventRuntimeData.CurrentEvent.Update(_timeProcessor.WorldTimePassed);
            HandleRulerVoting();

            // Get the current world time
            double currentTime = WorldUtils.CurrentTime(_timeProcessor.WorldTimePassed);

            // Check if we should end the current event 
            if (_gameEventRuntimeData.CurrentEvent != null && _gameEventRuntimeData.CurrentEvent.EventDuration + _gameEventRuntimeData.CurrentEvent.StartTime <= currentTime)
                EndCurrentEvent();

            // Return if there are no events in the queue
            if (_gameEventRuntimeData.EventQueue.Count == 0)
                return;

            // Peek at the next event in the queue
            GameEvent nextEvent = PeekNextEvent();

            if (nextEvent == null)
                return;

            // Check if the next event should start (either overrides current or no current event)
            if ((_gameEventRuntimeData.CurrentEvent != null && nextEvent.OverrideCurrentEvent) || _gameEventRuntimeData.CurrentEvent == null)
            {
                // Check if the event's start time has been reached
                if (nextEvent.StartTime <= currentTime)
                {
                    // Check if the event has no timeout (always valid)
                    if (nextEvent.Timeout == -1)
                        StartNextEvent();
                    // Check if the event has timed out (remove from queue)
                    else if (nextEvent.Timeout + nextEvent.StartTime <= currentTime)
                    {
                        _gameEventRuntimeData.EventQueue.Remove(nextEvent);
                        _debugProcessor.Log(DebugLogCategory.General, $"Event Timed Out: '{nextEvent.Event}', removed from queue.");
                    }
                    // Check if the event is still within its valid window
                    else if (nextEvent.Timeout + nextEvent.StartTime >= currentTime)
                    {
                        StartNextEvent();
                    }
                }
            }
        }

        /// <summary>
        /// Ends the currently active event by calling its stop function.
        /// </summary>
        public void EndCurrentEvent()
        {
            // Mark that no event is currently active
            _gameEventRuntimeData.EventActive = false;

            // Return if there is no current event
            if (_gameEventRuntimeData.CurrentEvent == null)
                return;

            // Log the event stop if logging is enabled
            if (_gameEventSettings.LogEvents)
                _debugProcessor.Log(DebugLogCategory.General, $"Event Stopped: '{_gameEventRuntimeData.CurrentEvent.Event}'.");

            // Call the event's stop function
            _gameEventRuntimeData.CurrentEvent.Stop();

            // Clear the current event
            _gameEventRuntimeData.CurrentEvent = null;
        }

        /// <summary>
        /// Starts the next event in the queue.
        /// Ends the current event if one is active, then starts the next one.
        /// </summary>
        public void StartNextEvent()
        {
            // End the current event if one is active
            if (_gameEventRuntimeData.EventActive)
                EndCurrentEvent();

            // Get and remove the next event from the queue
            _gameEventRuntimeData.CurrentEvent = _gameEventRuntimeData.EventQueue.Min;
            _gameEventRuntimeData.EventQueue.Remove(_gameEventRuntimeData.EventQueue.Min);

            // Start the event with the current world time
            _gameEventRuntimeData.CurrentEvent.Start(currentTime: _timeProcessor.WorldTimePassed);
            _gameEventRuntimeData.EventActive = true;

            // Log the event start if logging is enabled
            if (_gameEventSettings.LogEvents)
                _debugProcessor.Log(DebugLogCategory.General, $"Event Started: '{_gameEventRuntimeData.CurrentEvent.Event}'.");

            // Subscribe to the event's ended callback
            _gameEventRuntimeData.CurrentEvent.EventEnded += OnCurrentEventEnded;
        }

        /// <summary>
        /// Callback invoked when the current event ends.
        /// Clears the current event reference.
        /// </summary>
        /// <param name="success">Whether the event completed successfully.</param>
        /// <param name="eventType">The type of event that ended.</param>
        /// <param name="finishedEvent">The event object that finished.</param>
        private void OnCurrentEventEnded(bool success, GameEvent.EventType eventType, object finishedEvent)
        {
            // Clear the current event reference
            _gameEventRuntimeData.CurrentEvent = null;
        }

        /// <summary>
        /// Handles the ruler voting timer and triggers the vote when ready.
        /// </summary>
        private void HandleRulerVoting()
        {
            // Only process if a new ruler vote can be started
            if (_gameEventRuntimeData.CanStartNewRulerVote)
            {
                // Decrement the time until the next ruler vote
                _gameEventRuntimeData.TimeTillRulerVote -= UnityEngine.Time.deltaTime;

                // Check if it's time to start the vote
                if (_gameEventRuntimeData.TimeTillRulerVote <= 0)
                {
                    // Mark that a vote can no longer be started
                    _gameEventRuntimeData.CanStartNewRulerVote = false;
                    StartKeepRulerVote();
                }
            }
        }

        /// <summary>
        /// Starts a vote to keep the current ruler.
        /// Creates a KeepKingVote event and adds it to the queue.
        /// </summary>
        public void StartKeepRulerVote()
        {
            _debugProcessor.Log(DebugLogCategory.General, "Keep ruler vote");
            // Create a new keep ruler vote event with 1 minute duration and 1 hour timeout
            KeepKingVote keepKingVote = new KeepKingVote(1, 120, timeout: 3600);

            // Add the event to the queue and subscribe to its completion
            if (AddEvent(keepKingVote))
                keepKingVote.EventEnded += OnKeepRulerVoteEnded;
        }

        /// <summary>
        /// Starts the timer for the next ruler vote.
        /// Resets the timer to the minimum time before a vote can occur.
        /// </summary>
        public void StartNewRulerVote()
        {
            // Allow a new ruler vote to be started
            _gameEventRuntimeData.CanStartNewRulerVote = true;
            // Reset the timer to the minimum time
            _gameEventRuntimeData.TimeTillRulerVote = _gameEventRuntimeData.RulerVoteMinTime;
        }

        /// <summary>
        /// Callback invoked when the keep ruler vote ends.
        /// Processes the vote results and either keeps or removes the ruler.
        /// </summary>
        /// <param name="success">Whether the vote completed successfully.</param>
        /// <param name="eventType">The type of event that ended.</param>
        /// <param name="data">The vote result data.</param>
        private void OnKeepRulerVoteEnded(bool success, GameEvent.EventType eventType, object data)
        {
            // Check if no votes were cast
            if (data == null)
            {
                _debugProcessor.Log(DebugLogCategory.General, "No Votes Were Cast");
                return;
            }

            // Cast the data to a VoteOption
            VoteOption option = data as VoteOption;

            // Process the vote result
            if (option.OptionName == "yes")
            {
                // Keep Ruler - no action needed
            }
            else
            {
                // Remove Ruler - clear the ruler and start a new ruler vote
                _playerProcessor.SetRuler(null);
                StartNewRulerVote();
            }
        }

        /// <summary>
        /// Callback invoked when the new ruler vote ends.
        /// Processes the vote results and sets the new ruler.
        /// </summary>
        /// <param name="success">Whether the vote completed successfully.</param>
        /// <param name="eventType">The type of event that ended.</param>
        /// <param name="data">The vote result data.</param>
        private void OnNewRulerVoteEnded(bool success, GameEvent.EventType eventType, object data)
        {
            // Check if no votes were cast
            if (data == null)
            {
                _debugProcessor.Log(DebugLogCategory.General, "No Votes Were Cast");
                return;
            }

            // Cast the data to a VoteOption
            VoteOption option = data as VoteOption;

            // Try to find the player by name (case-insensitive)
            if (_playerProcessor.PlayerExistsByNameToLower(option.OptionName, out int index))
            {
                // Set the winning player as the new ruler
                _playerProcessor.SetRuler(_playerProcessor.GetPlayer(index));
                _debugProcessor.Log(DebugLogCategory.General, $"Winner Was {option.OptionName}");
            }
            else
                _debugProcessor.Log(DebugLogCategory.General, "No Player Found");
        }

        /// <summary>
        /// Initializes the game event processor.
        /// Creates RuntimeData after all processors are confirmed ready.
        /// Initializes with a 30-second initial timer.
        /// </summary>
        public void Initialize()
        {
            if (_gameEventRuntimeData == null)
                throw new InvalidOperationException("GameEventProcessor: GameEventRuntimeData has not been installed.");

            Initialize(30);
        }

        /// <summary>
        /// Processes game event logic every frame.
        /// Called every frame by the Coordinator.
        /// </summary>
        public void Process()
        {
            ProcessEvents();
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // GameEventProcessor does not have scene-specific settings to refresh
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
            if (_gameEventRuntimeData != null)
                throw new InvalidOperationException("GameEventProcessor: GameEventRuntimeData has already been installed.");

            _gameEventRuntimeData = new GameEventRuntimeData();
            containerBuilder.AddSingleton(_gameEventRuntimeData);
        }
    }
}
