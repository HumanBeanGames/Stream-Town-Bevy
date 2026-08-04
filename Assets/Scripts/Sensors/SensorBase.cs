using UnityEngine;
using Reflex.Attributes;

namespace Sensors
{
	/// <summary>
	/// Base class for all Sensors.
	/// </summary>
	public class SensorBase : MonoBehaviour
	{
        /// <summary>
        /// The sensor processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private SensorProcessor _processor;

        /// <summary>
        /// Called every frame by the sensor processor.
        /// </summary>
		public virtual void STUpdate()
		{

		}

        /// <summary>
        /// Initializes the sensor.
        /// </summary>
		protected virtual void Init()
		{

		}

		// Unity Functions.
        /// <summary>
        /// Adds this sensor to the processor after injection is complete.
        /// </summary>
		private void Awake()
		{
			// Processor will be injected by Reflex before this runs
			if (_processor != null)
				_processor.AddSensor(this);
			Init();
		}

        /// <summary>
        /// Adds this sensor when enabled.
        /// </summary>
		private void OnEnable()
		{
			if (_processor)
				_processor.AddSensor(this);
		}

        /// <summary>
        /// Removes this sensor when disabled.
        /// </summary>
		protected virtual void OnDisable()
		{
			if (_processor)
				_processor.RemoveSensor(this);
		}
	}
}
