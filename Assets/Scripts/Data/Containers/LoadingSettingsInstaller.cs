using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for LoadingSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class LoadingSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private LoadingSettingsScriptable _loadingSettingsScriptable;

		public LoadingSettingsScriptable LoadingSettingsScriptable => _loadingSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
