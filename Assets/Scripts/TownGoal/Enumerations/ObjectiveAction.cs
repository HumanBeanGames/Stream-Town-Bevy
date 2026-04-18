namespace TownGoal.Enumerations 
{
    /// <summary>
    /// Enum representing the type of objective.
    /// </summary>
    public enum ObjectiveType 
	{
        /// <summary>
        /// Build a specific building.
        /// </summary>
       Build,

        /// <summary>
        /// Build any building.
        /// </summary>
	   BuildAny,

        /// <summary>
        /// Collect a specific resource.
        /// </summary>
	   Collect,

        /// <summary>
        /// Kill a specific enemy type.
        /// </summary>
	   Kill,

        /// <summary>
        /// Kill any enemy.
        /// </summary>
	   KillAny,

        /// <summary>
        /// Earn a specific resource per hour.
        /// </summary>
	   EarnPerHour,

        /// <summary>
        /// Sell a specific resource.
        /// </summary>
	   Sell,

        /// <summary>
        /// Sell any resource.
        /// </summary>
	   SellAny,

        /// <summary>
        /// Buy a specific resource.
        /// </summary>
	   Buy,

        /// <summary>
        /// Buy any resource.
        /// </summary>
	   BuyAny
    }
}
