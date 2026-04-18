using System.Collections.Generic;
using GameResources;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
	/// <summary>
	/// Data-driven foliage processor for managing world foliage.
	/// All runtime state is stored in FoliageRuntimeData.
	/// </summary>
	public partial class FoliageProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// Nested runtime data class for foliage data.
		/// </summary>
		public class RuntimeData
		{
			/// <summary>
			/// List of on-land foliage data.
			/// </summary>
			public List<FoliageData> OnLandFoliage { get; set; } = new List<FoliageData>();

			/// <summary>
			/// List of underwater foliage data.
			/// </summary>
			public List<FoliageData> UnderWaterFoliage { get; set; } = new List<FoliageData>();

			/// <summary>
			/// Cached on-land foliage data grouped by mesh and material.
			/// </summary>
			public Dictionary<(Mesh mesh, Material material), FoliageData[]> OnLandFoliageCache { get; set; }

			/// <summary>
			/// Cached underwater foliage data grouped by mesh and material.
			/// </summary>
			public Dictionary<(Mesh mesh, Material material), FoliageData[]> UnderWaterFoliageCache { get; set; }

			/// <summary>
			/// Cached transformation matrices for on-land foliage.
			/// </summary>
			public Dictionary<(Mesh mesh, Material material), Matrix4x4[]> OnLandMatricesCache { get; set; }

			/// <summary>
			/// Cached transformation matrices for underwater foliage.
			/// </summary>
			public Dictionary<(Mesh mesh, Material material), Matrix4x4[]> UnderWaterMatricesCache { get; set; }
		}

		private readonly RuntimeData _foliageRuntimeData = new RuntimeData();

		/// <summary>
		/// Registers this processor as a singleton in the dependency injection container.
		/// Called by Reflex during container initialization.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}

		/// <summary>
		/// Initializes the processor by loading and caching foliage data from the ScriptableObject.
		/// Groups foliage by mesh/material and pre-calculates transformation matrices.
		/// </summary>
		private void InitializeFromScriptableObject()
		{
			// Check if foliage runtime data is available
			if (_foliageRuntimeData != null)
			{
				var data = _foliageRuntimeData;
				// Group on-land foliage by mesh and material for efficient instancing
				data.OnLandFoliageCache = GroupByMeshAndMaterial(data.OnLandFoliage.ToArray());
				// Group underwater foliage by mesh and material for efficient instancing
				data.UnderWaterFoliageCache = GroupByMeshAndMaterial(data.UnderWaterFoliage.ToArray());
				// Pre-calculate transformation matrices for on-land foliage
				data.OnLandMatricesCache = BuildMatricesDictionary(data.OnLandFoliageCache);
				// Pre-calculate transformation matrices for underwater foliage
				data.UnderWaterMatricesCache = BuildMatricesDictionary(data.UnderWaterFoliageCache);
			}
		}

		/// <summary>
		/// Groups foliage data by mesh and material for efficient GPU instancing.
		/// Foliage with the same mesh and material can be rendered in a single draw call.
		/// </summary>
		/// <param name="foliage">Array of foliage data to group.</param>
		/// <returns>Dictionary mapping (mesh, material) tuples to arrays of foliage data.</returns>
		private Dictionary<(Mesh mesh, Material material), FoliageData[]> GroupByMeshAndMaterial(FoliageData[] foliage)
		{
			Dictionary<(Mesh, Material), List<FoliageData>> grouped = new Dictionary<(Mesh, Material), List<FoliageData>>();

			// Group foliage by mesh and material combination
			foreach (var f in foliage)
			{
				// Skip foliage without valid mesh or material
				if (f.Mesh == null || f.Material == null)
					continue;

				var key = (f.Mesh, f.Material);

				// Create new list if this combination doesn't exist yet
				if (!grouped.ContainsKey(key))
					grouped[key] = new List<FoliageData>();

				// Add foliage to the appropriate group
				grouped[key].Add(f);
			}

			// Convert lists to arrays for final result
			Dictionary<(Mesh, Material), FoliageData[]> result = new Dictionary<(Mesh, Material), FoliageData[]>();
			foreach (var kvp in grouped)
				result[kvp.Key] = kvp.Value.ToArray();

			return result;
		}

		/// <summary>
		/// Initializes the foliage processor.
		/// Loads and caches foliage data from the ScriptableObject.
		/// </summary>
		public void Initialize()
		{
			InitializeFromScriptableObject();
		}

		/// <summary>
		/// Builds a dictionary of transformation matrices for GPU instancing.
		/// Converts position, rotation, and scale data into TRS matrices.
		/// </summary>
		/// <param name="foliageDict">Dictionary of foliage data grouped by mesh and material.</param>
		/// <returns>Dictionary mapping (mesh, material) tuples to arrays of transformation matrices.</returns>
		private Dictionary<(Mesh mesh, Material material), Matrix4x4[]> BuildMatricesDictionary(Dictionary<(Mesh mesh, Material material), FoliageData[]> foliageDict)
		{
			Dictionary<(Mesh, Material), Matrix4x4[]> matricesDict = new Dictionary<(Mesh, Material), Matrix4x4[]>();

			// Build transformation matrices for each foliage group
			foreach (var kvp in foliageDict)
			{
				var key = kvp.Key;
				FoliageData[] foliage = kvp.Value;

				// Create array of matrices for this foliage group
				Matrix4x4[] matrices = new Matrix4x4[foliage.Length];
				for (int i = 0; i < foliage.Length; i++)
					// Build TRS (Translation, Rotation, Scale) matrix for each foliage instance
					matrices[i] = Matrix4x4.TRS(foliage[i].Position, foliage[i].Rotation, foliage[i].Scale);

				matricesDict[key] = matrices;
			}

			return matricesDict;
		}

		/// <summary>
		/// Gets the on-land foliage list.
		/// </summary>
		/// <returns>List of on-land foliage data.</returns>
		public List<FoliageData> GetOnLandFoliage() => _foliageRuntimeData.OnLandFoliage;

		/// <summary>
		/// Gets the underwater foliage list.
		/// </summary>
		/// <returns>List of underwater foliage data.</returns>
		public List<FoliageData> GetUnderWaterFoliage() => _foliageRuntimeData.UnderWaterFoliage;

		/// <summary>
		/// Gets the cached on-land foliage data grouped by mesh and material.
		/// </summary>
		/// <returns>Dictionary of on-land foliage data.</returns>
		public Dictionary<(Mesh mesh, Material material), FoliageData[]> GetOnLandFoliageCache() => _foliageRuntimeData.OnLandFoliageCache;

		/// <summary>
		/// Gets the cached underwater foliage data grouped by mesh and material.
		/// </summary>
		/// <returns>Dictionary of underwater foliage data.</returns>
		public Dictionary<(Mesh mesh, Material material), FoliageData[]> GetUnderWaterFoliageCache() => _foliageRuntimeData.UnderWaterFoliageCache;

		/// <summary>
		/// Gets the cached transformation matrices for on-land foliage.
		/// </summary>
		/// <returns>Dictionary of on-land transformation matrices.</returns>
		public Dictionary<(Mesh mesh, Material material), Matrix4x4[]> GetOnLandMatrices() => _foliageRuntimeData.OnLandMatricesCache;

		/// <summary>
		/// Gets the cached transformation matrices for underwater foliage.
		/// </summary>
		/// <returns>Dictionary of underwater transformation matrices.</returns>
		public Dictionary<(Mesh mesh, Material material), Matrix4x4[]> GetUnderWaterMatrices() => _foliageRuntimeData.UnderWaterMatricesCache;

		/// <summary>
		/// Processes foliage logic every frame.
		/// Called every frame by the Coordinator.
		/// FoliageProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			// FoliageProcessor does not require per-frame updates
		}

	}
}
