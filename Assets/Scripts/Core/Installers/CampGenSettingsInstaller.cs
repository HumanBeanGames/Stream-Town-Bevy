using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for CampGenSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class CampGenSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private CampGenSettings _campGenSettings;

		public CampGenSettings CampGenSettings => _campGenSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
