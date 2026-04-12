using UnityEngine;

namespace GameResources
{
	/// <summary>
	/// Data structure representing a resource instance in the world.
	/// Used for data-driven resource management instead of object-based ResourceHolders.
	/// </summary>
	public struct ResourceData
	{
		public Vector3 Position;
		public Matrix4x4 Matrix;
		public Utils.Resource ResourceType;
		public int CurrentAmount;
		public bool IsUnlimited;
		public uint GUID;
		public int MeshIndex;
		public int MaterialIndex;

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
