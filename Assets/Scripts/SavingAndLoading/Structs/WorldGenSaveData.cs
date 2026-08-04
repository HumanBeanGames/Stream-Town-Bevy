using System.Collections.Generic;

namespace SavingAndLoading.Structs
{
    /// <summary>
    /// Struct Holding information needed to load the world generation from a save file
    /// </summary>
    [System.Serializable]
    public struct WorldGenSaveData
    {
        public const int CurrentTerrainGeneratorVersion = 1;

        // Schema 2 worlds normally regenerate terrain from these few values.
        // Legacy worlds retain MapMesh until they are generated from a known seed.
        public bool HasTerrainSeed;
        public int TerrainSeed;
        public int TerrainGeneratorVersion;
        public MeshSaveData MapMesh;

        public ResourceProcessorSaveData Resources;
        public FoliageProcessorSaveData Foliage;
        public List<EnemyCampSaveData> EnemyCamps;

        /// <summary>
        /// Sets the values of WorldSaveData
        /// </summary>
        /// <param name="mesh">The words mesh data</param>
        /// <param name="resources">The worlds resources</param>
        /// <param name="foliage">The worlds foliage</param>
        /// <param name="camps">The worlds camps</param>
        
        public WorldGenSaveData(bool hasTerrainSeed, int terrainSeed, MeshSaveData mesh, ResourceProcessorSaveData resources, FoliageProcessorSaveData foliage, List<EnemyCampSaveData> camps)
        {   
            HasTerrainSeed = hasTerrainSeed;
            TerrainSeed = terrainSeed;
            TerrainGeneratorVersion = CurrentTerrainGeneratorVersion;
            MapMesh = mesh;
            Resources = resources;
            Foliage = foliage;
            EnemyCamps = camps;
        }
    }
}
