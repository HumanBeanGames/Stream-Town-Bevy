using Reflex.Core;
using UnityEngine;
using TownGoal;

[RequireComponent(typeof(TownGoalManager))]
public class TownGoalManagerInstaller : MonoBehaviour, IInstaller
{
    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        containerBuilder.AddSingleton(GetComponent<TownGoalManager>());
    }
}
