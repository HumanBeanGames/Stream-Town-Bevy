using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for MainMenuSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class MainMenuSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private MainMenuSettings _mainMenuSettings;

		public MainMenuSettings MainMenuSettings => _mainMenuSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
