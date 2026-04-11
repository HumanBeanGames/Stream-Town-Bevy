using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(ObjectSelectionManager))]
public class ObjectSelectionManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<ObjectSelectionManager>());
    }
}
