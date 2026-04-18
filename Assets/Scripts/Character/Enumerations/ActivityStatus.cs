namespace Character.Enumerations 
{
    /// <summary>
    /// Represents the activity status of a character.
    /// </summary>
    public enum ActivityStatus 
	{
        /// <summary>
        /// Character is currently active.
        /// </summary>
        Active= 0,

        /// <summary>
        /// Character was active within the last ten minutes.
        /// </summary>
		LastTenMinutes = 1,

        /// <summary>
        /// Character was active within the last hour.
        /// </summary>
		LastHour = 2,

        /// <summary>
        /// Character is inactive.
        /// </summary>
		Inactive = 3
	}
}
