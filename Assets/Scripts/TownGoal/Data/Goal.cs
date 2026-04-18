using TownGoal.Data;
using System.Collections.Generic;
using UnityEngine;
using System;
using SavingAndLoading.Structs;
using Processors;

namespace TownGoal.Data
{
    /// <summary>
    /// Represents a goal composed of multiple objectives.
    /// </summary>
	public class Goal
	{
        /// <summary>
        /// Event invoked when the goal is completed.
        /// </summary>
		public Action<Goal> OnGoalCompleted;

        /// <summary>
        /// The dictionary of objective statuses.
        /// </summary>
		private Dictionary<Objective, bool> _objectiveStatuses;

        /// <summary>
        /// Gets the dictionary of objective statuses.
        /// </summary>
		public Dictionary<Objective, bool> ObjectivesStatuses => _objectiveStatuses;

        /// <summary>
        /// Initializes a new instance of the Goal class.
        /// </summary>
        /// <param name="objectiveData">The list of objective data.</param>
		public Goal(List<ObjectiveData> objectiveData)
		{
			_objectiveStatuses = new Dictionary<Objective, bool>();
			for (int i = 0; i < objectiveData.Count; i++)
			{
				Objective newObjective = new Objective(objectiveData[i]);
				newObjective.ObjectiveComplete += OnObjectiveComplete;
				_objectiveStatuses.Add(newObjective, false);
			}

			if (_objectiveStatuses.Count == 0)
				CheckAllObjectivesComplete();
		}

        /// <summary>
        /// Sets objectives from save data.
        /// </summary>
        /// <param name="data">The objective save data.</param>
		public void SetobjectivesFromSave(List<ObjectiveSaveData> data)
		{
			int index = 0;
			foreach (Objective objective in _objectiveStatuses.Keys)
			{
				objective.SetValues(data[index].Amount, data[index].RequiredAmount);
			}
		}

        /// <summary>
        /// Forces the goal to complete.
        /// </summary>
		public void ForceComplete()
		{
			if (_objectiveStatuses.Count <= 0)
			{
				Debug.LogError("No Objectives Set!");
			}
			else
			{
				var keys = new List<Objective>(_objectiveStatuses.Keys);

				for(int i = 0; i < keys.Count;i++)
				{
					_objectiveStatuses[keys[i]] = true;
					keys[i].CompleteObjective();
				}	
			}
			CheckAllObjectivesComplete();
		}

        /// <summary>
        /// Called when an objective is complete.
        /// </summary>
        /// <param name="objective">The objective.</param>
		private void OnObjectiveComplete(Objective objective)
		{
			_objectiveStatuses[objective] = true;
			CheckAllObjectivesComplete();
		}

        /// <summary>
        /// Checks if all objectives are complete.
        /// </summary>
		private void CheckAllObjectivesComplete()
		{
			bool allPassed = true;

			foreach (KeyValuePair<Objective, bool> status in _objectiveStatuses)
			{
				if (!status.Value)
				{
					allPassed = false;
					break;
				}

				allPassed = true;
			}

			if (allPassed)
				OnGoalCompleted?.Invoke(this);
		}
	}
}
