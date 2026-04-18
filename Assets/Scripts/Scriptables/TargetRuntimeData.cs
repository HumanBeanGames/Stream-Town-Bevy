using System.Collections.Generic;
using Target;
using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for TargetProcessor.
	/// </summary>
	public class TargetRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		[SerializeField]
		private Dictionary<TargetMask, List<Targetable>> _targetDictionary = new Dictionary<TargetMask, List<Targetable>>();

		public Dictionary<TargetMask, List<Targetable>> TargetDictionary => _targetDictionary;

		/// <summary>
		/// Initializes the target runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
