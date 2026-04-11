using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(WeatherManager))]
public class WeatherManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<WeatherManager>());
    }
}
