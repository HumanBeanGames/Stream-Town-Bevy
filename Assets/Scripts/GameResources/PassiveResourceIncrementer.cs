using Processors;
using Reflex.Attributes;
using System;
using UnityEngine;
using Utils;

namespace GameResources
{
	/// <summary>
	/// Passively increments a town resource over time.
	/// </summary>
	public class PassiveResourceIncrementer : MonoBehaviour
	{
        /// <summary>
        /// The resource type to increment.
        /// </summary>
		[SerializeField]
		protected Utils.Resource _resource;
        /// <summary>
        /// The amount to increment per second.
        /// </summary>
		[SerializeField]
		protected float _amountPerSecond;
        /// <summary>
        /// The additional amount per level.
        /// </summary>
		[SerializeField]
		protected float _amountPerLevel;

        /// <summary>
        /// The total amount per second.
        /// </summary>
		protected float _totalAmount;
        /// <summary>
        /// Whether the passive increment is enabled.
        /// </summary>
		protected bool _enabled = false;
        /// <summary>
        /// The accumulated amount.
        /// </summary>
		protected float _accumulated = 0;

        /// <summary>
        /// Town resource processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] protected TownResourceProcessor _townResourceProcessor;
        /// <summary>
        /// Event fired when the rate changes.
        /// </summary>
		public event Action<PassiveResourceIncrementer> OnRateChange;

        /// <summary>
        /// Called when a containing level handler has leveled up.
        /// </summary>
		public void OnLevelUp()
		{
			_totalAmount += _amountPerLevel;
			OnRateChange?.Invoke(this);
		}

        /// <summary>
        /// Creates a string used for displaying information.
        /// </summary>
        /// <returns>The information string.</returns>
		public string GetInformation()
		{
			float amountPerHour = _totalAmount * 60 * 60;
			return $"Rate +{StringUtils.GetShortenedNumberAsString((int)amountPerHour)} {_resource}/HR ";
		}

        /// <summary>
        /// Enables the passive resource income.
        /// </summary>
		public void Enable()
		{
			_enabled = true;
			_totalAmount = _amountPerSecond;
		}

        /// <summary>
        /// Disables the passive resource income.
        /// </summary>
		public void Disable()
		{
			_enabled = false;
			_totalAmount = _amountPerSecond;
		}

        // Disables the passive increment when the component is disabled.
		private void OnDisable()
		{
			Disable();
		}

        /// <summary>
        /// Updates the accumulated amount and adds resources when threshold is reached.
        /// </summary>
		private void Update()
		{
			if (!_enabled)
				return;

			_accumulated += _totalAmount * Time.deltaTime;

			if (_accumulated > 1)
			{
				int rounded = Mathf.FloorToInt(_accumulated);
				_accumulated -= rounded;

				_townResourceProcessor.AddResource(_resource, rounded);
			}
		}
	}
}
