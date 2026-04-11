using Reflex.Core;
using Reflex.Injectors;
using Managers;
using UnityEngine;

public class ObjectPoolingManagerInstaller : MonoBehaviour, IInstaller
{
    private static GameObject _poolingManagerRoot;

    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        // Instantiate the ObjectPoolingManager GameObject to ensure it exists in the scene
        var poolingManagerClone = Instantiate(gameObject);
        _poolingManagerRoot = poolingManagerClone;
        poolingManagerClone.name = name;

        // Bind the ObjectPoolingManager component from the instantiated instance
        var poolingManager = poolingManagerClone.GetComponent<ObjectPoolingManager>();
        containerBuilder.AddSingleton(poolingManager);

        // After the Project container is built, apply DontDestroyOnLoad and inject
        containerBuilder.OnContainerBuilt += container =>
        {
            DontDestroyOnLoad(poolingManagerClone);
            GameObjectInjector.InjectRecursive(poolingManagerClone, container);
        };
    }
}
