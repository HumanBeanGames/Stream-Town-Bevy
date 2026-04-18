using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for RoleData that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class RoleDataSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private RoleData _roleData;

		public RoleData RoleData => _roleData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
