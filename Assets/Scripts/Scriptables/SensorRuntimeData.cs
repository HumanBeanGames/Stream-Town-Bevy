using System.Collections.Generic;
using UnityEngine;
using Sensors;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime sensor state for the game.
	/// </summary>
	public class SensorRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		private float _updateTimer = 0;
		private List<SensorBase> _sensors = new List<SensorBase>();

		public float UpdateTimer
		{
			get { return _updateTimer; }
			set { _updateTimer = value; }
		}
		public List<SensorBase> Sensors => _sensors;
	}
}
