using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for PoolingConfig that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class PoolingConfigSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private PoolingConfig _poolingConfig;

		public PoolingConfig PoolingConfig => _poolingConfig;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
