using Enemies;
using UnityEngine;

namespace STStateMachine.States
{
	/// <summary>
	/// Simple action state for enemy attacks.
	/// </summary>
	public class STSM_Action_EnemyAttack : STSM_Action_Attack
	{
		protected EnemyModelHandler _enemyModelHandler;
		private bool _verticalAnchorActive;
		private float _verticalAnchorY;

		protected override void OnInit()
		{
			base.OnInit();
			_enemyModelHandler = GetComponentInChildren<EnemyModelHandler>();
		}

		public override void OnEnter()
		{
			if (_useAnimation)
			{
				_actionAnimation = _enemyModelHandler.GetAttackAnimation();
				_actionVariants = _enemyModelHandler.GetAttackVariantCount();
			}

			base.OnEnter();
			_verticalAnchorActive = _stateMachine.CurrentState == this;
			if (!_verticalAnchorActive)
				return;

			// Enemy attacks are stationary actions. A few imported clips (most
			// visibly Blargul's attack) contain vertical root translation. Target
			// changes also briefly re-enable AIPath, so preserving the current Y
			// would make an existing offset the new anchor on every re-entry.
			// Resolve the terrain once instead and synchronize the path simulation.
			_verticalAnchorY = ResolveGroundHeight(transform.position);
			Vector3 groundedPosition = transform.position;
			groundedPosition.y = _verticalAnchorY;
			transform.position = groundedPosition;
			_aiPath?.Teleport(groundedPosition, false);
		}

		private static float ResolveGroundHeight(Vector3 position)
		{
			int groundMask = World.WorldUtils.GroundLayerMask.value;
			if (groundMask == 0)
				return position.y;

			Vector3 rayOrigin = new Vector3(position.x, Mathf.Max(position.y + 5f, 100f), position.z);
			return Physics.Raycast(
				rayOrigin,
				Vector3.down,
				out RaycastHit hit,
				200f,
				groundMask,
				QueryTriggerInteraction.Ignore)
				? hit.point.y
				: position.y;
		}

		private void LateUpdate()
		{
			if (!_verticalAnchorActive || _stateMachine == null || _stateMachine.CurrentState != this)
				return;

			Vector3 anchoredPosition = transform.position;
			anchoredPosition.y = _verticalAnchorY;
			transform.position = anchoredPosition;
		}

		public override void OnExit()
		{
			if (_verticalAnchorActive && _aiPath != null)
			{
				Vector3 anchoredPosition = transform.position;
				anchoredPosition.y = _verticalAnchorY;
				transform.position = anchoredPosition;
				_aiPath.Teleport(anchoredPosition, false);
			}

			_verticalAnchorActive = false;
			base.OnExit();
		}
	}
}
