using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TargetSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TargetSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TargetSettings _targetSettings;

		public TargetSettings TargetSettings => _targetSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
