using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for AllRoleDataSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class AllRoleDataSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private AllRoleDataSettings _allRoleData;

		public AllRoleDataSettings AllRoleData => _allRoleData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
