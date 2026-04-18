namespace Twitch
{
    /// <summary>
    /// Static class for holding Twitch API secrets.
    /// </summary>
	public static class TL_Secrets
	{
        /// <summary>
        /// The client ID.
        /// </summary>
		public static string ClientID = "";

        /// <summary>
        /// The client secret.
        /// </summary>
		public static string ClientSecret = "";

        /// <summary>
        /// The bot access token.
        /// </summary>
		public static string BotAccessToken = "";

        /// <summary>
        /// The bot refresh token.
        /// </summary>
		public static string BotRefreshToken = "";

        /// <summary>
        /// The bot name.
        /// </summary>
		public static string BotName = "";

		// Automatically load secrets from StreamingAssets on first access.
		// Expects a JSON file at Assets/StreamingAssets/twitch_secrets.json with fields matching these names.
        /// <summary>
        /// Static constructor that loads secrets from StreamingAssets.
        /// </summary>
		static TL_Secrets()
		{
			TryLoadFromStreamingAssets();
		}

        /// <summary>
        /// Tries to load secrets from StreamingAssets.
        /// </summary>
		private static void TryLoadFromStreamingAssets()
		{
			try
			{
				var path = System.IO.Path.Combine(UnityEngine.Application.streamingAssetsPath, "twitch_secrets.json");
				if (!System.IO.File.Exists(path))
				{
					UnityEngine.Debug.LogWarning($"TL_Secrets: secrets file not found at {path}. Using empty defaults.");
					return;
				}

				string json = System.IO.File.ReadAllText(path);
				var data = UnityEngine.JsonUtility.FromJson<SecretsData>(json);
				if (data == null)
				{
					UnityEngine.Debug.LogError("TL_Secrets: Failed to parse twitch_secrets.json. Check JSON format.");
					return;
				}

				ClientID = data.ClientID ?? string.Empty;
				ClientSecret = data.ClientSecret ?? string.Empty;
				BotAccessToken = data.BotAccessToken ?? string.Empty;
				BotRefreshToken = data.BotRefreshToken ?? string.Empty;
				BotName = data.BotName ?? string.Empty;
			}
			catch (System.Exception ex)
			{
				UnityEngine.Debug.LogError($"TL_Secrets: Exception while loading secrets: {ex}");
			}
		}

        /// <summary>
        /// Data class for deserializing secrets from JSON.
        /// </summary>
		[System.Serializable]
		private class SecretsData
		{
            /// <summary>
            /// The client ID.
            /// </summary>
			public string ClientID;

            /// <summary>
            /// The client secret.
            /// </summary>
			public string ClientSecret;

            /// <summary>
            /// The bot access token.
            /// </summary>
			public string BotAccessToken;

            /// <summary>
            /// The bot refresh token.
            /// </summary>
			public string BotRefreshToken;

            /// <summary>
            /// The bot name.
            /// </summary>
			public string BotName;
		}
	}
}
