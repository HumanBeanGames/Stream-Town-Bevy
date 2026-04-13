using UnityEngine;

namespace World.Generation
{
	/// <summary>
	/// Holds Settings for Procedural Generation.
	/// </summary>
	[System.Serializable]
	public class GenerationSettings
	{
		public int Size;
		[Range(0, 6)]
		public int LevelOfDetail;
		public float NoiseScale;
		[Range(0, 8)]
		public int Octaves;
		[Range(0, 1)]
		public float Persistance;
		public float Lacunarity;
		public int Seed;
		public Vector2 Offset;
		[Range(0, 1)]
		public float SpawnThreshold = 0.5f;
		[Header("Spawn Bias")]
		public bool EnableSpawnBias = false;
		public AnimationCurve SpawnBiasCurve = AnimationCurve.Linear(0f, 1f, 1f, 0f);
		public float SpawnBiasMultiplier = 1f;
		public float SpawnBiasAddition = 0f;
		public float SpawnBiasMaxDistance = 150f;
		public float[,] HeightMap;
		public int Spacing;

		// Constructor.
		public GenerationSettings(int size, int levelOfDetail, float noiseScale, int octaves, float persistance, float lacunarity, int seed, Vector2 offset, float spawnThreshold = 0.5f)
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
			Spacing = 0;
		}

		/// <summary>
		/// Returns the name of a Pooled Object. For use with child classes.
		/// </summary>
		/// <returns></returns>
		public virtual string GetPoolName()
		{
			return "";
		}
	}
}