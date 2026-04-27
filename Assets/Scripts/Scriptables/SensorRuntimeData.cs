using System.Collections.Generic;

using ScriptablesProcessorInfrastructure;
using Sensors;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores sensor state for the game.
	/// </summary>
	public class SensorRuntimeData : IRuntimeDataScriptable
	{
		private float _updateTimer;
		private List<SensorBase> _sensors;

		public float UpdateTimer
		{
			get { return _updateTimer; }
			set { _updateTimer = value; }
		}
		public List<SensorBase> Sensors => _sensors;

		/// <summary>
		/// Initializes the sensor runtime data with default values.
		/// </summary>
		public SensorRuntimeData()
		{
			_updateTimer = 0;
			_sensors = new List<SensorBase>();
		}
	}
}
