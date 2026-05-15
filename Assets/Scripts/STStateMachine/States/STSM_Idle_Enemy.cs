using Buildings;
using Processors;
using Reflex.Attributes;

namespace STStateMachine.States
{
	/// <summary>
	/// A simple idle state for enemy units.
	/// </summary>
	public class STSM_Idle_Enemy : STSM_Idle
	{
		[Inject] private BuildingProcessor _buildingProcessor;

		private STSM_Action_Attack _attackAction;

		protected override void OnInit()
		{
			base.OnInit();
			_attackAction = _stateMachine.GetStateByName("Attack") as STSM_Action_Attack;
			if (_attackAction == null)
				throw new System.InvalidOperationException($"{GetType().Name} on '{gameObject.name}' requires a StateMachine state named 'Attack' of type {nameof(STSM_Action_Attack)}. Check the {_stateMachine.GetType().Name} state list on '{_stateMachine.gameObject.name}'.");
		}

		protected override void OnHasTarget()
		{
			base.OnHasTarget();

			if (!_targetSensor.TryAcquireNearestTarget() || _targetSensor.CurrentTarget == null)
				return;

			_goToState.UsePosition = false;
			_goToState.SetNextState(_attackAction);
			_goToState.SetTarget(_targetSensor.CurrentTarget.transform, _attackAction.Range);
			_goToState.SetDistanceSatisfaction(_targetSensor.CurrentTarget.SizeSqr + (_attackAction.Range * _attackAction.Range));
			_stateMachine.RequestStateChange(_goToState);
		}

		protected override void OnNewIdleLocation()
		{
			if (_targetSensor.TryAcquireNearestTarget() && _targetSensor.CurrentTarget != null)
			{
				OnHasTarget();
				return;
			}

			BuildingBase townhall = GetActiveTownhall();
			if (townhall != null)
			{
				_goToState.UsePosition = false;
				_goToState.SetNextState(this);
				_goToState.SetTarget(townhall.transform, 0f);
				_stateMachine.RequestStateChange(_goToState);
				return;
			}

			base.OnNewIdleLocation();
		}

		private BuildingBase GetActiveTownhall()
		{
			if (_buildingProcessor == null)
				return null;

			var townhalls = _buildingProcessor.GetBuildingsByType(Utils.BuildingType.Townhall);
			for (int i = 0; i < townhalls.Count; i++)
			{
				if (townhalls[i] != null && townhalls[i].gameObject.activeInHierarchy)
					return townhalls[i];
			}

			return null;
		}
	}
}
