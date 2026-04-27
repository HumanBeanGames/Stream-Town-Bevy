using System;
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
		/// Runtime sensor data.
		/// Assigned in InjectRuntimeData.
		/// </summary>
		private SensorRuntimeData _sensorRuntimeData;

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
			if (_sensorRuntimeData == null)
				throw new InvalidOperationException("SensorProcessor: SensorRuntimeData has not been installed.");

			_sensorRuntimeData.UpdateTimer = (float)(new System.Random().NextDouble() * _sensorSettings.UpdateRate);
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
				_sensorRuntimeData.UpdateTimer = 0f;
				UpdateSensors();
			}
		}

		private void UpdateSensors()
		{
			for (int i = 0; i < _sensorRuntimeData.Sensors.Count; i++)
			{
				if (_sensorRuntimeData.Sensors[i].gameObject.activeInHierarchy)
					_sensorRuntimeData.Sensors[i].STUpdate();
			}
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// SensorProcessor does not have scene-specific settings to refresh
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_sensorRuntimeData != null)
				throw new InvalidOperationException("SensorProcessor: SensorRuntimeData has already been installed.");

			_sensorRuntimeData = new SensorRuntimeData();
			containerBuilder.AddSingleton(_sensorRuntimeData);
		}
	}
}
