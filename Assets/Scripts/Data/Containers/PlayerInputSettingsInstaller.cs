using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for PlayerInputSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class PlayerInputSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField] private PlayerInputSettings _playerInputSettings;

		public PlayerInputSettings PlayerInputSettings => _playerInputSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
