using System;
using UnityEngine;

namespace GridSystem.Utils
{
	/// <summary>
	/// Dictates the type of collision.
	/// </summary>
	[Serializable, Flags]
	public enum CollisionType
	{
        /// <summary>
        /// Walkable terrain.
        /// </summary>
		Walkable = 0,

        /// <summary>
        /// Unwalkable terrain.
        /// </summary>
		Unwalkable = 1,

        /// <summary>
        /// Water terrain.
        /// </summary>
		Water = 2,

        /// <summary>
        /// Friendly territory.
        /// </summary>
		Friendly = 3
	}

	/// <summary>
	/// Holds all color data for different types of collisions.
	/// </summary>
	public static class CollisionColours
	{
        /// <summary>
        /// Color for walkable terrain.
        /// </summary>
		public static Color Walkable = Color.green;

        /// <summary>
        /// Color for unwalkable terrain.
        /// </summary>
		public static Color Unwalkable = Color.red;

        /// <summary>
        /// Color for water terrain.
        /// </summary>
		public static Color Water = Color.blue;

        /// <summary>
        /// Color for friendly territory.
        /// </summary>
		public static Color Friendly = Color.yellow;
	}
}
