using UnityEngine;
using STStateMachine;
using Pathfinding;
using Animation;
using Utils;
using Units;

namespace Character
{
	/// <summary>
	/// Used to handle what should happen when a player dies and revives.
	/// </summary>
	public class PlayerDeathHandler : MonoBehaviour
	{
        /// <summary>
        /// The time in seconds before the player revives.
        /// </summary>
		[SerializeField]
		private float _reviveTime = (60 * 1);

        /// <summary>
        /// Counter for revive time.
        /// </summary>
		private float _reviveCounter = 0;

        /// <summary>
        /// Whether the revive process is active.
        /// </summary>
		private bool _reviveActive = false;

        /// <summary>
        /// The state machine for the player.
        /// </summary>
		private StateMachine _stateMachine;

        /// <summary>
        /// The AI path for the player.
        /// </summary>
		private AIPath _aIPath;

        /// <summary>
        /// The spawn position for the player.
        /// </summary>
		private Vector3 _spawnPosition;

        /// <summary>
        /// The animation handler for the player.
        /// </summary>
		private AnimationHandler _animationHandler;

        /// <summary>
        /// The health handler for the player.
        /// </summary>
		private HealthHandler _healthHandler;

		/// <summary>
		/// Called when the player dies.
		/// </summary>
		/// <param name="killedByPlayer">Whether the player was killed by another player (unused).</param>
		public void OnDeath(bool killedByPlayer)
		{
			if (_stateMachine == null || _aIPath == null || _reviveActive)
				return;

			_stateMachine.enabled = false;
			_aIPath.enabled = false;
			_animationHandler.SetTrigger(AnimationName.Death);
			_reviveCounter = 0;
			_reviveActive = true;
		}

		/// <summary>
		/// Called when the player revives.
		/// </summary>
		public void OnRevive()
		{
			_stateMachine.enabled = true;
			_aIPath.enabled = true;
			_stateMachine.RequestStateChange(_stateMachine.GetStateByName("Idle"));
			_animationHandler.SetTrigger(AnimationName.Revive);
			_reviveActive = false;
			_reviveCounter = 0;
			transform.position = _spawnPosition;
		}

		// Unity Events.
        // Initializes the player death handler.
		private void Awake()
		{
			_stateMachine = GetComponent<StateMachine>();
			_aIPath = GetComponent<AIPath>();
			_animationHandler = GetComponentInChildren<AnimationHandler>();
			_healthHandler = GetComponent<HealthHandler>();
		}

		/// <summary>
		/// Captures the actual spawn position after a prewarmed Player is checked out.
		/// </summary>
		public void InitializeForSpawn()
		{
			_spawnPosition = transform.position;
		}

        // Updates the revive counter.
		private void Update()
		{
			if (!_reviveActive)
				return;

			_reviveCounter += Time.deltaTime;
			if (_reviveCounter >= _reviveTime)
			{
				_reviveCounter = 0;
				_healthHandler.Revive();
			}
		}
	}
}
