using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WorldGenScaleSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WorldGenScaleSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WorldGenScaleSettings _worldGenScaleSettings;

		public WorldGenScaleSettings WorldGenScaleSettings => _worldGenScaleSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_worldGenScaleSettings);
		}
	}
}
