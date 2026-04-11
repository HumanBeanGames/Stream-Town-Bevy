using Reflex.Core;
using UnityEngine;
using PlayerControls;

[RequireComponent(typeof(PlayerInputManager))]
public class PlayerInputManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<PlayerInputManager>());
    }
}
