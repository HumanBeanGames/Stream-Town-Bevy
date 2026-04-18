using Character;
using System.Collections.Generic;
using Target;
using UnityEngine;
using UserInterface;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for UtilDisplayProcessor.
	/// Manages active text displays and ping objects.
	/// </summary>
	public class UtilDisplayRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Dictionary mapping targetable objects to their text displays.
		/// </summary>
		[SerializeField]
		private Dictionary<Targetable, UnitTextDisplay> _activeTextDisplays = new Dictionary<Targetable, UnitTextDisplay>();

		/// <summary>
		/// Dictionary mapping players to their ping game objects.
		/// </summary>
		[SerializeField]
		private Dictionary<Player, GameObject> _pingObjects = new Dictionary<Player, GameObject>();

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
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
