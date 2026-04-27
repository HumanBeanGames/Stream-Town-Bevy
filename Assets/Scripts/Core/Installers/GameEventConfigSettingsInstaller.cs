using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for GameEventConfigSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class GameEventConfigSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private GameEventConfigSettings _gameEventConfig;

		public GameEventConfigSettings GameEventConfig => _gameEventConfig;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_gameEventConfig);
		}
	}
}
