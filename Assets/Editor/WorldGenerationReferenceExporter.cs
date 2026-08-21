using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using UnityEditor;
using UnityEngine;
using Utils;
using World.Generation;

namespace StreamTown.Migration
{
    /// <summary>
    /// Emits reference data from Unity's native Mathf.PerlinNoise implementation.
    /// The neutral converted fixture reproduces the shipping seed without a
    /// Unity runtime dependency. Unity save placements are a separate offline
    /// oracle and are never inputs to this exporter or the Bevy generator.
    /// </summary>
    public static class WorldGenerationReferenceExporter
    {
        private const string OutputArgument = "-streamTownWorldReference";
        private const string SeedArgument = "-streamTownWorldSeed";

        public static void ExportForBatch()
        {
            try
            {
                string destination = Argument(OutputArgument);
                int seed = int.Parse(Argument(SeedArgument), CultureInfo.InvariantCulture);
                Export(destination, seed);
                Debug.Log($"STREAM_TOWN_WORLD_REFERENCE_OK={destination}");
            }
            catch (Exception exception)
            {
                Debug.LogException(exception);
                EditorApplication.Exit(1);
            }
        }

        private static void Export(string destination, int terrainSeed)
        {
            GenerationReference export = new GenerationReference
            {
                SchemaVersion = 1,
                Purpose = "Unity generator reference converted to a neutral Bevy fixture; contains no save placements",
                TerrainSeed = terrainSeed,
                PerlinSamples = PerlinReferenceSamples(),
                Terrain = BuildTerrainReference(terrainSeed),
                Layers = new[]
                {
                    BuildLayerReference("resource:wood", 300, -1165233549, 17f, 6, 0.452f, 22.47f, 0.6f, 2, true),
                    BuildLayerReference("resource:ore", 300, -1165233548, 7f, 1, 1f, 0f, 0.85f, 1, false),
                    BuildLayerReference("resource:food", 300, -1165233547, 7f, 2, 1f, 0f, 0.85f, 1, false),
                    BuildLayerReference("foliage:land:0", 300, -430535522, 10f, 1, 0.847f, 1.53f, 0.6f, 1, false),
                    BuildLayerReference("foliage:land:1", 300, -430535523, 4f, 1, 0.847f, 1.53f, 0.8f, 1, false),
                    BuildLayerReference("foliage:underwater:0", 500, -430535520, 6.68f, 1, 0.8f, 1.53f, 0.7f, 1, false),
                    BuildLayerReference("foliage:underwater:1", 500, -430535519, 6.68f, 1, 0.8f, 1.53f, 0.7f, 1, false),
                }
            };
            string directory = Path.GetDirectoryName(destination);
            if (!string.IsNullOrEmpty(directory))
                Directory.CreateDirectory(directory);
            File.WriteAllText(destination, JsonUtility.ToJson(export, true));
        }

        private static TerrainReference BuildTerrainReference(int seed)
        {
            const int size = 200;
            GenerationSettings settings = new GenerationSettings(size, 0, 50f, 3, 0.827f, 2f, seed, Vector2.zero, 0f);
            float[,] noise = Noise.GenerateNoiseMap(settings);
            float[] heights = new float[size * size];
            float topLeft = (size - 1) * 2f / -2f;
            for (int y = 0; y < size; y++)
            {
                for (int x = 0; x < size; x++)
                {
                    float source = x <= 1 || x >= size - 2 || y <= 1 || y >= size - 2 ? -1f : noise[x, y];
                    float height = SmoothStep01(source);
                    Vector3 position = new Vector3(topLeft + x * 2f, 0f, -topLeft - y * 2f);
                    float normalizedDistance = Mathf.Clamp01(position.magnitude / 200f);
                    float bias = 3f * (1f - SmoothStep01(normalizedDistance));
                    heights[y * size + x] = Mathf.Round(height * bias / 0.5f) * 0.5f;
                }
            }
            return new TerrainReference { Size = size, Heights = heights };
        }

        private static float SmoothStep01(float value)
        {
            if (value < 0f || value > 1f)
                return value;
            return value * value * (3f - 2f * value);
        }

        private static LayerReference BuildLayerReference(
            string id,
            int size,
            int seed,
            float scale,
            int octaves,
            float persistence,
            float lacunarity,
            float threshold,
            int spacing,
            bool woodOffset)
        {
            System.Random random = new System.Random(seed);
            Vector2 offset = new Vector2(
                size * 0.5f + (float)random.NextDouble() * size * 2f,
                size * 0.5f + (float)random.NextDouble() * size * 2f);
            GenerationSettings settings = new GenerationSettings(size, 0, scale, octaves, persistence, lacunarity, seed, offset, threshold)
            {
                Spacing = spacing
            };
            float[,] noise = Noise.GenerateNoiseMap(settings);
            int half = size / 2;
            List<Vector2> candidates = new List<Vector2>();
            for (int y = -half + 2; y < half - 2; y += spacing)
            {
                for (int x = -half + 2; x < half - 2; x += spacing)
                {
                    if (noise[x + half, y + half] < threshold)
                        continue;
                    float centre = spacing * 0.5f;
                    float worldX = y + centre + (woodOffset ? 0.5f : 0f);
                    float worldZ = x + centre + (woodOffset ? 0.5f : 0f);
                    candidates.Add(new Vector2(worldX, worldZ));
                }
            }
            return new LayerReference
            {
                Id = id,
                Size = size,
                Seed = seed,
                Offset = offset,
                NoiseScale = scale,
                Octaves = octaves,
                Persistence = persistence,
                Lacunarity = lacunarity,
                Threshold = threshold,
                Spacing = spacing,
                Candidates = candidates.ToArray(),
            };
        }

        private static PerlinSample[] PerlinReferenceSamples()
        {
            Vector2[] points =
            {
                new Vector2(0f, 0f), new Vector2(0.1f, 0.1f), new Vector2(0.5f, 0.5f),
                new Vector2(1.25f, -2.75f), new Vector2(-0.25f, 0.75f),
                new Vector2(123.456f, 789.012f), new Vector2(-999.75f, 431.125f),
                new Vector2(100000.125f, -99999.875f),
            };
            return points.Select(point => new PerlinSample
            {
                Point = point,
                Value = Mathf.PerlinNoise(point.x, point.y)
            }).ToArray();
        }

        private static string Argument(string name)
        {
            string[] arguments = System.Environment.GetCommandLineArgs();
            for (int index = 0; index + 1 < arguments.Length; index++)
                if (arguments[index] == name)
                    return arguments[index + 1];
            throw new ArgumentException($"Missing required command-line argument {name}");
        }

        [Serializable]
        private sealed class GenerationReference
        {
            public int SchemaVersion;
            public string Purpose;
            public int TerrainSeed;
            public PerlinSample[] PerlinSamples;
            public TerrainReference Terrain;
            public LayerReference[] Layers;
        }

        [Serializable]
        private sealed class PerlinSample
        {
            public Vector2 Point;
            public float Value;
        }

        [Serializable]
        private sealed class TerrainReference
        {
            public int Size;
            public float[] Heights;
        }

        [Serializable]
        private sealed class LayerReference
        {
            public string Id;
            public int Size;
            public int Seed;
            public Vector2 Offset;
            public float NoiseScale;
            public int Octaves;
            public float Persistence;
            public float Lacunarity;
            public float Threshold;
            public int Spacing;
            public Vector2[] Candidates;
        }
    }
}
