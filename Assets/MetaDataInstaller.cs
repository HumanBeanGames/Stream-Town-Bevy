using Reflex.Core;
using UnityEngine;
using MetaData;

[RequireComponent(typeof(MetaData.MetaData))]
public class MetaDataInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton<MetaData.MetaData>(container => GetComponent<MetaData.MetaData>());
    }
}
