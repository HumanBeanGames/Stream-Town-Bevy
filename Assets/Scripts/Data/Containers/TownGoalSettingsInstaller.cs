using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TownGoalSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TownGoalSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TownGoalSettings _townGoalSettings;

		public TownGoalSettings TownGoalSettings => _townGoalSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_townGoalSettings);
		}
	}
}
