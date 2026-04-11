using Reflex.Core;
using UnityEngine;
using Audio;

[RequireComponent(typeof(AudioSourcesManager))]
public class AudioSourcesManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<AudioSourcesManager>());
    }
}
