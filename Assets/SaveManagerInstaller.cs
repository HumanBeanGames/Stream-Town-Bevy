using Reflex.Core;
using UnityEngine;
using SavingAndLoading;

[RequireComponent(typeof(SaveManager))]
public class SaveManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<SaveManager>());
    }
}
