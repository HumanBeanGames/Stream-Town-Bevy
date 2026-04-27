using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for ObjectSelectionSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class ObjectSelectionSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private ObjectSelectionSettings _objectSelectionSettings;

		public ObjectSelectionSettings ObjectSelectionSettings => _objectSelectionSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_objectSelectionSettings);
		}
	}
}
