using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for BuildingDataSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class BuildingDataSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private BuildingDataSettings _buildingData;

		public BuildingDataSettings BuildingData => _buildingData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
