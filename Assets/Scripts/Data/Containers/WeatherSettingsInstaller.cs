using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WeatherSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WeatherSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WeatherSettingsScriptable _weatherSettingsScriptable;

		public WeatherSettingsScriptable WeatherSettingsScriptable => _weatherSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
