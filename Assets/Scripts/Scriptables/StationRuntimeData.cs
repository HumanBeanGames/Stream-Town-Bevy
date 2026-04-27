using Buildings;

using ScriptablesProcessorInfrastructure;
using System.Collections.Generic;
using Utils;

namespace Processors
{
	/// <summary>
	/// Runtime data for StationProcessor.
	/// </summary>
	public class StationRuntimeData : IRuntimeDataScriptable
	{
		private Dictionary<StationMask, List<Station>> _stationsDictionary;
		private Queue<Station> _stationUpdateQueue;
		private Queue<Station> _clearDisabledQueue;

		public Dictionary<StationMask, List<Station>> StationsDictionary => _stationsDictionary;
		public Queue<Station> StationUpdateQueue => _stationUpdateQueue;
		public Queue<Station> ClearDisabledQueue => _clearDisabledQueue;

		/// <summary>
		/// Initializes the station runtime data with default values.
		/// </summary>
		public StationRuntimeData()
		{
			_stationsDictionary = new Dictionary<StationMask, List<Station>>();
			_stationUpdateQueue = new Queue<Station>();
			_clearDisabledQueue = new Queue<Station>();
		}
	}
}
