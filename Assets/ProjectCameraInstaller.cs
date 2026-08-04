using Reflex.Core;
using UnityEngine;
using UnityEngine.Rendering.Universal;

public class ProjectCameraInstaller : MonoBehaviour, IInstaller
{
    [SerializeField]
    private GameObject _cameraPrefab;

	private static Camera _installedCamera;

    public void InstallBindings(ContainerBuilder containerBuilder)
    {
		if (_cameraPrefab == null)
			throw new System.InvalidOperationException("ProjectCameraInstaller requires a camera prefab.");

		if (_installedCamera == null)
		{
			Camera[] cameras = Resources.FindObjectsOfTypeAll<Camera>();
			for (int i = 0; i < cameras.Length; i++)
			{
				Camera candidate = cameras[i];
				if (candidate != null && candidate.gameObject.scene.IsValid() && candidate.name == _cameraPrefab.name)
				{
					_installedCamera = candidate;
					break;
				}
			}
		}

		if (_installedCamera == null)
		{
			GameObject cameraInstance = Instantiate(_cameraPrefab);
			cameraInstance.name = _cameraPrefab.name;
			_installedCamera = cameraInstance.GetComponent<Camera>();
		}

		DontDestroyOnLoad(_installedCamera.gameObject);
		UniversalAdditionalCameraData cameraData = _installedCamera.GetComponent<UniversalAdditionalCameraData>();
		containerBuilder.AddSingleton(c => new ProjectCamera(_installedCamera, cameraData));
    }
}
