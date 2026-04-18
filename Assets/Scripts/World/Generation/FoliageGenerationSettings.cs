using System.Collections.Generic;
using UnityEngine;

namespace World.Generation
{
	/// <summary>
	/// Generation settings for Foliage to be placed in the world.
	/// </summary>
	[System.Serializable]
	public class FoliageGenerationSettings : GenerationSettings
	{
		public List<Mesh> Meshes;
		public Material Material;

		// Constructor.
		public FoliageGenerationSettings(int size, int levelOfDetail, float noiseScale, int octaves, float persistance, float lacunarity, int seed, Vector2 offset, float spawnThreshold = 0.5f, List<Mesh> meshes = null, Material material = null) : base(size, levelOfDetail, noiseScale, octaves, persistance, lacunarity, seed, offset, spawnThreshold)
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
			Meshes = meshes ?? new List<Mesh>();
			Material = material;
			HeightMap = new float[size, size];
		}
	}
}
