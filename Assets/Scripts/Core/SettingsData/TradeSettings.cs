using System.Collections.Generic;
using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "TradeSettings", menuName = "Scriptables/Trade Settings")]
	public class TradeSettings : ScriptableObject, IDataScriptable
	{
		[System.NonSerialized]
		private Dictionary<Resource, float> _resourceSellRates = new Dictionary<Resource, float>()
		{
			{ Resource.Wood, 0.25f },
			{ Resource.Ore, 0.25f },
			{ Resource.Food, 0.25f }
		};

		[SerializeField]
		private float _sellTaxRate = 0.5f;

		[SerializeField]
		private float _buyTaxRate = 0.6f;

		public Dictionary<Resource, float> ResourceSellRates => _resourceSellRates;
		public float SellTaxRate => _sellTaxRate;
		public float BuyTaxRate => _buyTaxRate;
	}
}
