using System.Collections.Generic;

using ScriptablesProcessorInfrastructure;
using TownGoal.Data;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores town goal state for the game.
	/// Manages current goals for the town.
	/// </summary>
	public class TownGoalRuntimeData : IRuntimeDataScriptable
	{
		private List<Goal> _currentGoals;

		public List<Goal> CurrentGoals
		{
			get => _currentGoals;
			set => _currentGoals = value;
		}

		/// <summary>
		/// Initializes the town goal runtime data with default values.
		/// </summary>
		public TownGoalRuntimeData()
		{
			_currentGoals = new List<Goal>();
		}
	}
}
