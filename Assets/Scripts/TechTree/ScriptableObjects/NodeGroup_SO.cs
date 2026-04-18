using UnityEngine;

namespace TechTree.ScriptableObjects
{
    /// <summary>
    /// ScriptableObject for a node group.
    /// </summary>
	public class NodeGroup_SO : ScriptableObject
	{
        /// <summary>
        /// The group name.
        /// </summary>
		[field: SerializeField]
		public string GroupName { get; set; }

        /// <summary>
        /// Initializes the node group.
        /// </summary>
        /// <param name="groupName">The group name.</param>
		public void Initialize(string groupName)
		{
			GroupName = groupName;
		}
	}
}
