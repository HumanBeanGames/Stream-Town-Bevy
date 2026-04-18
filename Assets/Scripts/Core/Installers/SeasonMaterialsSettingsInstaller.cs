using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for SeasonMaterials that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class SeasonMaterialsSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private SeasonMaterials _seasonMaterials;

		public SeasonMaterials SeasonMaterials => _seasonMaterials;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
