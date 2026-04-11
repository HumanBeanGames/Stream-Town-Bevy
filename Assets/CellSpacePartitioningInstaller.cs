using Reflex.Core;
using UnityEngine;
using GridSystem.Partitioning;

[RequireComponent(typeof(CellSpacePartitioning))]
public class CellSpacePartitioningInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<CellSpacePartitioning>());
    }
}
