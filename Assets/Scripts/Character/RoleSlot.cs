using UnityEngine;
using Utils;

namespace Character
{
	/// <summary>
	/// Handles the number of slots available for a role.
	/// </summary>
	public class RoleSlot
	{
        /// <summary>
        /// The player role.
        /// </summary>
		[SerializeField]
		private PlayerRole _playerRole;

        /// <summary>
        /// Whether the slots are infinite.
        /// </summary>
		[SerializeField]
		private bool _infinite;

        /// <summary>
        /// The maximum number of slots.
        /// </summary>
		[SerializeField]
		private int _maxSlots;

        /// <summary>
        /// The number of slots taken.
        /// </summary>
		[SerializeField]
		private int _slotsTaken;

        /// <summary>
        /// Gets whether slots are available.
        /// </summary>
		public bool Available => _slotsTaken < _maxSlots || _infinite;

        /// <summary>
        /// Gets whether the slots are full.
        /// </summary>
		public bool Full => _slotsTaken >= _maxSlots && !_infinite;

        /// <summary>
        /// Gets the number of slots taken.
        /// </summary>
		public int SlotsTaken => _slotsTaken;

        /// <summary>
        /// Gets the maximum number of slots.
        /// </summary>
		public int MaxSlots => _maxSlots;

        /// <summary>
        /// Gets whether the slots are infinite.
        /// </summary>
		public bool Infinite => _infinite;

        /// <summary>
        /// Gets the slot data as a string.
        /// </summary>
		public string SlotDataAsString => _infinite ? $"{_slotsTaken}" : $"{_slotsTaken}   /   {_maxSlots}";

		// Constructor.
        /// <summary>
        /// Initializes a new role slot instance.
        /// </summary>
        /// <param name="role">The player role.</param>
        /// <param name="maxSlots">The maximum number of slots.</param>
        /// <param name="infinite">Whether the slots are infinite.</param>
		public RoleSlot(PlayerRole role, int maxSlots, bool infinite)
		{
			_playerRole = role;
			_maxSlots = maxSlots;
			_slotsTaken = 0;
			_infinite = infinite;
		}

		/// <summary>
		/// Increments the number of slots taken for the role.
		/// </summary>
		public void OnSlotTaken()
		{
			_slotsTaken++;
		}

		/// <summary>
		/// Decrements the number of slots taken for the role.
		/// </summary>
		public void OnSlotRemoved()
		{
			_slotsTaken--;
		}

		/// <summary>
		/// Increases the max number of slots available for the role.
		/// </summary>
		/// <param name="amount">The amount to increase.</param>
		public void IncreaseMaxSlots(int amount)
		{
			_maxSlots += amount;
		}

		/// <summary>
		/// Reduces the max number of slots available for the role.
		/// </summary>
		/// <param name="amount">The amount to decrease.</param>
		public void DecreaseMaxSlots(int amount)
		{
			_maxSlots -= amount;
			if (_maxSlots < 0)
				Debug.LogError($"Max slots for {_playerRole} went below 0, this should not happen!");
		}

		/// <summary>
		/// Sets the maximum number of slots for the role.
		/// </summary>
		/// <param name="maxAmount">The maximum amount.</param>
		public void SetMaxSlots(int maxAmount)
		{
			_maxSlots = maxAmount;
		}
	}
}
