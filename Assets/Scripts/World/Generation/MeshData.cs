using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Rendering;

namespace World.Generation
{
	/// <summary>
	/// Holds the Mesh Data for use in World Generation.
	/// </summary>
	[System.Serializable]
	public class MeshData
	{
		public List<Vector3> Vertices;
		public List<int> Triangles;
		public List<Vector2> UVs;

		// Constructor for dynamic allocation (voxel terrain).
		public MeshData()
		{
			Vertices = new List<Vector3>();
			Triangles = new List<int>();
			UVs = new List<Vector2>();
		}

		// Constructor for fixed allocation (legacy terrain).
		public MeshData(int width, int height)
		{
			Vertices = new List<Vector3>(width * height);
			UVs = new List<Vector2>(width * height);
			Triangles = new List<int>((height - 1) * (height - 1) * 6);
		}

		/// <summary>
		/// Adds a vertex to the mesh and returns its index.
		/// </summary>
		public int AddVertex(Vector3 vertex, Vector2 uv)
		{
			int index = Vertices.Count;
			Vertices.Add(vertex);
			UVs.Add(uv);
			return index;
		}

		/// <summary>
		/// Adds a triangle to the mesh.
		/// </summary>
		/// <param name="a"></param>
		/// <param name="b"></param>
		/// <param name="c"></param>
		public void AddTriangle(int a, int b, int c)
		{
			Triangles.Add(a);
			Triangles.Add(b);
			Triangles.Add(c);
		}

		/// <summary>
		/// Returns a generated mesh with calculated normals.
		/// </summary>
		/// <returns></returns>
		public Mesh CreateMesh()
		{
			Mesh mesh = new Mesh();
			if (Vertices.Count > 65535)
				mesh.indexFormat = IndexFormat.UInt32;
			mesh.vertices = Vertices.ToArray();
			mesh.triangles = Triangles.ToArray();
			mesh.uv = UVs.ToArray();
			mesh.RecalculateNormals();
			return mesh;
		}
	}
}