using DataStructures;
using System.Collections.Generic;
using UnityEngine;

namespace TechTree.ScriptableObjects
{
    /// <summary>
    /// ScriptableObject for a tech tree.
    /// </summary>
	public class TechTree_SO : ScriptableObject
	{
        /// <summary>
        /// The file name.
        /// </summary>
		[field: SerializeField]
		public string FileName { get; set; }

        /// <summary>
        /// The node groups dictionary.
        /// </summary>
		[field: SerializeField]
		public SerializableDictionary<NodeGroup_SO, List<Node_SO>> NodeGroups { get; set; }

        /// <summary>
        /// The ungrouped nodes.
        /// </summary>
		[field: SerializeField]
		public List<Node_SO> UngroupedNodes { get; set; }

        /// <summary>
        /// Initializes the tech tree.
        /// </summary>
        /// <param name="fileName">The file name.</param>
		public void Initialize(string fileName)
		{
			FileName = fileName;

			NodeGroups = new SerializableDictionary<NodeGroup_SO, List<Node_SO>>();
			UngroupedNodes = new List<Node_SO>();
		}
	}
}
