using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for PoolingConfigSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class PoolingConfigSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private PoolingConfigSettings _poolingConfig;

		public PoolingConfigSettings PoolingConfig => _poolingConfig;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
