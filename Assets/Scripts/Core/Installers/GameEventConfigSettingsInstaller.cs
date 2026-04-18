using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for GameEventConfig that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class GameEventConfigSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private GameEventConfig _gameEventConfig;

		public GameEventConfig GameEventConfig => _gameEventConfig;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
