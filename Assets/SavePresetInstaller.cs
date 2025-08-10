using Reflex.Core;
using Settings;
using UnityEngine;
using UnityEngine.Rendering.Universal;
[RequireComponent(typeof(SavePreset))]

//TODO: WE WANT TO DELETE THIS LATER WHEN SETTINGSMANAGER IS NO LONGER A CENTRALIZED SYSTEM
public class SavePresetInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<SavePreset>());
    }
}