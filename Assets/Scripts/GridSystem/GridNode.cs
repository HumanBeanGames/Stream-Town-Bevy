using GridSystem.Utils;
using System.Collections.Generic;
using UnityEngine;
// System may be obsolete.
namespace GridSystem
{
	//TODO:: Remove this
    /// <summary>
    /// Represents a node in the grid system. NOTE: This system may be obsolete.
    /// </summary>
	[System.Serializable]
	public struct GridNode
	{
        /// <summary>
        /// The collision type of the node.
        /// </summary>
		public CollisionType CollisionType;

        /// <summary>
        /// List of connected grid nodes.
        /// </summary>
		public List<GridNode> Connections;

        /// <summary>
        /// The position of the node.
        /// </summary>
		public Vector2 Position;

        /// <summary>
        /// The cell index.
        /// </summary>
		public int CellIndex;

        /// <summary>
        /// Initializes a new grid node instance.
        /// </summary>
        /// <param name="collision">The collision type.</param>
        /// <param name="position">The position.</param>
        /// <param name="cellIndex">The cell index.</param>
		public GridNode(CollisionType collision, Vector2 position, int cellIndex)
		{
			CollisionType = collision;
			Connections = new List<GridNode>();
			Position = position;
			CellIndex = cellIndex;
		}

        /// <summary>
        /// Adds a connection to another grid node.
        /// </summary>
        /// <param name="node">The node to connect to.</param>
		public void AddConnection(GridNode node)
		{
			Connections.Add(node);
		}

        /// <summary>
        /// Removes a connection to another grid node.
        /// </summary>
        /// <param name="node">The node to disconnect from.</param>
		public void RemoveConnection(GridNode node)
		{
			Connections.Remove(node);
		}
	}
}
