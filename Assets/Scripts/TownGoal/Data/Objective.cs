using Processors;
using SavingAndLoading.Structs;
using System;
using TownGoal.Enumerations;
using UnityEngine;
using Utils;

namespace TownGoal.Data
{
    /// <summary>
    /// Represents an objective within a goal.
    /// </summary>
	public class Objective
	{
        /// <summary>
        /// Event invoked when the objective is complete.
        /// </summary>
		public Action<Objective> ObjectiveComplete;

        /// <summary>
        /// Event invoked when the amount changes.
        /// </summary>
		public Action<Objective, int> AmountChanged;

        /// <summary>
        /// The objective type.
        /// </summary>
		private ObjectiveType _objectiveType;

        /// <summary>
        /// The required amount.
        /// </summary>
		private int _requiredAmount;

        /// <summary>
        /// The current amount.
        /// </summary>
		private int _amount;

        /// <summary>
        /// The objective data.
        /// </summary>
		private ObjectiveData _data;

        /// <summary>
        /// Gets the current amount.
        /// </summary>
		public int Amount => _amount;

        /// <summary>
        /// Gets the required amount.
        /// </summary>
		public int RequiredAmount => _requiredAmount;

        /// <summary>
        /// Gets the objective type.
        /// </summary>
		public ObjectiveType ObjectiveType => _objectiveType;

        /// <summary>
        /// Gets the objective data.
        /// </summary>
		public ObjectiveData Data => _data;

        public string GetDisplayLabel()
        {
            return _data.GetDisplayLabel();
        }

        public string GetRequirementText()
        {
            return _data.GetRequirementText();
        }

        public string GetProgressText()
        {
            return $"{GetDisplayLabel()}: {_amount}/{_requiredAmount}";
        }

        /// <summary>
        /// Initializes a new instance of the Objective class.
        /// </summary>
        /// <param name="data">The objective data.</param>
		public Objective(ObjectiveData data)
		{
			_data = data;
			_objectiveType = data.ObjectiveType;
			_requiredAmount = (int)(data.IntValue);
			ObjectiveComplete += ClearListeners;
			_amount = 0;
		}

        /// <summary>
        /// Subscribes to game events based on objective type.
        /// </summary>
        /// <param name="eventProcessor">The game event processor.</param>
		public void SubscribeToEvents(GameEventProcessor eventProcessor)
		{
			switch (_objectiveType)
			{
				case ObjectiveType.Build:
					eventProcessor.BuildingBuilt += OnBuildingBuilt;
					break;
				case ObjectiveType.BuildAny:
					eventProcessor.BuildingBuilt += OnBuildingBuilt;
					break;
				case ObjectiveType.Collect:
					eventProcessor.ResourceGained += OnResourceGained;
					break;
				case ObjectiveType.Kill:
					eventProcessor.EnemyKilled += HandleEnemyKilled;
					break;
				case ObjectiveType.KillAny:
					eventProcessor.EnemyKilled += HandleEnemyKilled;
					break;
				case ObjectiveType.EarnPerHour:
					eventProcessor.ResourceGained += OnResourceGained;
					break;
				case ObjectiveType.Sell:
					eventProcessor.ResourceSold += OnResourceSold;
					break;
				case ObjectiveType.SellAny:
					eventProcessor.ResourceSold += OnResourceSold;
					break;
				case ObjectiveType.Buy:
					eventProcessor.ResourceBought += OnResourceBought;
					break;
				case ObjectiveType.BuyAny:
					eventProcessor.ResourceBought += OnResourceBought;
					break;
				default:
					break;
			}
		}

        /// <summary>
        /// Unsubscribes from game events based on objective type.
        /// </summary>
        /// <param name="eventProcessor">The game event processor.</param>
		public void UnsubscribeFromEvents(GameEventProcessor eventProcessor)
		{
			switch (_objectiveType)
			{
				case ObjectiveType.Build:
					eventProcessor.BuildingBuilt -= OnBuildingBuilt;
					break;
				case ObjectiveType.BuildAny:
					eventProcessor.BuildingBuilt -= OnBuildingBuilt;
					break;
				case ObjectiveType.Collect:
					eventProcessor.ResourceGained -= OnResourceGained;
					break;
				case ObjectiveType.Kill:
					eventProcessor.EnemyKilled -= HandleEnemyKilled;
					break;
				case ObjectiveType.KillAny:
					eventProcessor.EnemyKilled -= HandleEnemyKilled;
					break;
				case ObjectiveType.EarnPerHour:
					eventProcessor.ResourceGained -= OnResourceGained;
					break;
				case ObjectiveType.Sell:
					eventProcessor.ResourceSold -= OnResourceSold;
					break;
				case ObjectiveType.SellAny:
					eventProcessor.ResourceSold -= OnResourceSold;
					break;
				case ObjectiveType.Buy:
					eventProcessor.ResourceBought -= OnResourceBought;
					break;
				case ObjectiveType.BuyAny:
					eventProcessor.ResourceBought -= OnResourceBought;
					break;
				default:
					break;
			}

			ObjectiveComplete -= ClearListeners;
		}

        /// <summary>
        /// Handles enemy killed event.
        /// </summary>
        /// <param name="enemyType">The enemy type.</param>
		public void HandleEnemyKilled(EnemyType enemyType)
		{
			if (_objectiveType == ObjectiveType.KillAny)
			{
				_amount++;
				OnAmountChanged();
			}
			else if (_data.EnemyType == enemyType)
			{
				_amount++;
				OnAmountChanged();
			}
		}

        /// <summary>
        /// Handles resource gained event.
        /// </summary>
        /// <param name="resource">The resource type.</param>
        /// <param name="amount">The amount gained.</param>
		private void OnResourceGained(Resource resource, int amount)
		{
			if (_objectiveType == ObjectiveType.Collect && _data.ResourceType == resource)
			{
				_amount += amount;
				OnAmountChanged();
			}
			else if (_objectiveType == ObjectiveType.EarnPerHour && _data.ResourceType == resource)
			{
				_amount = _requiredAmount;
				OnAmountChanged();
			}
		}

        /// <summary>
        /// Handles building built event.
        /// </summary>
        /// <param name="type">The building type.</param>
		private void OnBuildingBuilt(BuildingType type)
		{
			if (_objectiveType == ObjectiveType.BuildAny)
			{
				_amount++;
				OnAmountChanged();
			}
			else if (type == _data.BuildingType)
			{
				_amount++;
				OnAmountChanged();
			}
		}

        /// <summary>
        /// Completes the objective.
        /// </summary>
		public void CompleteObjective()
		{
			_amount = _requiredAmount;
			ObjectiveComplete?.Invoke(this);
			Debug.Log($"Objective Complete '{_objectiveType}'");
		}

        /// <summary>
        /// Called when the amount changes.
        /// </summary>
		private void OnAmountChanged()
		{
			AmountChanged?.Invoke(this, _amount);
			if (_amount >= _requiredAmount)
			{
				CompleteObjective();
			}
		}

        /// <summary>
        /// Handles resource bought event.
        /// </summary>
        /// <param name="resourceType">The resource type.</param>
        /// <param name="amount">The amount bought.</param>
		private void OnResourceBought(Resource resourceType, int amount)
		{
			if (_objectiveType == ObjectiveType.BuyAny || _data.ResourceType == resourceType)
			{
				_amount += amount;
				OnAmountChanged();
			}
		}

        /// <summary>
        /// Handles resource sold event.
        /// </summary>
        /// <param name="resourceType">The resource type.</param>
        /// <param name="amount">The amount sold.</param>
		private void OnResourceSold(Resource resourceType, int amount)
		{
			if (_objectiveType == ObjectiveType.SellAny || _data.ResourceType == resourceType)
			{
				_amount += amount;
				OnAmountChanged();
			}
		}

		/// <summary>
		/// Clears all event listeners.
		/// </summary>
		/// <param name="objective">The objective (unused, for delegate compatibility).</param>
		private void ClearListeners(Objective objective)
		{
			// Clear all event subscriptions
			ObjectiveComplete = null;
			AmountChanged = null;
		}

		/// <summary>
		/// Sets the amount and required amount from save data.
		/// </summary>
		/// <param name="amount">The current amount.</param>
		/// <param name="requiredAmount">The required amount.</param>
		public void SetValues(int amount, int requiredAmount)
		{
			_amount = amount;
			_requiredAmount = requiredAmount;
		}
	}
}
