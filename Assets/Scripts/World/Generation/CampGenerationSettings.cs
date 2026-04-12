using UnityEngine;

namespace World.Generation
{
	/// <summary>
	/// Used for generating camps around the world using the given settings.
	/// </summary>
	[System.Serializable]
	public class CampGenerationSettings : GenerationSettings
	{
		public string PoolName;
		public Vector2 MaxBounds;
		public Vector2 MinBounds;
		public int MaxAmount;
		public float MinDistanceFromCenter;
		public float MinDistanceFromOther;
		public int CampSize = 10;

		// Constructor.
		public CampGenerationSettings(int size, int levelOfDetail, float noiseScale, int octaves, float persistance, float lacunarity, int seed, Vector2 offset, float spawnThreshold = 0.5f) : base(size, levelOfDetail, noiseScale, octaves, persistance, lacunarity, seed, offset, spawnThreshold)
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
			HeightMap = new float[size, size];
		}

		public override string GetPoolName()
		{
			return PoolName;
		}
	}
}