using Combat;
using Processors;
using Reflex.Attributes;
using Sensors;
using Target;
using UnityEngine;

namespace Buildings
{
    /// <summary>
    /// Shoots projectiles at targets within range.
    /// Handles projectile spawning from object pooling.
    /// </summary>
	public class ProjectileShooter : MonoBehaviour
	{
        /// <summary>
        /// The name of the projectile pool to use.
        /// </summary>
		[SerializeField]
		private string ProjectilePoolName;

        /// <summary>
        /// The movement speed of projectiles.
        /// </summary>
		[SerializeField]
		private float _moveSpeed;

        /// <summary>
        /// The damage dealt by projectiles.
        /// </summary>
		[SerializeField]
		private int _damage;

        /// <summary>
        /// The maximum range at which targets can be engaged.
        /// </summary>
		[SerializeField]
		private float _range = 10f;

        /// <summary>
        /// The rate at which projectiles are fired (in seconds).
        /// </summary>
		[SerializeField]
		private float _fireRate;

        /// <summary>
        /// Time remaining until the next attack.
        /// </summary>
		private float _timeUntilAttack;

        /// <summary>
        /// The target sensor for detecting enemies.
        /// </summary>
		private TargetSensor _targetSensor;

        /// <summary>
        /// Object pooling processor for spawning projectiles.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingProcessor _objectPoolingProcessor;

        // Initializes the target sensor and sets the initial attack timer.
		private void Start()
		{
			_targetSensor = GetComponent<TargetSensor>();
			_timeUntilAttack = _fireRate;
		}

        // Updates the attack timer and fires projectiles at targets when ready.
		private void Update()
		{
			_timeUntilAttack -= Time.deltaTime;

			if (_timeUntilAttack <= 0)
			{

				if (!_targetSensor.HasTarget)
					return;
				_timeUntilAttack = _fireRate;

				if (Vector3.SqrMagnitude(transform.position - _targetSensor.CurrentTarget.transform.position) > (_range * _range))
				{
					_targetSensor.ClearTarget();
					return;
				}

				Projectile proj = _objectPoolingProcessor.GetPooledObject(ProjectilePoolName, true).GetComponent<Projectile>();
				proj.gameObject.transform.position = transform.position;
				proj.Damage = _damage;
				proj.MoveSpeed = _moveSpeed;
				proj.Target = (TargetableHealth)_targetSensor.CurrentTarget;
				proj.gameObject.SetActive(true);
			}
		}

	}
}
