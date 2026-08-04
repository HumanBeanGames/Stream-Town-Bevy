using Processors;
using TwitchLib.Client.Events;
using TechTree;
using MetaData;

namespace Twitch.Commands
{
    /// <summary>
    /// Handles Twitch chat commands for the broadcaster.
    /// </summary>
	public class BroadcasterCommands
	{
        private PlayerProcessor _playerProcessor;
        private TechTreeProcessor _techTreeProcessor;
        private MetaData.MetaData _metaData;
        private Processors.TwitchChatProcessor _twitchChatProcessor;
        private PlayerCommands _playerCommands;

        public BroadcasterCommands(PlayerProcessor playerProcessor,
            TechTreeProcessor techTreeProcessor, MetaData.MetaData metaData,
            Processors.TwitchChatProcessor twitchChatProcessor,
            PlayerCommands playerCommands)
        {
            _playerProcessor = playerProcessor;
            _techTreeProcessor = techTreeProcessor;
            _metaData = metaData;
            _twitchChatProcessor = twitchChatProcessor;
            _playerCommands = playerCommands;
        }

        /// <summary>
        /// Connects the broadcaster to the game.
        /// </summary>
        /// <param name="arg">The connection code argument.</param>
        /// <param name="e">The chat command received args.</param>
		internal void Connect(string arg, OnChatCommandReceivedArgs e)
		{
			if (!_twitchChatProcessor.TryAuthorizeBroadcasterConnection(arg, e.Command.ChatMessage.IsBroadcaster))
				return;

			_twitchChatProcessor.CompleteBroadcasterConnection();
			_techTreeProcessor.RequestDelayedSetup();
			if (_metaData != null &&
				(_metaData.LoadType == LoadType.Generate || _metaData.LoadType == LoadType.Load) &&
				_playerProcessor.UserPlayer == null)
			{
				_playerProcessor.SetUserPlayer(_playerCommands.TryCreatePlayer(e));
			}
		}
	}
}
