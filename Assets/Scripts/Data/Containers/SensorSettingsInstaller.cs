using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for SensorSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class SensorSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private SensorSettings _sensorSettings;

		public SensorSettings SensorSettings => _sensorSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
