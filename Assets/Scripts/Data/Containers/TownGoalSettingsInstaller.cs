using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TownGoalSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TownGoalSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TownGoalSettingsScriptable _townGoalSettingsScriptable;

		public TownGoalSettingsScriptable TownGoalSettingsScriptable => _townGoalSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
