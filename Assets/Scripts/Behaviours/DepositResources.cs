using Processors;
using Reflex.Attributes;
using UnityEngine;
using Utils;

namespace Behaviours
{
	/// <summary>
	/// An attachable component used for Depositing Resources by a Unit
	/// </summary>
	public class DepositResources : MonoBehaviour
	{
		[Inject] private TownResourceProcessor _townResourceProcessor;
		/// <summary>
		/// Deposits resources of the determined type to the Town's Resource Processor.
		/// </summary>
		/// <param name="type"></param>
		/// <param name="amount"></param>
		public void Deposit(Utils.Resource type, int amount)
		{
			_townResourceProcessor.AddResource(type, amount);
		}
	}
}
