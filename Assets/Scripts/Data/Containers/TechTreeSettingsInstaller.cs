using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TechTreeSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TechTreeSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TechTreeSettings _techTreeSettings;

		public TechTreeSettings TechTreeSettings => _techTreeSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
