using Character;
using Buildings;
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
		[Inject] private BuildingProcessor _buildingProcessor;
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
			if (_townResourceProcessor.ResourceFull(_roleHandler.RoleData_SO.Resource))
			{
				MoveToTownhallAndIdle();
			}
			else
			{
				if (_stationSensor.HasStation)
				{
					_goToState.SetNextState(_depositState);
					_goToState.SetDistanceSatisfaction(_stationSensor.CurrentStation.Targetable.SizeSqr + 1);
					_goToState.UsePosition = false;
					Vector3 buildingCenter = _stationSensor.CurrentStation.transform.position;
					Vector3 toPlayer = transform.position - buildingCenter;
					toPlayer.y = 0;
					
					if (toPlayer.magnitude > 0)
					{
						toPlayer.Normalize();
						Vector3 dropoffPoint = buildingCenter + toPlayer * 2f;
						_goToState.SetTargetPosition(dropoffPoint);
						_goToState.UsePosition = true;
					}
					else
					{
						_goToState.SetTarget(_stationSensor.CurrentStation.transform);
					}
				}
				else
				{
					MoveToTownhallAndIdle();
				}
			}

			_stateMachine.RequestStateChange(_goToState);
			_roleHandler.EquipmentHandler.EnableCarry(_roleHandler.CurrentRole);
		}

		private void MoveToTownhallAndIdle()
		{
			BuildingBase townhall = GetActiveTownhall();
			_goToState.SetNextState(_idle);

			if (townhall == null)
				throw new System.InvalidOperationException($"{GetType().Name} on '{gameObject.name}' could not find an active Townhall for deposit fallback.");

			Vector3 buildingCenter = townhall.transform.position;
			Vector3 toPlayer = transform.position - buildingCenter;
			toPlayer.y = 0;

			if (toPlayer.magnitude > 0)
			{
				toPlayer.Normalize();
				Vector3 waitPoint = buildingCenter + toPlayer * 2f;
				_goToState.SetTargetPosition(waitPoint);
				_goToState.UsePosition = true;
				return;
			}

			_goToState.UsePosition = false;
			_goToState.SetTarget(townhall.transform);
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
