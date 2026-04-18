using GameEventSystem;
using System;
using System.Collections.Generic;
using UnityEngine;
using World;
using Buildings;
using GameResources;
using Enemies;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime game event state for the game.
	/// Manages event queue, current event status, ruler voting, and game event-related visual effects.
	/// </summary>
	public class GameEventRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Minimum time in seconds before a new ruler vote can be started.
		/// Prevents ruler votes from happening too frequently.
		/// </summary>
		private const float _RULER_VOTE_MIN_TIME = 3600;

		/// <summary>
		/// Time remaining until a new ruler vote can be started.
		/// Decrements over time and when it reaches zero, a new ruler vote can begin.
		/// </summary>
		private float _timeUntilRulerVote;

		/// <summary>
		/// Sorted queue of game events waiting to be triggered.
		/// Events are sorted by their start time for proper sequencing.
		/// </summary>
		private SortedSet<GameEvent> _eventQueue = new SortedSet<GameEvent>(new SortGameEventStartTime());

		/// <summary>
		/// The currently active game event.
		/// Null when no event is active.
		/// </summary>
		private GameEvent _currentEvent = null;

		/// <summary>
		/// Whether a game event is currently active.
		/// Set to true when an event starts, false when it ends.
		/// </summary>
		private bool _eventActive = false;

		/// <summary>
		/// Whether a new ruler vote can be started.
		/// Set to true when the minimum time has elapsed since the last vote.
		/// </summary>
		private bool _canStartNewRulerVote = false;

		/// <summary>
		/// Particle system for the falling fish visual effect.
		/// Used during specific game events to create visual flair.
		/// </summary>
		private ParticleSystem _fallingFishVFX;

		/// <summary>
		/// Transform for the fish god spawn location.
		/// Used during specific game events to spawn special entities.
		/// </summary>
		private Transform _fishGodSpawn;

		/// <summary>
		/// Event fired when a building is constructed.
		/// Passes the type of building that was built.
		/// </summary>
		public event Action<BuildingType> BuildingBuilt;

		/// <summary>
		/// Event fired when resources are gained.
		/// Passes the resource type and amount gained.
		/// </summary>
		public event Action<Resource, int> ResourceGained;

		/// <summary>
		/// Event fired when an enemy is killed.
		/// Passes the type of enemy that was killed.
		/// </summary>
		public event Action<EnemyType> EnemyKilled;

		/// <summary>
		/// Event fired when resources are sold.
		/// Passes the resource type and amount sold.
		/// </summary>
		public event Action<Resource, int> ResourceSold;

		/// <summary>
		/// Event fired when resources are bought.
		/// Passes the resource type and amount bought.
		/// </summary>
		public event Action<Resource, int> ResourceBought;

		/// <summary>
		/// Gets or sets the currently active game event.
		/// </summary>
		public GameEvent CurrentEvent
		{
			get { return _currentEvent; }
			set { _currentEvent = value; }
		}

		/// <summary>
		/// Gets or sets the time remaining until a ruler vote can start.
		/// </summary>
		public float TimeTillRulerVote
		{
			get { return _timeUntilRulerVote; }
			set { _timeUntilRulerVote = value;}
		}

		/// <summary>
		/// Gets or sets whether a new ruler vote can be started.
		/// </summary>
		public bool CanStartNewRulerVote
		{
			get => _canStartNewRulerVote;
			set => _canStartNewRulerVote = value;
		}

		/// <summary>
		/// Gets or sets the falling fish particle system.
		/// </summary>
		public ParticleSystem FallingFishVFX
		{
			get { return _fallingFishVFX; }
			set { _fallingFishVFX = value; }
		}

		/// <summary>
		/// Gets or sets the fish god spawn transform.
		/// </summary>
		public Transform FishGodSpawn
		{
			get { return _fishGodSpawn; }
			set { _fishGodSpawn = value; }
		}

		/// <summary>
		/// Gets the sorted queue of pending game events.
		/// </summary>
		public SortedSet<GameEvent> EventQueue => _eventQueue;

		/// <summary>
		/// Gets or sets whether a game event is currently active.
		/// </summary>
		public bool EventActive
		{
			get { return _eventActive; }
			set { _eventActive = value; }
		}

		/// <summary>
		/// Gets the minimum time in seconds before a new ruler vote can start.
		/// </summary>
		public float RulerVoteMinTime => _RULER_VOTE_MIN_TIME;

		public void InvokeBuildingBuilt(BuildingType buildingType)
		{
			BuildingBuilt?.Invoke(buildingType);
		}

		public void InvokeResourceGained(Resource resource, int amount)
		{
			ResourceGained?.Invoke(resource, amount);
		}

		public void InvokeEnemyKilled(EnemyType enemyType)
		{
			EnemyKilled?.Invoke(enemyType);
		}

		public void InvokeResourceSold(Resource resource, int amount)
		{
			ResourceSold?.Invoke(resource, amount);
		}

		public void InvokeResourceBought(Resource resource, int amount)
		{
			ResourceBought?.Invoke(resource, amount);
		}
	}
}
