using GridSystem.Utils;
using GridSystem.Partitioning;
using System;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Processors;

namespace GridSystem
{
	/// <summary>
	/// Processor that manages the game grid system.
	/// Generates and renders a grid of collision nodes for pathfinding and spatial awareness.
	/// </summary>
	[System.Serializable]
	public partial class GridProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// ScriptableObject containing grid settings.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private GridSettings _gridSettings;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		/// <summary>
		/// Cell space partitioning component for spatial queries.
		/// </summary>
		private CellSpacePartitioning _cellSpacePartitioning;

		/// <summary>
		/// Runtime data for grid data.
		/// Assigned in InjectRuntimeData.
		/// </summary>
		private GridRuntimeData _gridRuntimeData;

		/// <summary>
		/// Gets the current grid array.
		/// </summary>
		/// <returns>The array of grid nodes.</returns>
		public GridNode[] GetGrid()
		{
			return _gridRuntimeData.Grid;
		}

		/// <summary>
		/// Generates a new grid based on the configured settings.
		/// Calculates cell dimensions, offsets, and populates the grid with nodes.
		/// </summary>
		public void GenerateGrid()
		{
			// Calculate the number of cells in X and Z directions
			_gridRuntimeData.CellsX = (_gridSettings.GridWidth / _gridSettings.CellSize);
			_gridRuntimeData.CellsZ = (_gridSettings.GridLength / _gridSettings.CellSize);
			
			// Calculate offset to center the grid around the origin
			_gridRuntimeData.OffSetX = -(_gridRuntimeData.CellsZ * _gridSettings.CellSize / 2 + _gridSettings.OriginOffset.x - transform.position.x) + (_gridSettings.CellSize * 0.5f);
			_gridRuntimeData.OffSetZ = -(_gridRuntimeData.CellsX * _gridSettings.CellSize / 2 + _gridSettings.OriginOffset.y - transform.position.z) + (_gridSettings.CellSize * 0.5f);

			// Initialize the grid array
			_gridRuntimeData.Grid = new GridNode[_gridRuntimeData.CellsZ * _gridRuntimeData.CellsX];
			
			// Populate the grid with nodes
			for (int z = 0; z < _gridRuntimeData.CellsX; z++)
			{
				for (int x = 0; x < _gridRuntimeData.CellsX; x++)
				{
					// Create a new grid node with random collision type and calculated position
					_gridRuntimeData.Grid[_gridRuntimeData.CellsX * x + z] = new GridNode(
						(CollisionType)UnityEngine.Random.Range(0, (int)CollisionType.Friendly + 1),
						new Vector2(x * _gridSettings.CellSize + _gridRuntimeData.OffSetX, z * _gridSettings.CellSize + _gridRuntimeData.OffSetZ),
						-1);
				}
			}
		}

		/// <summary>
		/// Initializes the grid processor.
		/// GridProcessor doesn't require initialization logic.
		/// </summary>
		public void Initialize()
		{
			if (_gridRuntimeData == null)
				throw new InvalidOperationException("GridProcessor: GridRuntimeData has not been installed.");
		}

		/// <summary>
		/// Repopulates spatial partitioning indices after cells are created.
		/// Call this after world generation when resources/foliage are available.
		/// </summary>
		public void RepopulateSpatialIndices(Processors.ResourceProcessor resourceProcessor, Processors.FoliageProcessor foliageProcessor)
		{
			if (_cellSpacePartitioning == null)
				return;

			_debugProcessor.Log(DebugLogCategory.GridProcessor, "Repopulating spatial partitioning indices");
			_cellSpacePartitioning.PopulateResourceIndices(resourceProcessor);
			_cellSpacePartitioning.PopulateFoliageIndices(foliageProcessor);
		}

		/// <summary>Clears spatial references to instances from the abandoned world.</summary>
		public void ResetWorldState(Processors.ResourceProcessor resourceProcessor, Processors.FoliageProcessor foliageProcessor)
		{
			RepopulateSpatialIndices(resourceProcessor, foliageProcessor);
		}

		/// <summary>
		/// Processes grid logic every frame.
		/// Called every frame by the Coordinator.
		/// GridProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			// GridProcessor does not require per-frame updates
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// GridProcessor does not have scene-specific settings to refresh
		}

		/// <summary>
		/// Registers this processor as a singleton in the dependency injection container.
		/// Called by Reflex during container initialization.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			var cellSpacePartitioning = GetComponent<CellSpacePartitioning>();
			if (cellSpacePartitioning == null)
				throw new InvalidOperationException("GridProcessor requires a CellSpacePartitioning component on the same GameObject.");

			// Initialize cell space partitioning (runs on main thread during container construction)
			cellSpacePartitioning.GeneratePartitions();

			// Store reference for later repopulation after world generation
			_cellSpacePartitioning = cellSpacePartitioning;

			containerBuilder.AddSingleton(this);
			containerBuilder.AddSingleton(cellSpacePartitioning);
			InjectRuntimeData(containerBuilder);
		}

		/// <summary>
		/// Injects the GridRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_gridRuntimeData != null)
				throw new InvalidOperationException("GridProcessor: GridRuntimeData has already been installed.");

			_gridRuntimeData = new GridRuntimeData();
			containerBuilder.AddSingleton(_gridRuntimeData);
		}
	}
}
