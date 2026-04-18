using System.Collections.Generic;
using UnityEngine;
using UserInterface;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime UI state for the game.
	/// Manages UI interface references.
	/// </summary>
	public class UIRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		[SerializeField]
		private UserInterface_TownGoal _townGoalInterface;
		[SerializeField]
		private UserInterface_RulerVote _rulerVoteInterface;
		[SerializeField]
		private UserInterface_TownVote _townVoteInterface;
		[SerializeField]
		private UserInterface_Event _eventInterface;

		public UserInterface_TownGoal TownGoalInterface
		{
			get => _townGoalInterface;
			set => _townGoalInterface = value;
		}
		public UserInterface_RulerVote RulerVoteInterface
		{
			get => _rulerVoteInterface;
			set => _rulerVoteInterface = value;
		}
		public UserInterface_TownVote TownVoteInterface
		{
			get => _townVoteInterface;
			set => _townVoteInterface = value;
		}
		public UserInterface_Event EventInterface
		{
			get => _eventInterface;
			set => _eventInterface = value;
		}

		/// <summary>
		/// Initializes the UI runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
