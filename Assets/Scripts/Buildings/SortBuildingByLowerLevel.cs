using System.Collections.Generic;

namespace Buildings
{
    /// <summary>
    /// Comparer for sorting buildings by their level in ascending order.
    /// </summary>
	public class SortBuildingByLowerLevel : IComparer<BuildingBase>
	{
        /// <summary>
        /// Compares two buildings by their level.
        /// </summary>
        /// <param name="x">First building to compare.</param>
        /// <param name="y">Second building to compare.</param>
        /// <returns>1 if x has higher level, -1 if x has lower level, 0 if equal.</returns>
		public int Compare(BuildingBase x, BuildingBase y)
		{
			if (x.LevelHandler.Level > y.LevelHandler.Level)
				return 1;
			if (x.LevelHandler.Level < y.LevelHandler.Level)
				return -1;
			return 0;
		}
	}
}
