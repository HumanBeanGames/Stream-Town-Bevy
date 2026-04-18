using Processors;
using UnityEngine;
using Utils;
using Reflex.Attributes;

namespace Buildings
{
	/// <summary>
	/// A component that modifies the amount of role slots available for a given role.
	/// </summary>
	public class RoleSlotModifier : MonoBehaviour
	{
		/// <summary>
		/// Which role type this component modifies.
		/// </summary>
		[SerializeField, Tooltip("Which role type this component modifies.")]
		private PlayerRole _role;

		/// <summary>
		/// The base amount this component increases role slots by.
		/// </summary>
		[SerializeField, Tooltip("The base amount this component increases role slots by.")]
		private int _baseAmount;

		/// <summary>
		/// How much this component increases the amount of role slots per increment.
		/// </summary>
		[SerializeField, Tooltip("How much this component increases the amount of role slots per increment.")]
		private int _incrementAmount;

		/// <summary>
		/// Keeps track of the total number of role slots this component has added.
		/// </summary>
		private int _totalAmount = 0;

		// Required Components.
		/// <summary>
		/// Role processor for role slot operations.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private RoleProcessor _roleProcessor;

		/// <summary>
		/// Gets the role processor.
		/// </summary>
		public RoleProcessor RoleProcessor
		{
			get { return _roleProcessor; }
		}

		// Properties.
		/// <summary>
		/// Gets the role type this component modifies.
		/// </summary>
		public PlayerRole Role => _role;

		/// <summary>
		/// Increments the amount of role slots available.
		/// </summary>
		public void Increment()
		{
			RoleProcessor.AddSlots(_role, _incrementAmount);
			_totalAmount += _incrementAmount;
		}

		/// <summary>
		/// Adds base number of role slots.
		/// </summary>
		public void AddBaseSlots()
		{
			RoleProcessor.AddSlots(_role, _baseAmount);
			_totalAmount += _baseAmount;
		}

		/// <summary>
		/// Removes the total number of role slots contributed.
		/// </summary>
		public void RemoveTotalSlots()
		{
			_roleProcessor.RemoveSlots(_role, _totalAmount);
			_totalAmount = 0;
		}

		/// <summary>
		/// Called on object being disabled.
		/// Removes all role slots contributed by this component.
		/// </summary>
		private void OnDisable()
		{
			if (RoleProcessor)
				RemoveTotalSlots();
		}
	}
}
