using Reflex.Core;
using UnityEngine;
using TechTree;

[RequireComponent(typeof(TechTreeManager))]
public class TechTreeManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton<TechTreeManager>(container => GetComponent<TechTreeManager>());
    }
}
