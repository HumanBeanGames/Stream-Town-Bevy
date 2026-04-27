using System.Collections.Generic;

using ScriptablesProcessorInfrastructure;
using Target;
using Utils;

namespace Processors
{
	/// <summary>
	/// Runtime data for TargetProcessor.
	/// </summary>
	public class TargetRuntimeData : IRuntimeDataScriptable
	{
		private Dictionary<TargetMask, List<Targetable>> _targetDictionary;

		public Dictionary<TargetMask, List<Targetable>> TargetDictionary => _targetDictionary;

		/// <summary>
		/// Initializes the target runtime data with default values.
		/// </summary>
		public TargetRuntimeData()
		{
			_targetDictionary = new Dictionary<TargetMask, List<Targetable>>();
		}
	}
}
