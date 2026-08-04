using Character;
using Character.Enumerations;
using Processors;
using System;
using System.Collections.Generic;
using System.Linq;
using Twitch.Commands;
using Twitch.Utils;
using Twitch;
using TwitchLib.Client;
using TwitchLib.Client.Events;
using TwitchLib.Client.Models;
using TwitchLib.Communication.Clients;
using TwitchLib.Communication.Models;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Enemies;
using TownGoal;
using UserInterface;

namespace Processors
{
	/// <summary>
	/// General Processor for all functions relating to Twitch Chat integration.
	/// </summary>
	public class TwitchChatProcessor : MonoBehaviour, IInstaller, IProcessor, IMainThreadInitializableProcessor
	{
		[Inject] private GameEventProcessor _gameEventProcessor;
		[Inject] private LabelDisplayProcessor _utilDisplayProcessor;
		[Inject] private TwitchClientProcessor _twitchClientProcessor;
		[Inject] private RoleProcessor _roleProcessor;
		[Inject] private GameSettings _gameSettings;
		[Inject] private PlayerProcessor _playerProcessor;
		[Inject] private TechTreeProcessor _techTreeProcessor;
		[Inject] private WorldGenProcessor _worldGenProcessor;
		[Inject] private TownResourceProcessor _townResourceProcessor;
		[Inject] private ObjectPoolingProcessor _poolingProcessor;
		[Inject] private TownGoalProcessor _townGoalProcessor;
		[Inject] private BuildingProcessor _buildingProcessor;
		[Inject] private StationProcessor _stationProcessor;
		[Inject] private TradeProcessor _tradeProcessor;
		[Inject] private MetaData.MetaData _metaData;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		private TwitchChatRuntimeData _twitchChatRuntimeData;
		private MessageSender _messageSender;
		private EventCommands _eventCommands;
		private BuildingCommands _buildingCommands;
		private RoleCommands _roleCommands;
		private PlayerCommands _playerCommands;
		private GameMasterCommands _gameMasterCommands;
		private MiscCommands _miscCommands;
		private ModeratorCommands _moderatorCommands;
		private RulerCommands _rulerCommands;
		private BroadcasterCommands _broadcasterCommands;
		private CommandDictionary _commandDictionary;
		private TwitchConnectionPanelView _connectionPanelView;

		private void InitializeCommandInfrastructure()
		{
			if (_messageSender != null || _commandDictionary != null)
				throw new InvalidOperationException("TwitchChatProcessor command infrastructure has already been initialized.");

			TwitchChatRuntimeData runtimeData = RequireRuntimeData();
			_messageSender = new MessageSender(_twitchClientProcessor, runtimeData);
			_eventCommands = new EventCommands(_playerProcessor, _gameEventProcessor, _townResourceProcessor,
				_poolingProcessor, this);
			_buildingCommands = new BuildingCommands(_buildingProcessor, this);
			_roleCommands = new RoleCommands(_playerProcessor, _stationProcessor, _roleProcessor, this);
			_playerCommands = new PlayerCommands(_playerProcessor, _gameEventProcessor, _utilDisplayProcessor,
				_twitchClientProcessor, this, _eventCommands, _roleProcessor, _gameSettings);
			_gameMasterCommands = new GameMasterCommands(_roleProcessor,
				_gameSettings, _townResourceProcessor, _playerProcessor, _gameEventProcessor, _townGoalProcessor,
				_techTreeProcessor, _worldGenProcessor, _buildingProcessor, _poolingProcessor, this);
			_miscCommands = new MiscCommands(_buildingProcessor, this, _roleCommands, _buildingCommands, _roleProcessor);
			_moderatorCommands = new ModeratorCommands(_playerProcessor, _gameEventProcessor, _roleCommands);
			_rulerCommands = new RulerCommands(_playerProcessor, _roleProcessor, _townResourceProcessor,
				_gameEventProcessor, _tradeProcessor, this, _gameMasterCommands);
			_broadcasterCommands = new BroadcasterCommands(_playerProcessor,
				_techTreeProcessor, _metaData, this, _playerCommands);
			_commandDictionary = new CommandDictionary(_playerCommands, _roleCommands, _buildingCommands,
				_miscCommands, _gameMasterCommands, _moderatorCommands, _rulerCommands);
		}

		private void RequireCommandInfrastructure()
		{
			if (_messageSender == null || _commandDictionary == null || _playerCommands == null || _eventCommands == null || _gameMasterCommands == null || _broadcasterCommands == null)
				throw new InvalidOperationException("TwitchChatProcessor command infrastructure has not been initialized.");
		}

		private string GenerateBroadcasterConnectCode()
		{
			return UnityEngine.Random.Range(100000, 1000000).ToString();
		}

		private TwitchChatRuntimeData RequireRuntimeData()
		{
			if (_twitchChatRuntimeData == null)
				throw new InvalidOperationException("TwitchChatProcessor runtime data has not been installed.");

			return _twitchChatRuntimeData;
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_twitchChatRuntimeData != null)
				throw new InvalidOperationException("TwitchChatProcessor runtime data has already been installed.");

			_twitchChatRuntimeData = new TwitchChatRuntimeData();
			containerBuilder.AddSingleton(_twitchChatRuntimeData);
		}

		/// <summary>
		/// Processes any command sent in chat on Twitch and invokes the required action is the command is valid.
		/// </summary>
		/// <param name="e"></param>
		public void ProcessCommand(OnChatCommandReceivedArgs e)
		{
			TwitchChatRuntimeData runtimeData = RequireRuntimeData();
			RequireCommandInfrastructure();

			string command = (e.Command.CommandText ?? string.Empty).Trim().TrimStart('!').ToLowerInvariant();
			runtimeData.LastCommand = command;
			runtimeData.LastCommandUser = e.Command.ChatMessage.Username ?? string.Empty;
			runtimeData.LastCommandResult = "Received";

			if (!runtimeData.MessagesAllowed)
			{
				if (command == "connect" &&
					e.Command.ChatMessage.IsBroadcaster &&
					e.Command.ArgumentsAsList != null &&
					e.Command.ArgumentsAsList.Count > 0)
				{
					_broadcasterCommands.Connect(e.Command.ArgumentsAsList[0], e);
				}
				else
				{
					runtimeData.LastCommandResult = "Ignored while broadcaster authorization is pending";
				}
				return;
			}

			if (!runtimeData.PlayerReady)
			{
				runtimeData.LastCommandResult = "Ignored while world player data is not ready";
				return;
			}

			string[] normalizedArgs = e.Command.ArgumentsAsList == null
				? Array.Empty<string>()
				: e.Command.ArgumentsAsList
					.Select(argument => argument?.Trim().ToLowerInvariant() ?? string.Empty)
					.ToArray();

			if (!_commandDictionary.TryValidateArguments(command, normalizedArgs, out string usage))
			{
				runtimeData.LastCommandResult = $"Invalid arguments; usage: {usage}";
				SendMessage($"{e.Command.ChatMessage.Username}: Invalid arguments. Usage: {usage}");
				return;
			}

			if (_commandDictionary.SimpleCommands.ContainsKey(command))
			{
				runtimeData.LastCommandResult = "Dispatched";
				_commandDictionary.SimpleCommands[command].Invoke();
				return;
			}

			if (TryResolvePlayer(runtimeData, e.Command.ChatMessage.UserId, e.Command.ChatMessage.Username, out Player player))
			{
				player.TwitchUser.TimeSinceLastMessage = runtimeData.CurrentWorldTime;

				UpdateUserType(player, e);


				// Check if the command has arguments.
				if (normalizedArgs.Length > 0 && _commandDictionary.CommandsWithArgs.ContainsKey(command))
				{
					runtimeData.LastCommandResult = "Dispatched";
					_commandDictionary.CommandsWithArgs[command].Invoke(player, command, normalizedArgs);
				}
				else if (_commandDictionary.CommandsNoArgs.ContainsKey(command))
				{
					runtimeData.LastCommandResult = "Dispatched";
					_commandDictionary.CommandsNoArgs[command].Invoke(player);
				}
				else if (_commandDictionary.CommandsWithArgs.ContainsKey(command))
				{
					runtimeData.LastCommandResult = "Dispatched";
					_commandDictionary.CommandsWithArgs[command].Invoke(player, command, normalizedArgs);
				}
				else
				{
					runtimeData.LastCommandResult = "Unknown command";
				}
			}
			// Check if player is trying to create character or call a simple command
			else
			{
				if (CommandDictionary.CreateNameVariants.Contains(command))
				{
					_playerCommands.TryCreatePlayer(e);
					runtimeData.LastCommandResult = "Character created";
				}
				else
				{
					runtimeData.LastCommandResult = "Rejected because no matching character was found";
					if (_commandDictionary.CommandsWithArgs.ContainsKey(command) || _commandDictionary.CommandsNoArgs.ContainsKey(command))
						SendMessage($"{e.Command.ChatMessage.Username}: You need to create a character first with !join");
				}
			}
		}

		private static bool TryResolvePlayer(TwitchChatRuntimeData runtimeData, string userId, string username, out Player player)
		{
			player = null;
			int index = -1;
			bool found = runtimeData.PlayerExistsByID != null &&
				!string.IsNullOrWhiteSpace(userId) &&
				runtimeData.PlayerExistsByID(userId, out index);

			if (!found)
			{
				found = runtimeData.PlayerExistsByName != null &&
					!string.IsNullOrWhiteSpace(username) &&
					runtimeData.PlayerExistsByName(username.ToLowerInvariant(), out index);
			}

			if (!found || runtimeData.GetPlayer == null)
				return false;

			player = runtimeData.GetPlayer(index);
			return player != null;
		}

		public void ProcessMessage(OnMessageReceivedArgs e)
		{
			TwitchChatRuntimeData runtimeData = RequireRuntimeData();
			RequireCommandInfrastructure();
			if (!runtimeData.MessagesAllowed || !runtimeData.PlayerReady)
				return;

			if (runtimeData.PlayerExistsByID != null && runtimeData.PlayerExistsByID(e.ChatMessage.UserId, out int index))
			{
				Player player = runtimeData.GetPlayer(index);
				player.TwitchUser.TimeSinceLastMessage = runtimeData.CurrentWorldTime;
			}
			// Check for event
			if (_eventCommands.EventMessage(e))
				return;
		}

		/// <summary>
		/// Processes a debug command from the debug UI input field.
		/// Bypasses MessagesAllowed check and user type restrictions.
		/// </summary>
		/// <param name="commandText">The raw command text (e.g., "build House" or "confirm")</param>
		/// <param name="player">The player to execute the command as</param>
		public void ProcessDebugCommand(string commandText, Player player)
		{
			RequireCommandInfrastructure();

			if (string.IsNullOrWhiteSpace(commandText))
				return;

			// Parse command and arguments
			string[] parts = commandText.Split(new[] { ' ' }, StringSplitOptions.RemoveEmptyEntries);
			if (parts.Length == 0)
				return;

			string command = parts[0].ToLower();
			string[] args = parts.Length > 1 ? parts.Skip(1).ToArray() : new string[0];

			if (!_commandDictionary.TryValidateArguments(command, args, out string usage))
			{
				_debugProcessor.LogError(DebugLogCategory.TwitchClient,
					$"[Debug Command] Invalid arguments. Usage: {usage}");
				if (player != null)
					SendPlayerMessage(player, $"Invalid arguments. Usage: {usage}");
				return;
			}

			// Route to appropriate command handler
			if (_commandDictionary.SimpleCommands.ContainsKey(command))
			{
				_commandDictionary.SimpleCommands[command].Invoke();
				_debugProcessor.Log(DebugLogCategory.TwitchClient, $"[Debug Command] Executed: {command}");
				return;
			}

			if (player == null)
			{
				_debugProcessor.LogError(DebugLogCategory.TwitchClient, $"[Debug Command] Error: No player selected for command: {command}");
				return;
			}

			if (_commandDictionary.CommandsWithArgs.ContainsKey(command) &&
				(args.Length > 0 || !_commandDictionary.CommandsNoArgs.ContainsKey(command)))
			{
				_commandDictionary.CommandsWithArgs[command].Invoke(player, command, args);
				_debugProcessor.Log(DebugLogCategory.TwitchClient, $"[Debug Command] Executed: {command} {string.Join(" ", args)}");
			}
			else if (_commandDictionary.CommandsNoArgs.ContainsKey(command))
			{
				_commandDictionary.CommandsNoArgs[command].Invoke(player);
				_debugProcessor.Log(DebugLogCategory.TwitchClient, $"[Debug Command] Executed: {command}");
			}
			else
			{
				_debugProcessor.LogError(DebugLogCategory.TwitchClient, $"[Debug Command] Error: Unknown command: {command}");
			}
		}

		private void UpdateUserType(Player player, OnChatCommandReceivedArgs e)
		{
			RequireCommandInfrastructure();

			player.TwitchUser.TwitchUserType = e.Command.ChatMessage.UserType;

			if (_gameMasterCommands.IsGameMaster(player))
			{
				player.TwitchUser.GameUserType = Twitch.Utils.GameUserType.GameMaster;
			}
			else
			{
				switch (player.TwitchUser.TwitchUserType)
				{
					case TwitchLib.Client.Enums.UserType.Viewer:
						player.TwitchUser.GameUserType = Twitch.Utils.GameUserType.Normal;
						break;
					case TwitchLib.Client.Enums.UserType.Moderator:
						player.TwitchUser.GameUserType = Twitch.Utils.GameUserType.Moderator;
						break;
					case TwitchLib.Client.Enums.UserType.Broadcaster:
						player.TwitchUser.GameUserType = Twitch.Utils.GameUserType.Broadcaster;
						break;
					default:
						player.TwitchUser.GameUserType = Twitch.Utils.GameUserType.Normal;
						break;
				}
			}
			player.UnitTextDisplay.SetTextColor(Twitch.Utils.UserColours.GetColourByUserType(player.TwitchUser.GameUserType));
		}

		public void Initialize()
		{
			if (_twitchChatRuntimeData == null)
				throw new InvalidOperationException("TwitchChatProcessor runtime data has not been installed.");

			_twitchChatRuntimeData.BroadcasterConnectCode = GenerateBroadcasterConnectCode();
			InitializeCommandInfrastructure();
		}

		/// <summary>
		/// Sends a message to Twitch chat.
		/// </summary>
		/// <param name="message">The message to send.</param>
		public new void SendMessage(string message)
		{
			RequireCommandInfrastructure();
			_messageSender.SendMessage(message);
		}

		/// <summary>
		/// Sends a message to Twitch chat with the player's name.
		/// </summary>
		/// <param name="playerName">The player name.</param>
		/// <param name="message">The message.</param>
		public void SendMessage(string playerName, string message)
		{
			RequireCommandInfrastructure();
			_messageSender.SendMessage(playerName, message);
		}

		/// <summary>
		/// Sets the player data access methods for Twitch chat to use.
		/// Called by PlayerProcessor during initialization.
		/// </summary>
		public void SetPlayerDataAccess(PlayerExistsByIDDelegate playerExistsByID, PlayerExistsByNameDelegate playerExistsByName, Func<int, Player> getPlayer)
		{
			TwitchChatRuntimeData runtimeData = RequireRuntimeData();
			runtimeData.PlayerExistsByID = playerExistsByID;
			runtimeData.PlayerExistsByName = playerExistsByName;
			runtimeData.GetPlayer = getPlayer;
		}

		public void RecordCommandResult(string command, string result)
		{
			TwitchChatRuntimeData runtimeData = RequireRuntimeData();
			runtimeData.LastCommand = command ?? string.Empty;
			runtimeData.LastCommandResult = result ?? string.Empty;
		}

		public string LastCommand => RequireRuntimeData().LastCommand;
		public string LastCommandUser => RequireRuntimeData().LastCommandUser;
		public string LastCommandResult => RequireRuntimeData().LastCommandResult;

		/// <summary>
		/// Sets the current world time for Twitch chat to use.
		/// Called by TimeProcessor during updates.
		/// </summary>
		public void SetCurrentWorldTime(float time)
		{
			RequireRuntimeData().CurrentWorldTime = time;
		}

		/// <summary>
		/// Sets the player ready state for Twitch chat to use.
		/// Called by GameStateProcessor when the player becomes ready.
		/// </summary>
		public void SetPlayerReady(bool ready)
		{
			RequireRuntimeData().PlayerReady = ready;
		}

		public void EnqueueCameraRequest(Vector3 moveVector, int zoomFactor, bool reset)
		{
			RequireRuntimeData().CameraRequests.Enqueue(new TwitchCameraRequest(moveVector, zoomFactor, reset));
		}

		public bool TryDequeueCameraRequest(out TwitchCameraRequest request)
		{
			request = default;
			if (_twitchChatRuntimeData == null || _twitchChatRuntimeData.CameraRequests == null || _twitchChatRuntimeData.CameraRequests.Count == 0)
				return false;

			request = _twitchChatRuntimeData.CameraRequests.Dequeue();
			return true;
		}

		public string GetBroadcasterConnectCode()
		{
			return RequireRuntimeData().BroadcasterConnectCode;
		}

		public string RefreshBroadcasterConnectCode()
		{
			TwitchChatRuntimeData runtimeData = RequireRuntimeData();
			runtimeData.BroadcasterConnectCode = GenerateBroadcasterConnectCode();
			return runtimeData.BroadcasterConnectCode;
		}

		public bool TryAuthorizeBroadcasterConnection(string providedCode, bool isBroadcaster)
		{
			TwitchChatRuntimeData runtimeData = RequireRuntimeData();
			if (!isBroadcaster)
			{
				Debug.LogWarning("Twitch !connect rejected because the sender was not identified as the channel broadcaster.");
				return false;
			}

			if (string.IsNullOrWhiteSpace(providedCode))
			{
				Debug.LogWarning("Twitch !connect rejected because no connection code was supplied.");
				return false;
			}

			if (!string.Equals(providedCode.Trim(), runtimeData.BroadcasterConnectCode, StringComparison.Ordinal))
			{
				Debug.LogWarning("Twitch !connect rejected because the supplied connection code did not match the active challenge.");
				return false;
			}

			runtimeData.MessagesAllowed = true;
			Debug.Log("Twitch broadcaster connection authorized.");
			return true;
		}

		public void ClearBroadcasterConnectCode()
		{
			RequireRuntimeData().BroadcasterConnectCode = string.Empty;
		}

		public void CompleteBroadcasterConnection()
		{
			ClearBroadcasterConnectCode();
			if (_connectionPanelView != null)
				_connectionPanelView.Hide();
			ReleaseConnectionPause();
		}

		public void CancelBroadcasterConnection()
		{
			if (_twitchChatRuntimeData == null)
				return;

			_twitchChatRuntimeData.MessagesAllowed = false;
			_twitchChatRuntimeData.PlayerReady = false;
			_twitchChatRuntimeData.BroadcasterConnectCode = string.Empty;
			if (_connectionPanelView != null)
				_connectionPanelView.Hide();
			_connectionPanelView = null;
			ReleaseConnectionPause();
		}

		/// <summary>
		/// Sends a prebuilt message to Twitch chat with the player's name.
		/// </summary>
		/// <param name="playerName">The player name.</param>
		/// <param name="key">The message key.</param>
		public void SendPreBuiltMessage(string playerName, string key)
		{
			RequireCommandInfrastructure();
			_messageSender.SendPreBuiltMessage(playerName, key);
		}

		/// <summary>
		/// Sends a prebuilt message to Twitch chat.
		/// </summary>
		/// <param name="key">The message key.</param>
		public void SendPreBuiltMessage(string key)
		{
			RequireCommandInfrastructure();
			_messageSender.SendPreBuiltMessage(key);
		}

		/// <summary>
		/// Sends a message from a player.
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="message">The message.</param>
		public void SendPlayerMessage(Character.Player player, string message)
		{
			RequireCommandInfrastructure();
			_messageSender.SendPlayerMessage(player, message);
		}

		/// <summary>
		/// Gets or sets whether messages are allowed to be sent.
		/// </summary>
		public bool MessagesAllowed
		{
			get => RequireRuntimeData().MessagesAllowed;
			set => RequireRuntimeData().MessagesAllowed = value;
		}

		/// <summary>
		/// Processes Twitch chat logic every frame.
		/// Called every frame by the Coordinator.
		/// TwitchChatProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			// TwitchChatProcessor does not require per-frame updates
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			_connectionPanelView = FindConnectionPanelView();
			if (_connectionPanelView == null)
			{
				ReleaseConnectionPause();
				return;
			}

			TwitchChatRuntimeData runtimeData = RequireRuntimeData();
			runtimeData.MessagesAllowed = false;
			runtimeData.PlayerReady = false;
			runtimeData.BroadcasterConnectCode = GenerateBroadcasterConnectCode();
			AcquireConnectionPause(runtimeData);
			_connectionPanelView.Show(runtimeData.BroadcasterConnectCode);
		}

		private static void AcquireConnectionPause(TwitchChatRuntimeData runtimeData)
		{
			if (runtimeData.ConnectionPauseActive)
				return;

			runtimeData.TimeScaleBeforeConnectionPause = Time.timeScale;
			runtimeData.ConnectionPauseActive = true;
			Time.timeScale = 0f;
		}

		private void ReleaseConnectionPause()
		{
			if (_twitchChatRuntimeData == null || !_twitchChatRuntimeData.ConnectionPauseActive)
				return;

			Time.timeScale = _twitchChatRuntimeData.TimeScaleBeforeConnectionPause;
			_twitchChatRuntimeData.ConnectionPauseActive = false;
		}

		private static TwitchConnectionPanelView FindConnectionPanelView()
		{
			TwitchConnectionPanelView[] views = UnityEngine.Object.FindObjectsByType<TwitchConnectionPanelView>(
				FindObjectsInactive.Include);
			return views.Length > 0 ? views[0] : null;
		}
	}
}
