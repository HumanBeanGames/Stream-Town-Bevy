using System.Collections.Generic;
using Utils;

namespace SavingAndLoading.Structs
{
    /// <summary>
    /// Struct Holding information needed to load buildings from a save file
    /// </summary>
    [System.Serializable]
    public struct BuildingSaveData
    {
        public TransformSaveData BuildingTranform;
        public string BuildingType;
        public int BuildingHealth;
        public uint GUID;
        public BuildingState BuildingState;
        public int Level;
        public List<FoliageSaveData> DestroyedFoliage;
    }
}
