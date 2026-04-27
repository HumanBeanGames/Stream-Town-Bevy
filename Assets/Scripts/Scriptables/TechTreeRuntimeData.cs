using System.Collections.Generic;

using ScriptablesProcessorInfrastructure;
using TechTree.Data;
using TechTree.ScriptableObjects;
using TechTree;
using TownGoal;
using TownGoal.Data;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores tech tree state for the game.
	/// Manages technology tree, goals, and unlock state.
	/// </summary>
	public class TechTreeRuntimeData : IRuntimeDataScriptable
	{
		private int _timeSinceLastUnlock;
		private TechnologyTree _techTree;
		private Dictionary<Goal, Node_SO> _goalsFollowed;
		private int _techsUnlocked;
		private Node_SO _currentTech;
		private bool _requestStartTechVote;
		private float _requestedTechVoteDelay;
		private bool _requestDelayedSetup;

		public int TimeSinceLastUnlock
		{
			get => _timeSinceLastUnlock;
			set => _timeSinceLastUnlock = value;
		}
		public TechnologyTree TechTree => _techTree;
		public Dictionary<Goal, Node_SO> GoalsFollowed => _goalsFollowed;
		public int TechsUnlocked
		{
			get => _techsUnlocked;
			set => _techsUnlocked = value;
		}
		public Node_SO CurrentTech
		{
			get => _currentTech;
			set => _currentTech = value;
		}
		public bool RequestStartTechVote
		{
			get => _requestStartTechVote;
			set => _requestStartTechVote = value;
		}
		public float RequestedTechVoteDelay
		{
			get => _requestedTechVoteDelay;
			set => _requestedTechVoteDelay = value;
		}
		public bool RequestDelayedSetup
		{
			get => _requestDelayedSetup;
			set => _requestDelayedSetup = value;
		}

		/// <summary>
		/// Initializes the TechTreeRuntimeData with default values.
		/// </summary>
		public TechTreeRuntimeData()
		{
			_timeSinceLastUnlock = 0;
			_techsUnlocked = 0;
			_currentTech = null;
			_requestStartTechVote = false;
			_requestedTechVoteDelay = 0f;
			_requestDelayedSetup = false;
			_techTree = null;
			_goalsFollowed = new Dictionary<Goal, Node_SO>();
		}

		public void InitializeTechTree(TechnologyTree techTree)
		{
			_techTree = techTree;
		}

		public void InitializeGoalsFollowed(Dictionary<Goal, Node_SO> goalsFollowed)
		{
			_goalsFollowed = goalsFollowed ?? new Dictionary<Goal, Node_SO>();
		}

		public bool HasGoalFollowed(Goal goal)
		{
			return _goalsFollowed.ContainsKey(goal);
		}

		public Node_SO GetGoalNode(Goal goal)
		{
			return _goalsFollowed.TryGetValue(goal, out Node_SO node) ? node : null;
		}

		public void RemoveGoalFollowed(Goal goal)
		{
			_goalsFollowed.Remove(goal);
		}

		public void ClearCurrentTech()
		{
			_currentTech = null;
		}

		public void AddGoalFollowed(Goal goal, Node_SO node)
		{
			_goalsFollowed[goal] = node;
		}

		public void IncrementTechsUnlocked()
		{
			_techsUnlocked++;
		}
	}
}
