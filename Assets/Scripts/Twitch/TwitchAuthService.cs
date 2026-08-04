using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Networking;

namespace Twitch
{
	/// <summary>
	/// Stateless Twitch OAuth validation and refresh operations.
	/// </summary>
	public static class TwitchAuthService
	{
		private const string ValidateEndpoint = "https://id.twitch.tv/oauth2/validate";
		private const string TokenEndpoint = "https://id.twitch.tv/oauth2/token";
		private static readonly string[] RequiredIrcScopes = { "chat:read", "chat:edit" };

		public static IEnumerator EnsureValidCredentials(Action<TwitchAuthResult> completed)
		{
			if (!TL_Secrets.Reload())
			{
				completed?.Invoke(TwitchAuthResult.Failed(
					$"Twitch bot credentials are not configured. Open Tools > Stream Town > Twitch Bot Setup. Local file: {TL_Secrets.CredentialsPath}"));
				yield break;
			}

			string originalAccessToken = TL_Secrets.BotAccessToken;
			TwitchTokenValidationResponse validation = null;
			long validationStatus = 0;
			string validationError = string.Empty;

			yield return ValidateToken(TL_Secrets.BotAccessToken, (response, status, error) =>
			{
				validation = response;
				validationStatus = status;
				validationError = error;
			});

			if (validationStatus == 401)
			{
				if (string.IsNullOrWhiteSpace(TL_Secrets.BotRefreshToken))
				{
					completed?.Invoke(TwitchAuthResult.Failed(
						"The Twitch bot token is no longer valid and no refresh token is available. Authorize the bot again."));
					yield break;
				}

				TwitchTokenResponse refreshedToken = null;
				string refreshError = string.Empty;
				yield return RefreshToken((response, error) =>
				{
					refreshedToken = response;
					refreshError = error;
				});

				if (refreshedToken == null)
				{
					completed?.Invoke(TwitchAuthResult.Failed(
						$"The Twitch bot token could not be refreshed. Authorize the bot again. {refreshError}"));
					yield break;
				}

				TwitchCredentialData credentials = TL_Secrets.Snapshot();
				credentials.BotAccessToken = refreshedToken.access_token;
				if (!string.IsNullOrWhiteSpace(refreshedToken.refresh_token))
					credentials.BotRefreshToken = refreshedToken.refresh_token;

				if (!TL_Secrets.TrySave(credentials, out string saveError))
				{
					completed?.Invoke(TwitchAuthResult.Failed(saveError));
					yield break;
				}

				validation = null;
				validationStatus = 0;
				validationError = string.Empty;
				yield return ValidateToken(TL_Secrets.BotAccessToken, (response, status, error) =>
				{
					validation = response;
					validationStatus = status;
					validationError = error;
				});
			}

			if (validationStatus != 200 || validation == null)
			{
				completed?.Invoke(TwitchAuthResult.Failed(
					$"The Twitch bot token could not be validated. {validationError}"));
				yield break;
			}

			string credentialError = ValidateCredentialIdentity(validation);
			if (!string.IsNullOrEmpty(credentialError))
			{
				completed?.Invoke(TwitchAuthResult.Failed(credentialError));
				yield break;
			}

			completed?.Invoke(TwitchAuthResult.Succeeded(
				validation.login,
				validation.expires_in,
				!string.Equals(originalAccessToken, TL_Secrets.BotAccessToken, StringComparison.Ordinal)));
		}

		private static IEnumerator ValidateToken(
			string accessToken,
			Action<TwitchTokenValidationResponse, long, string> completed)
		{
			using (UnityWebRequest request = UnityWebRequest.Get(ValidateEndpoint))
			{
				request.SetRequestHeader("Authorization", $"OAuth {TL_Secrets.NormalizeAccessToken(accessToken)}");
				yield return request.SendWebRequest();

				if (request.responseCode == 200)
				{
					TwitchTokenValidationResponse response =
						JsonUtility.FromJson<TwitchTokenValidationResponse>(request.downloadHandler.text);
					completed?.Invoke(response, request.responseCode, string.Empty);
					yield break;
				}

				completed?.Invoke(null, request.responseCode, GetRequestError(request));
			}
		}

		private static IEnumerator RefreshToken(Action<TwitchTokenResponse, string> completed)
		{
			List<IMultipartFormSection> form = new List<IMultipartFormSection>
			{
				new MultipartFormDataSection("grant_type", "refresh_token"),
				new MultipartFormDataSection("refresh_token", TL_Secrets.BotRefreshToken),
				new MultipartFormDataSection("client_id", TL_Secrets.ClientID)
			};

			using (UnityWebRequest request = UnityWebRequest.Post(TokenEndpoint, form))
			{
				yield return request.SendWebRequest();
				if (request.responseCode == 200)
				{
					TwitchTokenResponse response = JsonUtility.FromJson<TwitchTokenResponse>(request.downloadHandler.text);
					completed?.Invoke(response, string.Empty);
					yield break;
				}

				completed?.Invoke(null, GetRequestError(request));
			}
		}

		private static string ValidateCredentialIdentity(TwitchTokenValidationResponse validation)
		{
			if (!string.Equals(validation.client_id, TL_Secrets.ClientID, StringComparison.Ordinal))
				return "The Twitch access token belongs to a different Client ID. Authorize it with this app's Client ID.";

			if (!string.Equals(validation.login, TL_Secrets.BotName, StringComparison.OrdinalIgnoreCase))
				return $"The Twitch token belongs to '{validation.login}', not the configured bot '{TL_Secrets.BotName}'. Authorize while signed in as the bot account.";

			foreach (string requiredScope in RequiredIrcScopes)
			{
				bool found = validation.scopes != null && Array.Exists(validation.scopes,
					scope => string.Equals(scope, requiredScope, StringComparison.OrdinalIgnoreCase));
				if (!found)
					return $"The Twitch bot token is missing the required '{requiredScope}' scope. Authorize the bot again.";
			}

			return string.Empty;
		}

		private static string GetRequestError(UnityWebRequest request)
		{
			if (!string.IsNullOrWhiteSpace(request.downloadHandler?.text))
			{
				TwitchErrorResponse error = JsonUtility.FromJson<TwitchErrorResponse>(request.downloadHandler.text);
				if (error != null && !string.IsNullOrWhiteSpace(error.message))
					return error.message;
			}

			return !string.IsNullOrWhiteSpace(request.error)
				? request.error
				: $"Twitch returned HTTP {request.responseCode}.";
		}

		[Serializable]
		private sealed class TwitchTokenValidationResponse
		{
			public string client_id;
			public string login;
			public string user_id;
			public string[] scopes;
			public int expires_in;
		}

		[Serializable]
		private sealed class TwitchTokenResponse
		{
			public string access_token;
			public string refresh_token;
			public int expires_in;
			public string[] scope;
			public string token_type;
		}

		[Serializable]
		private sealed class TwitchErrorResponse
		{
			public string message;
		}
	}

	public sealed class TwitchAuthResult
	{
		public bool Success { get; private set; }
		public bool TokenChanged { get; private set; }
		public string BotLogin { get; private set; }
		public int ExpiresInSeconds { get; private set; }
		public string Error { get; private set; }

		public static TwitchAuthResult Succeeded(string botLogin, int expiresInSeconds, bool tokenChanged)
		{
			return new TwitchAuthResult
			{
				Success = true,
				TokenChanged = tokenChanged,
				BotLogin = botLogin,
				ExpiresInSeconds = expiresInSeconds,
				Error = string.Empty
			};
		}

		public static TwitchAuthResult Failed(string error)
		{
			return new TwitchAuthResult
			{
				Success = false,
				Error = error ?? "Unknown Twitch authentication error."
			};
		}
	}
}
