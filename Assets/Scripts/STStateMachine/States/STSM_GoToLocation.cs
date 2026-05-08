using Pathfinding;
using Processors;
using Reflex.Attributes;
using Sensors;
using UnityEngine;

namespace STStateMachine.States
{
	/// <summary>
	/// A simple state action that allows a unit to move to a new location or target.
	/// </summary>
	public class STSM_GoToLocation : STStateBase
	{
		/// <summary>
		/// Minimum distance a unit has to move within the check window to count as meaningful motion.
		/// </summary>
		private const float MINIMUM_STUCK_MOVEMENT_CHECK_SQR = 0.04f;

		private const float MINIMUM_STUCK_PROGRESS_DISTANCE = 0.35f;

		/// <summary>
		/// Max Number of consecutive stuck true checks until the unit is dealt with
		/// </summary>
		private const int MAX_CONSECUTIVE_COUNT = 5;

		/// <summary>
		/// How often the unit will poll a stuck check
		/// </summary>
		private const float STUCK_DISTANCE_CHECK_RATE = 1.0f;

		/// <summary>
		/// Default satisfaction distance if there is no given distance satisfaction.
		/// Stops the unit from walking inside of the target.
		/// </summary>
		private const float ZERO_VECTOR_SATISFACTION_DISTANCE = 36.0f;

		[SerializeField]
		private float _distanceSatisfaction;
		private Vector3 _targetPosition;
		private Transform _targetTransform;
		private GameObject _temporaryTarget; // For temporary transform targets
		private AIPath _aiPath;
		private STStateBase _nextState;
		private Vector3 _destination;
		private TargetSensor _targetSensor;
		private float _stuckCheckTimer;
		private Vector3 _lastStuckCheckPos;
		private bool _stuck = false;
		private int _stuckConsecutiveCount = 0;
		private float _prevSlowDownDistance = 0;
		private Vector3 _onEnabledLocation = Vector3.zero;
		private float _lastRemainingDistance = float.PositiveInfinity;
		private Vector3 _lastDestination = Vector3.zero;

		[Inject] private Processors.DebugProcessor _debugProcessor;

		public bool UsePosition { get; set; }

		/// <summary>
		/// Sets the next state the unit will enter once they've reached their location.
		/// </summary>
		/// <param name="state"></param>
		public void SetNextState(STStateBase state)
		{
			_nextState = state;
		}

		/// <summary>
		/// Sets the distance in which the unit will be satisfied and consider their destination as "reached".
		/// </summary>
		/// <param name="distance"></param>
		public void SetDistanceSatisfaction(float distance)
		{
			_distanceSatisfaction = distance;
		}

		/// <summary>
		/// Sets the target position of the unit.
		/// </summary>
		/// <param name="position"></param>
		public void SetTargetPosition(Vector3 position)
		{
			SetTargetPosition(position, 0f);
		}

		/// <summary>
		/// Sets the target from a position with action range, creating a temporary transform for transform-based targeting.
		/// </summary>
		/// <param name="position">Position of the target object</param>
		/// <param name="actionRange">Desired distance from target object</param>
		public void SetTargetFromPosition(Vector3 position, float actionRange)
		{
			_debugProcessor.Log(DebugLogCategory.GoToLocation, $"SetTargetFromPosition called - position: {position}, actionRange: {actionRange}");

			// Clean up previous temporary target if exists
			if (_temporaryTarget != null)
			{
				Destroy(_temporaryTarget);
			}

			// Create a temporary GameObject with a Transform at the position
			_temporaryTarget = new GameObject("TempTarget");
			_temporaryTarget.transform.position = position;

			// Use the new SetTarget overload with action range
			SetTarget(_temporaryTarget.transform, actionRange);
		}

		/// <summary>
		/// Sets the target position of the unit with an optional action range.
		/// Selects the closest node to the character and adjusts distance satisfaction based on action range.
		/// </summary>
		/// <param name="targetPosition">Position of the target object (resource/building/enemy)</param>
		/// <param name="actionRange">Desired distance from target object</param>
		public void SetTargetPosition(Vector3 targetPosition, float actionRange)
		{
			// Get the node at the target position
			var targetNode = AstarPath.active.GetNearest(targetPosition, NNConstraint.Default).node;
			if (targetNode == null)
			{
				// Fallback to simple nearest if node not found
				_targetPosition = AstarPath.active.GetNearest(targetPosition, NNConstraint.Default).position;
				_aiPath.destination = _targetPosition;
				// Use default satisfaction if actionRange is 0
				_distanceSatisfaction = actionRange == 0 ? 1f : actionRange * actionRange;
				return;
			}

			// Check the 8 surrounding nodes to find the closest one to the character
			Vector3 targetNodePosition = (Vector3)targetNode.position;
			Vector3 closestNodePosition = targetNodePosition;
			float closestDistanceSqr = Vector3.SqrMagnitude(targetNodePosition - transform.position);

			int[] neighborOffsets = { -1, 0, 1 };
			foreach (int xOffset in neighborOffsets)
			{
				foreach (int zOffset in neighborOffsets)
				{
					if (xOffset == 0 && zOffset == 0)
						continue; // Skip the center node (already checked)

					Vector3 neighborPos = targetPosition + new Vector3(xOffset, 0, zOffset);
					var neighborNode = AstarPath.active.GetNearest(neighborPos, NNConstraint.Default).node;
					if (neighborNode != null && neighborNode.Walkable)
					{
						Vector3 neighborNodePosition = (Vector3)neighborNode.position;
						float distanceSqr = Vector3.SqrMagnitude(neighborNodePosition - transform.position);
						if (distanceSqr < closestDistanceSqr)
						{
							closestDistanceSqr = distanceSqr;
							closestNodePosition = neighborNodePosition;
						}
					}
				}
			}

			_targetPosition = closestNodePosition;
			_aiPath.destination = _targetPosition;

			// Adjust distance satisfaction based on action range
			// Use default satisfaction of 1f if actionRange is 0 (for idle wandering)
			_distanceSatisfaction = actionRange == 0 ? 1f : actionRange * actionRange; // Squared for distance check

			// Debug.Log($"[GoToLocation] SetTargetPosition - targetPos: {targetPosition}, actionRange: {actionRange}, selectedNode: {closestNodePosition}, distanceSatisfaction: {_distanceSatisfaction}");
		}

		/// <summary>
		/// Sets the target to travel to.
		/// </summary>
		/// <param name="target"></param>
		public void SetTarget(Transform target)
		{
			SetTarget(target, 0f);
		}

		/// <summary>
		/// Sets the target to travel to with an optional action range.
		/// Finds the closest node to the character around the target's position.
		/// Uses the closest point within the selected node to the target object.
		/// </summary>
		/// <param name="target"></param>
		/// <param name="actionRange">Desired distance from target object</param>
		public void SetTarget(Transform target, float actionRange)
		{
			_targetTransform = target;

			_debugProcessor.Log(DebugLogCategory.GoToLocation, $"SetTarget called - targetPos: {target.position}, actionRange: {actionRange}");

			// Get the node at the target position
			var targetNode = AstarPath.active.GetNearest(target.position, NNConstraint.Default).node;
			if (targetNode == null)
			{
				// Fallback to simple nearest if node not found
				Vector3 nearestPosition = AstarPath.active.GetNearest(target.position, NNConstraint.Default).position;
				_aiPath.destination = nearestPosition;
				_distanceSatisfaction = actionRange * actionRange;
				_debugProcessor.Log(DebugLogCategory.GoToLocation, $"SetTarget fallback - distanceSatisfaction: {_distanceSatisfaction}");
				return;
			}

			// Check the 8 surrounding nodes to find the closest one to the character
			Vector3 targetNodePosition = (Vector3)targetNode.position;
			Vector3 closestNodePosition = targetNodePosition;
			float closestDistanceSqr = Vector3.SqrMagnitude(targetNodePosition - transform.position);

			int[] neighborOffsets = { -1, 0, 1 };
			foreach (int xOffset in neighborOffsets)
			{
				foreach (int zOffset in neighborOffsets)
				{
					if (xOffset == 0 && zOffset == 0)
						continue; // Skip the center node (already checked)

					Vector3 neighborPos = target.position + new Vector3(xOffset, 0, zOffset);
					var neighborNode = AstarPath.active.GetNearest(neighborPos, NNConstraint.Default).node;
					if (neighborNode != null && neighborNode.Walkable)
					{
						Vector3 neighborNodePosition = (Vector3)neighborNode.position;
						float distanceSqr = Vector3.SqrMagnitude(neighborNodePosition - transform.position);
						if (distanceSqr < closestDistanceSqr)
						{
							closestDistanceSqr = distanceSqr;
							closestNodePosition = neighborNodePosition;
						}
					}
				}
			}

			// Instead of using the node center, find the closest point within the node's bounds to the target object
			// Node size is 1 unit, so bounds are [nodeX - 0.5, nodeX + 0.5] x [nodeZ - 0.5, nodeZ + 0.5]
			float nodeHalfSize = 0.5f;
			Vector3 closestPointInNode = closestNodePosition;
			closestPointInNode.x = Mathf.Clamp(target.position.x, closestNodePosition.x - nodeHalfSize, closestNodePosition.x + nodeHalfSize);
			closestPointInNode.z = Mathf.Clamp(target.position.z, closestNodePosition.z - nodeHalfSize, closestNodePosition.z + nodeHalfSize);
			// Keep the Y position from the node center (ground height)
			closestPointInNode.y = closestNodePosition.y;

			_aiPath.destination = closestPointInNode;
			_distanceSatisfaction = actionRange * actionRange; // Squared for distance check

			_debugProcessor.Log(DebugLogCategory.GoToLocation, $"SetTarget - targetPos: {target.position}, actionRange: {actionRange}, selectedNode: {closestNodePosition}, closestPointInNode: {closestPointInNode}, distanceSatisfaction: {_distanceSatisfaction}");
		}

		protected override void OnInit()
		{
			base.OnInit();
			_aiPath = GetComponent<AIPath>();
			_prevSlowDownDistance = _aiPath.slowdownDistance;
			_targetSensor = GetComponent<TargetSensor>();

			// Enable gravity on AIPath to prevent floating when walking down slopes
			_aiPath.gravity = new Vector3(0, -9.81f, 0);
		}

		public override void OnEnter()
		{
			_aiPath.enabled = true;
			_distanceSatisfaction += (UsePosition && _targetPosition == Vector3.zero ? ZERO_VECTOR_SATISFACTION_DISTANCE : 0);
			
			// Set AIPath endReachedDistance to match our distance satisfaction for precise stopping
			_aiPath.endReachedDistance = Mathf.Sqrt(_distanceSatisfaction);
			
			// Restore slowdown distance to allow precise stopping (prevents overshooting)
			_aiPath.slowdownDistance = _prevSlowDownDistance;

			_debugProcessor.Log(DebugLogCategory.GoToLocation, $"OnEnter - endReachedDistance: {_aiPath.endReachedDistance}, slowdownDistance: {_aiPath.slowdownDistance}, distanceSatisfaction: {_distanceSatisfaction}, nextState: {_nextState?.GetType().Name}");

			ResetStuckTracking();

			//Check path is possible to point otherwise mark it as bad
			if (!UsePosition)
			{
				GraphNode a = AstarPath.active.GetNearest(transform.position, NNConstraint.Default).node;
				GraphNode b = AstarPath.active.GetNearest(_targetTransform.position, NNConstraint.Default).node;

				if (!PathUtilities.IsPathPossible(a, b) || a == null || b == null)
				{
					_debugProcessor.Log(DebugLogCategory.GoToLocation, $"Path wasn't possible from {transform.gameObject.name} to {_targetTransform.gameObject.name}");
					_targetSensor.MarkCurrentTargetBad();

					b = AstarPath.active.GetNearest(Vector3.zero, NNConstraint.Default).node;
					if (!PathUtilities.IsPathPossible(a, b) || a == null || b == null)
					{
						transform.position = _onEnabledLocation;
					}

					_stateMachine.RequestStateChange("Idle");
				}
			}
		}

		public override void OnUpdate()
		{
			base.OnUpdate();

			// If we are using the target's position and the target is null, go to the idle state.
			if (!UsePosition && _targetTransform == null)
			{
				((STSM_Idle)_stateMachine.GetStateByName("Idle")).NewPositionOnEnter = true;
				_stateMachine.RequestStateChange("Idle");
				return;
			}

			// If we are using the target's position, set destination to target, otherwise set it to specified position.
			_destination = UsePosition ? _targetPosition : _targetTransform.position;
			_aiPath.destination = _destination;

			if (_nextState is STSM_Idle && _targetSensor != null && _targetSensor.CurrentTarget != null)
			{
				_stateMachine.RequestStateChange(_nextState, true);
				return;
			}

			// Check if the unit is within range of the target location, and switch to the next state.
			// Ignore Y component in distance check to account for height differences
			Vector3 horizontalDiff = _aiPath.destination - transform.position;
			horizontalDiff.y = 0;
			float sqr = Vector3.SqrMagnitude(horizontalDiff);
			
			if (sqr <= _distanceSatisfaction)
			{
				_debugProcessor.Log(DebugLogCategory.GoToLocation, $"Distance satisfied - currentDist: {Mathf.Sqrt(sqr)}, required: {Mathf.Sqrt(_distanceSatisfaction)}, nextState: {_nextState?.GetType().Name}");
				if (_nextState != null)
				{
					_debugProcessor.Log(DebugLogCategory.GoToLocation, $"Transitioning to next state: {_nextState.GetType().Name}");
					_stateMachine.RequestStateChange(_nextState, true);
					ResetStuckTracking();
					return; // Exit early to prevent repeated checks
				}
				else
				{
					_debugProcessor.Log(DebugLogCategory.GoToLocation, $"No next state, going to Idle");
					_stateMachine.RequestStateChange("Idle");
					return; // Exit early to prevent repeated checks
				}
			}
			// If path is not possible or pathfinding can't reach the target, bail out early if within grid node distance
			// Only apply this for transform-based targeting without a temporary target (idle wandering), not resource gathering
			else if (!UsePosition && !_aiPath.pathPending && _aiPath.remainingDistance >= 0 && _aiPath.remainingDistance < 2f && _temporaryTarget == null)
			{
				// Pathfinding can't reach the target but we're within grid node distance (2 units)
				_debugProcessor.Log(DebugLogCategory.GoToLocation, $"Path unreachable but close, transitioning to next state");
				if (_nextState != null)
				{
					_stateMachine.RequestStateChange(_nextState, true);
					ResetStuckTracking();
					return; // Exit early to prevent repeated bail-out checks
				}
				else
				{
					_stateMachine.RequestStateChange("Idle");
					return; // Exit early to prevent repeated bail-out checks
				}
			}

			// Check if the unit is stuck.
			StuckCheck();
		}

		public override void OnExit()
		{
			base.OnExit();
			_targetTransform = null;
			_distanceSatisfaction = 1;
			_prevSlowDownDistance = _aiPath.slowdownDistance;
			_nextState = null;

			// Clean up temporary target if exists
			if (_temporaryTarget != null)
			{
				Destroy(_temporaryTarget);
				_temporaryTarget = null;
			}
		}

		/// <summary>
		/// Checks if the unit is stuck and resets their position.
		/// </summary>
		// TODO:: Fix the detection and resolution. Not working correctly.
		private void StuckCheck()
		{
			if (_aiPath == null)
				return;

			if (_aiPath.pathPending)
			{
				ResetStuckTracking();
				return;
			}

			Vector3 horizontalToDestination = _aiPath.destination - transform.position;
			horizontalToDestination.y = 0f;
			if (Vector3.SqrMagnitude(horizontalToDestination) <= _distanceSatisfaction)
			{
				ResetStuckTracking();
				return;
			}

			if ((_aiPath.destination - _lastDestination).sqrMagnitude > 0.25f)
			{
				ResetStuckTracking();
				return;
			}

			_stuckCheckTimer += Time.deltaTime;
			if (_stuckCheckTimer < STUCK_DISTANCE_CHECK_RATE)
				return;

			float remainingDistance = GetRemainingDistance();
			Vector3 moved = transform.position - _lastStuckCheckPos;
			moved.y = 0f;
			float progressDistance = _lastRemainingDistance - remainingDistance;
			bool madeProgress = moved.sqrMagnitude >= MINIMUM_STUCK_MOVEMENT_CHECK_SQR || progressDistance >= MINIMUM_STUCK_PROGRESS_DISTANCE;

			if (madeProgress)
			{
				_stuckConsecutiveCount = 0;
				_stuck = false;
			}
			else
			{
				_stuck = true;
				_stuckConsecutiveCount++;
			}

			_stuckCheckTimer = 0f;
			_lastStuckCheckPos = transform.position;
			_lastRemainingDistance = remainingDistance;
			_lastDestination = _aiPath.destination;

			// Check if the unit has been stuck through multiple checks and reset their position.
			if (_stuckConsecutiveCount > MAX_CONSECUTIVE_COUNT)
			{
#if UNITY_EDITOR
				Debug.DrawLine(transform.position, transform.position + Vector3.up * 20, Color.red);
#endif
				// Raycast down to find ground height at the teleport location
				Vector3 teleportPos = _onEnabledLocation;
				if (Physics.Raycast(teleportPos + Vector3.up * 100, Vector3.down, out RaycastHit hit, 200))
				{
					teleportPos.y = hit.point.y;
				}
				transform.position = teleportPos;
				ResetStuckTracking();
				_stateMachine.RequestStateChange("Idle");
			}
		}

		private float GetRemainingDistance()
		{
			if (_aiPath != null && !_aiPath.pathPending && _aiPath.remainingDistance >= 0f)
				return _aiPath.remainingDistance;

			Vector3 horizontalToDestination = _aiPath.destination - transform.position;
			horizontalToDestination.y = 0f;
			return horizontalToDestination.magnitude;
		}

		private void ResetStuckTracking()
		{
			_stuckCheckTimer = 0f;
			_lastStuckCheckPos = transform.position;
			_lastRemainingDistance = _aiPath != null ? GetRemainingDistance() : 0f;
			_lastDestination = _aiPath != null ? _aiPath.destination : Vector3.zero;
			_stuckConsecutiveCount = 0;
			_stuck = false;
		}

		// Unity Functions.
		private void OnDrawGizmos()
		{
			if (_stuck)
			{
				Debug.DrawLine(transform.position, transform.position + (Vector3.up * 5), Color.black);
			}
		}

		private void OnEnable()
		{
			_onEnabledLocation = transform.position;
		}
	}
}
