using Reflex.Core;
using Managers;
using UnityEngine;

[RequireComponent(typeof(GameManager))]
public class GameManagerInstaller : MonoBehaviour, IInstaller
{
	public void InstallBindings(ContainerBuilder containerBuilder)
	{
		containerBuilder.AddSingleton(GetComponent<GameManager>());
	}
}
