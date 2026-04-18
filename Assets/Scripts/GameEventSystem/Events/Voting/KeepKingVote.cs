using Processors;
using Reflex.Attributes;
using System;
using System.Collections.Generic;
using System.Linq;
using UserInterface;

namespace GameEventSystem.Events.Voting
{
    /// <summary>
    /// Represents a vote event to keep or replace the current king/ruler.
    /// </summary>
	public class KeepKingVote : VoteEvent
	{
        /// <summary>
        /// List of tracked UI ruler options.
        /// </summary>
		private List<UI_RulerOption> _trackedOptions = new List<UI_RulerOption>();

        /// <summary>
        /// The ruler vote interface.
        /// </summary>
		private UserInterface_RulerVote _rulerVoteInterface;

        /// <summary>
        /// UI runtime scriptable data. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private UIProcessor _uiProcessor;

        /// <summary>
        /// Initializes a new Keep King vote instance.
        /// </summary>
        /// <param name="delay">The delay before the event starts.</param>
        /// <param name="eventDuration">The event duration.</param>
        /// <param name="eventType">The event type.</param>
        /// <param name="data">Additional data.</param>
        /// <param name="overrideCurrentEvent">Whether to override the current event.</param>
        /// <param name="timeout">The timeout.</param>
		public KeepKingVote(double delay, double eventDuration, EventType eventType = EventType.KeepKingVote, object data = null, bool overrideCurrentEvent = false, double timeout = -1) : base(delay, eventDuration, eventType, data, overrideCurrentEvent, timeout)
		{
			_alwaysReturnSuccess = true;
			_rulerVoteInterface = _uiProcessor.RulerVoteInterface;

			_options.Add("yes", new VoteOption("yes", null));
			_options.Add("no", new VoteOption("no", null));
			InitializeOptions();
		}

        /// <summary>
        /// Initializes the voting options.
        /// </summary>
		private void InitializeOptions()
		{
			var ui = _rulerVoteInterface.AddOption("no");
			ui.TextTMP.text = "!vote No";
			_trackedOptions.Add(ui);
			ui = _rulerVoteInterface.AddOption("yes");
			ui.TextTMP.text = "!vote Yes";
			_trackedOptions.Add(ui);
		}

        /// <summary>
        /// Called when a vote is added.
        /// </summary>
        /// <param name="vote">The player vote.</param>
		protected override void OnVoteAdded(PlayerVote vote)
		{
			UpdateOptions();
		}

        /// <summary>
        /// Called when the event starts.
        /// </summary>
		protected override void OnStarted()
		{
			base.OnStarted();
			_rulerVoteInterface.ActivateRulerContainer();
			_rulerVoteInterface.DescriptionTMP.text = "Keep the ruler?";
		}

        /// <summary>
        /// Called when the event stops.
        /// </summary>
		protected override void OnStopped()
		{
			base.OnStopped();
			_rulerVoteInterface.DisableRulerContainer();
		}

        /// <summary>
        /// Updates the vote event.
        /// </summary>
        /// <param name="currentTime">The current time.</param>
		public override void Update(double currentTime)
		{
			base.Update(currentTime);
			float val = 1 - (float)((_timePassed) / (EventDuration));
			_rulerVoteInterface.TimerSlider.value = val;
			TimeSpan timespan = TimeSpan.FromSeconds(Math.Ceiling(EventDuration - _timePassed));
			_rulerVoteInterface.TimerTMP.text = $"{string.Format("{0:D2}:{1:D2}", timespan.Minutes, timespan.Seconds)}";
		}

        /// <summary>
        /// Updates the voting options UI.
        /// </summary>
		private void UpdateOptions()
		{
			List<VoteOption> optionsSorted = new List<VoteOption>();

			foreach (var v in _options)
			{
				optionsSorted.Add(v.Value);
			}

			optionsSorted = optionsSorted.OrderByDescending(x => x.Votes).ToList();

			List<UI_RulerOption> rulerOptions = new List<UI_RulerOption>();

			if (_rulerVoteInterface.Options.Count <= 0)
				return;

			foreach (var v in _rulerVoteInterface.Options)
				rulerOptions.Add(v.Key);

			for (int i = 0; i < rulerOptions.Count; i++)
				rulerOptions[i].TextTMP.text = $"!vote {optionsSorted[i].OptionName} ({optionsSorted[i].Votes})";
		}

	}
}
