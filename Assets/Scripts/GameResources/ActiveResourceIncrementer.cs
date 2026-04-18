using Processors;
using Reflex.Attributes;
using UnityEngine;
using Utils;

namespace GameResources
{
	/// <summary>
	/// An active resource incrementer that can be called by an event.
	/// </summary>
	public class ActiveResourceIncrementer : MonoBehaviour
	{
        /// <summary>
        /// The resource type to increment.
        /// </summary>
		[SerializeField]
		protected Utils.Resource _resource;
        /// <summary>
        /// The amount to increment by.
        /// </summary>
		[SerializeField]
		protected int _amount;

        /// <summary>
        /// Town resource processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] protected TownResourceProcessor _townResourceProcessor;

		/// <summary>
		/// Increments the town resources of the specified type by the amount set.
		/// </summary>
		public void Increment()
		{
			_townResourceProcessor.AddResource(_resource, _amount);
		}

	}
}
