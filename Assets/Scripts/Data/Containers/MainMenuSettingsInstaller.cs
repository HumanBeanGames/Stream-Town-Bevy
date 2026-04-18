using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for MainMenuSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class MainMenuSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private MainMenuSettingsScriptable _mainMenuSettingsScriptable;

		public MainMenuSettingsScriptable MainMenuSettingsScriptable => _mainMenuSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
