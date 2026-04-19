using GridSystem.Utils;
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
		/// Runtime data ScriptableObject for grid data.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private GridRuntimeData _gridRuntimeData;

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
						(CollisionType)Random.Range(0, (int)CollisionType.Friendly + 1),
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
			// GridProcessor doesn't require initialization logic
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
		/// Injects the GridRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			GridRuntimeData gridRuntimeData = ScriptableObject.CreateInstance<GridRuntimeData>();
			containerBuilder.AddSingleton(gridRuntimeData);
		}
	}
}
