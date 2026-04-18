using Processors;
using Reflex.Attributes;
using System;
using System.Collections.Generic;
using TechTree.Data;
using UnityEngine;
using UserInterface;

namespace GameEventSystem.Events.Voting
{
    /// <summary>
    /// Represents a vote event for selecting a technology to research.
    /// </summary>
	public class TechVote : VoteEvent
	{
        /// <summary>
        /// Dictionary of tracked UI tech options to vote options.
        /// </summary>
		private Dictionary<UI_TechOption, VoteOption> _trackedOptions;

        /// <summary>
        /// The town vote interface.
        /// </summary>
		private UserInterface_TownVote _townVoteInterface;

        /// <summary>
        /// UI runtime scriptable data. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private UIProcessor _uiProcessor;

        /// <summary>
        /// Initializes a new Tech vote instance.
        /// </summary>
        /// <param name="delay">The delay before the event starts.</param>
        /// <param name="eventDuration">The event duration.</param>
        /// <param name="nodeDataArray">The tech node data to vote on.</param>
        /// <param name="eventType">The event type.</param>
        /// <param name="data">Additional data.</param>
        /// <param name="overrideCurrentEvent">Whether to override the current event.</param>
        /// <param name="timeout">The timeout.</param>
		public TechVote(double delay, double eventDuration, TechNodeData[] nodeDataArray, EventType eventType = EventType.TechVote, object data = null, bool overrideCurrentEvent = false, double timeout = -1) : base(delay, eventDuration, eventType, data, overrideCurrentEvent, timeout)
		{
			_townVoteInterface = _uiProcessor.TownVoteInterface;

			_trackedOptions = new Dictionary<UI_TechOption, VoteOption>();
			for (int i = 0; i < nodeDataArray.Length; i++)
			{
				if (nodeDataArray[i] == null)
					continue;

				VoteOption newOption = new VoteOption($"{i + 1}", nodeDataArray[i]);
				_options.Add($"{i + 1}", newOption);
				_trackedOptions.Add(_townVoteInterface.AddOption(nodeDataArray[i], i + 1), newOption);
			}
		}

        /// <summary>
        /// Called when a vote is added.
        /// </summary>
        /// <param name="vote">The player vote.</param>
		protected override void OnVoteAdded(PlayerVote vote)
		{
			base.OnVoteAdded(vote);
			UpdateOptions();
		}

        /// <summary>
        /// Called when the event starts.
        /// </summary>
		protected override void OnStarted()
		{
			base.OnStarted();
			_townVoteInterface.ActivateVoteContainer();
			_townVoteInterface.SetupButtons();
		}

        /// <summary>
        /// Called when the event stops.
        /// </summary>
		protected override void OnStopped()
		{
			base.OnStopped();
			_townVoteInterface.DeactivateVoteContainer();
		}

        /// <summary>
        /// Updates the vote event.
        /// </summary>
        /// <param name="currentTime">The current time.</param>
		public override void Update(double currentTime)
		{
			base.Update(currentTime);
			float val = 1 - (float)((_timePassed) / (EventDuration));
			_townVoteInterface.TimerSlider.value = val;
			TimeSpan timespan = TimeSpan.FromSeconds(Math.Ceiling(EventDuration - _timePassed));
			_townVoteInterface.TimerTMP.text = $"{string.Format("{0:D2}:{1:D2}", timespan.Minutes, timespan.Seconds)}";
		}

        /// <summary>
        /// Updates the voting options UI.
        /// </summary>
		private void UpdateOptions()
		{
			foreach (var option in _trackedOptions)
			{
				float percentage = option.Value.Votes / (float)Mathf.Max(1, _playerVotes.Count);
				option.Key.VotesSlider.value = percentage;
				option.Key.VotesAmountTMP.text = $"{Mathf.Floor(percentage * 100)}% ({option.Value.Votes})";
			}
		}

	}
}
