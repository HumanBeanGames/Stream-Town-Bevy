using UnityEngine;
using TwitchLib.Unity;

namespace Twitch
{
    /// <summary>
    /// Handles the Twitch API.
    /// </summary>
	public class TL_API : MonoBehaviour
	{
        /// <summary>
        /// The Twitch API instance.
        /// </summary>
		public static Api API;

        /// <summary>
        /// Initializes the Twitch API.
        /// </summary>
		public void InitApi()
		{
			Application.runInBackground = true;
			API = new Api();

			API.Settings.AccessToken = TL_Secrets.BotAccessToken;
			API.Settings.ClientId = TL_Secrets.ClientID;

		}

        /// <summary>
        /// Initializes the API on start.
        /// </summary>
		private void Start()
		{
			InitApi();
		}
	}
}
