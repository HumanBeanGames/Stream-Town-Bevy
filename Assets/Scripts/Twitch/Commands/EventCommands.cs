using GameEventSystem;
using GameEventSystem.Events;
using Processors;
using Core;
using Twitch.Utils;
using TwitchLib.Client.Events;
using Reflex.Attributes;

namespace Twitch.Commands 
{
    /// <summary>
    /// Handles Twitch chat event commands from channel point rewards.
    /// </summary>
    public class EventCommands 
	{
        /// <summary>
        /// The player processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// The game event processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GameEventProcessor _gameEventProcessor;

        /// <summary>
        /// The town resource processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TownResourceProcessor _townResourceProcessor;

        /// <summary>
        /// The object pooling processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// The game coordinator. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private Coordinator _gameProcessor;

        /// <summary>
        /// The UI processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private UIProcessor _uiProcessor;

        /// <summary>
        /// The message sender. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private MessageSender _messageSender;

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
				eventProcessor.AddEvent(new FishGodEvent(0, _gameEventProcessor, _townResourceProcessor, _playerProcessor, _poolingProcessor, _messageSender, _uiProcessor.EventInterface));
		}
	}
}
