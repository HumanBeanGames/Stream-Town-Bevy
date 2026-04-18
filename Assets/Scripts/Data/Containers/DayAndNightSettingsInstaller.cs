using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for DayAndNightSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class DayAndNightSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private DayAndNightSettingsScriptable _dayAndNightSettingsScriptable;

		public DayAndNightSettingsScriptable DayAndNightSettingsScriptable => _dayAndNightSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
