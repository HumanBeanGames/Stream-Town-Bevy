using Reflex.Core;
using UnityEngine;
using GameEventSystem;

[RequireComponent(typeof(GameEventManager))]
public class GameEventManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<GameEventManager>());
    }
}
