using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for ObjectPoolingSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class ObjectPoolingSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private ObjectPoolingSettings _objectPoolingSettings;

		public ObjectPoolingSettings ObjectPoolingSettings => _objectPoolingSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
