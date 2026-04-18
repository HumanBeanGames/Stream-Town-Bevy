using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for UISettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class UISettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private UISettingsScriptable _uiSettingsScriptable;

		public UISettingsScriptable UISettingsScriptable => _uiSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
