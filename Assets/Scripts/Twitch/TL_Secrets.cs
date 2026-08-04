using System;
using System.IO;
using UnityEngine;

namespace Twitch
{
	/// <summary>
	/// Local Twitch credentials used by the installed Stream Town client.
	/// Credentials are deliberately stored outside Assets so they are neither committed nor shipped in builds.
	/// </summary>
	public static class TL_Secrets
	{
		public const string CredentialsFileName = "twitch_credentials.json";
		public const string ClientIdEnvironmentVariable = "STREAM_TOWN_TWITCH_CLIENT_ID";
		public const string BotNameEnvironmentVariable = "STREAM_TOWN_TWITCH_BOT_NAME";
		public const string AccessTokenEnvironmentVariable = "STREAM_TOWN_TWITCH_BOT_ACCESS_TOKEN";
		public const string RefreshTokenEnvironmentVariable = "STREAM_TOWN_TWITCH_BOT_REFRESH_TOKEN";

		public static string ClientID { get; private set; } = string.Empty;
		public static string BotAccessToken { get; private set; } = string.Empty;
		public static string BotRefreshToken { get; private set; } = string.Empty;
		public static string BotName { get; private set; } = string.Empty;

		public static string CredentialsPath => Path.Combine(Application.persistentDataPath, CredentialsFileName);

		public static bool IsConfigured =>
			!string.IsNullOrWhiteSpace(ClientID) &&
			!string.IsNullOrWhiteSpace(BotAccessToken) &&
			!string.IsNullOrWhiteSpace(BotName);

		static TL_Secrets()
		{
			Reload();
		}

		/// <summary>
		/// Reloads credentials from the per-user application data directory, then applies optional environment overrides.
		/// </summary>
		public static bool Reload()
		{
			Clear();

			try
			{
				if (File.Exists(CredentialsPath))
				{
					string json = File.ReadAllText(CredentialsPath);
					TwitchCredentialData data = JsonUtility.FromJson<TwitchCredentialData>(json);
					if (data != null)
						Apply(data);
					else
						Debug.LogError($"Twitch credentials could not be parsed at {CredentialsPath}.");
				}

				ApplyEnvironmentOverrides();
				return IsConfigured;
			}
			catch (Exception ex)
			{
				Debug.LogError($"Twitch credentials could not be loaded from {CredentialsPath}: {ex.Message}");
				return false;
			}
		}

		/// <summary>
		/// Saves credentials to the current user's application data directory.
		/// </summary>
		public static bool TrySave(TwitchCredentialData data, out string error)
		{
			error = string.Empty;
			if (data == null)
			{
				error = "No Twitch credential data was supplied.";
				return false;
			}

			data.ClientID = Normalize(data.ClientID);
			data.BotName = Normalize(data.BotName).ToLowerInvariant();
			data.BotAccessToken = NormalizeAccessToken(data.BotAccessToken);
			data.BotRefreshToken = Normalize(data.BotRefreshToken);

			if (string.IsNullOrWhiteSpace(data.ClientID) ||
				string.IsNullOrWhiteSpace(data.BotName) ||
				string.IsNullOrWhiteSpace(data.BotAccessToken))
			{
				error = "Client ID, bot account name, and bot access token are required.";
				return false;
			}

			try
			{
				string directory = Path.GetDirectoryName(CredentialsPath);
				if (!string.IsNullOrWhiteSpace(directory))
					Directory.CreateDirectory(directory);

				File.WriteAllText(CredentialsPath, JsonUtility.ToJson(data, true));
				Apply(data);
				return true;
			}
			catch (Exception ex)
			{
				error = $"Twitch credentials could not be saved: {ex.Message}";
				return false;
			}
		}

		public static TwitchCredentialData Snapshot()
		{
			return new TwitchCredentialData
			{
				ClientID = ClientID,
				BotName = BotName,
				BotAccessToken = BotAccessToken,
				BotRefreshToken = BotRefreshToken
			};
		}

		public static string NormalizeAccessToken(string token)
		{
			string normalized = Normalize(token);
			return normalized.StartsWith("oauth:", StringComparison.OrdinalIgnoreCase)
				? normalized.Substring("oauth:".Length)
				: normalized;
		}

		private static void Apply(TwitchCredentialData data)
		{
			ClientID = Normalize(data.ClientID);
			BotName = Normalize(data.BotName).ToLowerInvariant();
			BotAccessToken = NormalizeAccessToken(data.BotAccessToken);
			BotRefreshToken = Normalize(data.BotRefreshToken);
		}

		private static void ApplyEnvironmentOverrides()
		{
			ClientID = EnvironmentOverride(ClientIdEnvironmentVariable, ClientID);
			BotName = EnvironmentOverride(BotNameEnvironmentVariable, BotName).ToLowerInvariant();
			BotAccessToken = NormalizeAccessToken(EnvironmentOverride(AccessTokenEnvironmentVariable, BotAccessToken));
			BotRefreshToken = EnvironmentOverride(RefreshTokenEnvironmentVariable, BotRefreshToken);
		}

		private static string EnvironmentOverride(string variableName, string fallback)
		{
			string value = System.Environment.GetEnvironmentVariable(variableName);
			return string.IsNullOrWhiteSpace(value) ? fallback : value.Trim();
		}

		private static string Normalize(string value)
		{
			return value?.Trim() ?? string.Empty;
		}

		private static void Clear()
		{
			ClientID = string.Empty;
			BotAccessToken = string.Empty;
			BotRefreshToken = string.Empty;
			BotName = string.Empty;
		}
	}

	/// <summary>
	/// Plain serialized credential data. Never place an instance of this type in an Asset or scene.
	/// </summary>
	[Serializable]
	public sealed class TwitchCredentialData
	{
		public string ClientID = string.Empty;
		public string BotAccessToken = string.Empty;
		public string BotRefreshToken = string.Empty;
		public string BotName = string.Empty;
	}
}
