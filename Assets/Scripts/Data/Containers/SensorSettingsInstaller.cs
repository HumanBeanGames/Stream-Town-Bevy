using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for SensorSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class SensorSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private SensorSettingsScriptable _sensorSettingsScriptable;

		public SensorSettingsScriptable SensorSettingsScriptable => _sensorSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
