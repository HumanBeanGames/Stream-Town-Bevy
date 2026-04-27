using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for GridSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class GridSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private GridSettings _gridSettings;

		public GridSettings GridSettings => _gridSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_gridSettings);
		}
	}
}
