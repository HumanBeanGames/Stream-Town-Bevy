using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WorldGenDebugSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WorldGenDebugSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WorldGenDebugSettings _worldGenDebugSettings;

		public WorldGenDebugSettings WorldGenDebugSettings => _worldGenDebugSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_worldGenDebugSettings);
		}
	}
}
