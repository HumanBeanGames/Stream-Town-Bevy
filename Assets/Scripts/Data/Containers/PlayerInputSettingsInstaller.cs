using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for PlayerInputSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class PlayerInputSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField] private PlayerInputSettingsScriptable _playerInputSettingsScriptable;

		public PlayerInputSettingsScriptable PlayerInputSettingsScriptable => _playerInputSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
