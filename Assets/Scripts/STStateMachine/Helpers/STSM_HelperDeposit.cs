using Character;
using Processors;
using Reflex.Attributes;
using Sensors;
using STStateMachine.States;
using UnityEngine;

namespace STStateMachine.Helpers
{
	/// <summary>
	/// A helper action that helps units deposit resources to the town storage.
	/// </summary>
	public class STSM_HelperDeposit : STSM_HelperBase
	{
		private STSM_GoToLocation _goToState;
		private STSM_Action_DepositResource _depositState;
		private RoleHandler _roleHandler;
		private StationSensor _stationSensor;
		[Inject] private TownResourceProcessor _townResourceProcessor;
		private STSM_Idle_Player _idle;

		public override void Init()
		{
			_goToState = (STSM_GoToLocation)_stateMachine.GetStateByName("GoTo");
			_depositState = (STSM_Action_DepositResource)_stateMachine.GetStateByName("DepositResource");
			_roleHandler = GetComponent<RoleHandler>();
			_stationSensor = GetComponent<StationSensor>();
			_idle = (STSM_Idle_Player)_stateMachine.GetStateByName("Idle");
		}

		public override void InvokeHelper()
		{
			// If the resources are already full, swap to idle and reset position.
			if (_townResourceProcessor.ResourceFull(_roleHandler.RoleData_SO.Resource))
			{
				_goToState.SetNextState(_idle);
				_goToState.UsePosition = true;
				_goToState.SetTargetPosition(Vector3.zero);
			}
			// Go ahead and deposit at the designated station.
			else
			{
				_goToState.SetNextState(_depositState);
				_goToState.SetDistanceSatisfaction(_stationSensor.CurrentStation.Targetable.SizeSqr + 1);
				_goToState.UsePosition = false;

				// If the unit has a station, set the location target to the station.
				if (_stationSensor.HasStation)
				{
					// Target the building center with a small offset
					// The previous edge calculation using SizeSqr was incorrect
					Vector3 buildingCenter = _stationSensor.CurrentStation.transform.position;
					Vector3 toPlayer = transform.position - buildingCenter;
					toPlayer.y = 0; // Keep it horizontal
					
					if (toPlayer.magnitude > 0)
					{
						toPlayer.Normalize();
						// Use a small fixed offset from building center (2 units)
						Vector3 dropoffPoint = buildingCenter + toPlayer * 2f;
						_goToState.SetTargetPosition(dropoffPoint);
						_goToState.UsePosition = true;
					}
					else
					{
						// If player is at center, use center
						_goToState.SetTarget(_stationSensor.CurrentStation.transform);
					}
				}
				// Otherwise return to world center.
				else
					_goToState.SetTargetPosition(Vector3.zero);
			}

			_stateMachine.RequestStateChange(_goToState);
			_roleHandler.EquipmentHandler.EnableCarry(_roleHandler.CurrentRole);
		}
	}
}
