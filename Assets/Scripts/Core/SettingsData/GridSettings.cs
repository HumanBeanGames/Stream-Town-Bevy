using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores grid system settings for the game.
	/// Contains grid dimensions, cell size, and debug visualization settings.
	/// </summary>
	[CreateAssetMenu(fileName = "GridSettings", menuName = "Scriptables/Grid Settings")]
	public class GridSettingsScriptable : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Offset from world origin to grid origin.
		/// Used to position the grid relative to the world coordinate system.
		/// </summary>
		[SerializeField]
		private Vector2 _originOffset;

		/// <summary>
		/// Length of the grid along the Z-axis in cells.
		/// Determines the depth of the grid.
		/// </summary>
		[SerializeField]
		private int _gridLength = 100;

		/// <summary>
		/// Width of the grid along the X-axis in cells.
		/// Determines the width of the grid.
		/// </summary>
		[SerializeField]
		private int _gridWidth = 100;

		/// <summary>
		/// Size of each grid cell in world units.
		/// Determines the spacing between grid nodes.
		/// </summary>
		[SerializeField]
		private int _cellSize = 1;

#if UNITY_EDITOR
		/// <summary>
		/// Whether to draw the grid in the Unity editor scene view.
		/// Used for debugging and visualization during development.
		/// </summary>
		[SerializeField]
		private bool _drawGrid = true;
#endif

		/// <summary>
		/// Gets the offset from world origin to grid origin.
		/// </summary>
		public Vector2 OriginOffset => _originOffset;

		/// <summary>
		/// Gets the grid length along the Z-axis.
		/// </summary>
		public int GridLength => _gridLength;

		/// <summary>
		/// Gets the grid width along the X-axis.
		/// </summary>
		public int GridWidth => _gridWidth;

		/// <summary>
		/// Gets the size of each grid cell.
		/// </summary>
		public int CellSize => _cellSize;

#if UNITY_EDITOR
		/// <summary>
		/// Gets whether to draw the grid in the editor.
		/// </summary>
		public bool DrawGrid => _drawGrid;
#endif
	}
}
