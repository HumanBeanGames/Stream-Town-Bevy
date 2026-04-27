using UnityEngine;

using ScriptablesProcessorInfrastructure;
using UnityEngine.VFX;

namespace Processors
{
	/// <summary>
	/// Runtime data for WeatherProcessor.
	/// </summary>
	public class WeatherRuntimeData : IRuntimeDataScriptable
	{
		private VisualEffect _currentVFX;
		private float _remainingRunTime;
		private float _particleLerpValue;
		private bool _weatherRunning;
		private SeasonDataSettings _activeSeasonData;

		public VisualEffect CurrentVFX
		{
			get { return _currentVFX; }
			set { _currentVFX = value; }
		}

		public float RemainingRunTime
		{
			get { return _remainingRunTime; }
			set { _remainingRunTime = value; }
		}

		public float ParticleLerpValue
		{
			get { return _particleLerpValue; }
			set { _particleLerpValue = value; }
		}

		public bool WeatherRunning
		{
			get { return _weatherRunning; }
			set { _weatherRunning = value; }
		}

		public SeasonDataSettings ActiveSeasonData
		{
			get { return _activeSeasonData; }
			set { _activeSeasonData = value; }
		}

		/// <summary>
		/// Initializes the weather runtime data with default values.
		/// </summary>
		public WeatherRuntimeData()
		{
			_currentVFX = null;
			_remainingRunTime = 0f;
			_particleLerpValue = 0f;
			_weatherRunning = false;
			_activeSeasonData = null;
		}
	}
}
