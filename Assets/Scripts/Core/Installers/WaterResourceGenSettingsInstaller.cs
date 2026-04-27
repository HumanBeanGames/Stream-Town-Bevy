using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WaterResourceGenSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WaterResourceGenSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WaterResourceGenSettings _waterResourceGenSettings;

		public WaterResourceGenSettings WaterResourceGenSettings => _waterResourceGenSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_waterResourceGenSettings);
		}
	}
}
