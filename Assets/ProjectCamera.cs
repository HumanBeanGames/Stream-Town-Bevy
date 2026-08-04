using UnityEngine;
using UnityEngine.Rendering.Universal;
public class ProjectCamera
{
    private Camera _camera;
    private UniversalAdditionalCameraData _cameraData;
	private readonly Vector3 _defaultPosition;
	private readonly Quaternion _defaultRotation;

    public ProjectCamera(Camera camera = null, UniversalAdditionalCameraData cameraData = null)
    {
        _camera = camera;
        _cameraData = cameraData;
		_defaultPosition = camera != null ? camera.transform.position : Vector3.zero;
		_defaultRotation = camera != null ? camera.transform.rotation : Quaternion.identity;
    }

    public Camera Cam
    {
        get => _camera;
        set => _camera = value;
    }

    public UniversalAdditionalCameraData Data
    {
        get => _cameraData;
        set => _cameraData = value;
    }

    public bool Exists => _camera != null;

	/// <summary>Restores the persistent project camera after leaving a world scene.</summary>
	public void RestoreMenuPose()
	{
		if (_camera == null)
			return;

		_camera.gameObject.SetActive(true);
		_camera.enabled = true;
		_camera.transform.SetPositionAndRotation(_defaultPosition, _defaultRotation);
	}
}
