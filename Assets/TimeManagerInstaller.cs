using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(TimeManager))]
public class TimeManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<TimeManager>());
    }
}
