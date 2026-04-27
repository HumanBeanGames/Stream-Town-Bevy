using System;
using UnityEngine;
using UnityEngine.Events;
using Utils;
using UserInterface;
using PlayerControls;
using Character;
using Buildings;
using Enemies;
using GameResources;
using System.Collections.Generic;
using Utils.Pooling;
using World;
using UnityEngine.EventSystems;
using Sensors;
using Target;
using SavingAndLoading.SavableObjects;
using PlayerControls.ObjectSelection;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Data;
using InputButton = Data.SharedTypes.InputButton;

namespace Processors
{
    /// <summary>
    /// Processor that manages object selection for the game.
    /// Handles single object selection, group selection, and selection events.
    /// </summary>
	public partial class ObjectSelectionProcessor : MonoBehaviour, IInstaller, IProcessor, IMainThreadInitializableProcessor
	{
        /// <summary>
        /// Object pooling processor for accessing pooled objects.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// ScriptableObject containing object selection settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private ObjectSelectionSettings _objectSelectionSettings;

        /// <summary>
        /// Player input processor for accessing input data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private PlayerInputProcessor _playerInputProcessor;

        /// <summary>
        /// Runtime data ScriptableObject for object selection data.
        /// Created and bound in InjectRuntimeData().
        /// </summary>
        private ObjectSelectionRuntimeData _objectSelectionRuntimeData;

        /// <summary>
        /// Subscribes a callback to the object selected event.
        /// </summary>
        /// <param name="callback">The callback to invoke when an object is selected.</param>
        public void SubscribeToObjectSelected(UnityEngine.Events.UnityAction<SelectableObject, object> callback)
		{
			_objectSelectionRuntimeData.OnObjectSelected.AddListener(callback);
		}

		/// <summary>
		/// Unsubscribes a callback from the object selected event.
		/// </summary>
		/// <param name="callback">The callback to remove from the event.</param>
		public void UnsubscribeFromObjectSelected(UnityEngine.Events.UnityAction<SelectableObject, object> callback)
		{
			_objectSelectionRuntimeData.OnObjectSelected.RemoveListener(callback);
		}

		/// <summary>
		/// Manually invokes the object selected event.
		/// </summary>
		/// <param name="selected">The selected object.</param>
		/// <param name="data">The data associated with the selected object.</param>
		public void InvokeObjectSelected(SelectableObject selected, object data)
		{
			_objectSelectionRuntimeData.OnObjectSelected.Invoke(selected, data);
		}

		/// <summary>
		/// Called when SelectableObject is selected by the mouse and displays it's data.
		/// </summary>
		/// <param name="selected">The selected object.</param>
		/// <param name="data">The data associated with the selected object.</param>
		private void ObjectSelected(SelectableObject selected, object data)
		{
			if (_objectSelectionSettings.SelectionUI == null)
				return;

			_objectSelectionRuntimeData.SelectedObject = (selected, data);

			//		Debug.Log($"Object Selected: {selected.gameObject.transform.parent.name}, {selected.SelectableType}");

			_objectSelectionRuntimeData.ObjectSelected = true;
			switch (selected.SelectableType)
			{
				case Selectable.Player:
					_objectSelectionSettings.SelectionUI.OnCharacterContext((RoleHandler)data);
					break;
				case Selectable.Building:
					_objectSelectionSettings.SelectionUI.OnBuildingContext((BuildingBase)data);
					break;
				case Selectable.Enemy:
					_objectSelectionSettings.SelectionUI.OnEnemyContext((Enemy)data);
					break;
				case Selectable.Resource:
					_objectSelectionSettings.SelectionUI.OnResourceContext((ResourceHolder)data);
					break;
				case Selectable.EnemyCamp:
					_objectSelectionSettings.SelectionUI.OnEnemyCampContext((Station)data);
					break;
				default:
					SetSelectionFalse();
					break;
			}
		}

		private void Select(InputButton button)
		{
			if (RayTraceFromCamera(Camera.main, _playerInputProcessor.MousePosition, out Vector3 hitPos))
				_objectSelectionRuntimeData.StartedSelectionPosition = hitPos;

			HandleSelect();
		}

		private void HandleSelect()
		{
			if (Physics.Raycast(Camera.main.ScreenPointToRay(_playerInputProcessor.MousePosition), out RaycastHit hitInfo, float.MaxValue) && !WorldUtils.IsPointerOverUI(EventSystem.current))
			{
				SelectableObject obj = hitInfo.transform.GetComponentInChildren<SelectableObject>();
				if (obj != null)
				{
					if (_objectSelectionRuntimeData.SelectedObject.Item1 == obj)
					{
						HideUI();
						return;
					}
					_objectSelectionRuntimeData.OnObjectSelected.Invoke(obj, obj.Data);
				}
				else
					HideUI();

				_objectSelectionSettings.SelectionUI.DisableCheckUI();
			}
		}

		private void StartGroupSelect(InputButton button)
		{
			_objectSelectionRuntimeData.StartedGroupSelection = true;
			_objectSelectionSettings.SelectionUI.HideContext();

			//Debug.Log("Started Group Selection");
		}

		/// <summary>
		/// Creates a ray from the camera that stops at the world height (y == 0).
		/// Used to convert mouse screen position to world position.
		/// </summary>
		/// <param name="cam">The camera doing the trace.</param>
		/// <param name="mousePos">The mouse position in screen space.</param>
		/// <param name="hitPosition">The world position where the ray ends.</param>
		/// <returns>True if the ray trace succeeded.</returns>
		public bool RayTraceFromCamera(Camera cam, Vector2 mousePos, out Vector3 hitPosition)
        {
			Vector3 camRay = cam.ScreenPointToRay(mousePos).direction;
			float t = (0 - cam.transform.position.y) / camRay.y;
			hitPosition = cam.transform.position + (camRay * t);

			//return Coordinator.Instance.WorldGenProcessor.IsPointWithinBounds(hitPosition);
			return true;
        }

		private void GroupSelect(InputButton button)
		{
			if (_objectSelectionRuntimeData.StartedGroupSelection)
			{
				if (RayTraceFromCamera(Camera.main, _playerInputProcessor.MousePosition, out Vector3 hitPos))
					_objectSelectionRuntimeData.EndedSelectionPosition = hitPos;
				else
					Debug.LogError("This should not be happening");

				Vector3 selectionMin = Vector3.Min(_objectSelectionRuntimeData.StartedSelectionPosition, _objectSelectionRuntimeData.EndedSelectionPosition);
				Vector3 selectionMax = Vector3.Max(_objectSelectionRuntimeData.StartedSelectionPosition, _objectSelectionRuntimeData.EndedSelectionPosition);
				List<PoolableObject> objs = _poolingProcessor.GetAllActiveObjectsOfTypeWithinAABB(selectionMin, selectionMax, "Player");
				List<RoleHandler> roleHandlers = new List<RoleHandler>();
				for (int i = 0; i < objs.Count; i++)
				{
					if (objs[i].PoolType == PoolType.Player)
					{
						SelectableObject selectable = objs[i].transform.GetComponentInChildren<SelectableObject>();
						if (((RoleHandler)selectable.Data).Player.TwitchUser.Username == "")
							roleHandlers.Add((RoleHandler)selectable.Data);
					}
				}
				if (roleHandlers.Count > 1)
				{
					_objectSelectionSettings.SelectionUI.OnCharacterGroupContext(roleHandlers);
					_objectSelectionRuntimeData.SelectedPlayerGroup = roleHandlers;
					_objectSelectionRuntimeData.GroupSelected = true;
					_objectSelectionRuntimeData.ObjectSelected = false;
				}
				else if (roleHandlers.Count == 1)
				{
					_objectSelectionSettings.SelectionUI.OnCharacterContext(roleHandlers[0]);
					_objectSelectionRuntimeData.ObjectSelected = true;
				}
				else
				{
					_objectSelectionRuntimeData.ObjectSelected = false;
					_objectSelectionRuntimeData.GroupSelected = false;
				}
				Debug.Log("Ended selections");
				_objectSelectionRuntimeData.StartedGroupSelection = false;

			}
			if (!_objectSelectionRuntimeData.ObjectSelected && !_objectSelectionRuntimeData.GroupSelected)
			{
				_objectSelectionSettings.SelectionUI.HideContext();
				_objectSelectionSettings.SelectionUI.DisableCheckUI();
			}
			// Find all object of selected type within the AABB of _startedSelectionPosition and _endedSelectionPosition
		}

		/// <summary>
		/// Hides the selection UI and clears the selected object.
		/// </summary>
		public void HideUI()
		{
			if (_objectSelectionSettings.SelectionUI == null)
				return;
			_objectSelectionRuntimeData.SelectedObject = (null, null);
			_objectSelectionSettings.SelectionUI.HideContext();
		}

		/// <summary>
		/// Sets all selection flags to false.
		/// </summary>
		public void SetSelectionFalse()
		{
			_objectSelectionRuntimeData.ObjectSelected = false;
			_objectSelectionRuntimeData.GroupSelected = false;
		}

		// Sets the target for the selected object(s) based on type.
		private void SetTarget(SelectableObject obj)
		{
			switch (obj.Type)
			{
				case Selectable.Building:
					Station station = ((BuildingBase)obj.Data).Station;
					if (_objectSelectionRuntimeData.GroupSelected)
						SetGroupStation(_objectSelectionRuntimeData.SelectedPlayerGroup, station);
					else
						((RoleHandler)_objectSelectionRuntimeData.SelectedObject.Item2).Player.StationSensor.TrySetStation(station, ((RoleHandler)_objectSelectionRuntimeData.SelectedObject.Item2).Player);
					break;
				case Selectable.Enemy:
					Enemy enemy = ((Enemy)obj.Data);
					if (_objectSelectionRuntimeData.GroupSelected)
						SetGroupTarget(_objectSelectionRuntimeData.SelectedPlayerGroup, enemy.GetComponent<Targetable>());
					else
						((RoleHandler)_objectSelectionRuntimeData.SelectedObject.Item2).Player.TargetSensor.TrySetTarget(enemy.gameObject.GetComponent<Targetable>(), ((RoleHandler)_objectSelectionRuntimeData.SelectedObject.Item2).Player);
					break;
				case Selectable.Resource:
					ResourceHolder resource = ((ResourceHolder)obj.Data);
					if (_objectSelectionRuntimeData.GroupSelected)
						SetGroupTarget(_objectSelectionRuntimeData.SelectedPlayerGroup, resource.GetComponent<Targetable>());
					else
						((RoleHandler)_objectSelectionRuntimeData.SelectedObject.Item2).Player.TargetSensor.TrySetTarget(resource.gameObject.GetComponent<Targetable>(), ((RoleHandler)_objectSelectionRuntimeData.SelectedObject.Item2).Player);
					break;
			}
		}

		/// <summary>
		/// Handles right-click input to set targets for selected objects.
		/// </summary>
		/// <param name="button">The button that was clicked.</param>
		public void OnRightClick(InputButton button)
		{
			HideUI();
			HandleRightClick();
		}

		private void HandleRightClick()
		{
			if (Physics.Raycast(Camera.main.ScreenPointToRay(_playerInputProcessor.MousePosition), out RaycastHit hitInfo, float.MaxValue) && !WorldUtils.IsPointerOverUI(EventSystem.current))
			{
				SelectableObject obj = hitInfo.transform.GetComponentInChildren<SelectableObject>();
				if (obj != null)
				{
					SetTarget(obj);
				}
			}
		}

		/// <summary>
		/// Sets the target for a group of players.
		/// </summary>
		/// <param name="recruits">The list of players to set the target for.</param>
		/// <param name="target">The target to set for the players.</param>
		public void SetGroupTarget(List<RoleHandler> recruits, Targetable target)
		{
			for (int i = 0; i < recruits.Count; i++)
				recruits[i].Player.TargetSensor.TrySetTarget(target, recruits[i].Player);
		}

		/// <summary>
		/// Sets the station for a group of players.
		/// </summary>
		/// <param name="recruits">The list of players to set the station for.</param>
		/// <param name="station">The station to set for the players.</param>
		public void SetGroupStation(List<RoleHandler> recruits, Station station)
		{
			for (int i = 0; i < recruits.Count; i++)
				recruits[i].Player.StationSensor.TrySetStation(station, recruits[i].Player);
		}

		public void Initialize()
		{
			if (_objectSelectionRuntimeData == null)
				throw new InvalidOperationException("ObjectSelectionProcessor: ObjectSelectionRuntimeData has not been installed.");

			_objectSelectionRuntimeData.OnObjectSelected = new UnityEvent<SelectableObject, object>();
			_objectSelectionRuntimeData.OnObjectSelected.AddListener(ObjectSelected);

			_playerInputProcessor.OnLeftClickPress += Select;
			_playerInputProcessor.OnRightClickPress += OnRightClick;
			_playerInputProcessor.OnLeftClickRelease += GroupSelect;
			_playerInputProcessor.OnLeftClickHold += StartGroupSelect;
		}

		public void Process()
		{
			if (_objectSelectionRuntimeData.StartedGroupSelection)
			{
				Vector3 mousePos = Vector3.zero;
				if (RayTraceFromCamera(Camera.main, _playerInputProcessor.MousePosition, out Vector3 hitPos))
					mousePos = hitPos;
				else
					return;

				_objectSelectionRuntimeData.GroupSelectionRect.anchoredPosition = _playerInputProcessor.MousePosition;
				_objectSelectionRuntimeData.GroupSelectionRect.sizeDelta = mousePos - _objectSelectionRuntimeData.GroupSelectionStartPos;
			}

			if (_playerInputProcessor.EscapePressed)
			{
				_objectSelectionSettings.SelectionUI.HideContext();
			}

			if (_objectSelectionRuntimeData.GroupSelected)
			{
				if (Physics.Raycast(Camera.main.ScreenPointToRay(_playerInputProcessor.MousePosition), out RaycastHit hitInfo, float.MaxValue) && !WorldUtils.IsPointerOverUI(EventSystem.current))
				{
					SelectableObject obj = hitInfo.transform.GetComponentInChildren<SelectableObject>();
					if (obj != null)
					{
						SetTarget(obj);
					}
				}
			}
			else if (_objectSelectionRuntimeData.ObjectSelected)
			{
				if (_objectSelectionRuntimeData.SelectedObject.Item1 != null && _objectSelectionRuntimeData.SelectedObject.Item1.Type == Selectable.Player && (((RoleHandler)_objectSelectionRuntimeData.SelectedObject.Item2).Player.TwitchUser.Username == "" || ((RoleHandler)_objectSelectionRuntimeData.SelectedObject.Item2).Player.TwitchUser.TwitchUserType == TwitchLib.Client.Enums.UserType.Broadcaster))
				{
					if (Physics.Raycast(Camera.main.ScreenPointToRay(_playerInputProcessor.MousePosition), out RaycastHit hitInfo, float.MaxValue) && !WorldUtils.IsPointerOverUI(EventSystem.current))
					{
						SelectableObject obj = hitInfo.transform.GetComponentInChildren<SelectableObject>();
						if (obj != null)
						{
							SetTarget(obj);
						}
					}
				}
			}
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// ObjectSelectionProcessor does not have scene-specific settings to refresh
		}

		/// <summary>
		/// Registers this processor as a singleton in the dependency injection container.
		/// Called by Reflex during container initialization.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		/// <summary>
		/// Injects the ObjectSelectionRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_objectSelectionRuntimeData != null)
				throw new InvalidOperationException("ObjectSelectionProcessor: ObjectSelectionRuntimeData has already been installed.");

			_objectSelectionRuntimeData = new ObjectSelectionRuntimeData();
			containerBuilder.AddSingleton(_objectSelectionRuntimeData);
		}
	}
}
