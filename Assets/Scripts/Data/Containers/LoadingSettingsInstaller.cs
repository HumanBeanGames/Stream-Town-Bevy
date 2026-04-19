using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for LoadingSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class LoadingSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private LoadingSettings _loadingSettings;

		public LoadingSettings LoadingSettings => _loadingSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
