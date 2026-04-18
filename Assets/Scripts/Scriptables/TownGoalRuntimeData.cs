using System.Collections.Generic;
using TownGoal.Data;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime town goal state for the game.
	/// Manages current goals for the town.
	/// </summary>
	public class TownGoalRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		[SerializeField]
		private List<Goal> _currentGoals;

		public List<Goal> CurrentGoals
		{
			get => _currentGoals;
			set => _currentGoals = value;
		}

		/// <summary>
		/// Initializes the town goal runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
