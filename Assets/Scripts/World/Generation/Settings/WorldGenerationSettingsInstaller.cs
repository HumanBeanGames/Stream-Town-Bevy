using Reflex.Core;
using World.Generation.Settings;
using UnityEngine;

namespace World.Generation
{
	/// <summary>
	/// Installer for world generation settings. Adds all settings containers to the SceneScope.
	/// </summary>
	public class WorldGenerationSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TerrainGenerationSettings _terrainGenerationSettings;
		[SerializeField]
		private ResourceGenerationSettingsContainer _resourceGenerationSettings;
		[SerializeField]
		private WaterResourceGenerationSettingsContainer _waterResourceGenerationSettings;
		[SerializeField]
		private FoliageGenerationSettingsContainer _foliageGenerationSettings;
		[SerializeField]
		private WaterFoliageGenerationSettingsContainer _waterFoliageGenerationSettings;
		[SerializeField]
		private CampGenerationSettingsContainer _campGenerationSettings;
		[SerializeField]
		private WorldGeneratorBehaviorSettings _behaviorSettings;
		[SerializeField]
		private WorldGeneratorDebugSettings _debugSettings;
		[SerializeField]
		private WorldGeneratorScaleSettings _scaleSettings;
		[SerializeField]
		private WorldGeneratorLayerSettings _layerSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_terrainGenerationSettings);
			containerBuilder.AddSingleton(_resourceGenerationSettings);
			containerBuilder.AddSingleton(_waterResourceGenerationSettings);
			containerBuilder.AddSingleton(_foliageGenerationSettings);
			containerBuilder.AddSingleton(_waterFoliageGenerationSettings);
			containerBuilder.AddSingleton(_campGenerationSettings);
			containerBuilder.AddSingleton(_behaviorSettings);
			containerBuilder.AddSingleton(_debugSettings);
			containerBuilder.AddSingleton(_scaleSettings);
			containerBuilder.AddSingleton(_layerSettings);
		}
	}
}
