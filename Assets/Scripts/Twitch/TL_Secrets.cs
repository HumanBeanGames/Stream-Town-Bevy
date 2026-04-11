namespace Twitch
{
	public static class TL_Secrets
	{
		public static string ClientID = "";
		public static string ClientSecret = "";
		public static string BotAccessToken = "";
		public static string BotRefreshToken = "";
		public static string BotName = "";

		// Automatically load secrets from StreamingAssets on first access.
		// Expects a JSON file at Assets/StreamingAssets/twitch_secrets.json with fields matching these names.
		static TL_Secrets()
		{
			TryLoadFromStreamingAssets();
		}

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

		[System.Serializable]
		private class SecretsData
		{
			public string ClientID;
			public string ClientSecret;
			public string BotAccessToken;
			public string BotRefreshToken;
			public string BotName;
		}
	}
}