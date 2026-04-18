using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for DebugSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class DebugSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private DebugSettingsScriptable _debugSettingsScriptable;

		public DebugSettingsScriptable DebugSettingsScriptable => _debugSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
