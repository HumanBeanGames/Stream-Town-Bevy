using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for AllBuildingDataSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class AllBuildingDataSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private AllBuildingDataSettings _allBuildingData;

		public AllBuildingDataSettings AllBuildingData => _allBuildingData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_allBuildingData);
		}
	}
}
