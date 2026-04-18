using Character.Enumerations;
using Processors;
using Twitch.Utils;
using TwitchLib.Client.Enums;

namespace Twitch
{
	/// <summary>
	/// Holds data for a registered Twitch User.
	/// </summary>
	public class TwitchUser
	{
        /// <summary>
        /// The user ID.
        /// </summary>
		public string UserID;

        /// <summary>
        /// The username.
        /// </summary>
		public string Username;

        /// <summary>
        /// The Twitch user type.
        /// </summary>
		public UserType TwitchUserType;

        /// <summary>
        /// The game user type.
        /// </summary>
		public GameUserType GameUserType;

        /// <summary>
        /// The activity status.
        /// </summary>
		public ActivityStatus ActivityStatus;

        /// <summary>
        /// The time since last message.
        /// </summary>
		public float TimeSinceLastMessage;

        /// <summary>
        /// Whether the user is the broadcaster.
        /// </summary>
		public bool IsBroadcaster = false;

        /// <summary>
        /// Initializes a new instance of the TwitchUser class.
        /// </summary>
        /// <param name="userID">The user ID.</param>
        /// <param name="username">The username.</param>
		public TwitchUser(string userID, string username)
		{
			UserID = userID;
			Username = username;
		}

        /// <summary>
        /// Updates the activity status based on time since last message.
        /// </summary>
        /// <param name="worldTimePassed">The world time passed.</param>
		public void UpdateActivity(float worldTimePassed)
		{
			float time = worldTimePassed - TimeSinceLastMessage;

			if (time < 300)
				ActivityStatus = ActivityStatus.Active;
			else if (time < 600)
				ActivityStatus = ActivityStatus.LastTenMinutes;
			else if (time < 3600)
				ActivityStatus = ActivityStatus.LastHour;
			else
				ActivityStatus = ActivityStatus.Inactive;
		}
	}
}
