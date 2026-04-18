using UnityEngine;
using Utils;
using GameResources;
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

        /// <summary>
        /// Sets the values of ResourceSaveData
        /// </summary>
        /// <param name="transform">The resources transform</param>
        /// <param name="type">Thye resource type</param>
        /// <param name="resourceAmount">The amount of resources left</param>
        /// <param name="poolName">The name of the resource in the object pooler</param>
        public ResourceSaveData (Transform transform, string type, int resourceAmount, uint gUID)
        {
            ResourceTransform = new TransformSaveData(transform);
            ResourceType = type;
            ResourceAmountLeft = resourceAmount;
            GUID = gUID;
        }
    }

    /// <summary>
    /// Struct for batch saving all ResourceData arrays from ResourceProcessor.
    /// Used for data-driven resource system save/load.
    /// </summary>
    [System.Serializable]
    public struct ResourceProcessorSaveData
    {
        public ResourceDataSaveData[] WoodResources;
        public ResourceDataSaveData[] OreResources;
        public ResourceDataSaveData[] FoodResources;
        public ResourceDataSaveData[] GoldResources;
        public ResourceDataSaveData[] RecruitResources;

        public ResourceProcessorSaveData(
            ResourceData[] woodResources,
            ResourceData[] oreResources,
            ResourceData[] foodResources,
            ResourceData[] goldResources,
            ResourceData[] recruitResources)
        {
            WoodResources = null;
            OreResources = null;
            FoodResources = null;
            GoldResources = null;
            RecruitResources = null;

            WoodResources = ConvertToSaveData(woodResources);
            OreResources = ConvertToSaveData(oreResources);
            FoodResources = ConvertToSaveData(foodResources);
            GoldResources = ConvertToSaveData(goldResources);
            RecruitResources = ConvertToSaveData(recruitResources);
        }

        private ResourceDataSaveData[] ConvertToSaveData(ResourceData[] resources)
        {
            if (resources == null || resources.Length == 0)
                return new ResourceDataSaveData[0];

            ResourceDataSaveData[] saveData = new ResourceDataSaveData[resources.Length];
            for (int i = 0; i < resources.Length; i++)
            {
                saveData[i] = new ResourceDataSaveData(resources[i]);
            }
            return saveData;
        }

        public ResourceData[] GetWoodResources() => ConvertFromSaveData(WoodResources, Utils.Resource.Wood);
        public ResourceData[] GetOreResources() => ConvertFromSaveData(OreResources, Utils.Resource.Ore);
        public ResourceData[] GetFoodResources() => ConvertFromSaveData(FoodResources, Utils.Resource.Food);
        public ResourceData[] GetGoldResources() => ConvertFromSaveData(GoldResources, Utils.Resource.Gold);
        public ResourceData[] GetRecruitResources() => ConvertFromSaveData(RecruitResources, Utils.Resource.Recruit);

        private ResourceData[] ConvertFromSaveData(ResourceDataSaveData[] saveData, Utils.Resource resourceType)
        {
            if (saveData == null || saveData.Length == 0)
                return new ResourceData[0];

            ResourceData[] resources = new ResourceData[saveData.Length];
            for (int i = 0; i < saveData.Length; i++)
            {
                // Note: Mesh/Material restoration will be handled by ResourceProcessor after load
                // We save the mesh/material indices, and ResourceProcessor will restore them
                resources[i] = saveData[i].ToResourceData();
            }
            return resources;
        }
    }

    /// <summary>
    /// Serializable version of ResourceData for save/load.
    /// </summary>
    [System.Serializable]
    public struct ResourceDataSaveData
    {
        public float PositionX, PositionY, PositionZ;
        public float Matrix00, Matrix01, Matrix02, Matrix03;
        public float Matrix10, Matrix11, Matrix12, Matrix13;
        public float Matrix20, Matrix21, Matrix22, Matrix23;
        public float Matrix30, Matrix31, Matrix32, Matrix33;
        public int ResourceType;
        public int CurrentAmount;
        public bool IsUnlimited;
        public uint GUID;
        public int MeshIndex;
        public int MaterialIndex;

        public ResourceDataSaveData(GameResources.ResourceData resourceData)
        {
            PositionX = resourceData.Position.x;
            PositionY = resourceData.Position.y;
            PositionZ = resourceData.Position.z;
            Matrix00 = resourceData.Matrix.m00;
            Matrix01 = resourceData.Matrix.m01;
            Matrix02 = resourceData.Matrix.m02;
            Matrix03 = resourceData.Matrix.m03;
            Matrix10 = resourceData.Matrix.m10;
            Matrix11 = resourceData.Matrix.m11;
            Matrix12 = resourceData.Matrix.m12;
            Matrix13 = resourceData.Matrix.m13;
            Matrix20 = resourceData.Matrix.m20;
            Matrix21 = resourceData.Matrix.m21;
            Matrix22 = resourceData.Matrix.m22;
            Matrix23 = resourceData.Matrix.m23;
            Matrix30 = resourceData.Matrix.m30;
            Matrix31 = resourceData.Matrix.m31;
            Matrix32 = resourceData.Matrix.m32;
            Matrix33 = resourceData.Matrix.m33;
            ResourceType = (int)resourceData.ResourceType;
            CurrentAmount = resourceData.CurrentAmount;
            IsUnlimited = resourceData.IsUnlimited;
            GUID = resourceData.GUID;
            MeshIndex = resourceData.MeshIndex;
            MaterialIndex = resourceData.MaterialIndex;
        }

        public ResourceData ToResourceData()
        {
            Matrix4x4 matrix = new Matrix4x4();
            matrix.m00 = Matrix00;
            matrix.m01 = Matrix01;
            matrix.m02 = Matrix02;
            matrix.m03 = Matrix03;
            matrix.m10 = Matrix10;
            matrix.m11 = Matrix11;
            matrix.m12 = Matrix12;
            matrix.m13 = Matrix13;
            matrix.m20 = Matrix20;
            matrix.m21 = Matrix21;
            matrix.m22 = Matrix22;
            matrix.m23 = Matrix23;
            matrix.m30 = Matrix30;
            matrix.m31 = Matrix31;
            matrix.m32 = Matrix32;
            matrix.m33 = Matrix33;

            return new ResourceData(
                new Vector3(PositionX, PositionY, PositionZ),
                (Utils.Resource)ResourceType,
                CurrentAmount,
                IsUnlimited,
                matrix,
                GUID,
                MeshIndex,
                MaterialIndex
            );
        }
    }
}
