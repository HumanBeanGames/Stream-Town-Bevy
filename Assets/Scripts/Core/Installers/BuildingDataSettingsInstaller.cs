using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for BuildingData that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class BuildingDataSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private BuildingData _buildingData;

		public BuildingData BuildingData => _buildingData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
