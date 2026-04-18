// CameraController.cs
//
// Purpose:
//   Player-controlled RTS-style camera with pan (MMB drag), zoom (mouse wheel),
//   keyboard/edge scrolling movement, and a simple "command move" coroutine for
//   scripted camera motions. Runtime tuning is driven by injected SettingsData.
//
// Key design notes:
//   � Uses SmoothDamp for pan/move/zoom for framerate-independent, velocity-based smoothing.
//   � Reads user-tunable speeds/toggles (pan/zoom/wasd sensitivity, mouse controls, edge scroll, etc.)
//     from SettingsData (injected).
//   � All user input is ignored while IsIdle == true (e.g., during cutscenes).
//   � CommandMove coroutine runs only when IsIdle == true. Starting a manual pan cancels it.
//
// Integration points:
//   � Requires a Camera component (see RequireComponent).
//   � Subscribes to PlayerInput (new Input System) for keyboard movement,
//     and to PlayerInputProcessor.OnMouseScroll for mouse wheel.
//   � GameStateProcessor.ReadiedPlayer ? enables this component when game is ready.
//   � Optionally interacts with SelectionProcessor in SetTarget() (if you wire that up).
//

using Processors;
using UnityEngine;
using Utils;
using System.Collections;
using World;
using UnityEngine.EventSystems;
using Reflex.Attributes;
using Data.Containers;

namespace PlayerControls
{
    [RequireComponent(typeof(Camera))]
    public class CameraController : MonoBehaviour
    {
        // Source of truth for user-tunable camera settings (sensitivities, toggles, etc.)
        [Inject] SettingsData CurrentSettings;
        [Inject] private ObjectSelectionProcessor _selectionProcessor;
        [Inject] private PlayerInputProcessor _playerInputProcessor;
        [Inject] private GameStateProcessor _gameStateProcessor;

        #region Inspector Fields (Constraints & Tunables)

        [Header("Camera Constraints")]
        [SerializeField] private float _maxCameraHeight = 30.0f;     // Upper clamp for zoom height
        [SerializeField] private float _minCameraHeight = 15.0f;     // Lower clamp for zoom height
        [SerializeField] private Vector2 _minimumCameraPosition = Vector2.zero; // XZ min bounds (x ? X, y ? Z)
        [SerializeField] private Vector2 _maximumCameraPosition = Vector2.zero; // XZ max bounds (x ? X, y ? Z)

        [Header("Pan")]
        [SerializeField] private bool _canPan = true;               // Global enable/disable for panning
        [SerializeField] private float _panSmoothness = 0.5f;        // SmoothDamp time (seconds) for pan

        [Header("Zoom")]
        [SerializeField] private bool _canZoom = true;              // Global enable/disable for zooming
        [SerializeField] private float _zoomSmoothness = 0.5f;       // SmoothDamp time (seconds) for zoom height

        [Header("Move")]
        [SerializeField] private bool _canMove = true;              // Global enable/disable for movement
        [SerializeField] private int _edgeSize = 5;                // Pixel threshold near screen edges for edge scroll
        [SerializeField] private float _moveSmoothness = 0.5f;       // SmoothDamp time (seconds) for move

        [Header("Command Settings")]
        [SerializeField] private float _moveTime = 2f;               // Duration used by the command-move coroutine

        #endregion

        #region Runtime Smoothing State (used by SmoothDamp)

        // Velocity refs used by Vector3.SmoothDamp and Mathf.SmoothDamp
        private Vector3 _panVelocity = Vector3.zero;
        private Vector3 _moveVelocity = Vector3.zero;
        private float _zoomVelocity = 0f;

        #endregion

        #region Private State & Dependencies

        private Transform _transform = null;       // Cached Transform
        private Camera _camera = null;          // Cached Camera, used by SetTarget()
        private SelectableObject _target = null;     // Optional: selected object target (if you wire SetTarget)

        private PlayerInput _playerInput;            // New Input System wrapper (project-specific)

        private float _scrollWheelInput = 0.0f;      // Accumulated scroll delta since last zoom step
        private float _scrollPosition = 15.0f;     // Target zoom height (interpolated toward)
        private float _transitionTime = 0.0f;      // Timer for command-move coroutine

        private bool _isPanning = false;             // True while MMB is held and pan is active
        private bool _isIdle = false;             // If true, ignore user-driven pan/move; allow command moves

        private Vector2 _keyboardInput;              // WASD/arrow input vector from Input System
        private Vector2 _previousMousePosition;       // Previous frame mouse position for delta calculation

        // Command-move state (used only by SmoothCameraMovement coroutine)
        private Vector3 _newMovePosition;            // Destination XZ for command move
        private Vector3 _startPosDuringMovement;     // Start position for current command move
        private Vector3 _movePos;                    // Accumulated movement target for free movement (pan/edge/keyboard)

        public Vector3 StartPosition { get; private set; } // Initial world position

        private Coroutine _moveRoutine;              // Handle for the command-move coroutine

        /// <summary>
        /// Global flag to pause/resume player-driven camera updates (pan/move/zoom).
        /// When set true, Pan() and Move() are skipped; ZoomCamera/MoveCamera can still be used for scripted motion.
        /// </summary>
        public bool IsIdle
        {
            get => _isIdle;
            set => _isIdle = value;
        }

        #endregion

        #region Main Update

        /// <summary>
        /// Central per-frame camera update. Pan/Move are skipped while idle; Zoom always allowed
        /// (but Zoom() will early-out if no scroll input).
        /// </summary>
        private void UpdateCamera()
        {
            if (!_isIdle)
            {
                Pan();
                Move();
            }

            Zoom();
        }

        #endregion

        #region Pan (Middle-Mouse Drag)

        /// <summary>
        /// Pans camera in XZ by dragging with the middle mouse button while mouse controls are enabled.
        /// Uses SmoothDamp for framerate-independent smoothing toward the constrained target.
        /// </summary>
        private void Pan()
        {
            // Block if globally disabled or if mouse controls are disabled via settings
            if (!_canPan || !CurrentSettings.mouseControls)
            {
                _isPanning = false;
                return;
            }

            // Only pan while MMB is held
            if (!_playerInputProcessor.IsButtonHeld(Data.SharedTypes.InputButton.MiddleMouse))
            {
                _isPanning = false;
                return;
            }

            _isPanning = true;

            // If a command-move is running, cancel it on manual pan
            if (_moveRoutine != null)
            {
                StopCoroutine(_moveRoutine);
                _moveRoutine = null;
            }

            // Convert mouse delta to a world-space pan vector (XZ), scaled by user pan sensitivity
            var delta = _playerInput.BasicControls.MousePosition.ReadValue<Vector2>() - _previousMousePosition;
            _previousMousePosition = _playerInput.BasicControls.MousePosition.ReadValue<Vector2>();
            var movementVec = new Vector3(delta.y, 0f, -delta.x) * CurrentSettings.panSensitivity;

            // Constrain target within XZ bounds
            var targetPos = MoveConstraints(_transform.position + movementVec);

            // Smoothly move toward target (XZ) while preserving smoothing velocity state
            var next = Vector3.SmoothDamp(
                _transform.position,
                targetPos,
                ref _panVelocity,
                _panSmoothness,          // smoothTime in seconds (smaller = snappier)
                Mathf.Infinity,
                Time.deltaTime
            );

            _transform.position = next;
        }

        #endregion

        #region Zoom (Mouse Wheel ? Smooth height)

        /// <summary>
        /// Smoothly adjusts camera Y height in response to mouse wheel input.
        /// _scrollPosition is the "desired" height and is smoothed via Mathf.SmoothDamp into the actual transform.y.
        /// </summary>
        private void Zoom()
        {
            if (!_canZoom) return;
            if (Mathf.Approximately(_scrollWheelInput, 0f)) return;

            // Convert scroll delta to desired height change using user zoom sensitivity
            _scrollPosition += _scrollWheelInput * CurrentSettings.zoomSensitivity * 0.004f;
            _scrollWheelInput = 0f;

            // Clamp to allowed height range
            float targetHeight = Mathf.Clamp(_scrollPosition, _minCameraHeight, _maxCameraHeight);

            // Smooth current height toward desired height
            float y = Mathf.SmoothDamp(
                _transform.position.y,
                targetHeight,
                ref _zoomVelocity,
                _zoomSmoothness,         // smoothTime in seconds
                Mathf.Infinity,
                Time.deltaTime
            );

            // Apply new height, preserve XZ
            _transform.position = new Vector3(_transform.position.x, y, _transform.position.z);
        }

        #endregion

        #region Move (Edge Scroll + Keyboard)

        /// <summary>
        /// Moves camera in XZ via:
        ///   � Edge scrolling: when mouse is near screen edges (if enabled in settings)
        ///   � Keyboard input: WASD/arrow keys via Input System
        /// A "zoom-out boost" raises speed at higher zoom heights to maintain perceived speed.
        /// </summary>
        private void Move()
        {
            // Skip if movement disabled or if we're actively panning this frame, or app not focused
            if (!_canMove || _isPanning) return;
            if (!Application.isFocused) return;

            // Edge scrolling: mouse near screen edges, if both mouse movement and edge scroll are enabled
            if (CurrentSettings.mouseControls &&
                CurrentSettings.edgeScrolling &&
                TryGetEdgeInput(out var edge))
            {
                var dir = new Vector3(edge.y, 0f, -edge.x).normalized;  // Map screen-edge to world XZ
                ApplyMove(dir * CurrentSettings.edgeScrollingSensitivity);
                return; // Edge scroll consumes this frame's move
            }

            // Keyboard movement path:
            // Increase speed the more zoomed-out we are (keeps travel speed "feeling" consistent)
            float zoomOutBoost = _scrollPosition / (_maxCameraHeight - _minCameraHeight);
            zoomOutBoost = Mathf.Pow(zoomOutBoost + 1f, zoomOutBoost + 1f);

            var keyboardDir = new Vector3(_keyboardInput.y, 0f, -_keyboardInput.x); // map input to world XZ
            ApplyMove(keyboardDir * (zoomOutBoost * CurrentSettings.wasdSensitivity));
        }

        /// <summary>
        /// Applies a delta (already in world XZ space) to the running movement target, constrained and smoothed.
        /// </summary>
        private void ApplyMove(Vector3 delta)
        {
            // Accumulate a target position and clamp within bounds
            _movePos += delta * Time.deltaTime;
            _movePos = MoveConstraints(_movePos);

            // SmoothDamp toward that target; keep the current Y (move is XZ-only)
            var next = Vector3.SmoothDamp(
                _transform.position,
                _movePos,
                ref _moveVelocity,
                _moveSmoothness,         // smoothTime in seconds
                Mathf.Infinity,
                Time.deltaTime
            );

            next.y = _transform.position.y;
            _transform.position = next;
        }

        #endregion

        #region Command Movement (scripted)

        /// <summary>
        /// Coroutine that moves camera smoothly over _moveTime from start to _newMovePosition (XZ only).
        /// Intended for scripted/cinematic moves while IsIdle == true.
        /// </summary>
        private IEnumerator SmoothCameraMovement()
        {
            Vector3 newPos = new Vector3();

            while (_transitionTime / _moveTime < 1f)
            {
                _transitionTime += Time.deltaTime;

                newPos = Vector3.Lerp(
                    _startPosDuringMovement,
                    _newMovePosition,
                    Easings.EaseInOutCubic(Mathf.Clamp01(_transitionTime / _moveTime))
                );

                // Maintain current Y; only move in XZ
                newPos.y = _transform.position.y;
                _movePos = _transform.position;
                _transform.position = newPos;

                yield return null;
            }

            _transform.position = new Vector3(_newMovePosition.x, _transform.position.y, _newMovePosition.z);
        }

        /// <summary>
        /// Starts a scripted move by a world-space delta (XZ) when the camera is idle.
        /// Cancels any prior command-move and begins a new one.
        /// </summary>
        public void MoveCamera(Vector3 moveVec)
        {
            if (!_isIdle) return;

            _startPosDuringMovement = _transform.position;
            _newMovePosition = MoveConstraints(_startPosDuringMovement + moveVec);
            _transitionTime = 0f;

            // NOTE: If _moveRoutine is null the following StopCoroutine would throw.
            // Guard if needed: if (_moveRoutine != null) StopCoroutine(_moveRoutine);
            if (_moveRoutine != null) StopCoroutine(_moveRoutine);
            _moveRoutine = StartCoroutine(SmoothCameraMovement());
        }

        /// <summary>
        /// Adjusts desired zoom height immediately by an integer step (when idle).
        /// The actual height is still smoothed in Zoom().
        /// </summary>
        public void ZoomCamera(int zoomFactor)
        {
            if (_isIdle)
            {
                _scrollPosition += zoomFactor;
                _scrollPosition = Mathf.Clamp(_scrollPosition, _minCameraHeight, _maxCameraHeight);
            }
        }

        /// <summary>
        /// Resets camera to its StartPosition and a default height. Clears smoothing velocities.
        /// </summary>
        public void ResetCamera()
        {
            if (!_isIdle)
                return;

            _newMovePosition = StartPosition;
            _scrollPosition = 20f;
            _transform.position = StartPosition;

            // Reset smoothing velocities so there�s no residual motion after a reset
            _panVelocity = Vector3.zero;
            _moveVelocity = Vector3.zero;
            _zoomVelocity = 0f;
        }

        #endregion

        #region Helpers & Input Plumbing

        /// <summary>
        /// Converts current mouse position to an edge direction (if within edge bands).
        /// Returns true if any edge band is active.
        /// </summary>
        private bool TryGetEdgeInput(out Vector2 dir)
        {
            dir = Vector2.zero;
            var mp = _playerInput.BasicControls.MousePosition.ReadValue<Vector2>();

            if (mp.x > Screen.width - _edgeSize) dir.x = 1f;
            else if (mp.x < _edgeSize) dir.x = -1f;

            if (mp.y > Screen.height - _edgeSize) dir.y = 1f;
            else if (mp.y < _edgeSize) dir.y = -1f;

            return dir != Vector2.zero;
        }

        /// <summary>
        /// Constrains an XZ world position within the configured bounds.
        /// </summary>
        private Vector3 MoveConstraints(Vector3 p)
        {
            float x = Mathf.Clamp(p.x, _minimumCameraPosition.x, _maximumCameraPosition.x);
            float z = Mathf.Clamp(p.z, _minimumCameraPosition.y, _maximumCameraPosition.y);
            return new Vector3(x, p.y, z);
        }

        /// <summary>
        /// Optional: raycasts under the cursor to set a "target" (if you wire an input event to call this).
        /// Demonstrates SelectionProcessor usage but is not required for camera motion.
        /// </summary>
        private void SetTarget(Data.SharedTypes.InputButton button)
        {
            Debug.Log("Clicked");
            if (Physics.Raycast(_camera.ScreenPointToRay(_playerInputProcessor.MousePosition), out RaycastHit hitInfo, float.MaxValue))
            {
                SelectableObject obj = hitInfo.transform.GetComponentInChildren<SelectableObject>();
                if (obj != null)
                {
                    _target = obj;
                    _selectionProcessor.InvokeObjectSelected(_target, _target.Data);
                }
                else
                {
                    _selectionProcessor.HideUI();
                }
            }
        }

        /// <summary>
        /// Mouse wheel callback ? caches scroll delta for Zoom().
        /// Ignored while idle or when cursor is over UI, or if mouse controls are disabled.
        /// </summary>
        private void UpdateScrollWheelInput(float value)
        {
            if (!_isIdle && !WorldUtils.IsPointerOverUI(EventSystem.current) && CurrentSettings.mouseControls)
            {
                _scrollWheelInput = -value;
            }
        }

        #endregion

        #region Unity Lifecycle

        private void Awake()
        {
            _playerInput = new PlayerInput();

            if (!TryGetComponent(out _transform))
                Debug.LogError("CameraController: missing transform component " + this);

            if (!TryGetComponent(out _camera))
                Debug.LogError("CameraController: missing camera component " + this);

            // Validate constraint setup
            if (_maxCameraHeight - _minCameraHeight < 0)
                Debug.LogError("CameraController: MaxCameraHeight is lower than MinCameraHeight " + this);

            if (_maximumCameraPosition.x - _minimumCameraPosition.x < 0)
                Debug.LogError("CameraController: MaximumCameraPosition.x is lower than MinimumCamerPosition.x " + this);

            if (_maximumCameraPosition.y - _minimumCameraPosition.y < 0)
                Debug.LogError("CameraController: MaximumCameraPosition.y is lower than MinimumCamerPosition.y " + this);

            StartPosition = transform.position;

            // Keyboard movement input wiring (project-specific input action names)
            _playerInput.BasicControls.KeyboardMovement.performed += ctx => _keyboardInput = ctx.ReadValue<Vector2>();
            _playerInput.BasicControls.KeyboardMovement.canceled += ctx => _keyboardInput = Vector2.zero;

            // Initialize move targets to current position
            _newMovePosition = transform.position;
            _scrollPosition = 15.0f;
            _movePos = transform.position;
            _previousMousePosition = _playerInput.BasicControls.MousePosition.ReadValue<Vector2>();

            // Start disabled; will be enabled when the game signals "ready"
            this.enabled = false;
            _gameStateProcessor.ReadiedPlayer += EnableSelf;
        }

        private void OnDestroy()
        {
            _gameStateProcessor.ReadiedPlayer -= EnableSelf;
        }

        /// <summary>
        /// One-shot callback from GameStateProcessor when the playable state is ready.
        /// </summary>
        private void EnableSelf()
        {
            this.enabled = true;
            _gameStateProcessor.ReadiedPlayer -= EnableSelf; // ensure one-shot
        }

        private void OnEnable()
        {
            _playerInputProcessor.OnMouseScroll += UpdateScrollWheelInput;
            _playerInput.Enable();
        }

        private void OnDisable()
        {
            _playerInputProcessor.OnMouseScroll -= UpdateScrollWheelInput;
            _playerInput.Disable();
        }

        private void Update()
        {
            UpdateCamera();
        }

        #endregion
    }
}

