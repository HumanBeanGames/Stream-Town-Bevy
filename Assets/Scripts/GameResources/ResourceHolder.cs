using System;
using UnityEngine;
using Utils;

namespace GameResources
{
	/// <summary>
	/// Holds a specified resource and amount, for use on resource objects such as trees and ore.
	/// </summary>
	public class ResourceHolder : MonoBehaviour
	{
        /// <summary>
        /// The resource type.
        /// </summary>
		[SerializeField]
		private Utils.Resource _resourceType;

        /// <summary>
        /// The amount of the resource.
        /// </summary>
		[SerializeField]
		private int _amount;

        /// <summary>
        /// Whether the resource is unlimited.
        /// </summary>
		[SerializeField]
		private bool _unlimited = false;

        /// <summary>
        /// Whether to set the amount by distance.
        /// </summary>
		[SerializeField]
		private bool _setByDistance = false;

        /// <summary>
        /// The curve for distance-based amount calculation.
        /// </summary>
		[SerializeField]
		private AnimationCurve _curve;

        /// <summary>
        /// The minimum amount.
        /// </summary>
		[SerializeField]
		private int _minAmount;

        /// <summary>
        /// The maximum amount.
        /// </summary>
		[SerializeField]
		private int _maxAmount;

        /// <summary>
        /// The owner object.
        /// </summary>
		private object _ownerObject;

        /// <summary>
        /// The maximum distance for distance-based calculation.
        /// </summary>
		private int _maxDistance = 150;

        /// <summary>
        /// Gets the owner object.
        /// </summary>
		public object OwnerObject => _ownerObject;

        /// <summary>
        /// Gets the resource type.
        /// </summary>
		public Resource ResourceType => _resourceType;

        /// <summary>
        /// Gets the amount of the resource.
        /// </summary>
		public int Amount => _amount;

        /// <summary>
        /// Event fired when the amount changes.
        /// </summary>
		public event Action<ResourceHolder> OnAmountChange;



        /// <summary>
        /// Removes resources from this source.
        /// </summary>
        /// <param name="value">The amount to remove.</param>
        /// <returns>The amount actually taken.</returns>
		public int TakeResource(int value)
		{
			if (_unlimited)
				return value;

			int taken = 0;

			if (_amount - value < 0)
				taken = _amount;
			else
				taken = value;

			_amount -= taken;

			if (_amount < 0)
				_amount = 0;

			OnAmountChanged();
			OnAmountChange?.Invoke(this);
			return taken;
		}

        /// <summary>
        /// Sets the remaining amount of resources.
        /// </summary>
        /// <param name="value">The amount to set.</param>
		public void SetResources(int value)
		{
			_amount = value;
		}

        /// <summary>
        /// Called when the resource amount has changed.
        /// </summary>
		private void OnAmountChanged()
		{
			// Disable the game object if the amount reaches zero
			if (_amount <= 0)
			{
				gameObject.SetActive(false);
			}
		}

		// Unity Functions.
        /// <summary>
        /// Sets the amount based on distance if enabled.
        /// </summary>
		private void OnEnable()
		{
			// Check if distance-based calculation is enabled
			if (_setByDistance)
			{
				// Evaluate the curve based on the object's distance from the origin
				var eval = _curve.Evaluate(transform.position.magnitude / (float)_maxDistance);
				// Remap the evaluated value to the range of minimum and maximum amounts
				var remap = MathExtended.RemapValue(eval, 0, 1, _minAmount, _maxAmount);
				// Set the amount based on the remapped value
				_amount = (int)remap;
			}
		}
	}
}
