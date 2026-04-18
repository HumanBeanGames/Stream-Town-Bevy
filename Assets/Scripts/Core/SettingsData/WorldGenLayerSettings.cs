using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject for world generator layer mask settings.
	/// 
	/// Existing values to copy:
	/// - CollisionMask: (copy from existing prefab)
	/// - TerrainMask: (copy from existing prefab)
	/// </summary>
	[CreateAssetMenu(fileName = "WorldGenLayerSettings", menuName = "Scriptables/World Generation/World Generator Layer Settings")]
	public class WorldGenLayerSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private LayerMask _collisionMask;
		[SerializeField]
		private LayerMask _terrainMask;

		public LayerMask CollisionMask => _collisionMask;
		public LayerMask TerrainMask => _terrainMask;
	}
}
