using Reflex.Core;
using Processors;
using Core;
using UnityEngine;

[RequireComponent(typeof(Coordinator))]
public class CoordinatorInstaller : MonoBehaviour, IInstaller
{
	public void InstallBindings(ContainerBuilder containerBuilder)
	{
		containerBuilder.AddSingleton(GetComponent<Coordinator>());
	}
}
