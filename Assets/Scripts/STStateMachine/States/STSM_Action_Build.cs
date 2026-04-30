using Behaviours;
using STStateMachine.Helpers;
using STStateMachine.States;
using Units;
using UnityEngine;
using Utils;

namespace STStateMachine.States
{
	/// <summary>
	/// An action state that allows a unit to build/construct a building.
	/// Unlike Attack state which deals damage, this state increases health (constructs).
	/// </summary>
	public class STSM_Action_Build : STSM_Action_PlayerBase
	{
		protected STSM_Helper_Build _buildHelper;

		public override void OnEnter()
		{
			base.OnEnter();
			Debug.Log($"[STSM_Action_Build] OnEnter - target: {_target?.name}, targetType: {_target?.TargetType}");
		}

		public override void OnUpdate()
		{
			base.OnUpdate();
		}

		public override void OnExit()
		{
			Debug.Log($"[STSM_Action_Build] OnExit - target: {_target?.name}, targetType: {_target?.TargetType}");
			base.OnExit();
			// Only force new position search if construction is complete, not after each build action
			if (_target != null && (_target.TargetType & TargetMask.Construction) != 0)
			{
				// Construction still in progress, don't force new position search
				((STSM_Idle)_stateMachine.GetStateByName("Idle")).NewPositionOnEnter = false;
			}
		}

		protected override void OnInit()
		{
			base.OnInit();
			_buildHelper = (STSM_Helper_Build)_stateMachine.GetHelperByName("Build");
			
			if (_buildHelper == null)
			{
				Debug.LogError("[STSM_Action_Build OnInit] Failed to get Build helper from StateMachine!");
			}
			else
			{
				Debug.Log("[STSM_Action_Build OnInit] Successfully got Build helper");
			}
		}

		protected override bool DoAction()
		{
			// Check if target is still valid and active
			if (_target == null || !_target.gameObject.activeInHierarchy)
			{
				Debug.LogWarning("[STSM_Action_Build] Target is null or inactive, exiting to Idle");
				_stateMachine.RequestStateChange("Idle");
				return false;
			}

			// Check if building is still under construction (has Construction target type)
			if ((_target.TargetType & TargetMask.Construction) == 0)
			{
				Debug.Log($"[STSM_Action_Build] Building { _target.name} is complete (no Construction flag), exiting to Idle");
				// Building is complete, go back to idle
				_stateMachine.RequestStateChange("Idle");
				return false;
			}

			// Check if unit is within range to build.
			float maxBuildRange = _actionRange * 2.5f + Mathf.Sqrt(_target.SizeSqr);
			float distanceSqr = Vector3.SqrMagnitude(_target.transform.position - transform.position);
			if (distanceSqr > maxBuildRange * maxBuildRange)
			{
				Debug.LogWarning($"[STSM_Action_Build] Out of build range - distance: {Mathf.Sqrt(distanceSqr)}, maxRange: {maxBuildRange}, actionRange: {_actionRange}");
				_stateMachine.RequestStateChange("Idle");
				return false;
			}

			Debug.Log($"[STSM_Action_Build] In range, executing build action - actionAmount: {_actionAmount}");
			_buildHelper.BuildAmount = _actionAmount;
			_buildHelper.Target = _target;
			_buildHelper.InvokeHelper();

			return true;
		}
	}
}
