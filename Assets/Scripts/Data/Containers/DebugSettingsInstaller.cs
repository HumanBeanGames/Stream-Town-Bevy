using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for DebugSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class DebugSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private DebugSettings _debugSettings;

		public DebugSettings DebugSettings => _debugSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_debugSettings);
		}
	}
}
