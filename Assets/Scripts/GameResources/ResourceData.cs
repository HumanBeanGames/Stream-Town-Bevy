using UnityEngine;

namespace GameResources
{
	/// <summary>
	/// Data structure representing a resource instance in the world.
	/// Used for data-driven resource management instead of object-based ResourceHolders.
	/// </summary>
	public struct ResourceData
	{
        /// <summary>
        /// The position of the resource.
        /// </summary>
		public Vector3 Position;

        /// <summary>
        /// The transformation matrix.
        /// </summary>
		public Matrix4x4 Matrix;

        /// <summary>
        /// The resource type.
        /// </summary>
		public Utils.Resource ResourceType;

        /// <summary>
        /// The current amount of the resource.
        /// </summary>
		public int CurrentAmount;

        /// <summary>
        /// Whether the resource is unlimited.
        /// </summary>
		public bool IsUnlimited;

        /// <summary>
        /// The GUID of the resource.
        /// </summary>
		public uint GUID;

        /// <summary>
        /// The mesh index.
        /// </summary>
		public int MeshIndex;

        /// <summary>
        /// The material index.
        /// </summary>
		public int MaterialIndex;

        /// <summary>
        /// Initializes a new resource data instance.
        /// </summary>
        /// <param name="position">The position.</param>
        /// <param name="resourceType">The resource type.</param>
        /// <param name="currentAmount">The current amount.</param>
        /// <param name="isUnlimited">Whether the resource is unlimited.</param>
        /// <param name="matrix">The transformation matrix.</param>
        /// <param name="guid">The GUID.</param>
        /// <param name="meshIndex">The mesh index.</param>
        /// <param name="materialIndex">The material index.</param>
		public ResourceData(Vector3 position, Utils.Resource resourceType, int currentAmount, bool isUnlimited, Matrix4x4 matrix = default, uint guid = 0, int meshIndex = -1, int materialIndex = -1)
		{
			Position = position;
			Matrix = matrix == default ? Matrix4x4.TRS(position, Quaternion.identity, Vector3.one) : matrix;
			ResourceType = resourceType;
			CurrentAmount = currentAmount;
			IsUnlimited = isUnlimited;
			GUID = guid;
			MeshIndex = meshIndex;
			MaterialIndex = materialIndex;
		}
	}
}
