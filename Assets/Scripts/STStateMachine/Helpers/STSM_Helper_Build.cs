using Sensors;
using STStateMachine.States;
using Target;
using Units;
using UnityEngine;

namespace STStateMachine.Helpers
{
	/// <summary>
	/// A state helper to help a unit build/construct a building.
	/// Unlike Attack helper which deals damage, this helper increases health (constructs).
	/// </summary>
	public class STSM_Helper_Build : STSM_HelperBase
	{
		public int BuildAmount;
		public STSM_GoToLocation GoToState;
		public TargetSensor TargetSensor;
		public HealthHandler TargetHealth;
		public Targetable Target;
		public Targetable Owner;

		private void Awake()
		{
			// Set HelperName in Awake to ensure it's set before prefab values are applied
			HelperName = "Build";
		}

		public override void Init()
		{
			GoToState = (STSM_GoToLocation)_stateMachine.GetStateByName("GoTo");
			TargetSensor = GetComponent<TargetSensor>();
			Owner = GetComponent<Targetable>();
		}

		public override void InvokeHelper()
		{
			// If we don't have a target, swap back to idle.
			if (Target == null || !Target.gameObject.activeInHierarchy)
			{
				Debug.LogWarning("[STSM_Helper_Build] Target is null or inactive");
				_stateMachine.RequestStateChange("Idle");
				return;
			}

			// Ensure that we have the target's health component.
			if (TargetHealth == null || TargetHealth.gameObject != Target.gameObject)
				TargetHealth = Target.GetComponent<HealthHandler>();

			if (TargetHealth == null)
			{
				Debug.LogError($"[STSM_Helper_Build] Target {Target.name} has no HealthHandler component!");
				_stateMachine.RequestStateChange("Idle");
				return;
			}

			// Build the target by increasing its health (construct it).
			Debug.Log($"[STSM_Helper_Build] Building {Target.name}, BuildAmount: {BuildAmount}, CurrentHealth: {TargetHealth.Health}, MaxHealth: {TargetHealth.MaxHealth}");
			TargetHealth.ModHealth(BuildAmount);
		}
	}
}
