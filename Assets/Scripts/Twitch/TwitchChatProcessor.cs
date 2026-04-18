using Character;
using Character.Enumerations;
using Processors;
using System;
using System.Collections.Generic;
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

namespace Processors
{
	/// <summary>
	/// General Processor for all functions relating to Twitch Chat integration.
	/// </summary>
	public class TwitchChatProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		[Inject] private PlayerProcessor _playerProcessor;
		[Inject] private TimeProcessor _timeProcessor;
		[Inject] private GameStateProcessor _gameStateProcessor;
		[Inject] private PlayerCommands _playerCommands;
		[Inject] private BroadcasterCommands _broadcasterCommands;
		[Inject] private MessageSender _messageSender;
		[Inject] private CommandDictionary _commandDictionary;
		[Inject] private EventCommands _eventCommands;
		[Inject] private GameMasterCommands _gameMasterCommands;
		[Inject] private RoleCommands _roleCommands;
		[Inject] private BuildingCommands _buildingCommands;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}

		/// <summary>
		/// Processes any command sent in chat on Twitch and invokes the required action is the command is valid.
		/// </summary>
		/// <param name="e"></param>
		public void ProcessCommand(OnChatCommandReceivedArgs e)
		{
			if (!_gameStateProcessor.PlayerReady)
				return;

			string command = e.Command.CommandText.ToLower();

			if (!_messageSender.MessagesAllowed)
			{
				if (e.Command.ArgumentsAsList == null || e.Command.ArgumentsAsList.Count < 0)
				return;
#if UNITY_EDITOR
				_broadcasterCommands.Connect("", e);
#else
				if (e.Command.CommandText.ToLower() == "connect" && e.Command.ChatMessage.IsBroadcaster)
					_broadcasterCommands.Connect(e.Command.ArgumentsAsList[0], e);
				return;
#endif

			}

			if (_commandDictionary.SimpleCommands.ContainsKey(command))
			{
				_commandDictionary.SimpleCommands[command].Invoke();
				return;
			}

			// Check that player exists
			if (_playerProcessor.PlayerExistsByID(e.Command.ChatMessage.UserId, out int index))
			{
				Player player = _playerProcessor.GetPlayer(index);
				if (player == null)
					return;
				player.TwitchUser.TimeSinceLastMessage = _timeProcessor.CurrentWorldTime;

				UpdateUserType(player, e);


				// Check if the command has arguments.
				if (e.Command.ArgumentsAsList.Count > 0 && _commandDictionary.CommandsWithArgs.ContainsKey(command))
				{
					string[] argsToLower = e.Command.ArgumentsAsList.ToArray();

					//Lowecase all arguments
					for (int i = 0; i < argsToLower.Length; i++)
					{
						argsToLower[i] = argsToLower[i].ToLower();
					}

					_commandDictionary.CommandsWithArgs[command].Invoke(player, command, argsToLower);
				}
				else if (_commandDictionary.CommandsNoArgs.ContainsKey(command))
				{
					_commandDictionary.CommandsNoArgs[command].Invoke(player);
				}
			}
			// Check if player is trying to create character or call a simple command
			else
			{
				if (CommandDictionary.CreateNameVariants.Contains(command))
					_playerCommands.TryCreatePlayer(e);
			}
		}

		public void ProcessMessage(OnMessageReceivedArgs e)
		{
			if (_playerProcessor.PlayerExistsByID(e.ChatMessage.UserId, out int index))
			{
				Player player = _playerProcessor.GetPlayer(index);
				player.TwitchUser.TimeSinceLastMessage = _timeProcessor.CurrentWorldTime;
			}
			// Check for event
			if (_eventCommands.EventMessage(e))
				return;
		}

		private void UpdateUserType(Player player, OnChatCommandReceivedArgs e)
		{
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
			// TwitchChatProcessor doesn't require initialization logic
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
	}
}
