using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TerrainGenSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TerrainGenSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TerrainGenSettings _terrainGenSettings;

		public TerrainGenSettings TerrainGenSettings => _terrainGenSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_terrainGenSettings);
		}
	}
}
