using Behaviours;
using Character;
using GameResources;
using Pets.Enumerations;
using Reflex.Attributes;
using STStateMachine.Helpers;
using Twitch;
using UnityEngine;
using Utils;

namespace STStateMachine.States
{
	/// <summary>
	/// An action state that allows a unit to gather resources.
	/// </summary>
	public class STSM_Action_GatherResource : STSM_Action_PlayerBase
	{
		[Inject] private Processors.TwitchChatProcessor _twitchChatProcessor;
		[Inject] private Processors.ResourceProcessor _resourceProcessor;
		protected CollectResource _collectResource;
		protected PlayerInventory _playerInventory;
		protected STSM_HelperDeposit _helperDeposit;

		public override void OnEnter()
		{
			// Set action variables from PlayerRoleData (from base class)
			_roleData = _roleHandler.RoleData_SO;
			_actionAnimation = _roleData.ActionAnimationName;
			_actionAmount = _roleHandler.PlayerRoleData.ActionAmount;
			_actionRate = _roleHandler.PlayerRoleData.ActionRate;
			_actionRange = _roleHandler.PlayerRoleData.ActionRange;
			_animationHandler.SetActionSpeed(Mathf.Max(1,MathExtended.RemapValue(_actionRate, 1, 0, 1, 3)));


			// Validate data-driven target from StateMachine (per-character, not shared state)
			if (!_stateMachine.HasResourceTarget || _stateMachine.ResourceTargetGUID == 0 || _stateMachine.ResourceTargetType == Utils.Resource.None)
			{
				Debug.LogWarning($"[ResourceGathering] Invalid resource target - GUID: {_stateMachine.ResourceTargetGUID}, Type: {_stateMachine.ResourceTargetType}");
				_stateMachine.RequestStateChange("Idle");
				return;
			}

			// If ActionAmount is 0, use a default value for gathering
			if (_actionAmount == 0)
			{
				_actionAmount = 10; // Default to gathering 10 resources per action
				Debug.Log($"[ResourceGathering] ActionAmount was 0, using default: {_actionAmount}");
			}

			// Disable AIPath to prevent pathfinding while gathering
			_aiPath.enabled = false;
			_aiPath.canMove = false;

			// Face the resource target
			var resourceTarget = _resourceProcessor.GetResourceTarget(_stateMachine.ResourceTargetGUID);
			if (resourceTarget.HasValue)
			{
				Vector3 directionToTarget = (resourceTarget.Value.Position - transform.position).normalized;
				directionToTarget.y = 0; // Keep rotation on horizontal plane only
				if (directionToTarget != Vector3.zero)
				{
					transform.rotation = Quaternion.Slerp(transform.rotation, Quaternion.LookRotation(directionToTarget), 0.1f);
					transform.rotation = Quaternion.LookRotation(directionToTarget); // Snap to face immediately
				}
			}

			// Initialize the action timer and animation
			_actionTimer = 0;

			if (_useAnimation)
			{
				_animationHandler.SetBool(AnimationName.Action, true);
				_animationHandler.SetTrigger(_actionAnimation);
				_animationHandler.SetAttackAnimationIndex(UnityEngine.Random.Range(0, _actionVariants));
			}
		}

		public override void OnExit()
		{
			// Re-enable AIPath for movement after gathering
			_aiPath.enabled = true;
			_aiPath.canMove = true;

			// Manually clean up animation handler since we're not calling base.OnEnter()
			if (_useAnimation)
			{
				_animationHandler.SetBool(AnimationName.Action, false);
			}
			// Note: Do NOT call CleanupDataDrivenTarget() here - states are shared across characters,
			// so one character's gather state exiting shouldn't clear another character's resource target.
			// The resource target should be cleared by the Idle state when the character returns to idle.
		}

		/// <summary>
		/// Cleans up the data-driven resource target and unassigns it.
		/// </summary>
		private void CleanupDataDrivenTarget()
		{
			if (_stateMachine.HasResourceTarget && _stateMachine.ResourceTargetGUID != 0)
			{
				_resourceProcessor.UnassignFromTarget(_stateMachine.ResourceTargetGUID);
			}

			// Note: Do NOT clear from StateMachine here - states are shared across characters,
			// so one character's gather state exiting shouldn't clear another character's resource target.
			// The resource target should be cleared by the Idle state when the character returns to idle.
		}

		protected override bool DoAction()
		{

			// Get resource type from StateMachine (per-character, not shared state)
			Utils.Resource resourceType = _stateMachine.ResourceTargetType;

			// If the player's resource inventory is full, go to deposit.
			if (_playerInventory.ResourceFull(resourceType))
			{
				_stateMachine.InvokeHelper(_helperDeposit);
				// Note: Do NOT unassign here - resource assignment should persist through gather-deposit cycle
				return false;
			}

			// Check if the resource exists before trying to take it
			var resourceTarget = _resourceProcessor.GetResourceTarget(_stateMachine.ResourceTargetGUID);
			if (!resourceTarget.HasValue)
			{
				Debug.LogWarning($"[ResourceGathering] Resource GUID {_stateMachine.ResourceTargetGUID} not found in ResourceProcessor");
				_stateMachine.RequestStateChange("Idle");
				CleanupDataDrivenTarget();
				return false;
			}


			int amountTaken = _resourceProcessor.TakeResource(_stateMachine.ResourceTargetGUID, _actionAmount);

			// If resource is depleted (returned 0 or less than requested), go to deposit and unassign
			if (amountTaken <= 0)
			{
				_stateMachine.InvokeHelper(_helperDeposit);
				CleanupDataDrivenTarget();
				return false;
			}

			_playerInventory.AddResource(resourceType, amountTaken);
			return true;
		}

        protected override void OnActionSuccess()
        {
            base.OnActionSuccess();
            // Note: Do NOT unassign here - resource assignment should persist through gather-deposit cycle
            // Only unassign when resource is depleted or player returns to idle without resource target

            // This needs to be redone, temporary implementation - only for players, not NPCs
            if (!_roleHandler.IsNPC)
            {
                int rand = Random.Range(0, 5000);
                if (rand == 0)
                {
					if (_actionAnimation == AnimationName.Gathering)
					{

						if (_roleHandler.Player.PetsUnlocked[PetType.Giraffe])
							return;

						_roleHandler.Player.PetsUnlocked[PetType.Giraffe] = true;
						_twitchChatProcessor.SendMessage($"{_roleHandler.Player.TwitchUser.Username} unlocked the giraffe pet!");
					}
					if(_actionAnimation == AnimationName.Fishing)
					{
						if (_roleHandler.Player.PetsUnlocked[PetType.Duck])
							return;

						_roleHandler.Player.PetsUnlocked[PetType.Duck] = true;
						_twitchChatProcessor.SendMessage($"{_roleHandler.Player.TwitchUser.Username} unlocked the duck pet!");
					}
					if (_actionAnimation == AnimationName.WoodCutting)
					{
						if (_roleHandler.Player.PetsUnlocked[PetType.Butterfly])
							return;

						_roleHandler.Player.PetsUnlocked[PetType.Butterfly] = true;
						_twitchChatProcessor.SendMessage($"{_roleHandler.Player.TwitchUser.Username} unlocked the butterfly pet!");
					}
				}
            }
        }

        protected override void OnInit()
		{
			base.OnInit();

			_collectResource = GetComponent<CollectResource>();
			_helperDeposit = (STSM_HelperDeposit)_stateMachine.GetHelperByName("Deposit");
			_playerInventory = GetComponent<PlayerInventory>();
		}
	}
}
