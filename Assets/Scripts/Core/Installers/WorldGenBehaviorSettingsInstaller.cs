using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WorldGenBehaviorSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WorldGenBehaviorSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WorldGenBehaviorSettings _worldGenBehaviorSettings;

		public WorldGenBehaviorSettings WorldGenBehaviorSettings => _worldGenBehaviorSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
