using System;
using System.Collections.Generic;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Processors;
using static Data.SharedTypes;

namespace Processors
{
	public class PlayerInputProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		[Inject] private PlayerInputSettings _playerInputSettings;

        /// <summary>
        /// Runtime player input data ScriptableObject.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private PlayerInputRuntimeData _playerInputRuntimeData;

		public event Action<InputButton> OnLeftClickPress;
		public event Action<InputButton> OnLeftClickHold;
		public event Action<InputButton> OnLeftClickRelease;

		public event Action<InputButton> OnRightClickPress;
		public event Action<InputButton> OnRightClickHold;
		public event Action<InputButton> OnRightClickRelease;

		public event Action<InputButton> OnMiddleClickPress;
		public event Action<InputButton> OnMiddleClickHold;
		public event Action<InputButton> OnMiddleClickRelease;

		public event Action<float> OnMouseScroll;

		public event Action<Vector2> OnMousePosition;

		public event Action OnBuildMenu;

		public event Action OnTechTree;

		public event Action OnRecruit;

		public event Action OnSaveGame;

		public event Action OnLoadGame;

		public event Action OnGenerateGame;

		public Vector2 MousePosition
		{
			get { return _playerInputRuntimeData.PlayerInput != null ? _playerInputRuntimeData.PlayerInput.BasicControls.MousePosition.ReadValue<Vector2>() : Vector2.zero; }
		}

		public bool IsButtonHeld(InputButton button)
		{
			if (_playerInputRuntimeData.PlayerInput == null)
				return false;

			switch (button)
			{
				case InputButton.LeftMouse:
					return _playerInputRuntimeData.PlayerInput.BasicControls.MouseLeftClick.ReadValue<float>() > 0;
				case InputButton.RightMouse:
					return _playerInputRuntimeData.PlayerInput.BasicControls.MouseRightClick.ReadValue<float>() > 0;
				case InputButton.MiddleMouse:
					return _playerInputRuntimeData.PlayerInput.BasicControls.MouseMiddleClick.ReadValue<float>() > 0;
				default:
					return false;
			}
		}

		public bool EscapePressed
		{
			get { return _playerInputRuntimeData.PlayerInput != null ? _playerInputRuntimeData.PlayerInput.BasicControls.Escape.ReadValue<float>() > 0.01f : false; }
		}

		public void Initialize()
		{
			_playerInputRuntimeData.PlayerInput = new PlayerInput();

			_playerInputRuntimeData.PlayerInput.BasicControls.MouseLeftClick.started += ctx => OnLeftClickPress?.Invoke(InputButton.LeftMouse);     // Started is called on button clicked
			_playerInputRuntimeData.PlayerInput.BasicControls.MouseLeftClick.performed += ctx => OnLeftClickHold?.Invoke(InputButton.LeftMouse);    // Performed is called on button hold
			_playerInputRuntimeData.PlayerInput.BasicControls.MouseLeftClick.canceled += ctx => OnLeftClickRelease?.Invoke(InputButton.LeftMouse);  // Cancelled is caled on button release

			_playerInputRuntimeData.PlayerInput.BasicControls.MouseRightClick.started += ctx => OnRightClickPress?.Invoke(InputButton.RightMouse);
			_playerInputRuntimeData.PlayerInput.BasicControls.MouseRightClick.performed += ctx => OnRightClickHold?.Invoke(InputButton.RightMouse);
			_playerInputRuntimeData.PlayerInput.BasicControls.MouseRightClick.canceled += ctx => OnRightClickRelease?.Invoke(InputButton.RightMouse);

			_playerInputRuntimeData.PlayerInput.BasicControls.MouseMiddleClick.started += ctx => OnMiddleClickPress?.Invoke(InputButton.MiddleMouse);
			_playerInputRuntimeData.PlayerInput.BasicControls.MouseMiddleClick.performed += ctx => OnMiddleClickHold?.Invoke(InputButton.MiddleMouse);
			_playerInputRuntimeData.PlayerInput.BasicControls.MouseMiddleClick.canceled += ctx => OnMiddleClickRelease?.Invoke(InputButton.MiddleMouse);

			_playerInputRuntimeData.PlayerInput.BasicControls.MouseScroll.started += ctx => OnMouseScroll?.Invoke(ctx.ReadValue<Vector2>().y);
			_playerInputRuntimeData.PlayerInput.BasicControls.MouseScroll.canceled += ctx => OnMouseScroll?.Invoke(ctx.ReadValue<Vector2>().y);

			_playerInputRuntimeData.PlayerInput.BasicControls.MousePosition.performed += ctx => OnMousePosition?.Invoke(ctx.ReadValue<Vector2>());

			//_playerInput.BasicControls.Escape.started += ctx => OnEscape?.Invoke();

			_playerInputRuntimeData.PlayerInput.BasicControls.BuildMenu.started += ctx => OnBuildMenu?.Invoke();

			_playerInputRuntimeData.PlayerInput.BasicControls.TechTree.started += ctx => OnTechTree?.Invoke();

			_playerInputRuntimeData.PlayerInput.BasicControls.Recruit.started += ctx => OnRecruit?.Invoke();

			_playerInputRuntimeData.PreviousMousePos = _playerInputRuntimeData.PlayerInput.BasicControls.MousePosition.ReadValue<Vector2>();

			// Temp
			_playerInputRuntimeData.PlayerInput.BasicControls.TempGenerateWorld.started += ctx => OnGenerateGame?.Invoke();
			_playerInputRuntimeData.PlayerInput.BasicControls.TempLoadWorld.started += ctx => OnLoadGame?.Invoke();
			_playerInputRuntimeData.PlayerInput.BasicControls.TempSaveWorld.started += ctx => OnSaveGame?.Invoke();
			//

			InitializeHeldKeys();

			_playerInputRuntimeData.PlayerInput.Enable();
			_playerInputRuntimeData.IsInitialized = true;

			OnLeftClickPress += ButtonPressed;
			OnLeftClickHold += ButtonHeld;
			OnLeftClickRelease += ButtonRelease;
			OnLeftClickPress += SetClickPosition;

			OnRightClickPress += ButtonPressed;
			OnRightClickHold += ButtonHeld;
			OnRightClickRelease += ButtonRelease;

			OnMiddleClickPress += ButtonPressed;
			OnMiddleClickHold += ButtonHeld;
			OnMiddleClickRelease += ButtonRelease;
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register PlayerInputRuntimeData ScriptableObject
			PlayerInputRuntimeData playerInputRuntimeData = ScriptableObject.CreateInstance<PlayerInputRuntimeData>();
			containerBuilder.AddSingleton(playerInputRuntimeData);
		}

		/// <summary>
		/// Runs whenever a button is pressed
		/// </summary>
		/// <param name="button">The button that is being pressed</param>
		private void ButtonPressed(InputButton button = InputButton.None)
		{
			//Debug.Log("Pressed:" + button.ToString());
		}

		/// <summary>
		/// Runs whenever a buton is being held
		/// </summary>
		/// <param name="button">The button that is being held</param>
		private void ButtonHeld(InputButton button = InputButton.None)
		{
			//Debug.Log("Holding: " + button.ToString());
			_playerInputRuntimeData.HeldKeys[button] = true;
		}

		/// <summary>
		/// Runs whenever a button is released
		/// </summary>
		/// <param name="button">The button that is being released</param>
		private void ButtonRelease(InputButton button = InputButton.None)
		{
			//Debug.Log("Released: " + button.ToString());
			_playerInputRuntimeData.HeldKeys[button] = false;
		}

		/// <summary>
		/// Sets the click position
		/// </summary>
		/// <param name="button"></param>
		private void SetClickPosition(InputButton button)
		{
			_playerInputRuntimeData.MouseLastClickPosition = _playerInputRuntimeData.PlayerInput.BasicControls.MousePosition.ReadValue<Vector2>();
			//Debug.Log(_mouseLastClickPosition);
		}

		/// <summary>
		/// Processes player input logic every frame.
		/// Called every frame by the Coordinator.
		/// PlayerInputProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			if (_playerInputRuntimeData.PlayerInput == null)
				return;

			Vector2 currentMousePos = _playerInputRuntimeData.PlayerInput.BasicControls.MousePosition.ReadValue<Vector2>();
			_playerInputRuntimeData.MouseDelta = _playerInputRuntimeData.PreviousMousePos - currentMousePos;
			_playerInputRuntimeData.PreviousMousePos = currentMousePos;
		}

		private void InitializeHeldKeys()
		{
			_playerInputRuntimeData.HeldKeys.Clear();
			for (int i = 0; i < (int)InputButton.Count; i++)
			{
				_playerInputRuntimeData.HeldKeys.Add((InputButton)i, false);
			}
		}

		private void ClearHeldKeys()
		{
			_playerInputRuntimeData.HeldKeys.Clear();
		}
	}
}
