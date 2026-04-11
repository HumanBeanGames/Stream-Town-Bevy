using Character;
using Managers;
using Pets.Enumerations;
using System.Collections;
using Twitch;
using UnityEngine;
using UserInterface;

namespace GameEventSystem.Events
{
	public class FishGodEvent : GameEvent
	{
		private int _praisesRequired = 20;
		private int _praisesGiven = 0;
		private GameObject _fishGod;
		private Animator _animator;
		private UserInterface_Event _eventInterface;
		private GameEventManager _gameEventManager;
		private TownResourceManager _townResourceManager;
		private PlayerManager _playerManager;
		private ObjectPoolingManager _poolingManager;

		public FishGodEvent(double startTime, GameEventManager gameEventManager, TownResourceManager townResourceManager, PlayerManager playerManager, ObjectPoolingManager poolingManager, UserInterface_Event eventInterface, double eventDuration = 300, EventType eventType = EventType.FishGod, object data = null, bool overrideCurrentEvent = false, double timeout = -1) : base(startTime, eventDuration, EventType.FishGod, data, overrideCurrentEvent, timeout)
		{
			_gameEventManager = gameEventManager;
			_townResourceManager = townResourceManager;
			_playerManager = playerManager;
			_poolingManager = poolingManager;
			_eventInterface = eventInterface;

			GetFishGodGameObject();
		}

		protected override void OnStarted()
		{
			_gameEventManager.FallingFishVFX.gameObject.SetActive(true);
			_eventInterface.Slider.gameObject.SetActive(true);
			UpdateSlider();
			_eventInterface.TitleTMP.text = "Fish God";
			_eventInterface.DescriptionTMP.text = "Praise the Fish God!";
			_eventInterface.ActivateEventContainer();
		}

		protected override void OnStopped()
		{
			_animator.SetTrigger("Exit");
			_gameEventManager.StartCoroutine(DisableAfterTime());
			_gameEventManager.FallingFishVFX.gameObject.SetActive(false);
			_eventInterface.DeactivateEventContainer();

			if (Success)
			{
				_townResourceManager.AddResource(Utils.Resource.Food, 1000, true);

				// Try to give a player a fish pet if roll hits
				int roll = Random.Range(0, 100);

				if (roll < 70)
				{
					if (_playerManager.PlayerCount() <= 0)
						return;
					
					Player player = null;
					int iters = 0;
					do
					{
						iters++;
						if (iters >= 50)
							break;

						int playerIndex = Random.Range(0, _playerManager.PlayerCount());
						player = _playerManager.GetPlayer(playerIndex);
						if (player.IsNPC)
							continue;
					}
					while (player.IsNPC);

					if (player == null || player.IsNPC)
						return;

					player.PetsUnlocked[PetType.FishGod] = true;
					MessageSender.SendMessage($"{player.TwitchUser.Username} unlocked the fishgod pet!");
				}
			}
		}

		protected void UpdateSlider()
		{
			_eventInterface.SliderTMP.text = $"{_praisesGiven}  /  {_praisesRequired}";
			_eventInterface.Slider.value = (float)_praisesGiven / _praisesRequired;
		}

		protected override void OnActioned(object data = null)
		{
			_praisesGiven++;

			if (_praisesGiven >= _praisesRequired)
				OnCompleteEvent();

			UpdateSlider();
		}

		private void GetFishGodGameObject()
		{
			_fishGod = _poolingManager.GetPooledObject("FishGod").gameObject;
			_animator = _fishGod.GetComponentInChildren<Animator>();
			_fishGod.transform.position = _gameEventManager.FishGodSpawn.position;
			_fishGod.SetActive(true);
		}

		public IEnumerator DisableAfterTime()
		{
			float time = 2.5f;
			float trackedTime = 0;

			while (trackedTime < time)
			{
				trackedTime += Time.deltaTime;
				yield return new WaitForEndOfFrame();
			}

			_fishGod.SetActive(false);
		}
	}
}