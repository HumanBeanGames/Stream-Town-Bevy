using Reflex.Core;
using System.Collections.Generic;
using UnityEngine;

public class VideoSettingsPresetsInstaller : MonoBehaviour, IInstaller
{
    [SerializeField]
    List<VideoSettingsPreset> presets;
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(presets.ToArray());
    }
}
