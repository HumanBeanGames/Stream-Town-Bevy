using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WeatherVFX that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WeatherVFXSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WeatherVFX _weatherVFX;

		public WeatherVFX WeatherVFX => _weatherVFX;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
