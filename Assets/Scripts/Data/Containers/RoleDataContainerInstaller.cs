using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour installer that registers RoleDataContainer in the DI container.
	/// Takes AllRoleDataSettings from the installer and creates the container.
	/// </summary>
	public class RoleDataContainerInstaller : MonoBehaviour, IInstaller
	{
		[Inject]
		private AllRoleDataSettings _allRoleData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton<RoleDataContainer>(container => new RoleDataContainer(_allRoleData));
		}
	}
}
