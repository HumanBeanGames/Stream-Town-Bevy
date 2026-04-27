using Reflex.Injectors;
using Reflex.Logging;
using System.Collections.Generic;
using UnityEngine;

namespace Reflex.Core
{
    public sealed class ProjectScope : MonoBehaviour
    {
        public void InstallBindings(ContainerBuilder containerBuilder)
        {
            var projectUiInjectables = new List<MonoBehaviour>();
            // Use a stack for depth-first search with pointer switching
            var stack = new Stack<GameObject>();
            stack.Push(gameObject);

            while (stack.Count > 0)
            {
                var current = stack.Pop();

                // Get IInstaller components on current GameObject
                var installers = current.GetComponents<IInstaller>();

                for (var i = 0; i < installers.Length; i++)
                {
                    // If this is an InstantiationBarrier, instantiate and switch pointer
                    if (installers[i].GetType().Name == "InstantiationBarrier")
                    {
                        var component = installers[i] as MonoBehaviour;
                        var instance = Instantiate(component.gameObject);
                        DontDestroyOnLoad(instance);
                        current = instance; // Switch pointer to instance
                        break; // Skip other installers on this GameObject
                    }

                    installers[i].InstallBindings(containerBuilder);
                }

                var components = current.GetComponents<MonoBehaviour>();
                for (var i = 0; i < components.Length; i++)
                {
                    var component = components[i];

                    if (component is IProjectUIInjectable && !projectUiInjectables.Contains(component))
                    {
                        projectUiInjectables.Add(component);
                    }
                }

                // Add children to stack for depth-first search
                for (int i = current.transform.childCount - 1; i >= 0; i--)
                {
                    stack.Push(current.transform.GetChild(i).gameObject);
                }
            }

            containerBuilder.OnContainerBuilt += container =>
            {
                for (var i = 0; i < projectUiInjectables.Count; i++)
                {
                    AttributeInjector.Inject(projectUiInjectables[i], container);

                    if (projectUiInjectables[i] is IProjectUIInjectable projectUiInjectable)
                    {
                        projectUiInjectable.OnProjectUIInjected();
                    }
                }
            };

            ReflexLogger.Log("ProjectScope Bindings Installed", LogLevel.Info, gameObject);
        }
    }
}
