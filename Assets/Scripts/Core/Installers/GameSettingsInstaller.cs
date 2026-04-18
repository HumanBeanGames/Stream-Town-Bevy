using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for GameSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class GameSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private GameSettings _gameSettings;

		public GameSettings GameSettings => _gameSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
