using UnityEngine;
using System;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for CreditsProcessor.
	/// Manages credits screen state.
	/// </summary>
	public class CreditsRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Initializes the credits runtime state.
		/// </summary>
		public void Initialize()
		{
			// CreditsProcessor doesn't require runtime state initialization
		}
	}
}
