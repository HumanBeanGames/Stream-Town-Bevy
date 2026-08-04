using UnityEngine;
using Utils;

namespace World.Generation
{
	/// <summary>
	/// Stable visual choices for generated world instances. These functions use
	/// coordinates and explicit salts only; they never consume Unity's global RNG.
	/// </summary>
	public static class WorldInstanceDeterminism
	{
		private const int CoordinatePrecision = 1000;
		private const uint FnvOffset = 2166136261u;
		private const uint FnvPrime = 16777619u;

		private const int ResourceMeshSalt = 0x13579B;
		private const int ResourceMaterialSalt = 0x2468AC;
		private const int ResourceRotationSalt = 0x5A17E1;
		private const int FoliageMeshSalt = 0x31C4D2;
		private const int FoliageRotationSalt = 0x7B29F3;

		public static int SelectResourceMesh(Vector3 position, Resource resourceType, int count)
		{
			return SelectIndex(position, ResourceMeshSalt ^ (int)resourceType, count);
		}

		public static int SelectResourceMaterial(Vector3 position, Resource resourceType, int count)
		{
			return SelectIndex(position, ResourceMaterialSalt ^ (int)resourceType, count);
		}

		public static Quaternion SelectResourceRotation(Vector3 position, Resource resourceType)
		{
			return Quaternion.Euler(0f, SelectIndex(position, ResourceRotationSalt ^ (int)resourceType, 4) * 90f, 0f);
		}

		public static int SelectFoliageMesh(Vector3 position, string settingsId, int count)
		{
			return SelectIndex(position, FoliageMeshSalt ^ StableStringHash(settingsId), count);
		}

		public static Quaternion SelectFoliageRotation(Vector3 position, string settingsId)
		{
			return Quaternion.Euler(0f, SelectIndex(position, FoliageRotationSalt ^ StableStringHash(settingsId), 4) * 90f, 0f);
		}

		public static int SelectIndex(Vector3 position, int salt, int count)
		{
			if (count <= 0)
				return -1;

			uint hash = FnvOffset;
			hash = Mix(hash, unchecked((uint)Mathf.RoundToInt(position.x * CoordinatePrecision)));
			hash = Mix(hash, unchecked((uint)Mathf.RoundToInt(position.z * CoordinatePrecision)));
			hash = Mix(hash, unchecked((uint)salt));
			return (int)(hash % (uint)count);
		}

		private static int StableStringHash(string value)
		{
			uint hash = FnvOffset;
			if (value != null)
			{
				for (int i = 0; i < value.Length; i++)
					hash = Mix(hash, value[i]);
			}

			return unchecked((int)hash);
		}

		private static uint Mix(uint hash, uint value)
		{
			unchecked
			{
				hash = (hash ^ (value & 0xFFu)) * FnvPrime;
				hash = (hash ^ ((value >> 8) & 0xFFu)) * FnvPrime;
				hash = (hash ^ ((value >> 16) & 0xFFu)) * FnvPrime;
				return (hash ^ ((value >> 24) & 0xFFu)) * FnvPrime;
			}
		}
	}
}
