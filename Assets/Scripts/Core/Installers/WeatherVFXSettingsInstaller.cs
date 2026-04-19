using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WeatherVFXSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WeatherVFXSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WeatherVFXSettings _weatherVFX;

		public WeatherVFXSettings WeatherVFX => _weatherVFX;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
