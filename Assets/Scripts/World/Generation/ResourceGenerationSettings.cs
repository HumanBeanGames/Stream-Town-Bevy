using System.Collections.Generic;
using UnityEngine;
using Utils;

namespace World.Generation
{
	/// <summary>
	/// Holds the Generation Settings Data for Resource Generation.
	/// </summary>
	[System.Serializable]
	public class ResourceGenerationSettings : GenerationSettings
	{
		public TargetMask TargetType;

		public List<Mesh> Meshes;
		public List<Material> Materials;

		[Header("Distance-Based Amount Settings")]
		public bool SetByDistance = false;
		public AnimationCurve AmountCurve;
		public int MinAmount = 50;
		public int MaxAmount = 100;
		public int MaxDistance = 150;

		public ResourceGenerationSettings(int size, int levelOfDetail, float noiseScale, int octaves, float persistance, float lacunarity, int seed, Vector2 offset, float spawnThreshold = 0.5f, List<Mesh> meshes = null, List<Material> materials = null)
			: base(size, levelOfDetail, noiseScale, octaves, persistance, lacunarity, seed, offset, spawnThreshold)
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
			Materials = materials ?? new List<Material>();
			HeightMap = new float[size, size];
		}
	}
}
