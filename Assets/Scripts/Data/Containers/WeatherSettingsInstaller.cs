using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WeatherSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WeatherSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WeatherSettings _weatherSettings;

		public WeatherSettings WeatherSettings => _weatherSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_weatherSettings);
		}
	}
}
