using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for ObjectSelectionSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class ObjectSelectionSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private ObjectSelectionSettingsScriptable _objectSelectionSettingsScriptable;

		public ObjectSelectionSettingsScriptable ObjectSelectionSettingsScriptable => _objectSelectionSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
