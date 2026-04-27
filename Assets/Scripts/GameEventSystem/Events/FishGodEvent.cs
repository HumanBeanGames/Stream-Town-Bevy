using Character;
using Processors;
using Pets.Enumerations;
using System.Collections;
using Twitch;
using UnityEngine;
using UserInterface;

namespace GameEventSystem.Events
{
    /// <summary>
    /// Represents a Fish God event where players praise the Fish God to receive rewards.
    /// </summary>
    public class FishGodEvent : GameEvent
    {
        /// <summary>
        /// The number of praises required.
        /// </summary>
        private int _praisesRequired = 20;

        /// <summary>
        /// The number of praises given.
        /// </summary>
        private int _praisesGiven = 0;

        /// <summary>
        /// The Fish God GameObject.
        /// </summary>
        private GameObject _fishGod;

        /// <summary>
        /// The animator for the Fish God.
        /// </summary>
        private Animator _animator;

        /// <summary>
        /// The event interface.
        /// </summary>
        private UserInterface_Event _eventInterface;

        /// <summary>
        /// The game event processor.
        /// </summary>
        private GameEventProcessor _gameEventProcessor;

        /// <summary>
        /// The town resource processor.
        /// </summary>
        private TownResourceProcessor _townResourceProcessor;

        /// <summary>
        /// The player processor.
        /// </summary>
        private PlayerProcessor _playerProcessor;

        /// <summary>
        /// The object pooling processor.
        /// </summary>
        private ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// The Twitch chat processor.
        /// </summary>
        private TwitchChatProcessor _twitchChatProcessor;

        /// <summary>
        /// Initializes a new Fish God event instance.
        /// </summary>
        /// <param name="startTime">The start time.</param>
        /// <param name="gameEventProcessor">The game event processor.</param>
        /// <param name="townResourceProcessor">The town resource processor.</param>
        /// <param name="playerProcessor">The player processor.</param>
        /// <param name="poolingProcessor">The object pooling processor.</param>
        /// <param name="twitchChatProcessor">The Twitch chat processor.</param>
        /// <param name="eventInterface">The event interface.</param>
        /// <param name="eventDuration">The event duration.</param>
        /// <param name="eventType">The event type.</param>
        /// <param name="data">Additional data.</param>
        /// <param name="overrideCurrentEvent">Whether to override the current event.</param>
        /// <param name="timeout">The timeout.</param>
        public FishGodEvent(double startTime, GameEventProcessor gameEventProcessor, TownResourceProcessor townResourceProcessor, PlayerProcessor playerProcessor, ObjectPoolingProcessor poolingProcessor, TwitchChatProcessor twitchChatProcessor, UserInterface_Event eventInterface, double eventDuration = 300, EventType eventType = EventType.FishGod, object data = null, bool overrideCurrentEvent = false, double timeout = -1) : base(startTime, eventDuration, EventType.FishGod, data, overrideCurrentEvent, timeout)
        {
            _gameEventProcessor = gameEventProcessor;
            _townResourceProcessor = townResourceProcessor;
            _playerProcessor = playerProcessor;
            _poolingProcessor = poolingProcessor;
            _twitchChatProcessor = twitchChatProcessor;
            _eventInterface = eventInterface;

            GetFishGodGameObject();
        }

        /// <summary>
        /// Called when the event starts.
        /// </summary>
        protected override void OnStarted()
        {
            _gameEventProcessor.FallingFishVFX.gameObject.SetActive(true);
            _eventInterface.Slider.gameObject.SetActive(true);
            UpdateSlider();
            _eventInterface.TitleTMP.text = "Fish God";
            _eventInterface.DescriptionTMP.text = "Praise the Fish God!";
            _eventInterface.ActivateEventContainer();
        }

        /// <summary>
        /// Called when the event stops.
        /// </summary>
        protected override void OnStopped()
        {
            _animator.SetTrigger("Exit");
            _gameEventProcessor.StartCoroutine(DisableAfterTime());
            _gameEventProcessor.FallingFishVFX.gameObject.SetActive(false);
            _eventInterface.DeactivateEventContainer();

            if (Success)
            {
                _townResourceProcessor.AddResource(Utils.Resource.Food, 1000, true);

                // Try to give a player a fish pet if roll hits.
                int roll = Random.Range(0, 100);

                if (roll < 70)
                {
                    if (_playerProcessor.PlayerCount() <= 0)
                        return;
                    
                    Player player = null;
                    int iters = 0;
                    do
                    {
                        iters++;
                        if (iters >= 50)
                            break;

                        int playerIndex = Random.Range(0, _playerProcessor.PlayerCount());
                        player = _playerProcessor.GetPlayer(playerIndex);
                        if (player.IsNPC)
                            continue;
                    }
                    while (player.IsNPC);

                    if (player == null || player.IsNPC)
                        return;

                    player.PetsUnlocked[PetType.FishGod] = true;
                    _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} unlocked the fishgod pet!");
                }
            }
        }

        /// <summary>
        /// Updates the slider UI.
        /// </summary>
        protected void UpdateSlider()
        {
            _eventInterface.SliderTMP.text = $"{_praisesGiven}  /  {_praisesRequired}";
            _eventInterface.Slider.value = (float)_praisesGiven / _praisesRequired;
        }

        /// <summary>
        /// Called when the event is actioned.
        /// </summary>
        /// <param name="data">The action data.</param>
        protected override void OnActioned(object data = null)
        {
            _praisesGiven++;

            if (_praisesGiven >= _praisesRequired)
                OnCompleteEvent();

            UpdateSlider();
        }

        /// <summary>
        /// Gets the Fish God GameObject from the object pool.
        /// </summary>
        private void GetFishGodGameObject()
        {
            _fishGod = _poolingProcessor.GetPooledObject("FishGod").gameObject;
            _animator = _fishGod.GetComponentInChildren<Animator>();
            _fishGod.transform.position = _gameEventProcessor.FishGodSpawn.position;
            _fishGod.SetActive(true);
        }

        /// <summary>
        /// Coroutine to disable the Fish God after a delay.
        /// </summary>
        /// <returns>The enumerator for the coroutine.</returns>
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
