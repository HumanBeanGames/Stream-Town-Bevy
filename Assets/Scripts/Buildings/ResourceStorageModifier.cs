using Level;
using Processors;
using TechTree;
using UnityEngine;
using Utils;
using Reflex.Attributes;
using Data.Containers;

namespace Buildings
{
	/// <summary>
	/// Component that modifies the town's resource storage
	/// </summary>
	public class ResourceStorageModifier : MonoBehaviour
	{
		/// <summary>
		/// Which resource type this component modifies.
		/// </summary>
		[SerializeField, Tooltip("Which resource type this component modifies.")]
		private Resource _resource;

		/// <summary>
		/// How much this component expands the resource when spawned or constructed.
		/// </summary>
		[SerializeField, Tooltip("How much it expands the storage on spawn/construction.")]
		private int _baseAmount;

		/// <summary>
		/// How much the storage expands when incremented.
		/// </summary>
		[SerializeField, Tooltip("How much it expands the storage per increment.")]
		private int _incrementAmount;

		/// <summary>
		/// How much the increment gets multiplied per increment.
		/// </summary>
		[SerializeField, Tooltip("How much the increment amount gets increased per increment")]
		private float _incrementMultiPerLevel = 4;

		/// <summary>
		/// Total amount of storage this component has added.
		/// </summary>
		private int _totalAmount = 0;

		// Required Components.
        /// <summary>
        /// Town resource processor for resource storage operations.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TownResourceProcessor _resourceProcessor;

        /// <summary>
        /// Tech tree processor for tech tree event subscriptions.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TechTreeProcessor _techTreeProcessor;

        /// <summary>
        /// Level handler for building level tracking.
        /// </summary>
		private LevelHandler _levelHandler;

		// Properties.
        /// <summary>
        /// Gets the resource type this component modifies.
        /// </summary>
		public Resource ResourceType => _resource;

		/// <summary>
		/// Increments the amount of storage based on set parameters.
		/// </summary>
		public void Increment()
		{
			RecalculateStorageAmount();
		}

		/// <summary>
		/// Adds the base amount of storage to the resource processor.
		/// </summary>
		public void AddBaseStorage()
		{
			//int amount = _baseAmount;
			//amount += (int)(amount * (_resourceProcessor.ResourceBoostValues[_resource] / 100.0f));
			//_resourceProcessor.IncreaseStorage(_resource, amount);
			//_totalAmount += amount;
			RecalculateStorageAmount();
		}

		/// <summary>
		/// Removes the total amount of storage that this component contributed.
		/// </summary>
		public void RemoveTotalStorage()
		{
			_resourceProcessor.ReduceStorage(_resource, _totalAmount);
			_totalAmount = 0;
		}

        // Recalculates the storage amount based on current level and resource boost.
		private void RecalculateStorageAmount()
		{
			if (_levelHandler == null)
				_levelHandler = GetComponent<LevelHandler>();

			int amount = _levelHandler.Level <= 1 ? _baseAmount : _incrementAmount * (int)(_levelHandler.Level * _incrementMultiPerLevel);
			amount += (int)(amount * (_resourceProcessor.GetResourceBoostValue(_resource) / 100.0f));
			RemoveTotalStorage();
			_resourceProcessor.IncreaseStorage(_resource, amount);
			_totalAmount = amount;
		}

        // Called when resource storage boost is unlocked via tech tree.
		private void OnResourceStorageIncreased(Resource type)
		{
			if (type != _resource)
				return;

			RecalculateStorageAmount();
		}

        // Initializes the level handler.
		private void Start()
		{
			_levelHandler = GetComponent<LevelHandler>();
		}

		/// <summary>
		/// Called when object is disabled.
		/// Removes storage and unsubscribes from events.
		/// </summary>
		private void OnDisable()
		{
			if (_resourceProcessor != null)
				RemoveTotalStorage();

			_techTreeProcessor.OnStorageBoostUnlocked -= OnResourceStorageIncreased;
		}

        // Subscribes to tech tree events when enabled.
		private void OnEnable()
		{
			_techTreeProcessor.OnStorageBoostUnlocked += OnResourceStorageIncreased;
		}
	}
}
