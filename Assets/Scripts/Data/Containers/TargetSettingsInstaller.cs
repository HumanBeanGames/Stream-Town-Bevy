using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TargetSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TargetSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TargetSettingsScriptable _targetSettingsScriptable;

		public TargetSettingsScriptable TargetSettingsScriptable => _targetSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
