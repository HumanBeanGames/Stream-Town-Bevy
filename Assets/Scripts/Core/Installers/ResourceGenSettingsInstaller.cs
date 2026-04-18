using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for ResourceGenSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class ResourceGenSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private ResourceGenSettings _resourceGenSettings;

		public ResourceGenSettings ResourceGenSettings => _resourceGenSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
