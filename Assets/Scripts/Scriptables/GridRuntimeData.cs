using GridSystem.Utils;

using ScriptablesProcessorInfrastructure;
using GridSystem;

namespace Processors
{
	/// <summary>
	/// Runtime data for GridProcessor.
	/// Contains the grid node array and grid dimensions for pathfinding and placement.
	/// </summary>
	public class GridRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Array of grid nodes representing the game world grid.
		/// Each node contains information about walkability, world position, and grid coordinates.
		/// </summary>
		private GridNode[] _grid;

		/// <summary>
		/// X-axis offset for grid positioning.
		/// Used to center the grid in world space.
		/// </summary>
		private float _offSetX;

		/// <summary>
		/// Z-axis offset for grid positioning.
		/// Used to center the grid in world space.
		/// </summary>
		private float _offSetZ;

		/// <summary>
		/// Number of grid cells along the X-axis.
		/// Determines the width of the grid.
		/// </summary>
		private int _cellsX;

		/// <summary>
		/// Number of grid cells along the Z-axis.
		/// Determines the depth of the grid.
		/// </summary>
		private int _cellsZ;

		/// <summary>
		/// Gets or sets the grid node array.
		/// </summary>
		public GridNode[] Grid
		{
			get { return _grid; }
			set { _grid = value; }
		}

		/// <summary>
		/// Gets or sets the X-axis offset.
		/// </summary>
		public float OffSetX
		{
			get { return _offSetX; }
			set { _offSetX = value; }
		}

		/// <summary>
		/// Gets or sets the Z-axis offset.
		/// </summary>
		public float OffSetZ
		{
			get { return _offSetZ; }
			set { _offSetZ = value; }
		}

		/// <summary>
		/// Gets or sets the number of cells along the X-axis.
		/// </summary>
		public int CellsX
		{
			get { return _cellsX; }
			set { _cellsX = value; }
		}

		/// <summary>
		/// Gets or sets the number of cells along the Z-axis.
		/// </summary>
		public int CellsZ
		{
			get { return _cellsZ; }
			set { _cellsZ = value; }
		}

		/// <summary>
		/// Initializes the grid runtime data with default values.
		/// </summary>
		public GridRuntimeData()
		{
			_grid = null;
			_offSetX = 0f;
			_offSetZ = 0f;
			_cellsX = 0;
			_cellsZ = 0;
		}
	}
}
