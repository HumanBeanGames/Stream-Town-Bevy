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
using Enemies;

namespace Twitch
{
	/// <summary>
	/// Handles connection to Twitch and reads events triggered by the API.
	/// </summary>
	public class TL_Client : MonoBehaviour
	{
        /// <summary>
        /// Gets the Twitch client.
        /// </summary>
		public Client Client { get; private set; }

        /// <summary>
        /// The channel name.
        /// </summary>
		private string _channelName = "";

        /// <summary>
        /// Whether the client is connecting.
        /// </summary>
		private bool _isConnecting = false;

        /// <summary>
        /// Whether the send ping loop is running.
        /// </summary>
		private bool _sendPingRunning = false;

        /// <summary>
        /// The current settings. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] SettingsData CurrentSettings;

        /// <summary>
        /// The game event processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GameEventProcessor _gameEventProcessor;

        /// <summary>
        /// The player processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// The object pooling processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// The enemy spawner. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private EnemySpawner _enemySpawner;

        /// <summary>
        /// The UI processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private UIProcessor _uiProcessor;

        /// <summary>
        /// The game coordinator. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private Coordinator _gameProcessor;

        /// <summary>
        /// The Twitch chat processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TwitchChatProcessor _twitchChatProcessor;

        /// <summary>
        /// The message sender. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private MessageSender _messageSender;

		/// <summary>
		/// Initalizes the Twitch Lib Client and connect the bot to the Twitch Channel.
		/// </summary>
		private void InitClient()
		{
			Application.runInBackground = true;

			// Set up our bot and tell it which channel to join
			ConnectionCredentials credentials = new ConnectionCredentials(TL_Secrets.BotName, TL_Secrets.BotAccessToken);
			Client = new Client();

            _channelName = CurrentSettings.channelName.ToLower();

			Client.Initialize(credentials, _channelName);
			Client.AddChatCommandIdentifier('!');
			// Subscribe to any events here.
			Client.OnMessageReceived += OnMessageReceived;
			Client.OnChatCommandReceived += OnChatCommandReceived;
			Client.OnNewSubscriber += OnNewSubscriber;
			Client.OnGiftedSubscription += OnGiftedSubscription;
			Client.OnPrimePaidSubscriber += OnPrimePaidSubscriber;
			Client.OnReSubscriber += OnReSubscriber;
			Client.OnContinuedGiftedSubscription += OnContinuedGiftedSubscription;
			Client.OnCommunitySubscription += OnCommunitySubscription;
			Client.OnRaidNotification += OnRaidNotification;
			// Connect bot to channel
			Client.Connect();

			if (Client.IsConnected)
				Debug.Log("Twitch Connected");
			else
				Debug.Log("Twitch Failed to Connect!");
		}

		/// <summary>
		/// Async initialization with timeout to prevent blocking during scene load
		/// </summary>
		private IEnumerator InitClientAsync()
		{
			if (_isConnecting)
				yield break;

			_isConnecting = true;

			float timeout = 10f; // 10 second timeout
			float elapsedTime = 0f;

			UserInterface.MainMenu.LoadingProgressReporter.Report(0.95f, "Connecting to Twitch...");

			InitClient();

			// Wait for connection with timeout
			while (!Client.IsConnected && elapsedTime < timeout)
			{
				elapsedTime += Time.deltaTime;
				yield return null;
			}

			_isConnecting = false;

			if (Client.IsConnected)
			{
				Debug.Log("[TWITCH] Connected successfully");
				UserInterface.MainMenu.LoadingProgressReporter.Report(0.96f, "Twitch connected");
				// Only start ping loop after successful connection if not already running
				if (!_sendPingRunning)
					StartCoroutine(SendPing());
			}
			else
			{
				Debug.LogWarning("[TWITCH] Connection timed out after " + timeout + " seconds");
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
				_gameEventProcessor.AddEvent(new RaidEvent(0, 1200, enemies, _poolingProcessor, _uiProcessor.EventInterface, _gameEventProcessor, _enemySpawner, _playerProcessor, boss: "MinotaurBoss", waves: 2, enemiesPerWave: viewerCount));
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
        /// Starts the Twitch client connection.
        /// </summary>
		private void Start()
		{
			// Use async initialization to prevent blocking during scene load
			StartCoroutine(InitClientAsync());
		}

        /// <summary>
        /// Subscribes to log message received event.
        /// </summary>
		private void Awake()
		{
			Application.logMessageReceived += OnErrorReceived;
		}

        /// <summary>
        /// Handles keyboard input for force connection/disconnection.
        /// </summary>
		private void Update()
		{
			if (Keyboard.current.f2Key.wasReleasedThisFrame)
				ForceConnection();

			if (Keyboard.current.f1Key.wasReleasedThisFrame)
				ForceDisconnect();
		}

        /// <summary>
        /// Sends ping to keep connection alive.
        /// </summary>
		private IEnumerator SendPing()
		{
			_sendPingRunning = true;

			for (; ; )
			{
				if (Client != null && Client.IsConnected && Client.JoinedChannels.Count > 0)
				{
					Client.SendRaw("PING");
				}
				else if (!_isConnecting)
				{
					// Connection lost, attempt reconnect asynchronously
					StartCoroutine(InitClientAsync());
				}

				yield return new WaitForSeconds(30);
			}
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
			if (Client != null)
			{
				Client.Disconnect();
                _messageSender.MessagesAllowed = false;
                _sendPingRunning = false;

                if (Client.IsConnected)
					Debug.Log("Twitch Connected");
				else
					Debug.Log("Twitch Not Connected!");

				Client = null;
			}
		}

        /// <summary>
        /// Forces connection to Twitch.
        /// </summary>
		public void ForceConnection()
		{
			StartCoroutine(InitClientAsync());
		}
	}
}
