using System.Collections.Generic;
using Target;
using UnityEngine;
using Utils;

namespace GridSystem.Partitioning
{
	/// <summary>
	/// A cell used in the Cell Space Partitioning System.
	/// </summary>
	[System.Serializable]
	public class BSPCell
	{
        /// <summary>
        /// Array of target lists organized by target type.
        /// </summary>
		public List<Targetable>[] _targetArray;

        /// <summary>
        /// Wood resource indices.
        /// </summary>
		public List<int> WoodResourceIndices;

        /// <summary>
        /// Ore resource indices.
        /// </summary>
		public List<int> OreResourceIndices;

        /// <summary>
        /// Food resource indices.
        /// </summary>
		public List<int> FoodResourceIndices;

        /// <summary>
        /// Gold resource indices.
        /// </summary>
		public List<int> GoldResourceIndices;

        /// <summary>
        /// Recruit resource indices.
        /// </summary>
		public List<int> RecruitResourceIndices;

        /// <summary>
        /// On-land foliage indices.
        /// </summary>
        public List<int> OnLandFoliageIndices;

        /// <summary>
        /// Underwater foliage indices.
        /// </summary>
        public List<int> UnderWaterFoliageIndices;

        /// <summary>
        /// The top-left position of the cell.
        /// </summary>
		public Vector2 TopLeft;

        /// <summary>
        /// The bottom-right position of the cell.
        /// </summary>
		public Vector2 BottomRight;

        /// <summary>
        /// The center position of the cell.
        /// </summary>
		public Vector2 Center;

        /// <summary>
        /// Whether the cell has been searched.
        /// </summary>
		public bool Searched;

        /// <summary>
        /// Gets the top boundary.
        /// </summary>
		public float Top => TopLeft.y;

        /// <summary>
        /// Gets the left boundary.
        /// </summary>
		public float Left => TopLeft.x;

        /// <summary>
        /// Gets the bottom boundary.
        /// </summary>
		public float Bottom => BottomRight.y;

        /// <summary>
        /// Gets the right boundary.
        /// </summary>
		public float Right => BottomRight.x;

		// Constructor.
        /// <summary>
        /// Initializes a new BSP cell instance.
        /// </summary>
        /// <param name="topLeft">The top-left position.</param>
        /// <param name="bottomRight">The bottom-right position.</param>
		public BSPCell(Vector2 topLeft, Vector2 bottomRight)
		{
			TopLeft = topLeft;
			BottomRight = bottomRight;
			Center = (topLeft + bottomRight) / 2;
			_targetArray = new List<Targetable>[TargetFlagHelper.TargetFlagCount - 1];
			Searched = false;
		}

		/// <summary>
		/// Returns true if overlapping.
		/// </summary>
		/// <param name="topLeft">The top-left position.</param>
		/// <param name="bottomRight">The bottom-right position.</param>
		/// <returns>True if overlapping.</returns>
		public bool IsOverlapping(Vector2 topLeft, Vector2 bottomRight)
		{
			return !(topLeft.x > Right
				|| bottomRight.x < Left
				|| topLeft.y > Bottom
				|| bottomRight.y < Top
				);
		}

		/// <summary>
		/// Returns true if two BSPCells are overlapping.
		/// </summary>
		/// <param name="other">The other BSP cell.</param>
		/// <returns>True if overlapping.</returns>
		public bool IsOverlapping(BSPCell other)
		{
			return IsOverlapping(other.TopLeft, other.BottomRight);
		}

		/// <summary>
		/// Adds a target to the Cell.
		/// </summary>
		/// <param name="target">The target to add.</param>
		public void AddTarget(Targetable target)
		{
			for (int i = 0; i < TargetFlagHelper.TargetFlagCount - 1; i++)
			{
				if (target.TargetType.HasFlag(TargetFlagHelper.TargetFlags[i + 1]))
				{
					AddTarget(i, target);
				}
			}
		}

		/// <summary>
		/// Removes a target from the Cell.
		/// </summary>
		/// <param name="target">The target to remove.</param>
		public void RemoveTarget(Targetable target)
		{
			for (int i = 0; i < TargetFlagHelper.TargetFlagCount - 1; i++)
			{
				if (target.TargetType.HasFlag(TargetFlagHelper.TargetFlags[i + 1]))
				{
					RemoveTarget(i, target);
				}
			}
		}

		/// <summary>
		/// Gets all targets defined by the Target mask into one list.
		/// </summary>
		/// <param name="targetMask">The target mask.</param>
		/// <param name="targets">The list to populate with targets.</param>
		public void GetTargetsByFlag(TargetMask targetMask, ref List<Targetable> targets)
		{
			Searched = true;
			for (int i = 0; i < _targetArray.Length; i++)
			{
				if ((targetMask & TargetFlagHelper.TargetFlags[i + 1]) != 0)
				{
					if (_targetArray[i] != null)
						targets.AddRange(_targetArray[i]);
				}
			}

		}

		/// <summary>
		/// Adds a target to the cell.
		/// </summary>
		/// <param name="index">The target type index.</param>
		/// <param name="target">The target to add.</param>
		private void AddTarget(int index, Targetable target)
		{
			if (_targetArray[index] == null)
				_targetArray[index] = new List<Targetable>();

			else if (_targetArray[index].Contains(target))
				return;

			_targetArray[index].Add(target);

		}

		/// <summary>
		/// Removes a target from the cell.
		/// </summary>
		/// <param name="index">The target type index.</param>
		/// <param name="target">The target to remove.</param>
		private void RemoveTarget(int index, Targetable target)
		{
			if (_targetArray[index] == null)
				return;
			if (!_targetArray[index].Contains(target))
				return;

			_targetArray[index].Remove(target);
		}
	}
}
