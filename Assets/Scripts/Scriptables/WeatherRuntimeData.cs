using System.Collections;
using UnityEngine;
using UnityEngine.VFX;
using ScriptablesProcessorInfrastructure;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for WeatherProcessor.
	/// </summary>
	public class WeatherRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		[SerializeField]
		private VisualEffect _currentVFX;
		[SerializeField]
		private float _remainingRunTime;
		[SerializeField]
		private float _particleLerpValue;
		[SerializeField]
		private bool _weatherRunning;
		[SerializeField]
		private SeasonSettings _activeSeasonData;

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

		public SeasonSettings ActiveSeasonData
		{
			get { return _activeSeasonData; }
			set { _activeSeasonData = value; }
		}

		/// <summary>
		/// Initializes the weather runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
