using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for DayAndNightSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class DayAndNightSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private DayAndNightSettings _dayAndNightSettings;

		public DayAndNightSettings DayAndNightSettings => _dayAndNightSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_dayAndNightSettings);
		}
	}
}
