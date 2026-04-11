using Reflex.Core;
using UnityEngine;
using Managers;

[RequireComponent(typeof(SeasonManager))]
public class SeasonManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<SeasonManager>());
    }
}
