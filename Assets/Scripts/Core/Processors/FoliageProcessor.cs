using System;
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
	/// Replaces object-based foliage with GPU instancing via FoliageRenderer.
	/// All runtime state is stored in FoliageRuntimeData.
	/// </summary>
	public partial class FoliageProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// Runtime data for foliage data.
		/// Assigned in InjectRuntimeData.
		/// </summary>
		private FoliageRuntimeData _foliageRuntimeData;

		/// <summary>
		/// Cell space partitioning for efficient spatial foliage queries.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private GridSystem.Partitioning.CellSpacePartitioning _cellSpacePartitioning;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		/// <summary>
		/// Set of foliage indices that have been removed.
		/// Used for efficient on-the-fly removal without rebuilding lists.
		/// </summary>
		private HashSet<int> _removedOnLandFoliageIndices = new HashSet<int>();
		private HashSet<int> _removedUnderWaterFoliageIndices = new HashSet<int>();

		/// <summary>
		/// Registers this processor as a singleton in the dependency injection container.
		/// Called by Reflex during container initialization.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
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
			if (_foliageRuntimeData == null)
				throw new InvalidOperationException("FoliageProcessor: RuntimeData has not been installed.");

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
		/// Sets generated foliage data and rebuilds caches for GPU instancing.
		/// </summary>
		public void SetGeneratedFoliage(List<FoliageData> onLandFoliage, List<FoliageData> underWaterFoliage)
		{
			_removedOnLandFoliageIndices.Clear();
			_removedUnderWaterFoliageIndices.Clear();
			_foliageRuntimeData.OnLandFoliage = onLandFoliage ?? new List<FoliageData>();
			_foliageRuntimeData.UnderWaterFoliage = underWaterFoliage ?? new List<FoliageData>();
			_foliageRuntimeData.OnLandFoliageCache = GroupByMeshAndMaterial(_foliageRuntimeData.OnLandFoliage.ToArray());
			_foliageRuntimeData.UnderWaterFoliageCache = GroupByMeshAndMaterial(_foliageRuntimeData.UnderWaterFoliage.ToArray());
			_foliageRuntimeData.OnLandMatricesCache = BuildMatricesDictionary(_foliageRuntimeData.OnLandFoliageCache);
			_foliageRuntimeData.UnderWaterMatricesCache = BuildMatricesDictionary(_foliageRuntimeData.UnderWaterFoliageCache);
		}

		/// <summary>
		/// Clears generated foliage and every derived render cache when a world is
		/// abandoned. The destination scene can then provide its own authored
		/// foliage without project-lifetime data leaking across the scene boundary.
		/// </summary>
		public void ResetWorldState()
		{
			SetGeneratedFoliage(new List<FoliageData>(), new List<FoliageData>());
		}

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

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// FoliageProcessor does not have scene-specific settings to refresh
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_foliageRuntimeData != null)
				throw new InvalidOperationException("FoliageProcessor: RuntimeData has already been installed.");

			_foliageRuntimeData = new FoliageRuntimeData();
			containerBuilder.AddSingleton(_foliageRuntimeData);
		}

		/// <summary>
		/// Checks if any foliage exists within the specified bounds.
		/// Uses CellSpacePartitioning for efficient spatial queries.
		/// </summary>
		/// <param name="bounds">The bounds to check for foliage.</param>
		/// <returns>True if any foliage is within the bounds, false otherwise.</returns>
		public bool HasFoliageInBounds(Bounds bounds)
		{
			if (_cellSpacePartitioning == null)
				return false;

			// Calculate radius from bounds (use max extent)
			float radius = Mathf.Max(bounds.extents.x, bounds.extents.z);
			Vector3 center = bounds.center;

			// Check both on-land and underwater foliage using spatial partitioning
			var foliageTypes = new[] { false, true }; // false = on-land, true = underwater

			foreach (var isUnderwater in foliageTypes)
			{
				List<GameResources.FoliageData> foliageInRange = new List<GameResources.FoliageData>();
				_cellSpacePartitioning.GetFoliageInRange(center, radius, isUnderwater, ref foliageInRange);

				foreach (var foliage in foliageInRange)
				{
					if (bounds.Contains(foliage.Position))
						return true;
				}
			}

			return false;
		}

		/// <summary>
		/// Removes foliage within the specified bounds.
		/// Uses CellSpacePartitioning for efficient spatial queries.
		/// Marks foliage as removed without rebuilding lists for on-the-fly performance.
		/// </summary>
		/// <param name="bounds">The bounds to remove foliage from.</param>
		public void RemoveFoliageInBounds(Bounds bounds)
		{
			if (_cellSpacePartitioning == null)
			{
				_debugProcessor.LogWarning(DebugLogCategory.FoliageProcessor, "CellSpacePartitioning is null, cannot remove foliage");
				return;
			}

			// Get cells that overlap the bounds
			List<GridSystem.Partitioning.BSPCell> overlappingCells = new List<GridSystem.Partitioning.BSPCell>();
			float radius = Mathf.Max(bounds.extents.x, bounds.extents.z);
			_cellSpacePartitioning.GetCellsInRange(bounds.center, radius, ref overlappingCells);

			// Mark foliage as removed from overlapping cells (O(k) where k is foliage in range)
			int onLandRemoved = 0;
			int underWaterRemoved = 0;

			foreach (var cell in overlappingCells)
			{
				// Mark on-land foliage in this cell
				if (cell.OnLandFoliageIndices != null)
				{
					foreach (var index in cell.OnLandFoliageIndices)
					{
						if (index >= 0 && index < _foliageRuntimeData.OnLandFoliage.Count)
						{
							var foliage = _foliageRuntimeData.OnLandFoliage[index];
							if (bounds.Contains(foliage.Position))
							{
								_removedOnLandFoliageIndices.Add(index);
								onLandRemoved++;
							}
						}
					}
				}

				// Mark underwater foliage in this cell
				if (cell.UnderWaterFoliageIndices != null)
				{
					foreach (var index in cell.UnderWaterFoliageIndices)
					{
						if (index >= 0 && index < _foliageRuntimeData.UnderWaterFoliage.Count)
						{
							var foliage = _foliageRuntimeData.UnderWaterFoliage[index];
							if (bounds.Contains(foliage.Position))
							{
								_removedUnderWaterFoliageIndices.Add(index);
								underWaterRemoved++;
							}
						}
					}
				}
			}

			// Rebuild GPU instancing matrices with removed foliage filtered out
			RebuildMatricesWithRemovedFoliage();
		}

		/// <summary>
		/// Rebuilds GPU instancing matrices with removed foliage filtered out.
		/// Filters complete lists but uses HashSet for O(1) removal checks.
		/// </summary>
		private void RebuildMatricesWithRemovedFoliage()
		{
			// Filter on-land foliage (O(N) but with O(1) HashSet lookup)
			var filteredOnLand = new List<GameResources.FoliageData>();
			for (int i = 0; i < _foliageRuntimeData.OnLandFoliage.Count; i++)
			{
				if (!_removedOnLandFoliageIndices.Contains(i))
					filteredOnLand.Add(_foliageRuntimeData.OnLandFoliage[i]);
			}
			_foliageRuntimeData.OnLandFoliageCache = GroupByMeshAndMaterial(filteredOnLand.ToArray());
			_foliageRuntimeData.OnLandMatricesCache = BuildMatricesDictionary(_foliageRuntimeData.OnLandFoliageCache);

			// Filter underwater foliage (O(N) but with O(1) HashSet lookup)
			var filteredUnderWater = new List<GameResources.FoliageData>();
			for (int i = 0; i < _foliageRuntimeData.UnderWaterFoliage.Count; i++)
			{
				if (!_removedUnderWaterFoliageIndices.Contains(i))
					filteredUnderWater.Add(_foliageRuntimeData.UnderWaterFoliage[i]);
			}
			_foliageRuntimeData.UnderWaterFoliageCache = GroupByMeshAndMaterial(filteredUnderWater.ToArray());
			_foliageRuntimeData.UnderWaterMatricesCache = BuildMatricesDictionary(_foliageRuntimeData.UnderWaterFoliageCache);
		}
	}
}
