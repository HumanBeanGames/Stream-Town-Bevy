using Reflex.Core;
using UnityEngine;
using Environment;

[RequireComponent(typeof(DayAndNightManager))]
public class DayAndNightManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<DayAndNightManager>());
    }
}
