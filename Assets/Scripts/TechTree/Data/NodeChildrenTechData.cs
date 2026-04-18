using UnityEngine;

namespace TechTree.Data
{
	using ScriptableObjects;
	using System;

	/// <summary>
	/// Holds a node's child data.
	/// </summary>
	[Serializable]
	public class NodeChildrenTechData
	{
        /// <summary>
        /// The node ID.
        /// </summary>
		[field: SerializeField]
		public string NodeID { get; set; }

        /// <summary>
        /// The next technology node.
        /// </summary>
		[field: SerializeField]
		public Node_SO NextTech { get; set; }
	}
}
