using UnityEngine;

namespace Sensors
{
	/// <summary>
	/// Base class for all Sensors.
	/// </summary>
	[RequireComponent(typeof(SensorProcessor))]
	public class SensorBase : MonoBehaviour
	{
        /// <summary>
        /// The sensor processor.
        /// </summary>
		private SensorProcessor _processor;

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
        /// Initializes the sensor processor and adds this sensor.
        /// </summary>
		private void Awake()
		{
			_processor = GetComponent<SensorProcessor>();
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
		private void OnDisable()
		{
			if (_processor)
				_processor.RemoveSensor(this);
		}
	}
}
