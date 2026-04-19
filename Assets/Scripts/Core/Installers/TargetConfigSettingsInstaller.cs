using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TargetConfigSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TargetConfigSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TargetConfigSettings _targetConfig;

		public TargetConfigSettings TargetConfig => _targetConfig;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
