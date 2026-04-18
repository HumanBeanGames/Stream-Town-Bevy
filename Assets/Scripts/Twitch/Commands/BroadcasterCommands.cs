using Processors;
using Core;
using System;
using TwitchLib.Client.Events;
using Reflex.Attributes;
using TechTree;
using GameEventSystem;
using MetaData;

namespace Twitch.Commands
{
    /// <summary>
    /// Handles Twitch chat commands for the broadcaster.
    /// </summary>
	public class BroadcasterCommands
	{
        /// <summary>
        /// The game coordinator. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private Coordinator _gameProcessor;

        /// <summary>
        /// The main menu runtime scriptable. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private MainMenuProcessor _mainMenuProcessor;

        /// <summary>
        /// The player runtime scriptable. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// The tech tree processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TechTreeProcessor _techTreeProcessor;

        /// <summary>
        /// The game event processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GameEventProcessor _gameEventProcessor;

        /// <summary>
        /// The message sender. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private MessageSender _messageSender;

        /// <summary>
        /// The player commands. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerCommands _playerCommands;

        /// <summary>
        /// Connects the broadcaster to the game.
        /// </summary>
        /// <param name="arg">The connection code argument.</param>
        /// <param name="e">The chat command received args.</param>
		internal void Connect(string arg, OnChatCommandReceivedArgs e)
		{
#if UNITY_EDITOR
			_messageSender.MessagesAllowed = true;
#else
			if (arg == _gameProcessor.Code && e.Command.ChatMessage.IsBroadcaster)
				_messageSender.MessagesAllowed = true;
			else
				return;
#endif
			_mainMenuProcessor.CodeDisplay?.Invoke("");
			_mainMenuProcessor.ConnectPanel.SetActive(false);
			_techTreeProcessor.RequestDelayedSetup();
			if (_mainMenuProcessor.LoadType == LoadType.Generate || _mainMenuProcessor.LoadType == LoadType.Load && _playerProcessor.UserPlayer == null)
				_playerProcessor.SetUserPlayer(_playerCommands.TryCreatePlayer(e));
		}
	}
}
