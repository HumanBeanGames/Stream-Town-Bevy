using System.Collections.Generic;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Processors;

namespace TownGoal
{
	using Data;
	using System;

	public class TownGoalProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		[Inject] private TownGoalSettings _townGoalSettings;
		[Inject] private TownGoalRuntimeData _townGoalRuntimeData;

		/// <summary>
		/// Gets or sets the current goals list.
		/// </summary>
		public List<Goal> CurrentGoals
		{
			get => _townGoalRuntimeData.CurrentGoals;
			set => _townGoalRuntimeData.CurrentGoals = value;
		}
		[Inject] private GameEventProcessor _gameEventProcessor;

		public static int MaxGoals => 2;

		// Objective UI.

		public void Initialize()
		{
			_townGoalRuntimeData.CurrentGoals = new List<Goal>(1);
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register TownGoalRuntimeData ScriptableObject
			TownGoalRuntimeData townGoalRuntimeData = ScriptableObject.CreateInstance<TownGoalRuntimeData>();
			containerBuilder.AddSingleton(townGoalRuntimeData);
		}

		public bool StartNewGoal(Goal goal)
		{
			if (ContainsGoal(goal))
			{
				Debug.LogWarning($"Attempted to start goal that was already started '{goal}'");
				return false;
			}

			goal.OnGoalCompleted += OnGoalCompleted;
			AddGoal(goal);

			// Subscribe objectives to game events
			foreach (var objective in goal.ObjectivesStatuses.Keys)
			{
				objective.SubscribeToEvents(_gameEventProcessor);
			}

			return true;
		}

		public bool ForceStopGoal(Goal goal)
		{
			List<KeyValuePair<Objective, bool>> statuses = new List<KeyValuePair<Objective, bool>>();
			foreach (var v in goal.ObjectivesStatuses)
				statuses.Add(v);

			for(int i = 0; i < statuses.Count;i++)
			{
				goal.ObjectivesStatuses[statuses[i].Key] = true;
				statuses[i].Key.CompleteObjective();
			}
			return true;
		}

		private void OnGoalCompleted(Goal goal)
		{
			goal.OnGoalCompleted -= OnGoalCompleted;
			if (!ContainsGoal(goal))
				return;

			Debug.Log("GOAL COMPLETED!");

			// Unsubscribe objectives from game events
			foreach (var objective in goal.ObjectivesStatuses.Keys)
			{
				objective.UnsubscribeFromEvents(_gameEventProcessor);
			}

			RemoveUIElement(goal);
			RemoveGoal(goal);

		}

		private void BuildObjectiveUI(Objective objective)
		{

		}

		private void RemoveUIElement(Goal goal)
		{

		}

		private bool AddGoal(Goal goal)
		{
			if (_townGoalRuntimeData.CurrentGoals.Contains(goal))
				return false;

			_townGoalRuntimeData.CurrentGoals.Add(goal);
			return true;
		}

		private bool RemoveGoal(Goal goal)
		{
			if (!_townGoalRuntimeData.CurrentGoals.Contains(goal))
				return false;

			_townGoalRuntimeData.CurrentGoals.Remove(goal);
			return true;
		}

		private bool ContainsGoal(Goal goal)
		{
			return _townGoalRuntimeData.CurrentGoals.Contains(goal);
		}

		/// <summary>
		/// Processes town goal logic every frame.
		/// Called every frame by the Coordinator.
		/// TownGoalProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			// TownGoalProcessor does not require per-frame updates
		}
	}
}
