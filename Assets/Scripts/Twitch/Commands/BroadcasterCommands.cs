using Processors;
using System;
using TwitchLib.Client.Events;
using TechTree;
using GameEventSystem;
using MetaData;
using UserInterface.MainMenu;
using Reflex.Attributes;

namespace Twitch.Commands
{
    /// <summary>
    /// Handles Twitch chat commands for the broadcaster.
    /// </summary>
	public class BroadcasterCommands
	{
        private MainMenuManager _mainMenuManager;
        private PlayerProcessor _playerProcessor;
        private TechTreeProcessor _techTreeProcessor;
        private GameEventProcessor _gameEventProcessor;
        private Processors.TwitchChatProcessor _twitchChatProcessor;
        private PlayerCommands _playerCommands;

        public BroadcasterCommands(MainMenuManager mainMenuManager,
            PlayerProcessor playerProcessor, TechTreeProcessor techTreeProcessor,
            GameEventProcessor gameEventProcessor, Processors.TwitchChatProcessor twitchChatProcessor,
            PlayerCommands playerCommands)
        {
            _mainMenuManager = mainMenuManager;
            _playerProcessor = playerProcessor;
            _techTreeProcessor = techTreeProcessor;
            _gameEventProcessor = gameEventProcessor;
            _twitchChatProcessor = twitchChatProcessor;
            _playerCommands = playerCommands;
        }

        /// <summary>
        /// Initializes the broadcaster commands by resolving the main menu manager via scene lookup.
        /// </summary>
		public void Initialize()
		{
			_mainMenuManager = UnityEngine.Object.FindFirstObjectByType<MainMenuManager>();
			if (_mainMenuManager != null)
				_mainMenuManager.CodeDisplay?.Invoke(_twitchChatProcessor.GetBroadcasterConnectCode());
		}

        /// <summary>
        /// Connects the broadcaster to the game.
        /// </summary>
        /// <param name="arg">The connection code argument.</param>
        /// <param name="e">The chat command received args.</param>
		internal void Connect(string arg, OnChatCommandReceivedArgs e)
		{
#if UNITY_EDITOR
			_twitchChatProcessor.MessagesAllowed = true;
#else
			if (!_twitchChatProcessor.TryAuthorizeBroadcasterConnection(arg, e.Command.ChatMessage.IsBroadcaster))
				return;
#endif
			_mainMenuManager?.CodeDisplay?.Invoke("");
			_mainMenuManager?.ConnectPanel?.SetActive(false);
			_techTreeProcessor.RequestDelayedSetup();
			if (_mainMenuManager != null && (_mainMenuManager.LoadType == LoadType.Generate || _mainMenuManager.LoadType == LoadType.Load) && _playerProcessor.UserPlayer == null)
				_playerProcessor.SetUserPlayer(_playerCommands.TryCreatePlayer(e));
		}
	}
}
