using UnityEngine;

namespace Utils
{
	/// <summary>
	/// A simple component that rotates a gameobject to look at the main camera.
	/// </summary>
	public class SimpleLookAtCamera : MonoBehaviour
	{
		private Camera _targetCamera;

		private void LateUpdate()
		{
			if (_targetCamera == null || !_targetCamera.isActiveAndEnabled)
				_targetCamera = Camera.main;

			if (_targetCamera == null)
				return;

			Vector3 cameraToLabel = transform.position - _targetCamera.transform.position;
			if (cameraToLabel.sqrMagnitude <= Mathf.Epsilon)
				return;

			// TextMeshPro's readable face points along its forward axis. Use the
			// actual camera-to-label direction so perspective labels remain readable
			// away from the centre of the screen, and update after camera movement.
			transform.rotation = Quaternion.LookRotation(cameraToLabel, _targetCamera.transform.up);
		}
	}
}
