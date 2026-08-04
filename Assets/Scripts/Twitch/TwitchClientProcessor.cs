using UnityEngine;
using TwitchLib.Client.Models;
using TwitchLib.Unity;
using TwitchLib.Client.Events;
using System.Collections;
using Character;
using GameEventSystem;
using Processors;
using Core;
using Pets;
using Pets.Enumerations;
using System;
using UnityEngine.InputSystem;
using GameEventSystem.Events;
using Reflex.Attributes;
using Reflex.Core;
using UserInterface;

namespace Twitch
{
	/// <summary>
	/// Processor for Twitch client connection and event handling.
	/// Manages connection to Twitch and reads events triggered by the API.
	/// </summary>
	public class TwitchClientProcessor : MonoBehaviour, IInstaller, IProcessor, IMainThreadInitializableProcessor
	{
		private TwitchClientRuntimeData _runtimeData;

		[Inject] private SettingsData _settingsData;
		[Inject] private GameEventProcessor _gameEventProcessor;
		[Inject] private PlayerProcessor _playerProcessor;
		[Inject] private ObjectPoolingProcessor _poolingProcessor;
		[Inject] private WorldGenProcessor _worldGenProcessor;
		[Inject] private Processors.TwitchChatProcessor _twitchChatProcessor;

		private UserInterface_Event _eventInterface;

		public Client Client => _runtimeData.Client;
		public bool IsConnected => _runtimeData?.Client != null && _runtimeData.Client.IsConnected;
		public bool IsConnecting => _runtimeData?.IsConnecting == true;
		public string ChannelName => _runtimeData?.ChannelName ?? string.Empty;
		public int JoinedChannelCount => _runtimeData?.Client?.JoinedChannels?.Count ?? 0;
		public string ConnectionStatus => _runtimeData?.ConnectionStatus ?? "Runtime data unavailable";

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_runtimeData != null)
				throw new InvalidOperationException("TwitchClientProcessor runtime data has already been installed.");

			_runtimeData = new TwitchClientRuntimeData();
			containerBuilder.AddSingleton(_runtimeData);
		}

		public void Initialize()
		{
			if (_runtimeData == null)
				throw new InvalidOperationException("TwitchClientProcessor runtime data has not been installed.");

			_runtimeData.ConnectionDesired = true;
			Application.logMessageReceived -= OnErrorReceived;
			Application.logMessageReceived += OnErrorReceived;
			EnsureConnectionForConfiguredChannel();
		}

		public void Process()
		{
			if (Keyboard.current.f2Key.wasReleasedThisFrame)
				ForceConnection();

			if (Keyboard.current.f1Key.wasReleasedThisFrame)
				ForceDisconnect();
		}

		public void RefreshSceneData(Container sceneContainer)
		{
			EnsureConnectionForConfiguredChannel();
		}

		/// <summary>
		/// Connects or reconnects the bot when a broadcaster channel is configured.
		/// Safe to call after first-time channel setup and on scene changes.
		/// </summary>
		public void EnsureConnectionForConfiguredChannel()
		{
			if (_runtimeData == null || _settingsData == null)
				return;

			string configuredChannel = NormalizeChannelName(_settingsData.channelName);
			if (string.IsNullOrEmpty(configuredChannel))
			{
				_runtimeData.ConnectionStatus = "No broadcaster channel configured";
				return;
			}

			if (_runtimeData.Client != null &&
				_runtimeData.Client.IsConnected &&
				string.Equals(_runtimeData.ChannelName, configuredChannel, StringComparison.OrdinalIgnoreCase) &&
				_runtimeData.Client.JoinedChannels.Count > 0)
			{
				_runtimeData.ConnectionStatus = $"Connected to {configuredChannel}";
				return;
			}

			if (_runtimeData.IsConnecting)
				return;

			_runtimeData.ConnectionDesired = true;
			StartCoroutine(InitClientAsync());
		}

		private static string NormalizeChannelName(string channelName)
		{
			return channelName?.Trim().TrimStart('#').ToLowerInvariant() ?? string.Empty;
		}

		private UserInterface_Event ResolveEventInterface()
		{
			if (_eventInterface == null)
				_eventInterface = UnityEngine.Object.FindAnyObjectByType<UserInterface_Event>();

			return _eventInterface;
		}

		/// <summary>
		/// Initalizes the Twitch Lib Client and connect the bot to the Twitch Channel.
		/// </summary>
		private void InitClient()
		{
			Application.runInBackground = true;

			// Set up our bot and tell it which channel to join
			ConnectionCredentials credentials = new ConnectionCredentials(TL_Secrets.BotName, TL_Secrets.BotAccessToken);
			_runtimeData.Client = new Client();

			_runtimeData.ChannelName = NormalizeChannelName(_settingsData.channelName);

			_runtimeData.Client.Initialize(credentials, _runtimeData.ChannelName);
			_runtimeData.Client.AddChatCommandIdentifier('!');
			// Subscribe to any events here.
			_runtimeData.Client.OnMessageReceived += OnMessageReceived;
			_runtimeData.Client.OnChatCommandReceived += OnChatCommandReceived;
			_runtimeData.Client.OnNewSubscriber += OnNewSubscriber;
			_runtimeData.Client.OnGiftedSubscription += OnGiftedSubscription;
			_runtimeData.Client.OnPrimePaidSubscriber += OnPrimePaidSubscriber;
			_runtimeData.Client.OnReSubscriber += OnReSubscriber;
			_runtimeData.Client.OnContinuedGiftedSubscription += OnContinuedGiftedSubscription;
			_runtimeData.Client.OnCommunitySubscription += OnCommunitySubscription;
			_runtimeData.Client.OnRaidNotification += OnRaidNotification;
			// Connect bot to channel
			_runtimeData.Client.Connect();
			_runtimeData.ConnectionStatus = $"Connecting to {_runtimeData.ChannelName}";
			Debug.Log($"Twitch connection requested for bot '{TL_Secrets.BotName}' in channel '{_runtimeData.ChannelName}'.");
		}

		/// <summary>
		/// Async initialization with timeout to prevent blocking during scene load
		/// </summary>
		private IEnumerator InitClientAsync()
		{
			if (_runtimeData.IsConnecting || !_runtimeData.ConnectionDesired)
				yield break;

			_runtimeData.IsConnecting = true;

			float timeout = 10f; // 10 second timeout
			float elapsedTime = 0f;

			UserInterface.MainMenu.LoadingProgressReporter.Report(0.95f, "Connecting to Twitch...");

			if (_settingsData == null || string.IsNullOrWhiteSpace(_settingsData.channelName))
			{
				_runtimeData.IsConnecting = false;
				_runtimeData.ConnectionStatus = "No broadcaster channel configured";
				Debug.LogWarning("Twitch connection skipped because no broadcaster channel is configured.");
				UserInterface.MainMenu.LoadingProgressReporter.Report(0.96f, "Twitch channel not configured (continuing without Twitch)");
				yield break;
			}

			TwitchAuthResult authResult = null;
			yield return TwitchAuthService.EnsureValidCredentials(result => authResult = result);
			if (authResult == null || !authResult.Success)
			{
				_runtimeData.IsConnecting = false;
				string error = authResult?.Error ?? "Twitch authentication returned no result.";
				_runtimeData.ConnectionStatus = error;
				Debug.LogWarning(error);
				UserInterface.MainMenu.LoadingProgressReporter.Report(0.96f, "Twitch bot not authorized (continuing without Twitch)");
				yield break;
			}

			if (!_runtimeData.ConnectionDesired)
			{
				_runtimeData.IsConnecting = false;
				yield break;
			}

			DisconnectClient(false);
			InitClient();

			// Wait for connection with timeout
			while (_runtimeData.Client != null && !_runtimeData.Client.IsConnected && elapsedTime < timeout)
			{
				elapsedTime += Time.unscaledDeltaTime;
				yield return null;
			}

			_runtimeData.IsConnecting = false;

			if (_runtimeData.Client != null && _runtimeData.Client.IsConnected)
			{
				_runtimeData.ConnectionStatus = $"Connected to {_runtimeData.ChannelName}";
				Debug.Log($"Twitch bot '{authResult.BotLogin}' connected to '{_runtimeData.ChannelName}'.");
				UserInterface.MainMenu.LoadingProgressReporter.Report(0.96f, "Twitch connected");
				// Only start ping loop after successful connection if not already running
				if (!_runtimeData.SendPingRunning)
					StartCoroutine(SendPing());
				if (!_runtimeData.CredentialsValidationRunning)
					StartCoroutine(ValidateCredentialsPeriodically());
			}
			else
			{
				_runtimeData.ConnectionStatus = $"Connection to {_runtimeData.ChannelName} timed out";
				Debug.LogWarning("Connection timed out after " + timeout + " seconds");
				UserInterface.MainMenu.LoadingProgressReporter.Report(0.96f, "Twitch connection timed out (continuing without Twitch)");
				// Don't block - continue without Twitch connection
			}
		}

		/// <summary>
		/// Called when a raid notification is received.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The raid notification args.</param>
		private void OnRaidNotification(object sender, OnRaidNotificationArgs e)
		{
			if (int.TryParse(e.RaidNotification.MsgParamViewerCount, out int viewerCount))
			{
				string[] enemies = new string[] { "Minotaur" };
				_gameEventProcessor.AddEvent(new RaidEvent(0, 1200, enemies, _poolingProcessor, ResolveEventInterface(), _gameEventProcessor, _worldGenProcessor, _playerProcessor, boss: "MinotaurBoss", waves: 2, enemiesPerWave: viewerCount));
			}
		}

		/// <summary>
		/// Called when a community subscription is received.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The community subscription args.</param>
		private void OnCommunitySubscription(object sender, OnCommunitySubscriptionArgs e)
		{
			UserIsSubscribed(e.GiftedSubscription.UserId);
		}

		/// <summary>
		/// Called when a continued gifted subscription is received.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The continued gifted subscription args.</param>
		private void OnContinuedGiftedSubscription(object sender, OnContinuedGiftedSubscriptionArgs e)
		{
			UserIsSubscribed(e.ContinuedGiftedSubscription.UserId);
		}

		/// <summary>
		/// Called when a re-subscription is received.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The re-subscriber args.</param>
		private void OnReSubscriber(object sender, OnReSubscriberArgs e)
		{
			UserIsSubscribed(e.ReSubscriber.UserId);
		}

		/// <summary>
		/// Called when a prime paid subscription is received.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The prime paid subscriber args.</param>
		private void OnPrimePaidSubscriber(object sender, OnPrimePaidSubscriberArgs e)
		{
			UserIsSubscribed(e.PrimePaidSubscriber.UserId);
		}

		/// <summary>
		/// Called when a gifted subscription is received.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The gifted subscription args.</param>
		private void OnGiftedSubscription(object sender, OnGiftedSubscriptionArgs e)
		{
			UserIsSubscribed(e.GiftedSubscription.MsgParamRecipientId);
		}

		/// <summary>
		/// Called when a Chat Command has been received.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The chat command received args.</param>
		private void OnChatCommandReceived(object sender, OnChatCommandReceivedArgs e)
		{
			if (string.Equals(e.Command.CommandText, "connect", StringComparison.OrdinalIgnoreCase))
			{
				Debug.Log($"Twitch !connect received from '{e.Command.ChatMessage.Username}' " +
					$"(broadcaster={e.Command.ChatMessage.IsBroadcaster}, arguments={e.Command.ArgumentsAsList?.Count ?? 0}).");
			}
			_twitchChatProcessor.ProcessCommand(e);
		}

		/// <summary>
		/// Called when a Chat Message has been receieved.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The message received args.</param>
		private void OnMessageReceived(object sender, OnMessageReceivedArgs e)
		{
			//Debug.Log($"Message | {e.ChatMessage.Username}: {e.ChatMessage.Message} | Id: {e.ChatMessage.UserId}");
			_twitchChatProcessor.ProcessMessage(e);
		}

		/// <summary>
		/// Called when a new subscriber is received.
		/// </summary>
		/// <param name="sender">The sender.</param>
		/// <param name="e">The new subscriber args.</param>
		private void OnNewSubscriber(object sender, OnNewSubscriberArgs e)
		{
			UserIsSubscribed(e.Subscriber.UserId);
		}

		/// <summary>
		/// Marks a user as subscribed and unlocks the Red Panda pet.
		/// </summary>
		/// <param name="userId">The user ID.</param>
		public void UserIsSubscribed(string userId)
		{
			if (_playerProcessor.PlayerExistsByID(userId, out int playerIndex))
			{
				Player player = _playerProcessor.GetPlayer(playerIndex);
				player.PetsUnlocked[PetType.RedPanda] = true;

				if (player.Pet.ActivePet == null)
					player.Pet.TrySetActivePet(PetType.RedPanda);
			}
		}

		/// <summary>
		/// Sends ping to keep connection alive.
		/// </summary>
		private IEnumerator SendPing()
		{
			_runtimeData.SendPingRunning = true;

			while (_runtimeData.ConnectionDesired)
			{
				if (_runtimeData.Client != null && _runtimeData.Client.IsConnected && _runtimeData.Client.JoinedChannels.Count > 0)
				{
					_runtimeData.Client.SendRaw("PING");
				}
				else if (!_runtimeData.IsConnecting)
				{
					// Connection lost, attempt reconnect asynchronously
					StartCoroutine(InitClientAsync());
				}

				yield return new WaitForSecondsRealtime(30f);
			}

			_runtimeData.SendPingRunning = false;
		}

		/// <summary>
		/// Twitch requires third-party chatbots to validate OAuth tokens at startup and hourly thereafter.
		/// </summary>
		private IEnumerator ValidateCredentialsPeriodically()
		{
			_runtimeData.CredentialsValidationRunning = true;

			while (_runtimeData.ConnectionDesired)
			{
				yield return new WaitForSecondsRealtime(3600f);
				if (!_runtimeData.ConnectionDesired)
					break;

				TwitchAuthResult authResult = null;
				yield return TwitchAuthService.EnsureValidCredentials(result => authResult = result);
				if (authResult == null || !authResult.Success)
				{
					Debug.LogWarning(authResult?.Error ?? "Twitch credential validation returned no result.");
					_runtimeData.ConnectionDesired = false;
					DisconnectClient(true);
					break;
				}
			}

			_runtimeData.CredentialsValidationRunning = false;
		}

		/// <summary>
		/// Called when an error is received.
		/// </summary>
		/// <param name="logString">The log string.</param>
		/// <param name="stackTrace">The stack trace.</param>
		/// <param name="type">The log type.</param>
		private void OnErrorReceived(string logString, string stackTrace, LogType type)
		{
			//if (type == LogType.Error || type == LogType.Exception)
			//{
			//	_messageSender.MessagesAllowed = true;
			//	_messageSender.SendMessage($"/w Uniquisher {logString} ");
			//	_messageSender.SendMessage($"/w Uniquisher {stackTrace}");
			//}
		}

		/// <summary>
		/// Forces disconnection from Twitch.
		/// </summary>
		public void ForceDisconnect()
		{
			_runtimeData.ConnectionDesired = false;
			DisconnectClient(true);
			_runtimeData.ConnectionStatus = "Connection disabled";
			Debug.Log("Twitch connection disabled.");
		}

		/// <summary>
		/// Forces connection to Twitch.
		/// </summary>
		public void ForceConnection()
		{
			_runtimeData.ConnectionDesired = true;
			EnsureConnectionForConfiguredChannel();
		}

		private void DisconnectClient(bool disableChatMessages)
		{
			if (_runtimeData.Client != null)
			{
				if (_runtimeData.Client.IsConnected)
					_runtimeData.Client.Disconnect();
				_runtimeData.Client = null;
			}

			if (disableChatMessages && _twitchChatProcessor != null)
				_twitchChatProcessor.MessagesAllowed = false;
		}

		private void OnDestroy()
		{
			Application.logMessageReceived -= OnErrorReceived;
			if (_runtimeData == null)
				return;

			_runtimeData.ConnectionDesired = false;
			DisconnectClient(false);
		}
	}
}
