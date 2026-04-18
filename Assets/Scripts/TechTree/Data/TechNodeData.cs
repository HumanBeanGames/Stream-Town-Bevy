using System.Collections.Generic;
using TownGoal.Data;
using Utils;

namespace TechTree.Data
{
    /// <summary>
    /// Data transfer object for technology node information.
    /// Used to pass tech node data without exposing ScriptableObjects.
    /// </summary>
    public class TechNodeData
    {
        /// <summary>
        /// The technology name.
        /// </summary>
        public string TechName { get; set; }

        /// <summary>
        /// The node title.
        /// </summary>
        public string NodeTitle { get; set; }

        /// <summary>
        /// The description.
        /// </summary>
        public string Description { get; set; }

        /// <summary>
        /// The icon path.
        /// </summary>
        public string IconPath { get; set; }

        /// <summary>
        /// The age.
        /// </summary>
        public Age Age { get; set; }

        /// <summary>
        /// The tier.
        /// </summary>
        public int Tier { get; set; }

        /// <summary>
        /// The unlocks data.
        /// </summary>
        public List<NodeUnlockData> Unlocks { get; set; }

        /// <summary>
        /// The objectives.
        /// </summary>
        public List<ObjectiveData> Objectives { get; set; }

        /// <summary>
        /// Whether the node is unavailable.
        /// </summary>
        public bool Unavailable { get; set; }

        /// <summary>
        /// Creates a TechNodeData from a Node_SO ScriptableObject.
        /// </summary>
        public static TechNodeData FromNodeSO(ScriptableObjects.Node_SO node)
        {
            if (node == null)
                return null;

            return new TechNodeData
            {
                TechName = node.TechName,
                NodeTitle = node.NodeTitle,
                Description = node.Description,
                IconPath = node.IconPath,
                Age = node.Age,
                Tier = node.Tier,
                Unlocks = node.Unlocks,
                Objectives = node.Objectives,
                Unavailable = node.Unavailable
            };
        }
    }
}
