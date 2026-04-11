using Reflex.Core;
using Reflex.Injectors;
using Settings;
using System.Collections.Generic;
using UnityEngine;

//This class is used to install the bindings for the UI. It should ONLY be used inside the ProjectScope prefab.
public class PersistentUIInstaller : MonoBehaviour, IInstaller
{
    private static GameObject _persistentRoot;

    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        // Instantiate a single PersistentUI root (this GameObject) to avoid fragmented persistence
        var rootClone = Instantiate(gameObject);
        _persistentRoot = rootClone;
        // Optional: keep the clone name, or customize if desired
        rootClone.name = name;

        // Bind SettingsManager with lazy resolution - it will be found when first requested
        containerBuilder.AddSingleton((container) =>
        {
            if (_persistentRoot != null)
            {
                var settingsManager = _persistentRoot.GetComponentInChildren<SettingsManager>(true);
                if (settingsManager != null)
                {
                    Debug.Log($"PersistentUIInstaller: Resolved SettingsManager from {settingsManager.gameObject.name}", this);
                    return settingsManager;
                }
            }

            // Fallback to FindAnyObjectByType if persistent root isn't available yet
            var fallback = FindAnyObjectByType<SettingsManager>();
            if (fallback != null)
            {
                Debug.Log($"PersistentUIInstaller: Resolved SettingsManager via FindAnyObjectByType from {fallback.gameObject.name}", this);
                return fallback;
            }

            Debug.LogError("PersistentUIInstaller: Could not resolve SettingsManager!", this);
            return null;
        });

        // After the Project container is built, inject all MonoBehaviours under the cloned roots
        containerBuilder.OnContainerBuilt += container =>
        {
            // Make the entire PersistentUI root survive scene loads
            DontDestroyOnLoad(rootClone);

            // Inject fields/properties/[Inject] methods on ALL MonoBehaviours (includes inactive)
            GameObjectInjector.InjectRecursive(rootClone, container);

            // Then run any UI accessor initialization logic
            var accessors = rootClone.GetComponentsInChildren<UIGameObjectAccessor>(true);
            for (int i = 0; i < accessors.Length; i++)
            {
                accessors[i].InitializeUI();
            }
        };
    }
}