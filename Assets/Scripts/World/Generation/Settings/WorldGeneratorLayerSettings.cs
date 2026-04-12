using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for world generator layer mask settings.
	/// </summary>
	public class WorldGeneratorLayerSettings : MonoBehaviour
	{
		[SerializeField]
		private LayerMask _collisionMask;
		[SerializeField]
		private LayerMask _terrainMask;

		public LayerMask CollisionMask => _collisionMask;
		public LayerMask TerrainMask => _terrainMask;
	}
}
