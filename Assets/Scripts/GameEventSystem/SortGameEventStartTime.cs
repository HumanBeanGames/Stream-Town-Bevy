using System.Collections.Generic;

namespace GameEventSystem
{
    /// <summary>
    /// Comparer for sorting game events by start time.
    /// </summary>
	public class SortGameEventStartTime : IComparer<GameEvent>
	{
        /// <summary>
        /// Compares two game events by their start time.
        /// </summary>
        /// <param name="x">The first game event.</param>
        /// <param name="y">The second game event.</param>
        /// <returns>A positive value if x starts after y, negative if x starts before y, or 0 if they start at the same time.</returns>
		public int Compare(GameEvent x, GameEvent y)
		{
			if (x.StartTime > y.StartTime)
				return 1;
			if (x.StartTime < y.StartTime)
				return -1;
			return 0;
		}
	}
}
