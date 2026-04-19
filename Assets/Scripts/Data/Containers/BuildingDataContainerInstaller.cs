using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour installer that registers BuildingDataContainer in the DI container.
	/// Takes AllBuildingDataSettings from the installer and creates the container.
	/// </summary>
	public class BuildingDataContainerInstaller : MonoBehaviour, IInstaller
	{
		[Inject]
		private AllBuildingDataSettings _allBuildingData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton<BuildingDataContainer>(container => new BuildingDataContainer(_allBuildingData));
		}
	}
}
