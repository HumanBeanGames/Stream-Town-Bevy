using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for WaterFoliageGenSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class WaterFoliageGenSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private WaterFoliageGenSettings _waterFoliageGenSettings;

		public WaterFoliageGenSettings WaterFoliageGenSettings => _waterFoliageGenSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
