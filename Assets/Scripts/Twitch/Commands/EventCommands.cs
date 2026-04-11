using GameEventSystem;
using GameEventSystem.Events;
using Managers;
using System;
using TwitchLib.Client.Events;
using Reflex.Attributes;

namespace Twitch.Commands 
{
    public static class EventCommands 
	{
		private static PlayerManager _playerManager;
		private static GameEventManager _gameEventManager;
		[Inject] private static TownResourceManager _townResourceManager;
		[Inject] private static ObjectPoolingManager _poolingManager;
		[Inject] private static GameManager _gameManager;

		public static void Initialize(PlayerManager playerManager, GameEventManager gameEventManager)
		{
			_playerManager = playerManager;
			_gameEventManager = gameEventManager;
		}

        public static bool EventMessage(OnMessageReceivedArgs e)
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

		private static void ProcessReward(string[] split, OnMessageReceivedArgs e)
		{
			if (!_playerManager.PlayerExistsByID(e.ChatMessage.UserId, out int index))
				return;

			switch(split[1])
			{
				// Fish God
				case "5a760033-50b5-4e47-911b-d63993d2860c":
					HandleFishGodEvent();
					break;
			}
		}

		public static void HandleFishGodEvent()
		{
			GameEventManager eventManager = _gameEventManager;

			if(eventManager.CurrentEvent != null && eventManager.CurrentEvent.Event == GameEvent.EventType.FishGod)
			{
				eventManager.CurrentEvent.Action();
				return;
			}

			if (eventManager.CurrentEvent != null)
				return;

			int rand = UnityEngine.Random.Range(0, 10);

			if (rand == 0)
				eventManager.AddEvent(new FishGodEvent(0, _gameEventManager, _townResourceManager, _playerManager, _poolingManager, _gameManager.UIManager.EventInterface));
		}
	}
}