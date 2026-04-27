using Reflex.Core;
using UnityEngine;
using UnityEngine.Rendering.Universal;

public class ProjectCameraInstaller : MonoBehaviour, IInstaller
{
    [SerializeField]
    private GameObject _cameraPrefab;

    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        GameObject cameraInstance = Instantiate(_cameraPrefab);
        Camera camera = cameraInstance.GetComponent<Camera>();
        UniversalAdditionalCameraData cameraData = cameraInstance.GetComponent<UniversalAdditionalCameraData>();
        containerBuilder.AddSingleton(c => new ProjectCamera(camera, cameraData));
    }
}
