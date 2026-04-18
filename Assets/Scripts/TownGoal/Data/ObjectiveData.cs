using UnityEngine;

namespace TownGoal.Data
{
	using Enumerations;
	using Utils;

    /// <summary>
    /// Data class for an objective.
    /// </summary>
	[System.Serializable]
	public class ObjectiveData
	{
        /// <summary>
        /// The objective type.
        /// </summary>
		[field: SerializeField]
		public ObjectiveType ObjectiveType { get; set; }

        /// <summary>
        /// The integer value.
        /// </summary>
		[field: SerializeField]
		public int IntValue { get; set; }

        /// <summary>
        /// The float value.
        /// </summary>
		[field: SerializeField]
		public float FloatValue { get; set; }

        /// <summary>
        /// The building type.
        /// </summary>
		[field: SerializeField]
		public BuildingType BuildingType { get; set; }

        /// <summary>
        /// The resource type.
        /// </summary>
		[field: SerializeField]
		public Resource ResourceType { get; set; }

        /// <summary>
        /// The enemy type.
        /// </summary>
		[field: SerializeField]
		public EnemyType EnemyType { get; set; }
	}
}
