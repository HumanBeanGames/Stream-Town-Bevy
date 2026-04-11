using Reflex.Core;
using UnityEngine;
using UserInterface;

[RequireComponent(typeof(UserInterface_GameMenu))]
public class UserInterface_GameMenuInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<UserInterface_GameMenu>());
    }
}
