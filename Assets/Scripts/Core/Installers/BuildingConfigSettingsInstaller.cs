using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for BuildingConfigSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class BuildingConfigSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private BuildingConfigSettings _buildingConfig;

		public BuildingConfigSettings BuildingConfig => _buildingConfig;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
