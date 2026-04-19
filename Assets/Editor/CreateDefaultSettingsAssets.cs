using UnityEngine;
using UnityEditor;
using System.IO;
using System.Linq;
using ScriptablesProcessorInfrastructure;
using System.Collections.Generic;
using System.Reflection;
using Data.Containers;

namespace StreamTownEditor
{
    public class CreateDefaultSettingsAssets
    {
        [MenuItem("Tools/Create Default Settings Assets")]
        public static void CreateAllDefaultSettings()
        {
            string folderPath = "Assets/DefaultSettings";
            
            // Ensure folder exists
            if (!Directory.Exists(folderPath))
            {
                Directory.CreateDirectory(folderPath);
            }

            CreateSettingsAssets(folderPath);
            UpdateDefaultSettingsPrefab(folderPath);
        }

        [MenuItem("Tools/Update Default Settings Prefab")]
        public static void UpdateDefaultSettingsPrefabOnly()
        {
            string folderPath = "Assets/DefaultSettings";
            UpdateDefaultSettingsPrefab(folderPath);
        }

        private static void CreateSettingsAssets(string folderPath)
        {
            // List of all IDataScriptable types to create assets for
            System.Type[] settingsTypes = new System.Type[]
            {
                typeof(AllSeasonsSettings),
                typeof(BuildingConfigSettings),
                typeof(GameEventConfigSettings),
                typeof(AllRoleDataSettings),
                typeof(ResourceDataSettings),
                typeof(SeasonDataContainer),
                typeof(AllBuildingDataSettings),
                typeof(TimeSettings),
                typeof(WeatherVFXSettings),
                typeof(DayAndNightSettings),
                typeof(GameEventSettings),
                typeof(GridSettings),
                typeof(MainMenuSettings),
                typeof(LoadingSettings),
                typeof(ObjectPoolingSettings),
                typeof(ObjectSelectionSettings),
                typeof(ResourceGenSettings),
                typeof(PlayerInputSettings),
                typeof(SaveSettings),
                typeof(SeasonSettings),
                typeof(GameSettings),
                typeof(SensorSettings),
                typeof(TargetSettings),
                typeof(TerrainGenSettings),
                typeof(TownGoalSettings),
                typeof(TradeSettings),
                typeof(UISettings),
                typeof(WaterFoliageGenSettings),
                typeof(WeatherSettings),
                typeof(WaterResourceGenSettings),
                typeof(TechTreeSettings),
                typeof(FoliageGenSettings),
                typeof(WorldGenBehaviorSettings),
                typeof(DebugSettings),
                typeof(WorldGenLayerSettings),
                typeof(WorldGenDebugSettings),
                typeof(WorldGenScaleSettings),
                typeof(CampGenSettings),
                typeof(BuildingSettings),
                typeof(ScriptablesProcessorInfrastructure.AudioSettings)
            };

            foreach (System.Type type in settingsTypes)
            {
                if (type == null)
                {
                    Debug.LogWarning($"Type is null, skipping.");
                    continue;
                }

                string assetName = $"D_{type.Name}";
                string assetPath = Path.Combine(folderPath, $"{assetName}.asset");

                // Check if asset already exists
                if (AssetDatabase.LoadAssetAtPath(assetPath, type) != null)
                {
                    Debug.Log($"Asset already exists: {assetPath}");
                    continue;
                }

                // Create the ScriptableObject instance
                ScriptableObject asset = ScriptableObject.CreateInstance(type);
                
                // Save the asset
                AssetDatabase.CreateAsset(asset, assetPath);
                AssetDatabase.SaveAssets();
                AssetDatabase.Refresh();
                
                Debug.Log($"Created asset: {assetPath}");
            }

            Debug.Log("Finished creating default settings assets.");
        }

        private static void UpdateDefaultSettingsPrefab(string folderPath)
        {
            string prefabPath = "Assets/DefaultSettings/DefaultSettings.prefab";
            GameObject prefabRoot;
            
            // Check if prefab exists
            if (AssetDatabase.LoadAssetAtPath<GameObject>(prefabPath) == null)
            {
                // Create new prefab
                prefabRoot = new GameObject("DefaultSettings");
                PrefabUtility.SaveAsPrefabAsset(prefabRoot, prefabPath);
                Object.DestroyImmediate(prefabRoot);
            }
            
            // Load prefab contents for editing
            prefabRoot = PrefabUtility.LoadPrefabContents(prefabPath);
            
            if (prefabRoot == null)
            {
                Debug.LogError("Failed to load prefab at: " + prefabPath);
                return;
            }

            // Mapping of settings type to installer type name
            Dictionary<System.Type, string> installerTypeMap = new Dictionary<System.Type, string>
            {
                { typeof(AllBuildingDataSettings), "AllBuildingDataSettingsInstaller" },
                { typeof(AllSeasonsSettings), "AllSeasonsSettingsInstaller" },
                { typeof(AllRoleDataSettings), "AllRoleDataSettingsInstaller" },
                { typeof(BuildingConfigSettings), "BuildingConfigSettingsInstaller" },
                { typeof(GameEventConfigSettings), "GameEventConfigSettingsInstaller" },
                { typeof(ResourceDataSettings), "ResourceDataSettingsInstaller" },
                { typeof(SeasonDataContainer), "SeasonDataContainerInstaller" },
                { typeof(TimeSettings), "TimeDataSettingsInstaller" },
                { typeof(WeatherVFXSettings), "WeatherVFXSettingsInstaller" },
                { typeof(DayAndNightSettings), "DayAndNightSettingsInstaller" },
                { typeof(GameEventSettings), "GameEventSettingsInstaller" },
                { typeof(GridSettings), "GridSettingsInstaller" },
                { typeof(MainMenuSettings), "MainMenuSettingsInstaller" },
                { typeof(LoadingSettings), "LoadingSettingsInstaller" },
                { typeof(ObjectPoolingSettings), "ObjectPoolingSettingsInstaller" },
                { typeof(ObjectSelectionSettings), "ObjectSelectionSettingsInstaller" },
                { typeof(ResourceGenSettings), "ResourceGenSettingsInstaller" },
                { typeof(PlayerInputSettings), "PlayerInputSettingsInstaller" },
                { typeof(SaveSettings), "SaveSettingsInstaller" },
                { typeof(GameSettings), "GameSettingsInstaller" },
                { typeof(SensorSettings), "SensorSettingsInstaller" },
                { typeof(TargetSettings), "TargetSettingsInstaller" },
                { typeof(TerrainGenSettings), "TerrainGenSettingsInstaller" },
                { typeof(TownGoalSettings), "TownGoalSettingsInstaller" },
                { typeof(TradeSettings), "TradeSettingsInstaller" },
                { typeof(UISettings), "UISettingsInstaller" },
                { typeof(WaterFoliageGenSettings), "WaterFoliageGenSettingsInstaller" },
                { typeof(WeatherSettings), "WeatherSettingsInstaller" },
                { typeof(WaterResourceGenSettings), "WaterResourceGenSettingsInstaller" },
                { typeof(TechTreeSettings), "TechTreeSettingsInstaller" },
                { typeof(FoliageGenSettings), "FoliageGenSettingsInstaller" },
                { typeof(WorldGenBehaviorSettings), "WorldGenBehaviorSettingsInstaller" },
                { typeof(DebugSettings), "DebugSettingsInstaller" },
                { typeof(WorldGenLayerSettings), "WorldGenLayerSettingsInstaller" },
                { typeof(WorldGenDebugSettings), "WorldGenDebugSettingsInstaller" },
                { typeof(WorldGenScaleSettings), "WorldGenScaleSettingsInstaller" },
                { typeof(CampGenSettings), "CampGenSettingsInstaller" },
                { typeof(BuildingSettings), "BuildingSettingsInstaller" }
            };

            // Add container installers (these don't wrap settings, they just register containers in DI)
            string[] containerInstallers = new string[]
            {
                "BuildingDataContainerInstaller",
                "RoleDataContainerInstaller"
            };

            foreach (var kvp in installerTypeMap)
            {
                System.Type settingsType = kvp.Key;
                string installerTypeName = kvp.Value;

                string assetName = $"D_{settingsType.Name}";
                string assetPath = Path.Combine(folderPath, $"{assetName}.asset");
                ScriptableObject asset = AssetDatabase.LoadAssetAtPath<ScriptableObject>(assetPath);
                
                if (asset == null)
                {
                    Debug.LogWarning($"Asset not found: {assetPath}, skipping GameObject creation.");
                    continue;
                }

                string gameObjectName = assetName;
                Transform existingChild = prefabRoot.transform.Find(gameObjectName);
                
                GameObject child;
                if (existingChild != null)
                {
                    child = existingChild.gameObject;
                }
                else
                {
                    child = new GameObject(gameObjectName);
                    child.transform.SetParent(prefabRoot.transform, false);
                }

                // Find or add installer component
                Component installerComponent = child.GetComponent(installerTypeName);
                if (installerComponent == null)
                {
                    // Try to find the type by name
                    System.Type installerType = System.AppDomain.CurrentDomain.GetAssemblies()
                        .SelectMany(a => a.GetTypes())
                        .FirstOrDefault(t => t.Name == installerTypeName && typeof(MonoBehaviour).IsAssignableFrom(t));
                    
                    if (installerType != null)
                    {
                        installerComponent = child.AddComponent(installerType);
                    }
                    else
                    {
                        Debug.LogError($"Could not find installer type: {installerTypeName}");
                        continue;
                    }
                }

                // Set the serialized field to reference the asset
                FieldInfo field = installerComponent.GetType().GetFields(
                    BindingFlags.NonPublic | BindingFlags.Instance)
                    .FirstOrDefault(f => f.FieldType == settingsType);
                
                if (field != null)
                {
                    field.SetValue(installerComponent, asset);
                }
                else
                {
                    // Try public properties
                    PropertyInfo property = installerComponent.GetType().GetProperty(
                        settingsType.Name, 
                        BindingFlags.Public | BindingFlags.Instance);
                    
                    if (property != null && property.CanWrite)
                    {
                        property.SetValue(installerComponent, asset);
                    }
                    else
                    {
                        Debug.LogWarning($"Could not find field or property for {settingsType.Name} in {installerTypeName}");
                    }
                }
            }

            // Add container installers (these don't wrap settings, they just register containers in DI)
            foreach (string containerInstallerName in containerInstallers)
            {
                string gameObjectName = containerInstallerName;
                Transform existingChild = prefabRoot.transform.Find(gameObjectName);
                
                GameObject child;
                if (existingChild != null)
                {
                    child = existingChild.gameObject;
                }
                else
                {
                    child = new GameObject(gameObjectName);
                    child.transform.SetParent(prefabRoot.transform, false);
                }

                // Find or add installer component
                Component installerComponent = child.GetComponent(containerInstallerName);
                if (installerComponent == null)
                {
                    // Try to find the type by name
                    System.Type installerType = System.AppDomain.CurrentDomain.GetAssemblies()
                        .SelectMany(a => a.GetTypes())
                        .FirstOrDefault(t => t.Name == containerInstallerName && typeof(MonoBehaviour).IsAssignableFrom(t));
                    
                    if (installerType != null)
                    {
                        installerComponent = child.AddComponent(installerType);
                    }
                    else
                    {
                        Debug.LogError($"Could not find installer type: {containerInstallerName}");
                        continue;
                    }
                }
            }

            // Save and unload the prefab
            PrefabUtility.SaveAsPrefabAsset(prefabRoot, prefabPath);
            PrefabUtility.UnloadPrefabContents(prefabRoot);
            AssetDatabase.SaveAssets();
            AssetDatabase.Refresh();
            
            Debug.Log("Updated DefaultSettings prefab with installer GameObjects.");
        }
    }
}
