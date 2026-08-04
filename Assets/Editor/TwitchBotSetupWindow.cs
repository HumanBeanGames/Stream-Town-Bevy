using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Twitch;
using UnityEditor;
using UnityEngine;
using UnityEngine.Networking;

public sealed class TwitchBotSetupWindow : EditorWindow
{
	private const string DeviceEndpoint = "https://id.twitch.tv/oauth2/device";
	private const string TokenEndpoint = "https://id.twitch.tv/oauth2/token";
	private const string ValidateEndpoint = "https://id.twitch.tv/oauth2/validate";
	private const string DeveloperConsoleUrl = "https://dev.twitch.tv/console/apps";
	private const string RequiredScopes = "chat:read chat:edit";
	private const string ClientIdPreference = "StreamTown.Twitch.ClientId";
	private const string BotNamePreference = "StreamTown.Twitch.BotName";

	private string _clientId = string.Empty;
	private string _expectedBotName = "HumanBeanBot";
	private string _status = string.Empty;
	private MessageType _statusType = MessageType.Info;
	private string _activationCode = string.Empty;
	private string _activationUrl = string.Empty;
	private bool _authorizing;
	private CancellationTokenSource _authorizationCancellation;

	[MenuItem("Tools/Stream Town/Twitch Bot Setup")]
	public static void Open()
	{
		TwitchBotSetupWindow window = GetWindow<TwitchBotSetupWindow>(true, "Twitch Bot Setup");
		window.minSize = new Vector2(540f, 440f);
		window.Show();
	}

	private void OnEnable()
	{
		TL_Secrets.Reload();
		_clientId = EditorPrefs.GetString(ClientIdPreference, TL_Secrets.ClientID);
		_expectedBotName = EditorPrefs.GetString(BotNamePreference,
			string.IsNullOrWhiteSpace(TL_Secrets.BotName) ? "HumanBeanBot" : TL_Secrets.BotName);
		RefreshCredentialStatus();
	}

	private void OnDisable()
	{
		_authorizationCancellation?.Cancel();
		_authorizationCancellation?.Dispose();
		_authorizationCancellation = null;
	}

	private void OnGUI()
	{
		EditorGUILayout.Space(8f);
		EditorGUILayout.LabelField("Stream Town Twitch Bot", EditorStyles.boldLabel);
		EditorGUILayout.HelpBox(
			"The app can be owned by HumanBeanGames, but authorization must be completed while signed in as HumanBeanBot. " +
			"The bot token receives only IRC chat read/write permissions. No Client Secret is used or stored.",
			MessageType.Info);

		EditorGUILayout.Space(6f);
		EditorGUILayout.LabelField("1. Register the Twitch application", EditorStyles.boldLabel);
		EditorGUILayout.LabelField("Use a unique app name, the Chat Bot category, http://localhost:3000 as the redirect URL, and Public as the client type.", EditorStyles.wordWrappedLabel);
		if (GUILayout.Button("Open Twitch Developer Console"))
			Application.OpenURL(DeveloperConsoleUrl);

		EditorGUILayout.Space(10f);
		EditorGUILayout.LabelField("2. Authorize the bot account", EditorStyles.boldLabel);
		using (new EditorGUI.DisabledScope(_authorizing))
		{
			_clientId = EditorGUILayout.TextField(new GUIContent("Client ID", "The public Client ID from the Twitch Developer Console."), _clientId).Trim();
			_expectedBotName = EditorGUILayout.TextField(new GUIContent("Bot account", "The Twitch account that must authorize the app."), _expectedBotName).Trim();

			if (GUILayout.Button("Authorize Bot With Twitch"))
				BeginAuthorization();
		}

		if (_authorizing)
		{
			EditorGUILayout.Space(6f);
			EditorGUILayout.HelpBox("Waiting for Twitch authorization. Confirm that the activation page is signed in as the bot account.", MessageType.Info);
			if (!string.IsNullOrWhiteSpace(_activationCode))
			{
				EditorGUILayout.SelectableLabel($"Activation code: {_activationCode}", EditorStyles.textField, GUILayout.Height(20f));
				EditorGUILayout.BeginHorizontal();
				if (GUILayout.Button("Copy Code"))
					EditorGUIUtility.systemCopyBuffer = _activationCode;
				if (GUILayout.Button("Open Activation Page"))
					Application.OpenURL(_activationUrl);
				if (GUILayout.Button("Cancel"))
					_authorizationCancellation?.Cancel();
				EditorGUILayout.EndHorizontal();
			}
		}

		EditorGUILayout.Space(10f);
		EditorGUILayout.LabelField("Local credential file", EditorStyles.boldLabel);
		EditorGUILayout.SelectableLabel(TL_Secrets.CredentialsPath, EditorStyles.textField, GUILayout.Height(20f));
		EditorGUILayout.LabelField("This file is in the current user's application-data directory, outside the repository and outside game builds.", EditorStyles.wordWrappedMiniLabel);

		if (!string.IsNullOrWhiteSpace(_status))
		{
			EditorGUILayout.Space(8f);
			EditorGUILayout.HelpBox(_status, _statusType);
		}
	}

	private async void BeginAuthorization()
	{
		if (string.IsNullOrWhiteSpace(_clientId) || string.IsNullOrWhiteSpace(_expectedBotName))
		{
			SetStatus("Enter the app Client ID and expected bot account name first.", MessageType.Error);
			return;
		}

		EditorPrefs.SetString(ClientIdPreference, _clientId);
		EditorPrefs.SetString(BotNamePreference, _expectedBotName);
		_authorizationCancellation?.Cancel();
		_authorizationCancellation?.Dispose();
		_authorizationCancellation = new CancellationTokenSource();
		CancellationToken cancellationToken = _authorizationCancellation.Token;
		_authorizing = true;
		_activationCode = string.Empty;
		_activationUrl = string.Empty;
		SetStatus("Requesting a Twitch activation code...", MessageType.Info);

		try
		{
			WebResponse deviceRequest = await PostFormAsync(DeviceEndpoint, new Dictionary<string, string>
			{
				{ "client_id", _clientId },
				{ "scopes", RequiredScopes }
			}, cancellationToken);

			if (deviceRequest.StatusCode != 200)
				throw new InvalidOperationException(GetError(deviceRequest, "Twitch did not issue an activation code."));

			DeviceCodeResponse device = JsonUtility.FromJson<DeviceCodeResponse>(deviceRequest.Body);
			if (device == null || string.IsNullOrWhiteSpace(device.device_code) || string.IsNullOrWhiteSpace(device.user_code))
				throw new InvalidOperationException("Twitch returned an incomplete activation response.");

			_activationCode = device.user_code;
			_activationUrl = device.verification_uri;
			EditorGUIUtility.systemCopyBuffer = device.user_code;
			SetStatus($"Code {device.user_code} was copied to the clipboard. Authorize while signed in as {_expectedBotName}.", MessageType.Info);
			Application.OpenURL(device.verification_uri);

			int pollIntervalSeconds = Math.Max(1, device.interval);
			DateTime expiresAt = DateTime.UtcNow.AddSeconds(Math.Max(1, device.expires_in));
			TokenResponse token = null;

			while (DateTime.UtcNow < expiresAt && !cancellationToken.IsCancellationRequested)
			{
				await Task.Delay(TimeSpan.FromSeconds(pollIntervalSeconds), cancellationToken);
				WebResponse tokenRequest = await PostFormAsync(TokenEndpoint, new Dictionary<string, string>
				{
					{ "client_id", _clientId },
					{ "scopes", RequiredScopes },
					{ "device_code", device.device_code },
					{ "grant_type", "urn:ietf:params:oauth:grant-type:device_code" }
				}, cancellationToken);

				if (tokenRequest.StatusCode == 200)
				{
					token = JsonUtility.FromJson<TokenResponse>(tokenRequest.Body);
					break;
				}

				string pendingError = GetError(tokenRequest, string.Empty);
				if (pendingError.IndexOf("authorization_pending", StringComparison.OrdinalIgnoreCase) >= 0)
					continue;
				if (pendingError.IndexOf("slow_down", StringComparison.OrdinalIgnoreCase) >= 0)
				{
					pollIntervalSeconds += 5;
					continue;
				}

				throw new InvalidOperationException(string.IsNullOrWhiteSpace(pendingError)
					? "Twitch bot authorization failed."
					: pendingError);
			}

			cancellationToken.ThrowIfCancellationRequested();
			if (token == null || string.IsNullOrWhiteSpace(token.access_token))
				throw new TimeoutException("The Twitch activation code expired. Start authorization again.");

			TokenValidationResponse validation = await ValidateTokenAsync(token.access_token, cancellationToken);
			if (!string.Equals(validation.client_id, _clientId, StringComparison.Ordinal))
				throw new InvalidOperationException("The returned token belongs to a different Twitch application.");
			if (!string.Equals(validation.login, _expectedBotName, StringComparison.OrdinalIgnoreCase))
				throw new InvalidOperationException(
					$"Twitch authorized '{validation.login}', not '{_expectedBotName}'. Sign out or use a private browser window, then try again as the bot account.");

			EnsureRequiredScopes(validation.scopes);
			TwitchCredentialData credentials = new TwitchCredentialData
			{
				ClientID = _clientId,
				BotName = validation.login,
				BotAccessToken = token.access_token,
				BotRefreshToken = token.refresh_token
			};

			if (!TL_Secrets.TrySave(credentials, out string saveError))
				throw new InvalidOperationException(saveError);

			SetStatus($"Authorized {validation.login}. Stream Town can now join the configured broadcaster channel.", MessageType.Info);
		}
		catch (OperationCanceledException)
		{
			SetStatus("Twitch bot authorization was cancelled.", MessageType.Warning);
		}
		catch (Exception ex)
		{
			SetStatus(ex.Message, MessageType.Error);
		}
		finally
		{
			_authorizing = false;
			_activationCode = string.Empty;
			_activationUrl = string.Empty;
			Repaint();
		}
	}

	private static async Task<TokenValidationResponse> ValidateTokenAsync(string accessToken, CancellationToken cancellationToken)
	{
		using (UnityWebRequest request = UnityWebRequest.Get(ValidateEndpoint))
		{
			request.SetRequestHeader("Authorization", $"OAuth {TL_Secrets.NormalizeAccessToken(accessToken)}");
			WebResponse response = await SendAsync(request, cancellationToken);
			if (response.StatusCode != 200)
				throw new InvalidOperationException(GetError(response, "Twitch could not validate the bot token."));

			TokenValidationResponse validation = JsonUtility.FromJson<TokenValidationResponse>(response.Body);
			if (validation == null || string.IsNullOrWhiteSpace(validation.login))
				throw new InvalidOperationException("Twitch returned an incomplete token validation response.");
			return validation;
		}
	}

	private static async Task<WebResponse> PostFormAsync(
		string url,
		Dictionary<string, string> values,
		CancellationToken cancellationToken)
	{
		List<IMultipartFormSection> form = new List<IMultipartFormSection>();
		foreach (KeyValuePair<string, string> value in values)
			form.Add(new MultipartFormDataSection(value.Key, value.Value));

		using (UnityWebRequest request = UnityWebRequest.Post(url, form))
			return await SendAsync(request, cancellationToken);
	}

	private static async Task<WebResponse> SendAsync(UnityWebRequest request, CancellationToken cancellationToken)
	{
		TaskCompletionSource<bool> completion = new TaskCompletionSource<bool>();
		UnityWebRequestAsyncOperation operation = request.SendWebRequest();
		operation.completed += _ => completion.TrySetResult(true);

		using (cancellationToken.Register(() =>
		{
			request.Abort();
			completion.TrySetCanceled(cancellationToken);
		}))
		{
			await completion.Task;
		}

		return new WebResponse(request.responseCode, request.downloadHandler?.text ?? string.Empty, request.error);
	}

	private static void EnsureRequiredScopes(string[] scopes)
	{
		foreach (string requiredScope in RequiredScopes.Split(' '))
		{
			bool found = scopes != null && Array.Exists(scopes,
				scope => string.Equals(scope, requiredScope, StringComparison.OrdinalIgnoreCase));
			if (!found)
				throw new InvalidOperationException($"The Twitch token is missing the required '{requiredScope}' scope.");
		}
	}

	private static string GetError(WebResponse response, string fallback)
	{
		if (!string.IsNullOrWhiteSpace(response.Body))
		{
			ErrorResponse error = JsonUtility.FromJson<ErrorResponse>(response.Body);
			if (error != null && !string.IsNullOrWhiteSpace(error.message))
				return error.message;
		}

		if (!string.IsNullOrWhiteSpace(response.Error))
			return response.Error;
		return fallback;
	}

	private void RefreshCredentialStatus()
	{
		if (TL_Secrets.IsConfigured)
			SetStatus($"Local credentials are configured for {TL_Secrets.BotName}. Re-authorizing safely replaces them.", MessageType.Info);
		else
			SetStatus("No local Twitch bot credentials are configured yet.", MessageType.Warning);
	}

	private void SetStatus(string status, MessageType type)
	{
		_status = status;
		_statusType = type;
		Repaint();
	}

	private readonly struct WebResponse
	{
		public readonly long StatusCode;
		public readonly string Body;
		public readonly string Error;

		public WebResponse(long statusCode, string body, string error)
		{
			StatusCode = statusCode;
			Body = body;
			Error = error;
		}
	}

	[Serializable]
	private sealed class DeviceCodeResponse
	{
		public string device_code;
		public int expires_in;
		public int interval;
		public string user_code;
		public string verification_uri;
	}

	[Serializable]
	private sealed class TokenResponse
	{
		public string access_token;
		public string refresh_token;
		public string[] scope;
		public string token_type;
	}

	[Serializable]
	private sealed class TokenValidationResponse
	{
		public string client_id;
		public string login;
		public string[] scopes;
	}

	[Serializable]
	private sealed class ErrorResponse
	{
		public string message;
	}
}
