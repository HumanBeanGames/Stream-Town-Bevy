using Reflex.Core;
using Settings;
using UnityEngine;

public class SettingsInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        SettingsData data = SettingsIO.LoadOrCreate();
        if (data == null) Debug.LogError("Something went wrong in loading the settings file!");
        containerBuilder.AddSingleton(data);
    }
}
