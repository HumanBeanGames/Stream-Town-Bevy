using System;
using System.Collections;
using UnityEngine;
using UnityEngine.InputSystem;
using Utils;
namespace Utils
{
	public class SnapToGridMouseMovement : MonoBehaviour
	{
		public Action OnPositionChanged;

		[SerializeField]
		private float _cellSize = 2.0f;

		[SerializeField]
		private LayerMask _collisionMask;

		[SerializeField]
		private LayerMask _terrainMask;
		
		private Camera _mainCamera;

		private Vector3 _lastSnappedPosition = Vector3.zero;

		private void MoveObject()
		{
			Ray ray = _mainCamera.ScreenPointToRay(Mouse.current.position.ReadValue());
			RaycastHit hit;

			if (Physics.Raycast(ray, out hit, Mathf.Infinity, _collisionMask))
			{
				Vector3 snappedPosition = MathExtended.SnapPosition(hit.point, _cellSize);

				if (Physics.Raycast(new Vector3(snappedPosition.x, 100, snappedPosition.z), Vector3.down, out RaycastHit terrainHit, 200, _terrainMask))
					snappedPosition.y = terrainHit.point.y;
				else
					snappedPosition.y = hit.point.y;

				transform.position = snappedPosition;

				if (_lastSnappedPosition != transform.position)
					OnPositionChanged?.Invoke();

				_lastSnappedPosition = transform.position;
			}
		}

		private void Awake()
		{
			_mainCamera = Camera.main;
		}
		
		private void Update()
		{
			MoveObject();
		}
	}
}