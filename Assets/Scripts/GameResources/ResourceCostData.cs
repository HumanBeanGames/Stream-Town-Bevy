namespace GameResources
{
	/// <summary>
	/// A simple struct that holds a resource cost.
	/// </summary>
	[System.Serializable]
	public struct ResourceCostData
	{
        /// <summary>
        /// The wood cost.
        /// </summary>
		public int WoodCost;

        /// <summary>
        /// The ore cost.
        /// </summary>
		public int OreCost;

        /// <summary>
        /// The food cost.
        /// </summary>
		public int FoodCost;

        /// <summary>
        /// The gold cost.
        /// </summary>
		public int GoldCost;
	}
}
