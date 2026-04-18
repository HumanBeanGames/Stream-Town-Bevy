using Character;
using GameResources;
using System.Collections.Generic;
using UnityEngine;
using Utils;
using UnityEngine.Events;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime town resource state for the game.
	/// Manages resource amounts, storage, and resource change events.
	/// </summary>
	public class TownResourceRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		public const float RESOURCE_RATE_TIME_PERIOD = 25;
		public const float RESOURCE_UPDATE_RATE = 1;

		[SerializeField]
		private Dictionary<Resource, ResourceInventory> _resources = new Dictionary<Resource, ResourceInventory>();
		[SerializeField]
		private Dictionary<Resource, UnityEvent<StorageStatus>> _onResourceChangeEventDict = new Dictionary<Resource, UnityEvent<StorageStatus>>();
		[SerializeField]
		private UnityEvent<Resource, int, bool> _onAnyResourceChangeEvent = new UnityEvent<Resource, int, bool>();
		[SerializeField]
		private Dictionary<Resource, ResourceRateOfChange> _resourceRatesOfChange = new Dictionary<Resource, ResourceRateOfChange>();
		[SerializeField]
		private Dictionary<Resource, int> _resourceBoostValues = new Dictionary<Resource, int>();
		[SerializeField]
		private UnityEvent<Resource, int> _resourceGained = new UnityEvent<Resource, int>();
		[SerializeField]
		private UnityEvent<Resource, int> _resourceSold = new UnityEvent<Resource, int>();
		[SerializeField]
		private UnityEvent<Resource, int> _resourceBought = new UnityEvent<Resource, int>();

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
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
