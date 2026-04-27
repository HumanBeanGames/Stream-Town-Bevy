using UnityEngine;

namespace Processors
{
	/// <summary>
	/// Marks the location where players should spawn in the scene.
	/// Place this component on a GameObject in the scene to define the spawn point.
	/// </summary>
	public class PlayerSpawnPoint : MonoBehaviour
	{
		/// <summary>
		/// The transform where players will spawn.
		/// </summary>
		public Transform SpawnTransform => transform;

		private void OnDrawGizmos()
		{
			Gizmos.color = Color.green;
			Gizmos.DrawWireSphere(transform.position, 1f);
			Gizmos.DrawLine(transform.position, transform.position + transform.forward * 2f);
		}

		private void OnDrawGizmosSelected()
		{
			Gizmos.color = Color.green;
			Gizmos.DrawWireSphere(transform.position, 1f);
			Gizmos.DrawLine(transform.position, transform.position + transform.forward * 2f);
			UnityEditor.Handles.Label(transform.position + Vector3.up * 1.5f, "Player Spawn Point");
		}
	}
}
