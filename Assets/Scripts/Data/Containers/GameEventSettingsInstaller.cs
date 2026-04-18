using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for GameEventSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class GameEventSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private GameEventSettingsScriptable _gameEventSettingsScriptable;

		public GameEventSettingsScriptable GameEventSettingsScriptable => _gameEventSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
