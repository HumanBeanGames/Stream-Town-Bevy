using System.Collections.Generic;
using UnityEngine;

namespace World.Generation
{
	[System.Serializable]
	public class FoliageMeshSettings
	{
		public Mesh Mesh;
		public Vector3 BaseScale = Vector3.one;
	}

	/// <summary>
	/// Generation settings for Foliage to be placed in the world.
	/// </summary>
	[System.Serializable]
	public class FoliageGenerationSettings : GenerationSettings
	{
		public string PoolName;
		public List<FoliageMeshSettings> MeshSettings;
		public Material Material;

		public override string GetPoolName()
		{
			return PoolName;
		}

		// Constructor.
		public FoliageGenerationSettings(int size, int levelOfDetail, float noiseScale, int octaves, float persistance, float lacunarity, int seed, Vector2 offset, float spawnThreshold = 0.5f, Material material = null) : base(size, levelOfDetail, noiseScale, octaves, persistance, lacunarity, seed, offset, spawnThreshold)
		{
			Size = size;
			LevelOfDetail = levelOfDetail;
			NoiseScale = noiseScale;
			Octaves = octaves;
			Persistance = persistance;
			Lacunarity = lacunarity;
			Seed = seed;
			Offset = offset;
			SpawnThreshold = spawnThreshold;
			MeshSettings = new List<FoliageMeshSettings>();
			Material = material;
			HeightMap = new float[size, size];
		}
	}
}
