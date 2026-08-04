namespace TechTree.Data
{
	using System;
	using UnityEngine;
	using Utils;

	/// <summary>
	/// Holds All Useful data for a tech node's unlocks.
	/// </summary>
	[Serializable]
	public class NodeUnlockData
	{
        /// <summary>
        /// The technology type.
        /// </summary>
		[field: SerializeField]
		public TechType TechType { get; set; }

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
        /// The string value.
        /// </summary>
		[field: SerializeField]
		public string StringValue { get; set; }

        /// <summary>
        /// The object value.
        /// </summary>
		[field: NonSerialized]
		public object ObjectValue { get; set; }

        /// <summary>
        /// The character value.
        /// </summary>
		[field: SerializeField]
		public char CharValue { get; set; }

        /// <summary>
        /// The boolean value.
        /// </summary>
		[field: SerializeField]
		public bool BoolValue { get; set; }

        /// <summary>
        /// The player role.
        /// </summary>
		[field: SerializeField]
		public PlayerRole PlayerRole { get; set; }

        /// <summary>
        /// The building type.
        /// </summary>
		[field: SerializeField]
		public BuildingType BuildingType { get; set; }

        /// <summary>
        /// The stat type.
        /// </summary>
		[field: SerializeField]
		public StatType StatType { get; set; }

        /// <summary>
        /// The resource type.
        /// </summary>
		[field: SerializeField]
		public Resource ResourceType { get; set; }
	}
}
