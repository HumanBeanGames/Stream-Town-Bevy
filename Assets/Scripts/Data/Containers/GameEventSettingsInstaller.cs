using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for GameEventSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class GameEventSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private GameEventSettings _gameEventSettings;

		public GameEventSettings GameEventSettings => _gameEventSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
