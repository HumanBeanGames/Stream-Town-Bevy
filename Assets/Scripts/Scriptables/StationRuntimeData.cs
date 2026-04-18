using Buildings;
using System;
using System.Collections.Generic;
using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for StationProcessor.
	/// </summary>
	public class StationRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		[SerializeField]
		private Dictionary<StationMask, List<Station>> _stationsDictionary = new Dictionary<StationMask, List<Station>>();
		[SerializeField]
		private Queue<Station> _stationUpdateQueue = new Queue<Station>();
		[SerializeField]
		private Queue<Station> _clearDisabledQueue = new Queue<Station>();

		public Dictionary<StationMask, List<Station>> StationsDictionary => _stationsDictionary;
		public Queue<Station> StationUpdateQueue => _stationUpdateQueue;
		public Queue<Station> ClearDisabledQueue => _clearDisabledQueue;

		/// <summary>
		/// Initializes the station runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
