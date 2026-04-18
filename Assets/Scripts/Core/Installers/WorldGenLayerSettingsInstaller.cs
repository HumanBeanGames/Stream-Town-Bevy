using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WorldGenLayerSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WorldGenLayerSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WorldGenLayerSettings _worldGenLayerSettings;

		public WorldGenLayerSettings WorldGenLayerSettings => _worldGenLayerSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
