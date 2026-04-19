using System.Collections.Generic;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Processors;

namespace Sensors
{
	/// <summary>
	/// Manages all sensors that a unit has.
	/// </summary>
	public class SensorProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		[Inject] private SensorSettings _sensorSettings;
		/// <summary>
		/// Runtime sensor data ScriptableObject.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private SensorRuntimeData _sensorRuntimeData;

		/// <summary>
		/// Adds a sensor to the unit.
		/// </summary>
		/// <param name="sensor"></param>
		public void AddSensor(SensorBase sensor)
		{
			if (_sensorRuntimeData.Sensors.Contains(sensor))
				return;

			_sensorRuntimeData.Sensors.Add(sensor);
		}	

		/// <summary>
		/// Removes a sensor from a unit.
		/// </summary>
		/// <param name="sensor"></param>
		public void RemoveSensor(SensorBase sensor)
		{
			if (!_sensorRuntimeData.Sensors.Contains(sensor))
				return;

			_sensorRuntimeData.Sensors.Remove(sensor);
		}

		public void Initialize()
		{
			_sensorRuntimeData.UpdateTimer = Random.Range(0, _sensorSettings.UpdateRate);
		}

		/// <summary>
		/// Processes sensor logic every frame.
		/// Called every frame by the Coordinator.
		/// </summary>
		public void Process()
		{
			_sensorRuntimeData.UpdateTimer += Time.deltaTime;

			if (_sensorRuntimeData.UpdateTimer >= _sensorSettings.UpdateRate)
			{
				_sensorRuntimeData.UpdateTimer -= _sensorSettings.UpdateRate;

				for (int i = 0; i < _sensorRuntimeData.Sensors.Count; i++)
				{
					if (_sensorRuntimeData.Sensors[i].gameObject.activeInHierarchy)
						_sensorRuntimeData.Sensors[i].STUpdate();
				}
			}
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register SensorRuntimeData ScriptableObject
			SensorRuntimeData sensorRuntimeData = ScriptableObject.CreateInstance<SensorRuntimeData>();
			containerBuilder.AddSingleton(sensorRuntimeData);
		}
	}
}
