using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TimeSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TimeDataSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TimeSettings _timeSettings;

		public TimeSettings TimeSettings => _timeSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
