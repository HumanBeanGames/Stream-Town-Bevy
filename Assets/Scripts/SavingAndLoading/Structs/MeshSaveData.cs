namespace SavingAndLoading.Structs
{
    /// <summary>
    /// Struct for converting Unitys Mesh into a custom data container
    /// </summary>
    [System.Serializable]
    public struct MeshSaveData
    {
        public Vector3SaveData[] Verticies;
        public int[] Triangles;
        public Vector2SaveData[] UVs;
		public bool Uses32BitIndices;

        /// <summary>
        /// Overloaded constructor,
        /// Takes in seperate Mesh information
        /// </summary>
        /// <param name="verts">The mesh verticies</param>
        /// <param name="tris">The mesh triangles</param>
        /// <param name="uvs">The mesh UVs</param>
        public MeshSaveData(Vector3SaveData[] verts, int[] tris, Vector2SaveData[] uvs)
        {
            Verticies = verts;
            Triangles = tris;
            UVs = uvs;
			Uses32BitIndices = verts != null && verts.Length > ushort.MaxValue;
        }

    }
}
