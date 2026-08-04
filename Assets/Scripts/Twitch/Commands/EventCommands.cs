using GameEventSystem;
using GameEventSystem.Events;
using Processors;
using Twitch.Utils;
using TwitchLib.Client.Events;
using Reflex.Attributes;
using UserInterface;
using UnityEngine;

namespace Twitch.Commands
{
    /// <summary>
    /// Handles Twitch chat event commands from channel point rewards.
    /// </summary>
    public class EventCommands
	{
        private PlayerProcessor _playerProcessor;
        private GameEventProcessor _gameEventProcessor;
        private TownResourceProcessor _townResourceProcessor;
        private ObjectPoolingProcessor _poolingProcessor;
		private UserInterface_Event _eventInterface;
        private Processors.TwitchChatProcessor _twitchChatProcessor;

        public EventCommands(PlayerProcessor playerProcessor, GameEventProcessor gameEventProcessor,
            TownResourceProcessor townResourceProcessor, ObjectPoolingProcessor poolingProcessor,
			Processors.TwitchChatProcessor twitchChatProcessor)
        {
            _playerProcessor = playerProcessor;
            _gameEventProcessor = gameEventProcessor;
            _townResourceProcessor = townResourceProcessor;
            _poolingProcessor = poolingProcessor;
            _twitchChatProcessor = twitchChatProcessor;
        }

		private UserInterface_Event ResolveEventInterface()
		{
			if (_eventInterface == null)
				_eventInterface = UnityEngine.Object.FindAnyObjectByType<UserInterface_Event>();

			return _eventInterface;
		}

        /// <summary>
        /// Processes an event message from Twitch.
        /// </summary>
        /// <param name="e">The message received args.</param>
        /// <returns>True if the message was processed, false otherwise.</returns>
        public bool EventMessage(OnMessageReceivedArgs e)
		{
			string[] words = e.ChatMessage.RawIrcMessage.Split(';');

			if(words.Length >= 3)
			{
				string[] split = words[3].Split('=');

				if(split[0] == "custom-reward-id")
				{
					ProcessReward(split, e);
					return true;
				}
			}

			return false;
		}

        /// <summary>
        /// Processes a channel point reward.
        /// </summary>
        /// <param name="split">The split reward data.</param>
        /// <param name="e">The message received args.</param>
		private void ProcessReward(string[] split, OnMessageReceivedArgs e)
		{
			if (!_playerProcessor.PlayerExistsByID(e.ChatMessage.UserId, out int index))
				return;

			switch(split[1])
			{
				// Fish God
				case "5a760033-50b5-4e47-911b-d63993d2860c":
					HandleFishGodEvent();
					break;
			}
		}

        /// <summary>
        /// Handles the Fish God event.
        /// </summary>
		public void HandleFishGodEvent()
		{
			GameEventProcessor eventProcessor = _gameEventProcessor;

			if(eventProcessor.CurrentEvent != null && eventProcessor.CurrentEvent.Event == GameEvent.EventType.FishGod)
			{
				eventProcessor.CurrentEvent.Action();
				return;
			}

			if (eventProcessor.CurrentEvent != null)
				return;

			int rand = UnityEngine.Random.Range(0, 10);

			if (rand == 0)
				eventProcessor.AddEvent(new FishGodEvent(0, gameEventProcessor: _gameEventProcessor, townResourceProcessor: _townResourceProcessor, playerProcessor: _playerProcessor, poolingProcessor: _poolingProcessor, twitchChatProcessor: _twitchChatProcessor, eventInterface: ResolveEventInterface()));
		}
	}
}
