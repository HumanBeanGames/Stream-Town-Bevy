using System.Collections.Generic;
using UserInterface;
using System.Linq;
using System;
using Processors;

namespace GameEventSystem.Events.Voting
{
    /// <summary>
    /// Represents a vote event to elect a new king/ruler.
    /// </summary>
    public class NewKingVote : VoteEvent
    {
        /// <summary>
        /// Maximum number of tracked options.
        /// </summary>
        private const int MAX_TRACKED_OPTIONS = 5;

        /// <summary>
        /// List of tracked UI ruler options.
        /// </summary>
        private List<UI_RulerOption> _trackedOptions = new List<UI_RulerOption>();

        private readonly PlayerProcessor _playerProcessor;

        /// <summary>
        /// The ruler vote interface.
        /// </summary>
        private UserInterface_RulerVote _rulerVoteInterface;

        /// <summary>
        /// Initializes a new New King vote instance.
        /// </summary>
        /// <param name="delay">The delay before the event starts.</param>
        /// <param name="eventDuration">The event duration.</param>
		/// <param name="rulerVoteInterface">The scene-local UI adapter used to render the vote.</param>
		/// <param name="playerProcessor">The processor used to validate player-name options.</param>
        /// <param name="eventType">The event type.</param>
        /// <param name="data">Additional data.</param>
        /// <param name="overrideCurrentEvent">Whether to override the current event.</param>
        /// <param name="timeout">The timeout.</param>
        public NewKingVote(double delay, double eventDuration, UserInterface_RulerVote rulerVoteInterface, PlayerProcessor playerProcessor, EventType eventType = EventType.NewKingVote, object data = null, bool overrideCurrentEvent = false, double timeout = -1) : base(delay, eventDuration, eventType, data, overrideCurrentEvent, timeout)
        {
            _alwaysReturnSuccess = true;
            _rulerVoteInterface = rulerVoteInterface ?? throw new ArgumentNullException(nameof(rulerVoteInterface));
            _playerProcessor = playerProcessor ?? throw new ArgumentNullException(nameof(playerProcessor));
        }

        /// <summary>
        /// Checks if a vote option is valid.
        /// </summary>
        /// <param name="vote">The player vote.</param>
        /// <returns>True if the option is valid.</returns>
        protected override bool CheckOptionIsValid(PlayerVote vote)
        {
            string optionName = vote.VoteOption.OptionName;

            if (!_options.ContainsKey(vote.VoteOption.OptionName))
            {
                if (_playerProcessor.PlayerExistsByNameToLower(optionName, out int index))
                {
                    vote.VoteOption.Votes = 0;
                    _options.Add(optionName, vote.VoteOption);
                    OnOptionAdded(vote);
                }
                else
                    return false;
            }

            return true;
        }

        /// <summary>
        /// Called when a new option is added.
        /// </summary>
        /// <param name="vote">The player vote.</param>
        private void OnOptionAdded(PlayerVote vote)
        {
            if (_options.Count >= MAX_TRACKED_OPTIONS)
            {
                UpdateOptions();
                return;
            }

            var ui = _rulerVoteInterface.AddOption(vote.VoteOption.OptionName);

            ui.TextTMP.text = $"{vote.VoteOption.OptionName} ({vote.VoteOption.Votes})";
            _trackedOptions.Add(ui);
        }

        /// <summary>
        /// Called when the event starts.
        /// </summary>
        protected override void OnStarted()
        {
            base.OnStarted();
            _rulerVoteInterface.DisableRulerContainer();
            _rulerVoteInterface.ActivateRulerContainer();
            _rulerVoteInterface.DescriptionTMP.text = "Who should be Ruler? \n type !vote playername";
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
        /// Called when a vote is added.
        /// </summary>
        /// <param name="vote">The player vote.</param>
        protected override void OnVoteAdded(PlayerVote vote)
        {
            UpdateOptions();
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

            for (int i = 0; i < MAX_TRACKED_OPTIONS && i < rulerOptions.Count && i < optionsSorted.Count; i++)
            {
                rulerOptions[i].TextTMP.text = $"{optionsSorted[i].OptionName} ({optionsSorted[i].Votes})";
            }
        }
    }
}
