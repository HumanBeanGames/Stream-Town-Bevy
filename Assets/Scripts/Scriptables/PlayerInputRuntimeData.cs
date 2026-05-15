using System;

using ScriptablesProcessorInfrastructure;
using PlayerControls;
using System.Collections.Generic;
using UnityEngine;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores player input state for the game.
	/// Manages PlayerInput component, held keys, mouse position/delta, and input events.
	/// </summary>
	public class PlayerInputRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Reference to the Unity PlayerInput component.
		/// Used to handle input actions and callbacks.
		/// </summary>
		private PlayerInput _playerInput;

		/// <summary>
		/// Whether the player input system has been initialized.
		/// Set to true after PlayerInput component is set up.
		/// </summary>
		private bool _isInitialized;

		/// <summary>
		/// Dictionary tracking which input buttons are currently held down.
		/// True indicates the button is being held, false indicates it's not.
		/// </summary>
		private Dictionary<Data.SharedTypes.InputButton, bool> _heldKeys;

		/// <summary>
		/// Screen position of the last mouse click.
		/// Used for drag-and-drop and click-based interactions.
		/// </summary>
		private Vector2 _mouseLastClickPosition;

		/// <summary>
		/// Previous frame's mouse position.
		/// Used to calculate mouse movement delta.
		/// </summary>
		private Vector2 _previousMousePos;

		/// <summary>
		/// Mouse movement delta since last frame.
		/// Calculated as current position minus previous position.
		/// </summary>
		private Vector2 _mouseDelta;

		/// <summary>
		/// Current mouse screen position.
		/// Updated every frame from the input system.
		/// </summary>
		private Vector2 _mousePosition;

		/// <summary>
		/// Whether the escape key was pressed this frame.
		/// Used to cancel actions or close menus.
		/// </summary>
		private bool _escapePressed;

		/// <summary>
		/// Whether gameplay input dispatch is currently suppressed.
		/// Used while text-entry UI owns focus.
		/// </summary>
		private bool _suppressGameplayInput;

		/// <summary>
		/// Gets or sets the PlayerInput component reference.
		/// </summary>
		public PlayerInput PlayerInput
		{
			get => _playerInput;
			set => _playerInput = value;
		}

		/// <summary>
		/// Gets or sets whether the input system is initialized.
		/// </summary>
		public bool IsInitialized
		{
			get => _isInitialized;
			set => _isInitialized = value;
		}

		/// <summary>
		/// Gets the dictionary of held key states.
		/// </summary>
		public Dictionary<Data.SharedTypes.InputButton, bool> HeldKeys => _heldKeys;

		/// <summary>
		/// Gets or sets the position of the last mouse click.
		/// </summary>
		public Vector2 MouseLastClickPosition
		{
			get => _mouseLastClickPosition;
			set => _mouseLastClickPosition = value;
		}

		/// <summary>
		/// Gets or sets the previous mouse position.
		/// </summary>
		public Vector2 PreviousMousePos
		{
			get => _previousMousePos;
			set => _previousMousePos = value;
		}

		/// <summary>
		/// Gets or sets the mouse movement delta.
		/// </summary>
		public Vector2 MouseDelta
		{
			get => _mouseDelta;
			set => _mouseDelta = value;
		}

		/// <summary>
		/// Gets or sets the current mouse position.
		/// </summary>
		public Vector2 MousePosition
		{
			get => _mousePosition;
			set => _mousePosition = value;
		}

		/// <summary>
		/// Gets whether escape was pressed.
		/// </summary>
		public bool EscapePressed => _escapePressed;

		/// <summary>
		/// Gets or sets whether gameplay input dispatch is currently suppressed.
		/// </summary>
		public bool SuppressGameplayInput
		{
			get => _suppressGameplayInput;
			set => _suppressGameplayInput = value;
		}

		/// <summary>
		/// Event fired when left mouse button is pressed.
		/// Passes the input button that was pressed.
		/// </summary>
		public event Action<Data.SharedTypes.InputButton> OnLeftClickPress;

		/// <summary>
		/// Event fired when right mouse button is pressed.
		/// Passes the input button that was pressed.
		/// </summary>
		public event Action<Data.SharedTypes.InputButton> OnRightClickPress;

		/// <summary>
		/// Event fired when left mouse button is released.
		/// Passes the input button that was released.
		/// </summary>
		public event Action<Data.SharedTypes.InputButton> OnLeftClickRelease;

		/// <summary>
		/// Event fired while left mouse button is held down.
		/// Passes the input button being held.
		/// </summary>
		public event Action<Data.SharedTypes.InputButton> OnLeftClickHold;

		/// <summary>
		/// Initializes the player input runtime data with default values.
		/// </summary>
		public PlayerInputRuntimeData()
		{
			_playerInput = null;
			_isInitialized = false;
			_heldKeys = new Dictionary<Data.SharedTypes.InputButton, bool>();
			_mouseLastClickPosition = Vector2.zero;
			_previousMousePos = Vector2.zero;
			_mouseDelta = Vector2.zero;
			_mousePosition = Vector2.zero;
			_escapePressed = false;
			_suppressGameplayInput = false;
		}
	}
}
