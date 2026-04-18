using UnityEngine;

namespace GameResources
{
	/// <summary>
	/// Data structure for foliage instances used in GPU instancing.
	/// Similar to ResourceData but without resource-specific fields.
	/// </summary>
	public struct FoliageData
	{
        /// <summary>
        /// The position of the foliage instance.
        /// </summary>
		public Vector3 Position;

        /// <summary>
        /// The rotation of the foliage instance.
        /// </summary>
		public Quaternion Rotation;

        /// <summary>
        /// The scale of the foliage instance.
        /// </summary>
		public Vector3 Scale;

        /// <summary>
        /// The mesh of the foliage instance.
        /// </summary>
		public Mesh Mesh;

        /// <summary>
        /// The material of the foliage instance.
        /// </summary>
		public Material Material;

        /// <summary>
        /// Initializes a new foliage data instance.
        /// </summary>
        /// <param name="position">The position.</param>
        /// <param name="rotation">The rotation.</param>
        /// <param name="scale">The scale.</param>
        /// <param name="mesh">The mesh.</param>
        /// <param name="material">The material.</param>
		public FoliageData(Vector3 position, Quaternion rotation, Vector3 scale, Mesh mesh, Material material)
		{
			Position = position;
			Rotation = rotation;
			Scale = scale;
			Mesh = mesh;
			Material = material;
		}
	}
}
