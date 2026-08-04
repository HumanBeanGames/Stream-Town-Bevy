using System;
using System.Collections.Generic;

namespace SavingAndLoading.Structs
{
	/// <summary>
	/// Raw GPU-foliage instance. SettingsId selects immutable mesh/material
	/// settings and MeshIndex selects the variant inside those settings.
	/// </summary>
	[Serializable]
	public struct FoliageInstanceSaveData
	{
		public TransformSaveData Transform;
		public string SettingsId;
		public int MeshIndex;
	}

	/// <summary>
	/// Schema 2 foliage stores its immutable settings identity once and only the
	/// positions of its instances. Mesh, rotation and scale are deterministic.
	/// </summary>
	[Serializable]
	public struct FoliageGroupSaveData
	{
		public string SettingsId;
		public List<Vector3SaveData> Positions;
	}

	[Serializable]
	public struct FoliageProcessorSaveData
	{
		public List<FoliageGroupSaveData> OnLandGroups;
		public List<FoliageGroupSaveData> UnderWaterGroups;

		// Schema 1 compatibility. New saves leave these collections null.
		public List<FoliageInstanceSaveData> OnLand;
		public List<FoliageInstanceSaveData> UnderWater;
	}
}
