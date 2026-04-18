using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for SaveSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class SaveSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private SaveSettingsScriptable _saveSettingsScriptable;

		public SaveSettingsScriptable SaveSettingsScriptable => _saveSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
