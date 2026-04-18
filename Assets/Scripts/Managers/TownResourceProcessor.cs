using Character;
using GameResources;
using System.Collections.Generic;
using UnityEngine;
using Utils;
using UnityEngine.Events;
using System;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using ResourceData = ScriptablesProcessorInfrastructure.ResourceData;
using GameEventSystem;

namespace Processors
{
	/// <summary>
	/// Processes all the resources for the town.
	/// Manages resource amounts, storage, and resource change events.
	/// </summary>
	public class TownResourceProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// ScriptableObject containing resource data.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ResourceData _resourceData;

        /// <summary>
        /// ScriptableObject containing town resource runtime data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TownResourceRuntimeData _townResourceRuntimeData;

        /// <summary>
        /// Gets the resource boost values dictionary.
        /// </summary>
        public Dictionary<Resource, int> ResourceBoostValues => _townResourceRuntimeData.ResourceBoostValues;

        /// <summary>
        /// ScriptableObject containing game event processor.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GameEventProcessor _gameEventProcessor;

        /// <summary>
        /// Event invoked when any resource amount changes.
        /// </summary>
		public event Action<Resource, int, bool> OnAnyResourceChangeEvent;

        /// <summary>
        /// Gets the resource change event for a specific resource type.
        /// </summary>
        /// <param name="type">The resource type to get the event for.</param>
        /// <returns>The unity event for the resource type.</returns>
		public UnityEvent<StorageStatus> GetResourceChangeEvent(Resource type)
		{
			return _townResourceRuntimeData.OnResourceChangeEventDict[type];
		}

		/// <summary>
		/// Gets the current amount of a resource.
		/// </summary>
		/// <param name="resourceType">The resource type to get the amount for.</param>
		/// <returns>The current amount of the resource.</returns>
		public int GetResourceAmount(Resource resourceType)
		{
			return _townResourceRuntimeData.Resources[resourceType].Amount;
		}

		/// <summary>
		/// Sets the amount of a resource to a specific value.
		/// </summary>
		/// <param name="resourceType">The resource type to set.</param>
		/// <param name="resourceAmount">The amount to set.</param>
		public void SetResourceAmount(Resource resourceType, int resourceAmount)
		{
			_townResourceRuntimeData.Resources[resourceType].Amount = resourceAmount;
		}

		/// <summary>
		/// Adds an amount to a resource.
		/// </summary>
		/// <param name="type">The resource type to add to.</param>
		/// <param name="amount">The amount to add.</param>
		/// <param name="purchase">Whether this is a purchase action.</param>
		public void AddResource(Resource type, int amount, bool purchase = false)
		{
			_townResourceRuntimeData.Resources[type].Amount += amount;
			ResourceChanged(type, amount, purchase);
		}

		/// <summary>
		/// Removes an amount from a resource.
		/// </summary>
		/// <param name="type">The resource type to remove from.</param>
		/// <param name="amount">The amount to remove.</param>
		/// <param name="purchase">Whether this is a purchase action.</param>
		public void RemoveResource(Resource type, int amount, bool purchase = false)
		{
			_townResourceRuntimeData.Resources[type].Amount -= amount;
			ResourceChanged(type, -amount, purchase);
		}

		/// <summary>
		/// Checks if a resource storage is full.
		/// </summary>
		/// <param name="type">The resource type to check.</param>
		/// <returns>True if the resource storage is full.</returns>
		public bool ResourceFull(Resource type)
		{
			return _townResourceRuntimeData.Resources[type].Full;
		}

		/// <summary>
		/// Checks if a resource amount is greater than or equal to a specified amount.
		/// </summary>
		/// <param name="type">The resource type to check.</param>
		/// <param name="amount">The amount to compare against.</param>
		/// <returns>True if the resource amount is greater than or equal to the specified amount.</returns>
		public bool MoreThanEqualComparison(Resource type, int amount)
		{
			return _townResourceRuntimeData.Resources[type].Amount >= amount;
		}

		/// <summary>
		/// Increases the maximum storage capacity for a resource.
		/// </summary>
		/// <param name="type">The resource type to increase storage for.</param>
		/// <param name="amount">The amount to increase storage by.</param>
		public void IncreaseStorage(Resource type, int amount)
		{
			_townResourceRuntimeData.Resources[type].MaxAmount += amount;
			ResourceChanged(type, amount, true);
		}

		/// <summary>
		/// Reduces the maximum storage capacity for a resource.
		/// </summary>
		/// <param name="type">The resource type to reduce storage for.</param>
		/// <param name="amount">The amount to reduce storage by.</param>
		public void ReduceStorage(Resource type, int amount)
		{
			_townResourceRuntimeData.Resources[type].MaxAmount -= amount;
			ResourceChanged(type, -amount, true);
		}

		/// <summary>
		/// Gets the storage status of a resource.
		/// </summary>
		/// <param name="type">The resource type to get status for.</param>
		/// <returns>The storage status of the resource.</returns>
		public StorageStatus GetResourceStatus(Resource type)
		{
			StorageStatus storageStatus = StorageStatus.Empty;

			if (_townResourceRuntimeData.Resources[type].Full)
				storageStatus = StorageStatus.Full;
			else if (_townResourceRuntimeData.Resources[type].HalfFull)
				storageStatus = StorageStatus.HalfFull;

			return storageStatus;
		}

		/// <summary>
		/// Gets a formatted string displaying resource data.
		/// </summary>
		/// <param name="type">The resource type to get data for.</param>
		/// <returns>A formatted string showing resource amount and max amount.</returns>
		public string ResourcePrint(Resource type)
		{
			return _townResourceRuntimeData.Resources[type].ResourceDataToString;
		}

		/// <summary>
		/// Gets the current amount of a resource.
		/// </summary>
		/// <param name="type">The resource type to get amount for.</param>
		/// <returns>The current amount of the resource.</returns>
		public int CurrentResourceAmount(Resource type)
		{
			return _townResourceRuntimeData.Resources[type].Amount;
		}

		/// <summary>
		/// Gets the maximum storage capacity for a resource.
		/// </summary>
		/// <param name="type">The resource type to get max amount for.</param>
		/// <returns>The maximum storage capacity of the resource.</returns>
		public int MaxResourceAmount(Resource type)
		{
			return _townResourceRuntimeData.Resources[type].MaxAmount;
		}

		/// <summary>
		/// Gets the average rate of change for a resource over time.
		/// </summary>
		/// <param name="resource">The resource to get rate of change for.</param>
		/// <returns>The average rate of change over time.</returns>
		public int RateOfChangeForResource(Resource resource)
		{
			return _townResourceRuntimeData.ResourceRatesOfChange[resource].AverageOverTime;
		}

		/// <summary>
		/// Initializes the town resource processor.
		/// Sets up resource inventories, rate of change trackers, and events.
		/// </summary>
		public void Initialize()
		{
			_townResourceRuntimeData.Resources.Add(Resource.Food, new ResourceInventory(
				_resourceData.FoodStartingAmount,
				_resourceData.FoodMaxAmount,
				_resourceData.FoodInfinite));
			_townResourceRuntimeData.Resources.Add(Resource.Ore, new ResourceInventory(
				_resourceData.OreStartingAmount,
				_resourceData.OreMaxAmount,
				_resourceData.OreInfinite));
			_townResourceRuntimeData.Resources.Add(Resource.Wood, new ResourceInventory(
				_resourceData.WoodStartingAmount,
				_resourceData.WoodMaxAmount,
				_resourceData.WoodInfinite));
			_townResourceRuntimeData.Resources.Add(Resource.Gold, new ResourceInventory(
				_resourceData.GoldStartingAmount,
				_resourceData.GoldMaxAmount,
				_resourceData.GoldInfinite));
			_townResourceRuntimeData.Resources.Add(Resource.Recruit, new ResourceInventory(
				_resourceData.RecruitStartingAmount,
				_resourceData.RecruitMaxAmount,
				_resourceData.RecruitInfinite));

			_townResourceRuntimeData.ResourceRatesOfChange.Add(Resource.Food, new ResourceRateOfChange(Resource.Food, TownResourceRuntimeData.RESOURCE_RATE_TIME_PERIOD, TownResourceRuntimeData.RESOURCE_UPDATE_RATE, this));
			_townResourceRuntimeData.ResourceRatesOfChange.Add(Resource.Ore, new ResourceRateOfChange(Resource.Ore, TownResourceRuntimeData.RESOURCE_RATE_TIME_PERIOD, TownResourceRuntimeData.RESOURCE_UPDATE_RATE, this));
			_townResourceRuntimeData.ResourceRatesOfChange.Add(Resource.Wood, new ResourceRateOfChange(Resource.Wood, TownResourceRuntimeData.RESOURCE_RATE_TIME_PERIOD, TownResourceRuntimeData.RESOURCE_UPDATE_RATE, this));
			_townResourceRuntimeData.ResourceRatesOfChange.Add(Resource.Gold, new ResourceRateOfChange(Resource.Gold, TownResourceRuntimeData.RESOURCE_RATE_TIME_PERIOD, TownResourceRuntimeData.RESOURCE_UPDATE_RATE, this));
			_townResourceRuntimeData.ResourceRatesOfChange.Add(Resource.Recruit, new ResourceRateOfChange(Resource.Recruit, TownResourceRuntimeData.RESOURCE_RATE_TIME_PERIOD, TownResourceRuntimeData.RESOURCE_UPDATE_RATE, this));

			_townResourceRuntimeData.OnResourceChangeEventDict.Add(Resource.Food, new UnityEvent<StorageStatus>());
			_townResourceRuntimeData.OnResourceChangeEventDict.Add(Resource.Ore, new UnityEvent<StorageStatus>());
			_townResourceRuntimeData.OnResourceChangeEventDict.Add(Resource.Wood, new UnityEvent<StorageStatus>());
			_townResourceRuntimeData.OnResourceChangeEventDict.Add(Resource.Gold, new UnityEvent<StorageStatus>());
			_townResourceRuntimeData.OnResourceChangeEventDict.Add(Resource.Recruit, new UnityEvent<StorageStatus>());

			_townResourceRuntimeData.ResourceBoostValues.Add(Resource.Food, 0);
			_townResourceRuntimeData.ResourceBoostValues.Add(Resource.Ore, 0);
			_townResourceRuntimeData.ResourceBoostValues.Add(Resource.Wood, 0);
			_townResourceRuntimeData.ResourceBoostValues.Add(Resource.Recruit, 0);
		}

		/// <summary>
		/// Registers this processor as a singleton in the dependency injection container.
		/// Called by Reflex during container initialization.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register TownResourceRuntimeData ScriptableObject
			TownResourceRuntimeData townResourceRuntimeData = ScriptableObject.CreateInstance<TownResourceRuntimeData>();
			containerBuilder.AddSingleton(townResourceRuntimeData);
		}

		/// <summary>
		/// Attempts to take the cost for reviving a character.
		/// </summary>
		/// <param name="type">The revive type (self or others).</param>
		/// <returns>True if the cost was successfully taken.</returns>
		public bool TryTakeReviveCost(Utils.ReviveType type)
		{
			if(type == ReviveType.Others && CurrentResourceAmount(Resource.Food) >= 200)
			{
				RemoveResource(Resource.Food, 200);
				return true;
			}
			else if(type == ReviveType.Self && CurrentResourceAmount(Resource.Food) >= 400)
			{
				RemoveResource(Resource.Food, 400);
				return true;
			}
			else return false;
		}

		/// <summary>
		/// Processes resource rate of change queues.
		/// Called every frame by the Coordinator.
		/// </summary>
		public void Process()
		{
			foreach (var r in _townResourceRuntimeData.ResourceRatesOfChange)
			{
				r.Value.ProcessQueue();
			}
		}

		/// <summary>
		/// Gets the boost value for a resource.
		/// </summary>
		/// <param name="resource">The resource to get boost value for.</param>
		/// <returns>The boost value for the resource.</returns>
		public int GetResourceBoostValue(Resource resource)
		{
			if (_townResourceRuntimeData.ResourceBoostValues.ContainsKey(resource))
				return _townResourceRuntimeData.ResourceBoostValues[resource];
			return 0;
		}

		// Handles resource change events and invokes appropriate events.
		private void ResourceChanged(Resource type, int amount, bool purchase = false)
		{
			_townResourceRuntimeData.OnAnyResourceChangeEvent.Invoke(type, amount, purchase);
			OnAnyResourceChangeEvent?.Invoke(type, amount, purchase);
			_townResourceRuntimeData.OnResourceChangeEventDict[type].Invoke(GetResourceStatus(type));

			if (amount > 0)
				_gameEventProcessor.InvokeResourceGained(type, amount);
			else if (purchase)
			{
				if (type == Resource.Gold)
					_gameEventProcessor.InvokeResourceBought(type, -amount);
				else
					_gameEventProcessor.InvokeResourceSold(type, -amount);
			}
		}
	}
}
