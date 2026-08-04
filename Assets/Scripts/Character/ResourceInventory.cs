using Utils;

namespace Character
{
	/// <summary>
	/// Used for when multiple resource types need to be stored.
	/// </summary>
	[System.Serializable]
	public class ResourceInventory
	{
        /// <summary>
        /// The maximum amount of the resource.
        /// </summary>
		private int _maxAmount;

        /// <summary>
        /// The current amount of the resource.
        /// </summary>
		private int _amount;

        /// <summary>
        /// Whether the resource storage is unlimited.
        /// </summary>
		private bool _unlimited = false;

        /// <summary>
        /// Gets whether the resource storage is full.
        /// </summary>
		public bool Full => (Amount >= MaxAmount && !_unlimited);

        /// <summary>
        /// Gets whether the resource storage is half full.
        /// </summary>
		public bool HalfFull => (Amount >= MaxAmount * 0.5f && !_unlimited);

        /// <summary>
        /// Gets whether the resource storage is empty.
        /// </summary>
		public bool Empty => (Amount == 0 && !_unlimited);

        /// <summary>
        /// Gets the resource data as a string.
        /// </summary>
		public string ResourceDataToString => _unlimited ? $"{StringUtils.GetShortenedNumberAsString(_amount)}" : $"{StringUtils.GetShortenedNumberAsString(_amount)}/{StringUtils.GetShortenedNumberAsString(_maxAmount)}";

        /// <summary>
        /// Gets or sets the maximum amount.
        /// </summary>
		public int MaxAmount
		{
			get { return _maxAmount; }
			set
			{
				_maxAmount = value;
				OnMaxAmountChanged();
			}
		}

        /// <summary>
        /// Gets or sets the current amount.
        /// </summary>
		public int Amount
		{
			get { return _amount; }
			set
			{
				_amount = value;
				OnAmountChanged();
			}
		}

		/// <summary>
		/// Gets whether this inventory entry ignores its maximum amount.
		/// </summary>
		public bool IsUnlimited => _unlimited;

		/// <summary>
		/// Initializes a new instance of the ResourceInventory class.
		/// </summary>
		/// <param name="startingAmount">The initial amount of the resource.</param>
		/// <param name="maxAmount">The maximum amount of the resource.</param>
		/// <param name="unlimited">Whether the resource storage is unlimited.</param>
		public ResourceInventory(int startingAmount, int maxAmount, bool unlimited = false)
		{
			_amount = startingAmount;
			_maxAmount = maxAmount;
			_unlimited = unlimited;
		}

		/// <summary>
		/// Called when the resource amount has changed and ensures it is kept within the bounds of the storage amount.
		/// </summary>
		private void OnAmountChanged()
		{
			if ( _amount > _maxAmount && !_unlimited)
				_amount = _maxAmount;

			if (_amount < 0)
				_amount = 0;
		}

		/// <summary>
		/// Called when the max amount of a resource has changed to ensure it doesn't go into the negatives.
		/// </summary>
		private void OnMaxAmountChanged()
		{
			if (_maxAmount < 0)
				_maxAmount = 0;
		}
	}
}
