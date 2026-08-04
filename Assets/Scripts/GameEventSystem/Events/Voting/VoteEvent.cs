using Character;
using System.Collections.Generic;
using UnityEngine;

namespace GameEventSystem.Events.Voting
{
	/// <summary>
	/// An event used for handling any type of voting needed for the game.
	/// </summary>
	public class VoteEvent : GameEvent
	{
        /// <summary>
        /// The added time.
        /// </summary>
		protected float _addedTime = 0;

        /// <summary>
        /// The winning vote option.
        /// </summary>
		protected VoteOption _winningOption;

        /// <summary>
        /// Dictionary of option names to vote options.
        /// </summary>
		protected Dictionary<string, VoteOption> _options;

        /// <summary>
        /// Dictionary of players to their votes.
        /// </summary>
		protected Dictionary<Player, PlayerVote> _playerVotes;

        /// <summary>
        /// The time passed.
        /// </summary>
		protected float _timePassed = 0;

        /// <summary>
        /// Gets the voting options.
        /// </summary>
		public Dictionary<string, VoteOption> Options => _options;

        /// <summary>
        /// Gets the winning vote option.
        /// </summary>
		public VoteOption WinningOption => _winningOption;

		public IReadOnlyDictionary<Player, PlayerVote> PlayerVotes => _playerVotes;

        /// <summary>
        /// Initializes a new vote event instance.
        /// </summary>
        /// <param name="delay">The delay before the event starts.</param>
        /// <param name="eventDuration">The event duration.</param>
        /// <param name="eventType">The event type.</param>
        /// <param name="data">Additional data.</param>
        /// <param name="overrideCurrentEvent">Whether to override the current event.</param>
        /// <param name="timeout">The timeout.</param>
		public VoteEvent(double delay, double eventDuration, EventType eventType = EventType.Vote, object data = null, bool overrideCurrentEvent = false, double timeout = -1) : base(delay, eventDuration, eventType, data, overrideCurrentEvent, timeout)
		{
			_options = new Dictionary<string, VoteOption>();
			_playerVotes = new Dictionary<Player, PlayerVote>();
			_alwaysReturnSuccess = true;
		}

        /// <summary>
        /// Called when the event is actioned.
        /// </summary>
        /// <param name="data">The action data.</param>
		protected override void OnActioned(object data = null)
		{
			PlayerVote vote = data as PlayerVote;
			TryAddVote(vote, out _);
		}

        /// <summary>
        /// Checks if a player has voted.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <returns>True if the player has voted.</returns>
		public bool HasVoted(Player player)
		{
			return _playerVotes.ContainsKey(player);
		}

        /// <summary>
        /// Updates the vote event.
        /// </summary>
        /// <param name="currentTime">The current time.</param>
		public override void Update(double currentTime)
		{
			base.Update(currentTime);
			if (_playerVotes.Count == 0)
			{
				_addedTime += Time.deltaTime;
				//_eventDuration += Time.deltaTime;
				_eventStartTime += Time.deltaTime;
			}
			else
				_timePassed += Time.deltaTime;
		}

		/// <summary>
		/// Adds an option to the voting event.
		/// </summary>
		/// <param name="option">The vote option to add.</param>
		public void AddOption(VoteOption option)
		{
			if (_options.ContainsKey(option.OptionName))
			{
				Debug.LogError($"{option.OptionName} already exists");
				return;
			}

			_options.Add(option.OptionName, option);
		}

        /// <summary>
        /// Called when the event stops.
        /// </summary>
		protected override void OnStopped() => CalculateWinningVote();

		/// <summary>
		/// Checks if the chosen option is a valid option for this vote.
		/// </summary>
		/// <param name="vote">The player vote.</param>
		/// <returns>True if the option is valid.</returns>
		protected virtual bool CheckOptionIsValid(PlayerVote vote) => _options.ContainsKey(vote.VoteOption.OptionName);

		/// <summary>
		/// Adds a player's vote the tallies.
		/// </summary>
		/// <param name="vote">The player vote.</param>
		public bool TryAddVote(PlayerVote vote, out string failureReason)
		{
			failureReason = string.Empty;
			if (vote == null || vote.Player == null || vote.VoteOption == null || string.IsNullOrWhiteSpace(vote.VoteOption.OptionName))
			{
				failureReason = "The vote was missing a player or option.";
				return false;
			}

			if (_playerVotes.ContainsKey(vote.Player))
			{
				failureReason = "You have already voted.";
				return false;
			}

			if (!CheckOptionIsValid(vote) || !_options.TryGetValue(vote.VoteOption.OptionName, out VoteOption selectedOption))
			{
				failureReason = $"'{vote.VoteOption.OptionName}' is not a valid option. Available options: {string.Join(", ", _options.Keys)}.";
				return false;
			}

			selectedOption.Votes++;
			_playerVotes.Add(vote.Player, vote);
			OnVoteAdded(vote);
			return true;
		}

        /// <summary>
        /// Called when a vote is added.
        /// </summary>
        /// <param name="vote">The player vote.</param>
		protected virtual void OnVoteAdded(PlayerVote vote) { }

		/// <summary>
		/// Calculates the winning vote.
		/// </summary>
		private void CalculateWinningVote()
		{
			VoteOption bestOption = null;

			foreach (KeyValuePair<string, VoteOption> option in _options)
			{
				if (bestOption == null || bestOption.Votes < option.Value.Votes)
					bestOption = option.Value;
			}

			_winningOption = bestOption;
			_returnData = bestOption;
		}
	}
}
