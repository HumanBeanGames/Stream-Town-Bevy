using Character;

using ScriptablesProcessorInfrastructure;
using System.Collections.Generic;
using Target;
using UserInterface;
using UnityEngine;

namespace Processors
{
	/// <summary>
	/// Runtime data for UtilDisplayProcessor.
	/// Manages active text displays and ping objects.
	/// </summary>
	public class UtilDisplayRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Dictionary mapping targetable objects to their text displays.
		/// </summary>
		private Dictionary<Targetable, UnitTextDisplay> _activeTextDisplays;

		/// <summary>
		/// Dictionary mapping players to their ping game objects.
		/// </summary>
		private Dictionary<Player, GameObject> _pingObjects;

		/// <summary>
		/// Gets the dictionary of active text displays.
		/// </summary>
		public Dictionary<Targetable, UnitTextDisplay> ActiveTextDisplays => _activeTextDisplays;

		/// <summary>
		/// Gets the dictionary of ping objects.
		/// </summary>
		public Dictionary<Player, GameObject> PingObjects => _pingObjects;

		/// <summary>
		/// Initializes the util display runtime data with default values.
		/// </summary>
		public UtilDisplayRuntimeData()
		{
			_activeTextDisplays = new Dictionary<Targetable, UnitTextDisplay>();
			_pingObjects = new Dictionary<Player, GameObject>();
		}
	}
}
