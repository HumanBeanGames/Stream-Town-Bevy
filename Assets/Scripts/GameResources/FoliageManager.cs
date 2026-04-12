using System.Collections.Generic;
using Reflex.Core;
using UnityEngine;

namespace GameResources
{
	/// <summary>
	/// Data-driven foliage manager for managing world foliage using GPU instancing.
	/// Similar to ResourceManager but for non-resource foliage (grass, flowers, seaweed, corals, etc.).
	/// </summary>
	public class FoliageManager : MonoBehaviour, IInstaller
	{
		private Dictionary<(Mesh mesh, Material material), FoliageData[]> _onLandFoliage;
		private Dictionary<(Mesh mesh, Material material), FoliageData[]> _underWaterFoliage;

		private Dictionary<(Mesh mesh, Material material), Matrix4x4[]> _onLandMatrices;
		private Dictionary<(Mesh mesh, Material material), Matrix4x4[]> _underWaterMatrices;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}

		public void SetOnLandFoliage(FoliageData[] foliage)
		{
			_onLandFoliage = GroupByMeshAndMaterial(foliage);
			_onLandMatrices = BuildMatricesDictionary(_onLandFoliage);
		}

		public void SetUnderWaterFoliage(FoliageData[] foliage)
		{
			_underWaterFoliage = GroupByMeshAndMaterial(foliage);
			_underWaterMatrices = BuildMatricesDictionary(_underWaterFoliage);
		}

		private Dictionary<(Mesh mesh, Material material), FoliageData[]> GroupByMeshAndMaterial(FoliageData[] foliage)
		{
			Dictionary<(Mesh, Material), List<FoliageData>> grouped = new Dictionary<(Mesh, Material), List<FoliageData>>();

			foreach (var f in foliage)
			{
				if (f.Mesh == null || f.Material == null)
					continue;

				var key = (f.Mesh, f.Material);

				if (!grouped.ContainsKey(key))
					grouped[key] = new List<FoliageData>();

				grouped[key].Add(f);
			}

			Dictionary<(Mesh, Material), FoliageData[]> result = new Dictionary<(Mesh, Material), FoliageData[]>();
			foreach (var kvp in grouped)
				result[kvp.Key] = kvp.Value.ToArray();

			return result;
		}

		private Dictionary<(Mesh mesh, Material material), Matrix4x4[]> BuildMatricesDictionary(Dictionary<(Mesh mesh, Material material), FoliageData[]> foliageDict)
		{
			Dictionary<(Mesh, Material), Matrix4x4[]> matricesDict = new Dictionary<(Mesh, Material), Matrix4x4[]>();

			foreach (var kvp in foliageDict)
			{
				var key = kvp.Key;
				FoliageData[] foliage = kvp.Value;

				Matrix4x4[] matrices = new Matrix4x4[foliage.Length];
				for (int i = 0; i < foliage.Length; i++)
					matrices[i] = Matrix4x4.TRS(foliage[i].Position, foliage[i].Rotation, foliage[i].Scale);

				matricesDict[key] = matrices;
			}

			return matricesDict;
		}

		public Dictionary<(Mesh mesh, Material material), FoliageData[]> GetOnLandFoliage() => _onLandFoliage;
		public Dictionary<(Mesh mesh, Material material), FoliageData[]> GetUnderWaterFoliage() => _underWaterFoliage;

		public Dictionary<(Mesh mesh, Material material), Matrix4x4[]> GetOnLandMatrices() => _onLandMatrices;
		public Dictionary<(Mesh mesh, Material material), Matrix4x4[]> GetUnderWaterMatrices() => _underWaterMatrices;
	}
}
