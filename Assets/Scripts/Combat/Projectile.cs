using Target;
using UnityEngine;

namespace Combat 
{
    /// <summary>
    /// Represents a projectile that moves towards a target and deals damage on impact.
    /// </summary>
    public class Projectile : MonoBehaviour 
	{
        /// <summary>
        /// Gets or sets the target of the projectile.
        /// </summary>
		public TargetableHealth Target { get; set; }

        /// <summary>
        /// Gets or sets the movement speed of the projectile.
        /// </summary>
		public float MoveSpeed { get; set; }

        /// <summary>
        /// Gets or sets the damage dealt by the projectile.
        /// </summary>
		public int Damage { get; set; }

        /// <summary>
        /// Called when the projectile hits the target.
        /// </summary>
		private void OnHitTarget()
		{
			transform.position = Vector3.zero;
			Target.HealthHandler.TakeDamage(Damage, null);
			gameObject.SetActive(false);
		}

        /// <summary>
        /// Updates the projectile's position and rotation.
        /// </summary>
		private void Update()
		{
			if (Target == null)
				return;

			float sqrDistToTarget = Vector3.SqrMagnitude(transform.position - (Target.gameObject.transform.position + (Vector3.up * 2)));

			if (sqrDistToTarget <= 1f)
				OnHitTarget();

			transform.position = Vector3.MoveTowards(transform.position, Target.transform.position + (Vector3.up * 2), Time.deltaTime * MoveSpeed);

			Vector3 lookPos = Target.transform.position - transform.position;
            lookPos.y = 0;

			Quaternion rot = Quaternion.LookRotation(lookPos);
			transform.rotation = Quaternion.Slerp(transform.rotation, rot, Time.deltaTime * 50);
		}
	}
}
