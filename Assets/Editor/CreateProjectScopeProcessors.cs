using UnityEngine;
using UnityEditor;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Reflection;

namespace StreamTownEditor
{
    public class CreateProjectScopeProcessors
    {
        [MenuItem("Tools/Update ProjectScope Prefab Processors")]
        public static void UpdateProjectScopePrefab()
        {
            string prefabPath = "Assets/ProjectScope.prefab";
            GameObject prefabRoot;
            
            // Check if prefab exists
            if (AssetDatabase.LoadAssetAtPath<GameObject>(prefabPath) == null)
            {
                Debug.LogError("ProjectScope.prefab not found at: " + prefabPath);
                return;
            }
            
            // Load prefab contents for editing
            prefabRoot = PrefabUtility.LoadPrefabContents(prefabPath);
            
            if (prefabRoot == null)
            {
                Debug.LogError("Failed to load prefab at: " + prefabPath);
                return;
            }

            // Find or create Processors GameObject
            Transform processorsParent = prefabRoot.transform.Find("Processors");
            GameObject processorsGameObject;
            
            if (processorsParent == null)
            {
                processorsGameObject = new GameObject("Processors");
                processorsGameObject.transform.SetParent(prefabRoot.transform, false);
            }
            else
            {
                processorsGameObject = processorsParent.gameObject;
            }

            // Find all types that implement IProcessor and are MonoBehaviour
            System.Type iProcessorType = System.AppDomain.CurrentDomain.GetAssemblies()
                .SelectMany(a => a.GetTypes())
                .FirstOrDefault(t => t.Name == "IProcessor");

            List<System.Type> processorTypes = new List<System.Type>();
            
            if (iProcessorType != null)
            {
                processorTypes = System.AppDomain.CurrentDomain.GetAssemblies()
                    .SelectMany(a => a.GetTypes())
                    .Where(t => typeof(MonoBehaviour).IsAssignableFrom(t) && 
                               iProcessorType.IsAssignableFrom(t) &&
                               !t.IsAbstract &&
                               t.Name.EndsWith("Processor"))
                    .ToList();
            }
            else
            {
                Debug.LogError("Could not find IProcessor interface");
                PrefabUtility.UnloadPrefabContents(prefabRoot);
                return;
            }

            foreach (System.Type processorType in processorTypes)
            {
                if (processorType == null)
                {
                    Debug.LogWarning($"Processor type is null, skipping.");
                    continue;
                }

                string gameObjectName = processorType.Name;
                Transform existingChild = processorsGameObject.transform.Find(gameObjectName);
                
                GameObject child;
                if (existingChild != null)
                {
                    child = existingChild.gameObject;
                }
                else
                {
                    child = new GameObject(gameObjectName);
                    child.transform.SetParent(processorsGameObject.transform, false);
                }

                // Find or add processor component
                Component processorComponent = child.GetComponent(processorType);
                if (processorComponent == null)
                {
                    processorComponent = child.AddComponent(processorType);
                }
            }

            // Save and unload the prefab
            PrefabUtility.SaveAsPrefabAsset(prefabRoot, prefabPath);
            PrefabUtility.UnloadPrefabContents(prefabRoot);
            AssetDatabase.SaveAssets();
            AssetDatabase.Refresh();
            
            Debug.Log("Updated ProjectScope prefab with processor GameObjects.");
        }
    }
}
