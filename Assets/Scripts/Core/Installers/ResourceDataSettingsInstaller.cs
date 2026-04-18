using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for ResourceData that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class ResourceDataSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private ResourceData _resourceData;

		public ResourceData ResourceData => _resourceData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
