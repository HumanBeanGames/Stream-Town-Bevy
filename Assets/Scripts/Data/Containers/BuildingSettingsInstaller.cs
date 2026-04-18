using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for BuildingSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class BuildingSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private BuildingSettings _buildingSettingsScriptable;

		public BuildingSettings BuildingSettings => _buildingSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
