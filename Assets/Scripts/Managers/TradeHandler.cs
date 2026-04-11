using Reflex.Attributes;
using System.Collections.Generic;
using Utils;

namespace Managers
{
	public static class TradeHandler
	{
		/// <summary>
		/// How much gold each resource sells for.
		/// </summary>
		public static Dictionary<Resource, float> ResourceSellRates = new Dictionary<Resource, float>()
		{
			{ Resource.Wood, 0.25f },
			{Resource.Ore, 0.25f },
			{Resource.Food, 0.25f }
		};

		public static float SellTaxRate = 0.5f;
		public static float BuyTaxRate = 0.6f;


		[Inject] private static TownResourceManager _townResourceManager;

		/// <summary>
		/// Sells an amount of a resource for gold.
		/// </summary>
		/// <param name="resource"></param>
		/// <param name="amount"></param>
		/// <param name="message"></param>
		public static void SellResource(Resource resource, int amount, out string message)
		{
			message = "";

			int availableAmount = _townResourceManager.CurrentResourceAmount(resource);

			if (amount <= 0)
			{
				message = $"No {resource} available to sell!";
				return;
			}

			if (amount > availableAmount)
				amount = availableAmount;

			_townResourceManager.RemoveResource(resource, amount, true);

			int goldValue = (int)(amount * ResourceSellRates[resource]);
			goldValue -= (int)(goldValue * SellTaxRate);

			_townResourceManager.AddResource(Resource.Gold, goldValue, true);
			EventManager.ResourceSold?.Invoke(resource, amount);
			message = $"Sold {amount} {resource} for {goldValue} gold.";
		}

		/// <summary>
		/// Buys an amount of a resource for gold.
		/// </summary>
		/// <param name="resource"></param>
		/// <param name="amount"></param>
		/// <param name="message"></param>
		public static void BuyResource(Resource resource, int amount, out string message)
		{
			message = "";

			int availableGold = _townResourceManager.CurrentResourceAmount(Resource.Gold);
			int remainingStorageAmount = _townResourceManager.MaxResourceAmount(resource) - _townResourceManager.CurrentResourceAmount(resource);

			if (remainingStorageAmount <= 0)
			{
				message = "Storages are full, can't buy!";
				return;
			}

			if (remainingStorageAmount < amount)
				amount = remainingStorageAmount;

			float costPerResource = ResourceSellRates[resource] / BuyTaxRate;
			int costForAll = (int)(costPerResource * amount);

			if (costForAll > availableGold)
			{
				amount = (int)((float)availableGold / costPerResource);
				costForAll = (int)(costPerResource * amount);
			}

			_townResourceManager.RemoveResource(Resource.Gold, costForAll, true);
			_townResourceManager.AddResource(resource, amount, true);
			EventManager.ResourceBought?.Invoke(resource, amount);
			message = $"Bought {amount} {resource} for {costForAll} gold.";
		}
	}
}