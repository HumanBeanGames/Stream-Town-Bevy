using System;
using System.Collections.Generic;

namespace SavingAndLoading.Structs
{
    /// <summary>
    /// Struct for holding information for each resource
    /// </summary>
    [System.Serializable]
    public struct ResourceSaveData
    {
        public TransformSaveData ResourceTransform;
        public string ResourceType;
        public int ResourceAmountLeft;
        public uint GUID;

    }

    /// <summary>Raw snapshot of all generated world-resource groups.</summary>
    [Serializable]
    public struct ResourceProcessorSaveData
    {
        public List<ResourceGroupSaveData> Groups;
    }

    /// <summary>
    /// ResourceType is a stable enum name, rather than its numeric position.
    /// Instances contain no Unity objects or runtime behaviour.
    /// </summary>
    [Serializable]
    public struct ResourceGroupSaveData
    {
        public string ResourceType;
        public List<ResourceDataSaveData> Instances;
    }

    /// <summary>
    /// Serializable version of ResourceData for save/load.
    /// </summary>
    [Serializable]
    public struct ResourceDataSaveData
    {
        public float PositionX, PositionY, PositionZ;
		public int CurrentAmount;
		public bool IsUnlimited;
		public uint GUID;

		// Schema 1 visual payload. Schema 2 derives all of these from the resource
		// group's type and the stable hash of PositionX/PositionZ.
        public float Matrix00, Matrix01, Matrix02, Matrix03;
        public float Matrix10, Matrix11, Matrix12, Matrix13;
        public float Matrix20, Matrix21, Matrix22, Matrix23;
        public float Matrix30, Matrix31, Matrix32, Matrix33;
        public string ResourceType;
        public int MeshIndex;
        public int MaterialIndex;
    }
}
