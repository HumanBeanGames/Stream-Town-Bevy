using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for SaveSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class SaveSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private SaveSettings _saveSettings;

		public SaveSettings SaveSettings => _saveSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_saveSettings);
		}
	}
}
