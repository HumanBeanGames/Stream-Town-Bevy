using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for SeasonDataContainer that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class SeasonDataContainerInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private SeasonDataContainer _seasonDataContainer;

		public SeasonDataContainer SeasonDataContainer => _seasonDataContainer;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
