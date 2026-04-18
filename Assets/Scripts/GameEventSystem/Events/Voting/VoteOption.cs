namespace GameEventSystem.Events.Voting 
{
    /// <summary>
    /// Represents a voting option in a vote event.
    /// </summary>
	public class VoteOption
	{
        /// <summary>
        /// The name of the option.
        /// </summary>
		public string OptionName;

        /// <summary>
        /// The data associated with the option.
        /// </summary>
		public object OptionData;

        /// <summary>
        /// The number of votes for this option.
        /// </summary>
		public int Votes;

        /// <summary>
        /// Initializes a new vote option instance.
        /// </summary>
        /// <param name="optionName">The name of the option.</param>
        /// <param name="data">The data associated with the option.</param>
		public VoteOption(string optionName, object data)
		{
			OptionName = optionName;
			OptionData = data;
			Votes = 0;
		}
	}
}
