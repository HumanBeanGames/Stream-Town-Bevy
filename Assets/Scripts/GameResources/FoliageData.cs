using UnityEngine;

namespace GameResources
{
	/// <summary>
	/// Data structure for foliage instances used in GPU instancing.
	/// Similar to ResourceData but without resource-specific fields.
	/// </summary>
	public struct FoliageData
	{
		public Vector3 Position;
		public Quaternion Rotation;
		public Vector3 Scale;
		public Mesh Mesh;
		public Material Material;

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
