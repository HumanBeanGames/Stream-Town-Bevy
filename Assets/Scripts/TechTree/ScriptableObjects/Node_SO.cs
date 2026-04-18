using System.Collections.Generic;
using UnityEngine;
using TownGoal.Data;


namespace TechTree.ScriptableObjects
{
	using Data;
	using Utils;

    /// <summary>
    /// ScriptableObject for a tech tree node.
    /// </summary>
	public class Node_SO : ScriptableObject
	{
        /// <summary>
        /// The technology name.
        /// </summary>
		[field: SerializeField]
		public string TechName { get; set; }

        /// <summary>
        /// The node title.
        /// </summary>
		[field: SerializeField]
		public string NodeTitle { get; set; }

        /// <summary>
        /// The description.
        /// </summary>
		[field: SerializeField, TextArea]
		public string Description { get; set; }

        /// <summary>
        /// The children nodes.
        /// </summary>
		[field: SerializeField]
		public List<NodeChildrenTechData> Children { get; set; }

        /// <summary>
        /// The unlocks data.
        /// </summary>
		[field: SerializeField]
		public List<NodeUnlockData> Unlocks { get; set; }

        /// <summary>
        /// The objectives.
        /// </summary>
		[field: SerializeField]
		public List<ObjectiveData> Objectives {get;set;}

        /// <summary>
        /// Whether the node is unlocked.
        /// </summary>
		[field: SerializeField]
		public bool IsUnlocked { get; set; }

        /// <summary>
        /// The age.
        /// </summary>
		[field: SerializeField]
		public Age Age { get; set; }

        /// <summary>
        /// The tier.
        /// </summary>
		[field: SerializeField]
		public int Tier { get; set; }

        /// <summary>
        /// The parent node.
        /// </summary>
		[field: SerializeField]
		public Node_SO Parent { get; set; }

        /// <summary>
        /// The icon path.
        /// </summary>
		[field: SerializeField]
		public string IconPath { get; set; }

        /// <summary>
        /// Whether the node is unavailable.
        /// </summary>
		[field: SerializeField]
		public bool Unavailable { get; set; }

        /// <summary>
        /// Initializes the node.
        /// </summary>
        /// <param name="techName">The technology name.</param>
        /// <param name="nodeTitle">The node title.</param>
        /// <param name="text">The description text.</param>
        /// <param name="children">The children nodes.</param>
        /// <param name="unlocks">The unlocks data.</param>
        /// <param name="objectives">The objectives.</param>
        /// <param name="isUnlocked">Whether the node is unlocked.</param>
        /// <param name="age">The age.</param>
        /// <param name="tier">The tier.</param>
        /// <param name="iconPath">The icon path.</param>
        /// <param name="unavailable">Whether the node is unavailable.</param>
		public void Initialize(string techName,string nodeTitle, string text, List<NodeChildrenTechData> children, List<NodeUnlockData> unlocks, List<ObjectiveData> objectives,	bool isUnlocked, Age age, int tier, string iconPath, bool unavailable)
		{
			NodeTitle = nodeTitle;
			TechName = techName;
			Description = text;
			Children = children;
			Unlocks = unlocks;
			IsUnlocked = isUnlocked;
			Age = age;
			Tier = tier;
			Objectives = objectives;
			IconPath = iconPath;
			Unavailable = unavailable;
		}
	}
}
