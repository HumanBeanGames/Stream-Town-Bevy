using Character;
using GUIDSystem;
using Processors;
using System;
using System.Collections.Generic;
using Target;
using Units;
using UnityEngine;
using UnityEngine.Events;
using Utils;
using Reflex.Attributes;
using GridSystem.Partitioning;

namespace Sensors
{
	/// <summary>
	/// A sensor that finds all targets based on a TargetType mask.
	/// </summary>
	public class TargetSensor : SensorBase
	{
        /// <summary>
        /// The target mask.
        /// </summary>
		[SerializeField]
		private TargetMask _targetMask = TargetMask.Player;

        /// <summary>
        /// The current target.
        /// </summary>
		[SerializeField]
		private Targetable _currentTarget = null;

        /// <summary>
        /// Event invoked when the target changes.
        /// </summary>
		[SerializeField]
		private UnityEvent _onTargetChange;

        /// <summary>
        /// Whether to use station targets.
        /// </summary>
		[SerializeField]
		private bool _useStationTargets = true;

        /// <summary>
        /// Whether to attack the attacker.
        /// </summary>
		[SerializeField]
		private bool _attackAttacker = false;

        /// <summary>
        /// The target processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TargetProcessor _targetProcessor;

        /// <summary>
        /// The cell space partitioning. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private CellSpacePartitioning _cellSpacePartition;

        /// <summary>
        /// The debug processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

        /// <summary>
        /// Gets the current target.
        /// </summary>
		public Targetable CurrentTarget => _currentTarget;

        /// <summary>
        /// The previous target.
        /// </summary>
		private Targetable _previousTarget = null;

        /// <summary>
        /// The station sensor.
        /// </summary>
		private StationSensor _stationSensor = null;

        /// <summary>
        /// The GUID component.
        /// </summary>
		private GUIDComponent _gUIDComponent = null;

        /// <summary>
        /// The target search range.
        /// </summary>
		[SerializeField]
		private float _targetSearchRange = 100f;

        /// <summary>
        /// Gets or sets whether to update the target.
        /// </summary>
		public bool UpdateTarget { get; set; }

        /// <summary>
        /// Gets whether the sensor has a target.
        /// </summary>
		public bool HasTarget => _currentTarget == null ? false : true;

        /// <summary>
        /// Gets the distance to the current target.
        /// </summary>
		public float DistanceToTarget => _currentTarget == null ? float.MaxValue : Vector3.Distance(transform.position, _currentTarget.transform.position);

        /// <summary>
        /// Gets whether to use station targets.
        /// </summary>
		public bool UseStationTargets => _useStationTargets;

        /// <summary>
        /// Gets the GUID component.
        /// </summary>
		public GUIDComponent GUIDComponent => _gUIDComponent;

        /// <summary>
        /// Gets the target search range.
        /// </summary>
        public float TargetSearchRange => _targetSearchRange;

		/// <summary>
		/// Returns the Target Mask.
		/// </summary>
		public TargetMask TargetMask
		{
			get { return _targetMask; }
			set { SetTargetMask(value); }
		}

        /// <summary>
        /// Called every frame by the sensor processor.
        /// </summary>
		public override void STUpdate()
		{
			base.STUpdate();

			// If we should be updated the target, check that we don't have a target or need a new target.
			if (HasTarget && !_currentTarget.gameObject.activeInHierarchy)
				_currentTarget = null;

			if (_useStationTargets && ((UpdateTarget && _currentTarget == null && _stationSensor.HasStation) || (_currentTarget != null && !_currentTarget.gameObject.activeInHierarchy)))
				GetTarget();

			// Check if the current target is the wrong target type and get a new target.
			else if (_currentTarget != null && !IsTargetTypeMatch(_currentTarget.TargetType))
				GetTarget();

			else if (_currentTarget == null)
				GetTarget();
		}

		/// <summary>
		/// Marks the current target as bad, informing the station to remove this from the valid target's list.
		/// </summary>
		public void MarkCurrentTargetBad()
		{
			if (_stationSensor == null || _currentTarget == null || _stationSensor.CurrentStation == null)
				return;

			_stationSensor.CurrentStation.MarkTargetInvald(_currentTarget);
			ClearTarget();
		}

		/// <summary>
		/// Clears the current target to null.
		/// </summary>
		public void ClearTarget()
		{
			UpdateTarget = true;
			_currentTarget = null;
			OnTargetChanged();
		}

		/// <summary>
		/// Attempts to set the current target.
		/// </summary>
		/// <param name="target">The target to set.</param>
		/// <returns>True if successful.</returns>
		public bool TrySetTarget(Targetable target)
		{
			_currentTarget = target;
			OnTargetChanged();
			return true;
		}

        /// <summary>
        /// Attempts to acquire the nearest target.
        /// </summary>
        /// <returns>True if successful.</returns>
        public bool TryAcquireNearestTarget()
        {
            Targetable nearestTarget = GetNearestTarget();
            _currentTarget = nearestTarget;
            OnTargetChanged();
            return nearestTarget != null;
        }

        /// <summary>
        /// Attempts to set the current target for a player.
        /// </summary>
        /// <param name="target">The target to set.</param>
        /// <param name="player">The player.</param>
        /// <returns>True if successful.</returns>
		public bool TrySetTarget(Targetable target, Player player)
		{
			if (player.TargetSensor.IsTargetTypeMatch(target.TargetType))
			{
				_currentTarget = target;
				OnTargetChanged();
				_debugProcessor.Log(DebugLogCategory.Targetable, $"Set {player.RoleHandler.CurrentRole}'s target to {target.TargetType}");

				return true;
			}

			_debugProcessor.Log(DebugLogCategory.Targetable, $"Can't set {player.RoleHandler.CurrentRole}'s target to {target.TargetType}");
			return false;
		}

        /// <summary>
        /// Initializes the target sensor.
        /// </summary>
		protected override void Init()
		{
			base.Init();
			_stationSensor = GetComponent<StationSensor>();
			_gUIDComponent = GetComponent<GUIDComponent>();
		}

		/// <summary>
		/// Attempts to get a new target from the station.
		/// </summary>
		private void GetTarget()
		{
			if (!CurrentTargetValid())
			{
				if (ShouldUseNearestTargeting())
				{
					_currentTarget = GetNearestTarget();
				}
				else if (_useStationTargets)
				{
					if (_stationSensor.CurrentStation != null)
						_stationSensor.CurrentStation.GetBestScoredTarget(transform.position, _targetMask, ref _currentTarget);
				}
				else
				{
					_currentTarget = GetNearestTarget();
				}
			}

			OnTargetChanged();
		}

        /// <summary>
        /// Gets the nearest target using cell space partitioning.
        /// </summary>
		private Targetable GetNearestTarget()
		{
			List<Targetable> validTargets = new List<Targetable>();

			_cellSpacePartition.GetTargetablesInRange(GetExpandedTargetMask(), transform.position, _targetSearchRange, ref validTargets);

			// Get closest target
			float closestDistSqr = float.MaxValue;
			Targetable closestTarget = null;
			float distance = 0;

			for (int i = 0; i < validTargets.Count; i++)
			{
				if (!validTargets[i].gameObject.activeInHierarchy)
					continue;

				distance = Vector3.SqrMagnitude(validTargets[i].transform.position - transform.position);

				if (distance < closestDistSqr)
				{
					closestDistSqr = distance;
					closestTarget = validTargets[i];
				}
			}

			if (closestTarget != null)
			{
				return closestTarget;
			}

			return null;
		}

		private bool ShouldUseNearestTargeting()
		{
			return _targetMask.HasFlag(TargetMask.Enemy)
				|| _targetMask.HasFlag(TargetMask.Boss)
				|| _targetMask.HasFlag(TargetMask.Player)
				|| _targetMask.HasFlag(TargetMask.InjuredPlayer)
				|| _targetMask.HasFlag(TargetMask.Building)
				|| _targetMask.HasFlag(TargetMask.DamagedBuilding);
		}

		private TargetMask GetExpandedTargetMask()
		{
			TargetMask expandedMask = _targetMask;

			if (_targetMask.HasFlag(TargetMask.Player))
				expandedMask |= TargetMask.InjuredPlayer;

			if (_targetMask.HasFlag(TargetMask.Building))
				expandedMask |= TargetMask.DamagedBuilding;

			if (_targetMask.HasFlag(TargetMask.Enemy))
				expandedMask |= TargetMask.Boss;

			return expandedMask;
		}

		public bool IsTargetTypeMatch(TargetMask targetType)
		{
			return GetExpandedTargetMask().HasFlag(targetType);
		}

		/// <summary>
		/// Returns true if the current target is a valid target.
		/// </summary>
		/// <returns>True if valid, false otherwise.</returns>
		private bool CurrentTargetValid()
		{
			// If the target is not null, enabled and the correct flag, then the target is valid.
			if (_currentTarget != null && _currentTarget.gameObject.activeInHierarchy && IsTargetTypeMatch(_currentTarget.TargetType))
				return true;
			else
				return false;
		}

		/// <summary>
		/// Sets the current Target Mask of the unit.
		/// </summary>
		/// <param name="type">The target mask type.</param>
		private void SetTargetMask(TargetMask type)
		{
			_targetMask = type;

			ClearTarget();
		}

		/// <summary>
		/// Called when the current target was changed.
		/// </summary>
		private void OnTargetChanged()
		{
			if (_currentTarget != _previousTarget)
			{
				if (_previousTarget != null)
					_previousTarget.UnassignFromTarget();

				_onTargetChange.Invoke();
				_previousTarget = _currentTarget;

				if (_currentTarget != null)
					_currentTarget.AssignToTarget();
			}
		}

        /// <summary>
        /// Called when attacked.
        /// </summary>
        /// <param name="target">The attacker.</param>
		private void OnAttacked(Targetable target)
		{
			if (!_attackAttacker || target == null)
				return;

			// If its a valid target for our target mask, focus it.
			if (IsTargetTypeMatch(target.TargetType))
				_currentTarget = target;
		}

        /// <summary>
        /// Clears the target on disable.
        /// </summary>
		private void OnDisable()
		{
			ClearTarget();
		}

        /// <summary>
        /// Enables target updates and subscribes to damage events on start.
        /// </summary>
		private void Start()
		{
			UpdateTarget = true;

			if (TryGetComponent(out HealthHandler h))
			{
				h.OnTookDamage += OnAttacked;
			}
		}
	}
}
