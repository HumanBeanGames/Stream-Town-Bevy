using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for SeasonMaterialsSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class SeasonMaterialsSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private SeasonMaterialsSettings _seasonMaterials;

		public SeasonMaterialsSettings SeasonMaterials => _seasonMaterials;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
