using Character;

namespace GameEventSystem.Events.Voting 
{
    /// <summary>
    /// Represents a player's vote in a voting event.
    /// </summary>
	public class PlayerVote
	{
        /// <summary>
        /// The player who cast the vote.
        /// </summary>
		public Player Player;

        /// <summary>
        /// The vote option selected.
        /// </summary>
		public VoteOption VoteOption;

        /// <summary>
        /// Initializes a new player vote instance.
        /// </summary>
        /// <param name="player">The player who cast the vote.</param>
        /// <param name="option">The vote option selected.</param>
		public PlayerVote(Player player, VoteOption option)
		{
			Player = player;
			VoteOption = option;
		}
	}
}
