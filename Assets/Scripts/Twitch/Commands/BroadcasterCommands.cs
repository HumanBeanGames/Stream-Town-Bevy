using Managers;
using System;
using TwitchLib.Client.Events;
using Reflex.Attributes;
using TechTree;
using GameEventSystem;

namespace Twitch.Commands
{
	public static class BroadcasterCommands
	{
		private static GameManager _gameManager;
		[Inject] private static TechTreeManager _techTreeManager;
		[Inject] private static GameEventManager _gameEventManager;

		public static void Initialize(GameManager gameManager)
		{
			_gameManager = gameManager;
		}

		internal static void Connect(string arg, OnChatCommandReceivedArgs e)
		{
#if UNITY_EDITOR
			MessageSender.MessagesAllowed = true;
#else
			if (arg == _gameManager.Code && e.Command.ChatMessage.IsBroadcaster)
				MessageSender.MessagesAllowed = true;
			else
				return;
#endif
			_gameManager.CodeDisplay.text = "";
			_gameManager.ConnectPanel.SetActive(false);
			_techTreeManager.StartCoroutine(_techTreeManager.DelayedSetup());
			_gameEventManager.CanStartNewRulerVote = true;
			if (_gameManager.MetaDatas.LoadType == MetaData.LoadType.Generate || _gameManager.MetaDatas.LoadType == MetaData.LoadType.Load && _gameManager.UserPlayer == null)
				_gameManager.SetUserPlayer(PlayerCommands.TryCreatePlayer(e));
		}
	}
}