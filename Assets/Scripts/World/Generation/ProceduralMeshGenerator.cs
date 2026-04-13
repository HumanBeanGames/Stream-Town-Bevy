using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Utils;

namespace World.Generation
{
	/// <summary>
	/// Corner position enum for top face vertices
	/// </summary>
	public enum Corner
	{
		TopLeft,
		TopRight,
		BottomLeft,
		BottomRight
	}

	/// <summary>
	/// Struct to store the four corner positions of a top face
	/// </summary>
	public struct TopFace
	{
		public Vector3 TopLeft;
		public Vector3 TopRight;
		public Vector3 BottomLeft;
		public Vector3 BottomRight;

		public Vector3 this[Corner corner]
		{
			get
			{
				switch (corner)
				{
					case Corner.TopLeft: return TopLeft;
					case Corner.TopRight: return TopRight;
					case Corner.BottomLeft: return BottomLeft;
					case Corner.BottomRight: return BottomRight;
					default: return Vector3.zero;
				}
			}
			set
			{
				switch (corner)
				{
					case Corner.TopLeft: TopLeft = value; break;
					case Corner.TopRight: TopRight = value; break;
					case Corner.BottomLeft: BottomLeft = value; break;
					case Corner.BottomRight: BottomRight = value; break;
				}
			}
		}
	}

	/// <summary>
	/// Used for generating meshes from a height map
	/// </summary>
	public static class ProceduralMeshGenerator
	{
		/// <summary>
		/// Generates Mesh Data from a GenerationSettings profile.
		/// </summary>
		/// <param name="settings"></param>
		/// <param name="meshHeightMultiplier"></param>
		/// <param name="meshHeightCurve"></param>
		/// <param name="enableIslandBias"></param>
		/// <param name="islandBiasCurve"></param>
		/// <param name="islandSize"></param>
		/// <param name="islandMultiplier"></param>
		/// <param name="islandAddition"></param>
		/// <param name="quantizationFactor"></param>
		/// <returns></returns>
		public static MeshData GenerateTerrainMeshData(GenerationSettings settings, float meshHeightMultiplier, AnimationCurve meshHeightCurve, bool enableIslandBias = false, AnimationCurve islandBiasCurve = null, float islandSize = 150f, float islandMultiplier = 1f, float islandAddition = 0f, float quantizationFactor = 0.1f, float topFaceProportion = 1f)
		{
			float[,] noiseMap = Noise.GenerateNoiseMap(settings);

			settings.HeightMap = new float[settings.Size, settings.Size];

			int dimension1 = noiseMap.GetLength(0) - 1;
			int dimension2 = noiseMap.GetLength(1) - 1;

			//Transform edge values to be 0
			Func<int, int, float> floatValueSetter = (i1, i2) =>
			{
				return (i1 <= 1 || i1 >= dimension1 - 1 || i2 <=  1 || i2 >= dimension2 - 1) ? -1 : noiseMap[i1, i2];
			};

			MathExtended.Set2DArrayValues<float>(ref noiseMap, floatValueSetter);

			float topLeftX = (settings.Size - 1) / -2f;
			float topLeftZ = (settings.Size - 1) / 2f;

			int meshSimplificationIncrement = (settings.LevelOfDetail == 0) ? 1 : settings.LevelOfDetail * 2;

			MeshData meshData = new MeshData();
			TopFace[,] topFaces = new TopFace[settings.Size, settings.Size];

			for (int y = 0; y < settings.Size; y += meshSimplificationIncrement)
			{
				for (int x = 0; x < settings.Size; x += meshSimplificationIncrement)
				{
					float height = meshHeightCurve.Evaluate(noiseMap[x, y]);

					// Apply island bias if enabled
					if (enableIslandBias && islandBiasCurve != null)
					{
						Vector3 vertexPosition = new Vector3(topLeftX + x, 0, topLeftZ - y);
						float distanceFromOrigin = vertexPosition.magnitude;
						float normalizedDistance = Mathf.Clamp01(distanceFromOrigin / islandSize);
						float bias = islandBiasCurve.Evaluate(normalizedDistance);
						height = (height * bias * islandMultiplier) + (bias * islandAddition);
					}

					// Quantize height to nearest unit
					height = Mathf.Round(height / quantizationFactor) * quantizationFactor;

					settings.HeightMap[x, y] = height;

					// Generate voxel-style terrain
					GenerateVoxelPixel(meshData, topFaces, x, y, height, settings, meshHeightMultiplier, topLeftX, topLeftZ, topFaceProportion);
				}
			}

			return meshData;
		}

		private static void GenerateVoxelPixel(MeshData meshData, TopFace[,] topFaces, int x, int y, float height, GenerationSettings settings, float meshHeightMultiplier, float topLeftX, float topLeftZ, float topFaceProportion)
		{
			float worldX = topLeftX + x;
			float worldZ = topLeftZ - y;
			float worldHeight = height * meshHeightMultiplier;
			float halfSize = 0.5f * topFaceProportion;
			
			// UV coordinates for this pixel
			Vector2 uv = new Vector2(x / (float)settings.Size, y / (float)settings.Size);

			// Create top face struct
			TopFace topFace = new TopFace
			{
				TopLeft = new Vector3(worldX - halfSize, worldHeight, worldZ + halfSize),
				TopRight = new Vector3(worldX + halfSize, worldHeight, worldZ + halfSize),
				BottomLeft = new Vector3(worldX - halfSize, worldHeight, worldZ - halfSize),
				BottomRight = new Vector3(worldX + halfSize, worldHeight, worldZ - halfSize)
			};

			// Store in array for side face generation
			topFaces[x, y] = topFace;

			// Add vertices and get indices
			Dictionary<Corner, int> cornerIndices = new Dictionary<Corner, int>();
			cornerIndices[Corner.TopLeft] = meshData.AddVertex(topFace.TopLeft, uv);
			cornerIndices[Corner.TopRight] = meshData.AddVertex(topFace.TopRight, uv);
			cornerIndices[Corner.BottomLeft] = meshData.AddVertex(topFace.BottomLeft, uv);
			cornerIndices[Corner.BottomRight] = meshData.AddVertex(topFace.BottomRight, uv);

			// Top face triangles (2 triangles forming a square)
			meshData.AddTriangle(cornerIndices[Corner.TopLeft], cornerIndices[Corner.TopRight], cornerIndices[Corner.BottomLeft]);
			meshData.AddTriangle(cornerIndices[Corner.BottomLeft], cornerIndices[Corner.TopRight], cornerIndices[Corner.BottomRight]);

			// Generate side faces (left and down to avoid duplicates)
			if (x > 0)
				GenerateFace(meshData, topFaces[x, y], topFaces[x - 1, y], Corner.BottomLeft, Corner.TopLeft, Corner.BottomRight, Corner.TopRight, uv, isRowConnection: false);  // Left
			if (y > 0)
				GenerateFace(meshData, topFaces[x, y], topFaces[x, y - 1], Corner.TopRight, Corner.TopLeft, Corner.BottomRight, Corner.BottomLeft, uv, isRowConnection: true);  // Down (top to bottom in noise map)

			// Generate corner face at the intersection of left/up side faces
			if (x > 0 && y > 0)
				GenerateCornerFace(meshData, topFaces[x, y], topFaces[x - 1, y], topFaces[x, y - 1], topFaces[x - 1, y - 1], Corner.TopLeft, Corner.TopRight, Corner.BottomLeft, Corner.BottomRight, uv);
		}

		private static void GenerateFace(MeshData meshData, TopFace currentTopFace, TopFace neighborTopFace, Corner currentCorner1, Corner currentCorner2, Corner neighborCorner1, Corner neighborCorner2, Vector2 uv, bool isRowConnection = false)
		{
			// Skip edge pixels (negative height from -1 noise value)
			if (neighborTopFace.TopLeft.y < 0)
				return;
			
			// Get the four corner positions
			Vector3 c1 = currentTopFace[currentCorner1];
			Vector3 c2 = currentTopFace[currentCorner2];
			Vector3 n1 = neighborTopFace[neighborCorner1];
			Vector3 n2 = neighborTopFace[neighborCorner2];

			// Add vertices
			int i1 = meshData.AddVertex(c1, uv);
			int i2 = meshData.AddVertex(n1, uv);
			int i3 = meshData.AddVertex(c2, uv);
			int i4 = meshData.AddVertex(n2, uv);

			// Add two triangles with appropriate winding order
			if (isRowConnection)
			{
				meshData.AddTriangle(i1, i3, i2);
				meshData.AddTriangle(i2, i3, i4);
			}
			else
			{
				meshData.AddTriangle(i1, i2, i3);
				meshData.AddTriangle(i3, i2, i4);
			}
		}

		private static void GenerateCornerFace(MeshData meshData, TopFace currentTopFace, TopFace leftNeighbor, TopFace rowNeighbor, TopFace diagonalNeighbor, Corner currentCorner, Corner leftCorner, Corner rowCorner, Corner diagonalCorner, Vector2 uv)
		{
			// Skip edge pixels (negative height from -1 noise value)
			if (leftNeighbor.TopLeft.y < 0 || rowNeighbor.TopLeft.y < 0 || diagonalNeighbor.TopLeft.y < 0)
				return;
			
			// Get the four corner positions
			Vector3 c = currentTopFace[currentCorner];
			Vector3 l = leftNeighbor[leftCorner];
			Vector3 r = rowNeighbor[rowCorner];
			Vector3 diag = diagonalNeighbor[diagonalCorner];

			// Add vertices
			int i1 = meshData.AddVertex(c, uv);
			int i2 = meshData.AddVertex(l, uv);
			int i3 = meshData.AddVertex(r, uv);
			int i4 = meshData.AddVertex(diag, uv);

			// Add two triangles to fill the corner hole
			meshData.AddTriangle(i1, i2, i3);
			meshData.AddTriangle(i2, i4, i3);
		}

		public static IEnumerator GenerateTerrainMeshDataCoroutine(GenerationSettings settings, float meshHeightMultiplier, AnimationCurve meshHeightCurve, float frameBudgetSeconds, Action<MeshData> onComplete, bool enableIslandBias = false, AnimationCurve islandBiasCurve = null, float islandSize = 150f, float islandMultiplier = 1f, float islandAddition = 0f, float quantizationFactor = 0.1f, float topFaceProportion = 1f)
		{
			float[,] noiseMap = null;
			yield return Noise.GenerateNoiseMapCoroutine(settings, frameBudgetSeconds, result => noiseMap = result);

			settings.HeightMap = new float[settings.Size, settings.Size];
			int dimension1 = noiseMap.GetLength(0) - 1;
			int dimension2 = noiseMap.GetLength(1) - 1;
			float frameStartTime = Time.realtimeSinceStartup;

			for (int y = 0; y < settings.Size; y++)
			{
				for (int x = 0; x < settings.Size; x++)
				{
					if (x <= 1 || x >= dimension1 - 1 || y <= 1 || y >= dimension2 - 1)
						noiseMap[x, y] = -1f;

					if (Time.realtimeSinceStartup - frameStartTime >= frameBudgetSeconds)
					{
						frameStartTime = Time.realtimeSinceStartup;
						yield return null;
					}
				}
			}

			float topLeftX = (settings.Size - 1) / -2f;
			float topLeftZ = (settings.Size - 1) / 2f;

			int meshSimplificationIncrement = (settings.LevelOfDetail == 0) ? 1 : settings.LevelOfDetail * 2;

			MeshData meshData = new MeshData();
			TopFace[,] topFaces = new TopFace[settings.Size, settings.Size];

			for (int y = 0; y < settings.Size; y += meshSimplificationIncrement)
			{
				for (int x = 0; x < settings.Size; x += meshSimplificationIncrement)
				{
					float height = meshHeightCurve.Evaluate(noiseMap[x, y]);

					// Apply island bias if enabled
					if (enableIslandBias && islandBiasCurve != null)
					{
						Vector3 vertexPosition = new Vector3(topLeftX + x, 0, topLeftZ - y);
						float distanceFromOrigin = vertexPosition.magnitude;
						float normalizedDistance = Mathf.Clamp01(distanceFromOrigin / islandSize);
						float bias = islandBiasCurve.Evaluate(normalizedDistance);
						height = (height * bias * islandMultiplier) + (bias * islandAddition);
					}

					// Quantize height to nearest unit
					height = Mathf.Round(height / quantizationFactor) * quantizationFactor;

					settings.HeightMap[x, y] = height;

					// Generate voxel-style terrain
					GenerateVoxelPixel(meshData, topFaces, x, y, height, settings, meshHeightMultiplier, topLeftX, topLeftZ, topFaceProportion);

					if (Time.realtimeSinceStartup - frameStartTime >= frameBudgetSeconds)
					{
						frameStartTime = Time.realtimeSinceStartup;
						yield return null;
					}
				}
			}

			onComplete?.Invoke(meshData);
		}

		/// <summary>
		/// Converts MeshData into a Mesh and applies it to a GameObject.
		/// </summary>
		/// <param name="data"></param>
		/// <param name="terrainObject"></param>
		public static Mesh CreateMesh(MeshData data, GameObject terrainObject)
		{
			return CreateMesh(data.CreateMesh(), terrainObject);
		}

		/// <summary>
		/// Applies mesh to a GameObject.
		/// </summary>
		/// <param name="data"></param>
		/// <param name="terrainObject"></param>
		public static Mesh CreateMesh(Mesh mesh, GameObject terrainObject)
		{
			MeshFilter filter;

			if (!terrainObject.TryGetComponent(out filter))
			{
				filter = terrainObject.AddComponent<MeshFilter>();
			}
			filter.sharedMesh = mesh;

			MeshCollider collider;

			if (!terrainObject.TryGetComponent(out collider))
			{
				collider = terrainObject.AddComponent<MeshCollider>();
			}

			collider.sharedMesh = mesh;
			return mesh;
		}
	}
}
