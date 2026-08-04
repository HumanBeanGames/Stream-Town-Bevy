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
		private TownGoalRuntimeData _townGoalRuntimeData;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

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
			if (_townGoalRuntimeData == null)
				throw new InvalidOperationException("TownGoalProcessor runtime data has not been installed.");

			_townGoalRuntimeData.CurrentGoals = new List<Goal>(1);
		}

		public void ResetWorldState()
		{
			for (int i = 0; i < _townGoalRuntimeData.CurrentGoals.Count; i++)
			{
				Goal goal = _townGoalRuntimeData.CurrentGoals[i];
				if (goal == null)
					continue;

				goal.OnGoalCompleted -= OnGoalCompleted;
				foreach (Objective objective in goal.ObjectivesStatuses.Keys)
					objective.UnsubscribeFromEvents(_gameEventProcessor);
			}

			_townGoalRuntimeData.CurrentGoals.Clear();
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_townGoalRuntimeData != null)
				throw new InvalidOperationException("TownGoalProcessor runtime data has already been installed.");

			_townGoalRuntimeData = new TownGoalRuntimeData();
			containerBuilder.AddSingleton(_townGoalRuntimeData);
		}

		public bool StartNewGoal(Goal goal)
		{
			if (ContainsGoal(goal))
			{
				_debugProcessor.LogWarning(DebugLogCategory.TownGoalProcessor, $"Attempted to start goal that was already started '{goal}'");
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

			_debugProcessor.Log(DebugLogCategory.TownGoalProcessor, "GOAL COMPLETED!");

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

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// TownGoalProcessor does not have scene-specific settings to refresh
		}
	}
}
