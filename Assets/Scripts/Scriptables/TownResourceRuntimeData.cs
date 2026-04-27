using Character;

using ScriptablesProcessorInfrastructure;
using GameResources;
using System.Collections.Generic;
using Utils;
using UnityEngine.Events;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores town resource state for the game.
	/// Manages resource amounts, storage, and resource change events.
	/// </summary>
	public class TownResourceRuntimeData : IRuntimeDataScriptable
	{
		public const float RESOURCE_RATE_TIME_PERIOD = 25;
		public const float RESOURCE_UPDATE_RATE = 1;

		private Dictionary<Resource, ResourceInventory> _resources;
		private Dictionary<Resource, UnityEvent<StorageStatus>> _onResourceChangeEventDict;
		private UnityEvent<Resource, int, bool> _onAnyResourceChangeEvent;
		private Dictionary<Resource, ResourceRateOfChange> _resourceRatesOfChange;
		private Dictionary<Resource, int> _resourceBoostValues;
		private UnityEvent<Resource, int> _resourceGained;
		private UnityEvent<Resource, int> _resourceSold;
		private UnityEvent<Resource, int> _resourceBought;

		public Dictionary<Resource, ResourceInventory> Resources => _resources;
		public Dictionary<Resource, UnityEvent<StorageStatus>> OnResourceChangeEventDict => _onResourceChangeEventDict;
		public UnityEvent<Resource, int, bool> OnAnyResourceChangeEvent => _onAnyResourceChangeEvent;
		public UnityEvent<Resource, int> ResourceGained => _resourceGained;
		public UnityEvent<Resource, int> ResourceSold => _resourceSold;
		public UnityEvent<Resource, int> ResourceBought => _resourceBought;
		public Dictionary<Resource, ResourceRateOfChange> ResourceRatesOfChange => _resourceRatesOfChange;
		public Dictionary<Resource, int> ResourceBoostValues => _resourceBoostValues;

		/// <summary>
		/// Initializes the town resource runtime data with default values.
		/// </summary>
		public TownResourceRuntimeData()
		{
			_resources = new Dictionary<Resource, ResourceInventory>();
			_onResourceChangeEventDict = new Dictionary<Resource, UnityEvent<StorageStatus>>();
			_onAnyResourceChangeEvent = new UnityEvent<Resource, int, bool>();
			_resourceRatesOfChange = new Dictionary<Resource, ResourceRateOfChange>();
			_resourceBoostValues = new Dictionary<Resource, int>();
			_resourceGained = new UnityEvent<Resource, int>();
			_resourceSold = new UnityEvent<Resource, int>();
			_resourceBought = new UnityEvent<Resource, int>();
		}
	}
}
