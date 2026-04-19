using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for UISettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class UISettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private UISettings _uiSettings;

		public UISettings UISettings => _uiSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
