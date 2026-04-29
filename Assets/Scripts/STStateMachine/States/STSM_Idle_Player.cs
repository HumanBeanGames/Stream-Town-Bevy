using System.Collections.Generic;
using Character;
using Processors;
using Reflex.Attributes;
using STStateMachine.Helpers;
using UnityEngine;
using Utils;

namespace STStateMachine.States
{
	/// <summary>
	/// Idle state logic for Player characters.
	/// </summary>
	public class STSM_Idle_Player : STSM_Idle
	{
		private RoleHandler _roleHandler;
		private PlayerInventory _inventory;
		[Inject] private TownResourceProcessor _townResourceProcessor;
		[Inject] private ResourceProcessor _resourceProcessor;

		// State References.
		private STSM_Action_GatherResource _gatherResourceAction;
		private STSM_Action_PlayerAttack _attackAction;
		private STSM_Action_Heal _healAction;
		private STSM_HelperDeposit _helperDeposit;

		// Data-driven resource targeting.
		private uint _currentResourceGUID;
		private Vector3 _currentResourcePosition;
		private Utils.Resource _currentResourceType;
		private bool _hasDataDrivenResourceTarget;

		// Configuration.
		[SerializeField] private float _resourceSearchRange = 100f;
		[SerializeField] private float _resourceSearchInterval = 1f;
		private float _resourceSearchTimer;

		/// <summary>
		/// Initialize all required data.
		/// </summary>
		protected override void OnInit()
		{
			base.OnInit();
			_roleHandler = GetComponent<RoleHandler>();
			_inventory = GetComponent<PlayerInventory>();
			_gatherResourceAction = (STSM_Action_GatherResource)_stateMachine.GetStateByName("GatherResource");
			_attackAction = (STSM_Action_PlayerAttack)_stateMachine.GetStateByName("Attack");
			_healAction = (STSM_Action_Heal)_stateMachine.GetStateByName("Heal");
			_helperDeposit = (STSM_HelperDeposit)_stateMachine.GetHelperByName("Deposit");
			_goToState = (STSM_GoToLocation)_stateMachine.GetStateByName("GoTo");
		}

		public override void OnEnter()
		{
			base.OnEnter();

			// Clear local resource target fields when entering Idle to ensure fresh resource search
			_currentResourceGUID = 0;
			_currentResourcePosition = Vector3.zero;
			_currentResourceType = Utils.Resource.None;
			_hasDataDrivenResourceTarget = false;

			// Note: Do NOT clear from StateMachine here - let the gather state handle that
		}

		/// <summary>
		/// Called periodically when the player has a target.
		/// </summary>
		protected override void OnHasTarget()
		{
			base.OnHasTarget();

			bool success = false;

			// Set next state based on role type.
			switch (_roleHandler.RoleData_SO.RoleFlags)
			{
				case PlayerRoleType.Resource:
					success = ResourceRole();
					break;
				case PlayerRoleType.Damage:
					success = DamageRole();
					break;
				case PlayerRoleType.Healer:
					success = HealerRole();
					break;
				case PlayerRoleType.Other:
					success = OtherRole();
					break;
			}

			if (success)
			{
				// Use data-driven resource targeting (position-based)
				_goToState.UsePosition = true;
				_goToState.SetTargetPosition(_currentResourcePosition, _roleHandler.PlayerRoleData.ActionRange);
				_stateMachine.RequestStateChange(_goToState);
			}
		}

		/// <summary>
		/// Returns true if player can gather resources.
		/// </summary>
		/// <returns></returns>
		protected bool ResourceRole()
		{
			//Check inventory is not full and town resource isnt full
			Utils.Resource resourceType = _roleHandler.RoleData_SO.Resource;

			if (_townResourceProcessor.ResourceFull(resourceType))
				return false;

			if (_inventory.ResourceFull(resourceType))
			{
				_stateMachine.InvokeHelper(_helperDeposit);
				return false;
			}

			// Resource target is now set on StateMachine instead of on the gather state
			_goToState.SetNextState(_gatherResourceAction);
			return true;
		}

		/// <summary>
		/// Returns true after setting next state to attack action.
		/// </summary>
		/// <returns></returns>
		protected bool DamageRole()
		{
			_goToState.SetNextState(_attackAction);
			return true;
		}

		/// <summary>
		/// Returns true after setting next state to heal action.
		/// </summary>
		/// <returns></returns>
		protected bool HealerRole()
		{
			_goToState.SetNextState(_healAction);
			return true;
		}

		/// <summary>
		/// Returns false, used for unhandled roles.
		/// </summary>
		/// <returns></returns>
		protected bool OtherRole()
		{
			Debug.LogWarning($"Role type behaviour not handlded.");
			return false;
		}

		/// <summary>
		/// Finds a resource target via ResourceProcessor for data-driven resource gathering.
		/// </summary>
		/// <returns>True if a resource target was found and claimed.</returns>
		private bool FindResourceTarget()
		{
			Utils.Resource resourceType = _roleHandler.RoleData_SO.Resource;

			if (resourceType == Utils.Resource.None)
				return false;

			// Get the top 20 closest resources of the required type
			List<Processors.ResourceTarget> closestResources = _resourceProcessor.GetClosestResources(
				transform.position,
				20,
				resourceType
			);

			if (closestResources == null || closestResources.Count == 0)
				return false;

			// Calculate scores for all resources and sort by score
			List<(uint guid, Vector3 position, float score)> scoredResources = new List<(uint, Vector3, float)>();
			foreach (var resource in closestResources)
			{
				float score = _resourceProcessor.CalculateTargetScore(resource.GUID, transform.position);
				scoredResources.Add((resource.GUID, resource.Position, score));
			}

			// Sort by score (lower is better)
			scoredResources.Sort((a, b) => a.score.CompareTo(b.score));

			// Try to assign to resources in order of score until one succeeds
			// Only assign if count is 0 (simple reservation system)
			uint assignedGUID = 0;
			Vector3 assignedPosition = Vector3.zero;

			foreach (var (guid, position, score) in scoredResources)
			{
				// Only claim if no one else has claimed it yet
				if (_resourceProcessor.GetAssignmentCount(guid) == 0)
				{
					_resourceProcessor.AssignToTarget(guid);
					assignedGUID = guid;
					assignedPosition = position;
					break;
				}
			}

			if (assignedGUID == 0)
				return false;

			// Unassign from any existing resource before assigning to a new one
			// This prevents old resources from remaining assigned when players switch to closer trees
			if (_currentResourceGUID != 0 && _currentResourceGUID != assignedGUID)
			{
				_resourceProcessor.UnassignFromTarget(_currentResourceGUID);
			}
			if (_stateMachine.ResourceTargetGUID != 0 && _stateMachine.ResourceTargetGUID != assignedGUID)
			{
				_resourceProcessor.UnassignFromTarget(_stateMachine.ResourceTargetGUID);
			}

			// Use resource position directly - let pathfinding find nearest navmesh point
			// This avoids grid alignment issues between resource and navmesh grids
			Vector3 targetPosition = assignedPosition;

			// Store the target data in StateMachine (shared across states)
			_stateMachine.SetResourceTarget(assignedGUID, resourceType);

			// Store the target data locally for this state
			_currentResourceGUID = assignedGUID;
			_currentResourcePosition = targetPosition;
			_currentResourceType = resourceType;
			_hasDataDrivenResourceTarget = true;

			return true;
		}

		/// <summary>
		/// Clears the data-driven resource target.
		/// </summary>
		private void ClearResourceTarget()
		{
	
			if (_currentResourceGUID != 0)
			{
				_resourceProcessor.UnassignFromTarget(_currentResourceGUID);
			}

			_currentResourceGUID = 0;
			_currentResourcePosition = Vector3.zero;
			_currentResourceType = Utils.Resource.None;
			_hasDataDrivenResourceTarget = false;

			// Note: Do NOT clear from StateMachine here - the resource target needs to persist
			// when transitioning from Idle to GoTo so the gather state can use it.
			// The gather state will clear the resource target in its OnExit/CleanupDataDrivenTarget.
		}

		protected override void OnNewIdleLocation()
		{
			// If we have a data-driven resource target, don't do the base idle wandering
			if (_hasDataDrivenResourceTarget)
				return;

			base.OnNewIdleLocation();
		}

		/// <summary>
		/// Called when the state is exited. Cleans up resource assignment.
		/// </summary>
		public override void OnExit()
		{
			base.OnExit();
			// Note: Do NOT call ClearResourceTarget() here - states are shared across characters,
			// so one character's idle state exiting shouldn't clear another character's resource target.
			// The resource target should be cleared when the gather state completes or when the character
			// explicitly returns to idle after gathering.
		}

		/// <summary>
		/// Called every frame. Updates resource search timer and finds targets when appropriate.
		/// </summary>
		public override void OnUpdate()
		{
			base.OnUpdate();

			// Return early if role hasn't been set yet
			if (_roleHandler.RoleData_SO == null)
				return;

			_resourceSearchTimer += Time.deltaTime;

			// If we have a GameObject target, let the base class handle it
			if (_targetSensor.HasTarget)
				return;

			// If we're a resource role and don't have a target, try to find one
			if (_roleHandler.RoleData_SO.RoleFlags == Utils.PlayerRoleType.Resource)
			{
				if (_resourceSearchTimer >= _resourceSearchInterval)
				{
					_resourceSearchTimer = 0f;

					// Check if we should find a new resource target
					if (!_hasDataDrivenResourceTarget)
					{
						if (CanGatherResource())
						{
							if (FindResourceTarget())
							{
								// Set up GoToLocation to go to the resource position, then gather
								// Use SetTargetFromPosition for transform-based targeting with action range
								_goToState.SetTargetFromPosition(_currentResourcePosition, _roleHandler.PlayerRoleData.ActionRange);
								_goToState.UsePosition = false; // Using transform-based targeting now

								_goToState.SetNextState(_gatherResourceAction);
								_stateMachine.RequestStateChange(_goToState);
							}
						}
					}
					else if (_stateMachine.HasResourceTarget && _stateMachine.ResourceTargetGUID != 0)
					{
						// We have an existing resource target from StateMachine (after depositing)
						// Reuse it to continue gathering from the same resource
						var resourceTarget = _resourceProcessor.GetResourceTarget(_stateMachine.ResourceTargetGUID);
						if (resourceTarget.HasValue)
						{
							// Update local fields from StateMachine
							_currentResourceGUID = _stateMachine.ResourceTargetGUID;
							_currentResourcePosition = resourceTarget.Value.Position;
							_currentResourceType = _stateMachine.ResourceTargetType;

							// Set up GoToLocation to go to the resource position, then gather
							_goToState.SetTargetFromPosition(_currentResourcePosition, _roleHandler.PlayerRoleData.ActionRange);
							_goToState.UsePosition = false;

							_goToState.SetNextState(_gatherResourceAction);
							_stateMachine.RequestStateChange(_goToState);
						}
						else
						{
							// Resource no longer exists, clear target and find new one
							ClearResourceTarget();
						}
					}
				}
			}
		}

		/// <summary>
		/// Checks if the player can gather resources (inventory not full, town resources not full).
		/// </summary>
		private bool CanGatherResource()
		{
			Utils.Resource resourceType = _roleHandler.RoleData_SO.Resource;

			if (_townResourceProcessor.ResourceFull(resourceType))
				return false;

			if (_inventory.ResourceFull(resourceType))
				return false;

			return true;
		}
	}
}
