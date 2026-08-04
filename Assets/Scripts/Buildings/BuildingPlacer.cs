using Character;
using Processors;
using Pathfinding;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;
using UserInterface;
using Utils;
using Utils.Pooling;
using Reflex.Attributes;

namespace Buildings
{
    /// <summary>
    /// Used for placing new buildings in the world.
    /// </summary>
    public class BuildingPlacer : MonoBehaviour
    {
        /// <summary>
        /// Used as a collision mask against any buildings or obstacles.
        /// </summary>
        [SerializeField]
        private LayerMask _collisionMask;

        /// <summary>
        /// Layer mask for terrain detection.
        /// </summary>
        [SerializeField]
        private LayerMask _terrainMask;

        /// <summary>
        /// Holds all build data for each buildable building type.
        /// </summary>
        public List<BuildPlacerData> _buildData;

        /// <summary>
        /// Current Building Index.
        /// </summary>
        private int _currentIndex = 0;

        /// <summary>
        /// True if placer is colliding with any buildings or obstacles.
        /// </summary>
        private bool _colliding = false;

        /// <summary>
        /// Color displayed on building meshes when placement is valid.
        /// </summary>
        [SerializeField]
        private Color _successColor;

        /// <summary>
        /// Color displayed on building meshes when placement is invalid.
        /// </summary>
        [SerializeField]
        private Color _failColor;

        // Required Components
        /// <summary>
        /// The player that owns this building placer.
        /// </summary>
        private Player _owner;

        /// <summary>
        /// The current building being placed.
        /// </summary>
        private BuildPlacerData _currentBuilding;

        /// <summary>
        /// The box collider used for collision detection.
        /// </summary>
        private BoxCollider _boxCollider;

        /// <summary>
        /// The bounds visualizer used to display the building's bounds.
        /// </summary>
        private BoundsVisualizer _boundsVisualizer;

        /// <summary>
        /// The text display used to show the player's username.
        /// </summary>
        private UnitTextDisplay _textDisplay;

        /// <summary>
        /// The simple cancel building placer used to cancel building placement.
        /// </summary>
        private SimpleCancelBuildingPlacer _simpleCallTimer;

        /// <summary>
        /// The snap to grid mouse movement used to move the building placer.
        /// </summary>
        private SnapToGridMouseMovement _snapMovement;

        /// <summary>
        /// Building processor for building operations.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private BuildingProcessor _buildingProcessor;

        /// <summary>
        /// Object pooling processor for spawning buildings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// Game coordinator for game state access.
        /// Injected via Reflex dependency injection.
        /// </summary>

        /// <summary>
        /// Player runtime scriptable for player data access.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// Foliage processor for foliage management.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Processors.FoliageProcessor _foliageProcessor;

        /// <summary>
        /// Resource processor for resource management.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Processors.ResourceProcessor _resourceProcessor;

        private string _rejectionReason = "";

        /// <summary>
        /// Called when gameobject has been pooled.
        /// </summary>
        /// <param name="player">The player that owns this building placer.</param>
        public void OnPooled(Player player)
        {
            _owner = player;

            //TODO:: Fix this
            if (_textDisplay != null && player.TwitchUser.Username != "")
            {
                if (_playerProcessor.UserPlayer != null && player.TwitchUser.Username == _playerProcessor.UserPlayer.TwitchUser.Username)
                    _textDisplay.SetDisplayText("");
                else
                    _textDisplay.SetDisplayText(player.TwitchUser.Username);
            }

            // Enable snap movement for UserPlayer or debug player (Debugger)
            bool isUserPlayer = _playerProcessor.UserPlayer != null && player.TwitchUser.Username == _playerProcessor.UserPlayer.TwitchUser.Username;
            bool isDebugPlayer = player.TwitchUser.Username == "Debugger";

            if (isUserPlayer || isDebugPlayer)
            {
                _snapMovement.enabled = true;
                _snapMovement.OnPositionChanged += UpdateCollision;
            }
            else
            {
                _snapMovement.enabled = false;
            }

            _simpleCallTimer.SetPlayer(player);
        }

        /// <summary>
        /// Sets the current Building Index to change which building is being placed. <br/>
        /// Activates the model of the building that was indexed.
        /// </summary>
        /// <param name="index"></param>
        public void SetBuildingIndex(int index)
        {
            // Limit index between list bounds.
            if (index < 0)
                index = _buildData.Count - 1;

            if (index >= _buildData.Count)
                index = 0;

            // Hide previous building model.
            _buildData[_currentIndex].BuildingModel.SetActive(false);

            // Show next building model.
            _buildData[index].BuildingModel.SetActive(true);

            // Set current index and building to the new value.
            _currentIndex = index;
            _currentBuilding = _buildData[_currentIndex];

            // Update the bounds visualizer to the new building size.
            _boundsVisualizer.SetSize(_currentBuilding.BuildingSize);

            // Check if Probe Handler is set, otherwise set and store it
            if (_currentBuilding.ProbeProcessor == null)
                _currentBuilding.ProbeProcessor = _currentBuilding.BuildingModel.GetComponentInChildren<PlacementProbeHandler>();

            // Call update on the collision.
            UpdateCollision();
        }

        /// <summary>
        /// Sets the current Building Index by the type of building
        /// </summary>
        /// <param name="type"></param>
        public void SetBuildingByType(BuildingType type)
        {
            // Get the index of the building data by type
            int index = GetBuildingIndex(type);

            // Check that the index was valid
            if (index == -1)
            {
                Debug.LogError($"Attempted to set building to one it should not have been {type}");
            }
            else
                SetBuildingIndex(index);
        }

        /// <summary>
        /// Gets the type of the current building being placed.
        /// </summary>
        /// <returns>The building type.</returns>
        public BuildingType GetBuildingType()
        {
            return _currentBuilding.BuildingType;
        }

        /// <summary>
        /// Returns the building data index by the Building Type. <br/>
        /// Returns -1 if data does not exist.
        /// </summary>
        /// <param name="type"></param>
        /// <returns></returns>
        public int GetBuildingIndex(BuildingType type)
        {
            // Loop through all the building data and return index if available.
            for (int i = 0; i < _buildData.Count; i++)
            {
                if (_buildData[i].BuildingType == type)
                    return i;
            }

            return -1;
        }

        /// <summary>
        /// Moves the building placer by the specified Move Vector.
        /// </summary>
        /// <param name="moveVector"></param>
        public void MovePlacer(Vector3 moveVector)
        {
            transform.position += moveVector;
            UpdateCollision();
        }

        /// <summary>
        /// Rotates the placer on the spot by the rotation in multiples of 90 degrees.
        /// </summary>
        /// <param name="right"></param>
        /// <param name="amount"></param>
        public void RotatePlacer(bool right = true, int amount = 1)
        {
            transform.Rotate(new Vector3(0, (right ? 90 : -90) * amount, 0));
            UpdateCollision();
        }

        /// <summary>
        /// Returns True if building can be afforded.
        /// </summary>
        /// <returns></returns>
        public bool CanAfford()
        {
            return _buildingProcessor.CanAffordToBuild(_currentBuilding.BuildingType);
        }

        /// <summary>
        /// Attemps to spawn the building and returns the placer back to the pool.
        /// </summary>
        /// <param name="placementPos"></param>
        /// <param name="disableOnSpawn"></param>
        /// <returns></returns>
        public bool TrySpawnBuilding(out Vector3 placementPos, out string errorMessage, bool disableOnSpawn = true)
        {
            placementPos = Vector3.zero;

            // Check if building is colliding with an obstalce, if so, return out.
            if (_colliding)
            {
                errorMessage = _rejectionReason;
                return false;
            }

            // Check for resource collision
            if (CheckResourceCollision())
            {
                errorMessage = _rejectionReason;
                return false;
            }

            // Check ground height consistency
            int terrainMask = _terrainMask.value == 0 ? LayerMask.GetMask("Ground") : _terrainMask;
            if (!CheckGroundHeightConsistency(terrainMask))
            {
                errorMessage = _rejectionReason;
                return false;
            }

            // A last check if the building can be afforded, if not, return out.
            //if (!CanAfford())
            //{
            //    errorMessage = "Can't Afford!";
            //    return false;
            //}

            Vector3 alignedPosition = transform.position;
            if (Physics.Raycast(new Vector3(alignedPosition.x, 100, alignedPosition.z), Vector3.down, out RaycastHit terrainHit, 200, terrainMask))
                alignedPosition.y = terrainHit.point.y;
            else
            {
                errorMessage = "No terrain found!";
                return false;
            }

            // Get building from pooling processor and set it's position and rotation.
            PoolableObject obj = _poolingProcessor.GetPooledObject(_currentBuilding.BuildingType.ToString());

            obj.transform.position = alignedPosition;
            obj.transform.rotation = transform.rotation;
            obj.gameObject.SetActive(true);

            // Add building to building dictionary.
            _buildingProcessor.OnBuiltNewBuilding(obj.GetComponent<BuildingBase>());

            // Set player's last placement position to building's position
            placementPos = obj.transform.position;

            errorMessage = "";
            BoxCollider collider = obj.GetComponent<BoxCollider>();
            Vector3 center = obj.transform.position;
            center.y = 0;

            BuildingBase buildingBase = obj.GetComponent<BuildingBase>();
            buildingBase.FoliageRemoved = new List<PoolableObject>();

            // Remove foliage within building area using FoliageProcessor
            RemoveFoliageInArea(collider, center);

            // Return building placer to pool via pooling processor instead of SetActive
            if (_poolingProcessor != null)
            {
                _poolingProcessor.AddToPool("BuildingPlacer", GetComponent<PoolableObject>());
            }
            else
            {
                gameObject.SetActive(false);
            }

            return true;
        }

        /// <summary>
        /// Removes foliage within the specified area when a building is placed.
        /// Uses FoliageProcessor spatial partitioning for efficient removal.
        /// </summary>
        /// <param name="collider">The building's collider defining the area to clear.</param>
        /// <param name="center">The center position of the building.</param>
        private void RemoveFoliageInArea(BoxCollider collider, Vector3 center)
        {
            try
            {
                // Use building footprint size instead of collider bounds for accurate foliage removal
                Vector2 buildingSize = _currentBuilding.BuildingSize;
                Vector3 size = new Vector3(buildingSize.x, 10, buildingSize.y);
                Bounds bounds = new Bounds(center, size);

                // Use FoliageProcessor's efficient spatial partitioning method
                _foliageProcessor.RemoveFoliageInBounds(bounds);
            }
            catch (System.Exception)
            {
                // Silently fail if foliage removal fails
            }
        }

        /// <summary>
        /// Updates the building placer each frame.
        /// Handles left-click confirm for debug player.
        /// </summary>
        private void Update()
        {
            // Allow left-click confirm for debug player only
            if (_owner != null && _owner.TwitchUser.Username == "Debugger")
            {
                if (Mouse.current?.leftButton.wasPressedThisFrame == true)
                {
                    // Check if placer is still active in the processor
                    if (_buildingProcessor != null)
                    {
                        if (_buildingProcessor.TryPlaceBuilding(_owner, out string errorMessage))
                        {
                            // Placement succeeded - the placer should be returned to pool by TryPlaceBuilding
                            // Disable this component to prevent further input processing
                            this.enabled = false;
                        }
                    }
                }
            }
        }

        /// <summary>
        /// Updates the collision of the building placer to determine if it can be placed or not.
        /// </summary>
        public void UpdateCollision()
        {
            if (_currentBuilding.ProbeProcessor == null)
                return;

            if (_currentBuilding == null)
                return;

            int terrainMask = _terrainMask.value == 0 ? LayerMask.GetMask("Ground") : _terrainMask;

            Vector3 alignedPosition = transform.position;
            if (Physics.Raycast(new Vector3(alignedPosition.x, 100, alignedPosition.z), Vector3.down, out RaycastHit terrainHit, 200, terrainMask))
            {
                alignedPosition.y = terrainHit.point.y;
                transform.position = alignedPosition;
            }

            // Get the half extents of the building
            Vector3 halfExtents = Vector3.zero;
            halfExtents.x = _currentBuilding.BuildingSize.x * 0.45f;
            halfExtents.z = _currentBuilding.BuildingSize.y * 0.45f;

            // Box cast from above the building to the ground to see if it hits any obstacles or buildings.
            _colliding = (Physics.BoxCast(transform.position + Vector3.up * 10, halfExtents, -transform.up, transform.rotation, 10, _collisionMask));

            // If we aren't colliding with anything, also check that the building's probes passed their check.
            if (!_colliding && !_currentBuilding.ProbeProcessor.AllProbesPassedCheck())
                _colliding = true;

            // Check for resources within building footprint
            if (!_colliding && _resourceProcessor != null)
            {
                if (CheckResourceCollision())
                    _colliding = true;
            }

            // Check ground height consistency across building footprint
            if (!_colliding)
            {
                if (!CheckGroundHeightConsistency(terrainMask))
                    _colliding = true;
            }

            // Check for collisions with objects
            if (Physics.CheckBox(transform.position, _currentBuilding.BuildingSize / 2, transform.rotation, _collisionMask))
            {
                _colliding = true;
                if (string.IsNullOrEmpty(_rejectionReason))
                    _rejectionReason = "Obstacle in the way";
            }

            // Update text display with rejection reason if colliding
            if (_colliding && !string.IsNullOrEmpty(_rejectionReason) && _textDisplay != null)
            {
                _textDisplay.SetDisplayText(_rejectionReason);
            }
            else if (_textDisplay != null)
            {
                // Show player username when not colliding
                if (_owner != null && _owner.TwitchUser.Username != "")
                {
                    if (_playerProcessor.UserPlayer != null && _owner.TwitchUser.Username == _playerProcessor.UserPlayer.TwitchUser.Username)
                        _textDisplay.SetDisplayText("");
                    else
                        _textDisplay.SetDisplayText(_owner.TwitchUser.Username);
                }
                else
                {
                    _textDisplay.SetDisplayText("");
                }
            }

            // Set colour of the visualizer and buiding to show if there is a collision or not.
            _boundsVisualizer.OnCollisionChange(_colliding, _failColor, _successColor);
            SetBuildingRenderer(_colliding);
            _simpleCallTimer.ResetTimer();
        }

        /// <summary>
        /// Sets the material colour of the building to match whether there is a collision or not.
        /// </summary>
        /// <param name="_colliding">Whether the building is colliding.</param>
        private void SetBuildingRenderer(bool _colliding)
        {
            // Get all renderers
            _currentBuilding.Renderer.material.SetColor("_boundsVisColor", _colliding ? _failColor : _successColor);
        }

        /// <summary>
        /// Checks if any resources are within the building footprint.
        /// </summary>
        /// <returns>True if resources are within bounds, false otherwise.</returns>
        private bool CheckResourceCollision()
        {
            // Get building footprint bounds
            Vector2 buildingSize = _currentBuilding.BuildingSize;
            Vector3 center = transform.position;
            center.y = 0;
            Vector3 size = new Vector3(buildingSize.x, 10, buildingSize.y);
            Bounds bounds = new Bounds(center, size);

            // Use ResourceProcessor API to get resources in bounds
            string blockingResources = _resourceProcessor.GetResourcesInBounds(bounds);

            if (!string.IsNullOrEmpty(blockingResources))
            {
                _rejectionReason = $"Blocked by: {blockingResources}";
                return true;
            }

            return false;
        }

        /// <summary>
        /// Checks if ground height is consistent across the building footprint.
        /// </summary>
        /// <param name="terrainMask">Layer mask for terrain.</param>
        /// <returns>True if height variance is within threshold, false otherwise.</returns>
        private bool CheckGroundHeightConsistency(int terrainMask)
        {
            Vector2 buildingSize = _currentBuilding.BuildingSize;
            Vector3 center = transform.position;

            // Raycast at corners and center of building footprint
            var testPoints = new[]
            {
                center,
                center + new Vector3(buildingSize.x / 2, 0, buildingSize.y / 2),
                center + new Vector3(buildingSize.x / 2, 0, -buildingSize.y / 2),
                center + new Vector3(-buildingSize.x / 2, 0, buildingSize.y / 2),
                center + new Vector3(-buildingSize.x / 2, 0, -buildingSize.y / 2)
            };

            float? firstHeight = null;
            const float heightThreshold = 0.05f;

            foreach (var point in testPoints)
            {
                Vector3 rayOrigin = new Vector3(point.x, 100, point.z);
                if (Physics.Raycast(rayOrigin, Vector3.down, out RaycastHit hit, 200, terrainMask))
                {
                    if (!firstHeight.HasValue)
                    {
                        firstHeight = hit.point.y;
                    }
                    else
                    {
                        float heightDiff = Mathf.Abs(hit.point.y - firstHeight.Value);
                        if (heightDiff > heightThreshold)
                        {
                            _rejectionReason = "Uneven ground";
                            return false;
                        }
                    }
                }
                else
                {
                    // If raycast fails (no ground), consider it invalid
                    _rejectionReason = "No ground";
                    return false;
                }
            }

            return true;
        }

        /// <summary>
        /// Returns the current building's index.
        /// </summary>
        /// <returns>The current building index.</returns>
        public int GetBuildingIndex() => _currentIndex;

        // Unity Functions.
        /// <summary>
        /// Initializes components and sets up the building placer.
        /// </summary>
        public void Awake()
        {
            // Get Components
            _boxCollider = GetComponent<BoxCollider>();
            _boundsVisualizer = GetComponent<BoundsVisualizer>();
            _textDisplay = GetComponentInChildren<UnitTextDisplay>();
            _simpleCallTimer = GetComponent<SimpleCancelBuildingPlacer>();
            _snapMovement = GetComponent<SnapToGridMouseMovement>();

            for (int i = 1; i < _buildData.Count; i++)
            {
                _buildData[i].BuildingModel.SetActive(false);
            }

            _currentBuilding = _buildData[0];
        }

        /// <summary>
        /// Draws gizmos for the building placer.
        /// </summary>
        private void OnDrawGizmos()
        {
            if (_currentBuilding == null)
                return;

            if (_colliding)
                Gizmos.color = _failColor;
            else
                Gizmos.color = _successColor;

            // Draw bounding box of the building.
            Matrix4x4 prevMat = Gizmos.matrix;
            Matrix4x4 newMat = transform.localToWorldMatrix;
            Gizmos.matrix = newMat;
            Gizmos.DrawWireCube(Vector3.zero, new Vector3(_currentBuilding.BuildingSize.x, 1, _currentBuilding.BuildingSize.y));

            // Reset gizmo matrix.
            Gizmos.matrix = prevMat;
        }
    }
}
