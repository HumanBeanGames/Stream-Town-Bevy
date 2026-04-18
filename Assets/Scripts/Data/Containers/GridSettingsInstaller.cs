using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for GridSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class GridSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private GridSettingsScriptable _gridSettingsScriptable;

		public GridSettingsScriptable GridSettingsScriptable => _gridSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
