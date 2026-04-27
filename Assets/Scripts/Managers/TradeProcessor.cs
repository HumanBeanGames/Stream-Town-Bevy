using Data.Containers;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;
using Utils;
using GameEventSystem;

namespace Processors
{
    /// <summary>
    /// Manages resource trading between resources and gold.
    /// Handles buying and selling resources with tax rates.
    /// </summary>
	public class TradeProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// ScriptableObject containing trade settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TradeSettings _tradeSettings;

        /// <summary>
        /// Town resource processor for accessing resource data.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TownResourceProcessor _townResourceProcessor;

        /// <summary>
        /// ScriptableObject containing game event processor.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GameEventProcessor _gameEventProcessor;

        /// <summary>
        /// Registers this processor as a singleton in the dependency injection container.
        /// Called by Reflex during container initialization.
        /// </summary>
        /// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}

		/// <summary>
		/// Sells an amount of a resource for gold.
		/// </summary>
		/// <param name="resource">The resource type to sell.</param>
		/// <param name="amount">The amount to sell.</param>
		/// <param name="message">Output message describing the transaction.</param>
		public void SellResource(Resource resource, int amount, out string message)
		{
			message = "";

			int availableAmount = _townResourceProcessor.CurrentResourceAmount(resource);

			if (amount <= 0)
			{
				message = $"No {resource} available to sell!";
				return;
			}

			if (amount > availableAmount)
				amount = availableAmount;

			_townResourceProcessor.RemoveResource(resource, amount, true);

			int goldValue = (int)(amount * _tradeSettings.ResourceSellRates[resource]);
			goldValue -= (int)(goldValue * _tradeSettings.SellTaxRate);

			_townResourceProcessor.AddResource(Resource.Gold, goldValue, true);
			_gameEventProcessor.InvokeResourceSold(resource, amount);
			message = $"Sold {amount} {resource} for {goldValue} gold.";
		}

		/// <summary>
		/// Buys an amount of a resource for gold.
		/// </summary>
		/// <param name="resource">The resource type to buy.</param>
		/// <param name="amount">The amount to buy. If the amount exceeds the available storage space or gold, it will be adjusted accordingly.</param>
		/// <param name="message">Output message describing the transaction.</param>
		public void BuyResource(Resource resource, int amount, out string message)
		{
			message = "";

			int availableGold = _townResourceProcessor.CurrentResourceAmount(Resource.Gold);
			int remainingStorageAmount = _townResourceProcessor.MaxResourceAmount(resource) - _townResourceProcessor.CurrentResourceAmount(resource);

			if (remainingStorageAmount <= 0)
			{
				message = "Storages are full, can't buy!";
				return;
			}

			if (remainingStorageAmount < amount)
				amount = remainingStorageAmount;

			float costPerResource = _tradeSettings.ResourceSellRates[resource] / _tradeSettings.BuyTaxRate;
			int costForAll = (int)(costPerResource * amount);

			if (costForAll > availableGold)
			{
				amount = (int)((float)availableGold / costPerResource);
				costForAll = (int)(costPerResource * amount);
			}

			_townResourceProcessor.RemoveResource(Resource.Gold, costForAll, true);
			_townResourceProcessor.AddResource(resource, amount, true);
			_gameEventProcessor.InvokeResourceBought(resource, amount);
			message = $"Bought {amount} {resource} for {costForAll} gold.";
		}

        /// <summary>
        /// Initializes the trade processor.
        /// No initialization logic required.
        /// </summary>
		public void Initialize()
		{
			// TradeProcessor doesn't require initialization logic
		}

        /// <summary>
        /// Processes trade logic every frame.
        /// Called every frame by the Coordinator.
        /// TradeProcessor does not require per-frame updates.
        /// </summary>
        public void Process()
        {
            // TradeProcessor does not require per-frame updates
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // TradeProcessor does not have scene-specific settings to refresh
        }
    }
}
