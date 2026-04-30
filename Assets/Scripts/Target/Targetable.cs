using GridSystem.Partitioning;
using GUIDSystem;
using Processors;
using UnityEngine;
using UserInterface;
using Utils;
using Utils.Pooling;
using Reflex.Attributes;

namespace Target
{
	/// <summary>
	/// Base class for all Targetable objects in the game
	/// </summary>
	public class Targetable : MonoBehaviour, Utils.Pooling.IPooledObjectReset
	{
        /// <summary>
        /// The target mask.
        /// </summary>
		[SerializeField]
		protected TargetMask _targetType;

        /// <summary>
        /// Whether to update the partition index.
        /// </summary>
		[SerializeField]
		protected bool _updatePartitionIndex = false;

        /// <summary>
        /// The partition update rate.
        /// </summary>
		[SerializeField]
		protected float _partitionUpdateRate = 3f;

        /// <summary>
        /// The partition update time.
        /// </summary>
		protected float _partitionUpdateTime = 0;

        /// <summary>
        /// The cell space partitioning. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] protected CellSpacePartitioning _cellSpacePartition;

        /// <summary>
        /// The target processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] protected TargetProcessor _targetProcessor;

        /// <summary>
        /// The cell index.
        /// </summary>
		protected int _cellIndex = -1;

        /// <summary>
        /// If false, use the box colliders bounds. This is used for determining how close a unit should get to the target.
        /// </summary>
		[SerializeField, Tooltip("If false, use the box colliders bounds. This is used for determining how close a unit should get to the target.")]
		private bool _useCustomSize = false;

        /// <summary>
        /// The custom size.
        /// </summary>
		[SerializeField]
		private float _customSize = 0;

        /// <summary>
        /// The box collider.
        /// </summary>
		protected BoxCollider _boxCollider;

        /// <summary>
        /// The size squared.
        /// </summary>
		protected float _sizeSqr;

        /// <summary>
        /// Cost for each additional unit assigned to this target.
        /// </summary>
		[SerializeField, Tooltip("Cost for each additional unit assigned to this target.")]
		private float _assignmentPenaltyMod = 15;

        /// <summary>
        /// Cost per distance unit.
        /// </summary>
		[SerializeField, Tooltip("Cost per distance unit.")]
		private float _distancePenaltyMod = 0.5f;

        /// <summary>
        /// The current assigned count.
        /// </summary>
		private int _currentAssignedCount = 0;

        /// <summary>
        /// Whether the target was pooled.
        /// </summary>
		protected bool _wasPooled = false;

        /// <summary>
        /// The text display transform.
        /// </summary>
		[SerializeReference]
		private Transform _textDisplayTransform;

        /// <summary>
        /// The cached transform.
        /// </summary>
		protected Transform _transform;

        /// <summary>
        /// The GUID component.
        /// </summary>
		private GUIDComponent _gUIDComponent;

		// Properties
        /// <summary>
        /// Gets the text display transform.
        /// </summary>
		public Transform TextDisplayTransform => _textDisplayTransform;

        /// <summary>
        /// Gets the size squared.
        /// </summary>
		public float SizeSqr => _sizeSqr;

        /// <summary>
        /// Gets the target mask.
        /// </summary>
		public TargetMask TargetType => _targetType;

        /// <summary>
        /// Gets the cached transform.
        /// </summary>
		public Transform CachedTransform => _transform;

        /// <summary>
        /// Gets the GUID component.
        /// </summary>
		public GUIDComponent GUIDComponent => _gUIDComponent;

		/// <summary>
		/// Sets the Target Type.
		/// </summary>
		/// <param name="type">The target mask type.</param>
		public void SetTargetType(TargetMask type)
		{
			RemoveThisTarget();
			_targetType = type;

			// Only add to cell if partition is available (not null and index is not -1)
			// Pooled objects will add to cell in OnReset() after injection has occurred
			if (_cellSpacePartition != null && _cellIndex != -1)
				AddThisTargetToCell();
		}

		/// <summary>
		/// Calculates the score for targeting this object.
		/// </summary>
		/// <param name="position">The position.</param>
		/// <returns>The calculated score.</returns>
		public float CalculateScore(Vector3 position)
		{
			return (Vector3.Distance(position, transform.position) * _distancePenaltyMod) + (_currentAssignedCount * _assignmentPenaltyMod);
		}

		/// <summary>
		/// Increases the assigned units count.
		/// </summary>
		public void AssignToTarget()
		{
			_currentAssignedCount++;
		}

		/// <summary>
		/// Decreases the assigned units count.
		/// </summary>
		public void UnassignFromTarget()
		{
			_currentAssignedCount--;
		}

		/// <summary>
		/// Used for initializing any required data.
		/// </summary>
		protected virtual void Init() { }

		/// <summary>
		/// Adds this target to the cell index of the cell space partition.
		/// </summary>
		protected void AddThisTargetToCell()
		{
			if (_cellSpacePartition == null || _cellIndex == -1)
			{
				Debug.Log($"[Targetable] Cannot add {name} to cell - partition null or index -1");
				return;
			}

			Debug.Log($"[Targetable] Adding {name} (type={_targetType}) to cell {_cellIndex}, position={transform.position}");
			_cellSpacePartition.GetCellAtIndex(_cellIndex).AddTarget(this);
		}

		/// <summary>
		/// Removes this target from the cell index of the cell space partition.
		/// </summary>
		protected void RemoveThisTarget()
		{
			if (_cellSpacePartition == null || _cellIndex == -1)
				return;

			_cellSpacePartition.GetCellAtIndex(_cellIndex).RemoveTarget(this);
		}

		/// <summary>
		/// Calculates and stores the size of the box collider squared.
		/// </summary>
		private void CalculateSizeSquared()
		{
			if (!_useCustomSize && TryGetComponent(out _boxCollider))
			{
				if (_boxCollider.size.x > _boxCollider.size.z)
					_sizeSqr = _boxCollider.size.x;
				else
					_sizeSqr = _boxCollider.size.z;

				_sizeSqr *= _sizeSqr;
			}
			else if (_useCustomSize)
			{
				_sizeSqr = _customSize * _customSize;
			}
			else
				_sizeSqr = 0;
		}

		/// <summary>
		/// Updates the cell index that this target belongs to.
		/// </summary>
		protected void UpdateIndex()
		{
			int newCellIndex = _cellSpacePartition.PositionToIndex(transform.position);

			Debug.Log($"[Targetable] UpdateIndex for {name}: oldIndex={_cellIndex}, newIndex={newCellIndex}, position={transform.position}");

			if (newCellIndex != _cellIndex)
			{
				RemoveThisTarget();
				_cellIndex = newCellIndex;
				AddThisTargetToCell();
			}
		}

		/// <summary>
		/// Checks if enough time has elasped to update which partition and cell index the target belongs to.
		/// </summary>
		protected void CheckUpdatePartitionTime()
		{
			if (!_updatePartitionIndex)
				return;

			_partitionUpdateTime += Time.deltaTime;

			if (_partitionUpdateTime >= _partitionUpdateRate)
			{
				_partitionUpdateTime -= _partitionUpdateRate;
				UpdateIndex();
			}
		}

		// Unity Functions.
        /// <summary>
        /// Checks for partition update time.
        /// </summary>
		public void Update()
		{
			CheckUpdatePartitionTime();
		}

        /// <summary>
        /// Adds this target to the cell and processor when enabled.
        /// Note: Cell addition is deferred to OnReset() to ensure dependency injection has occurred.
        /// </summary>
		protected void OnEnable()
		{
			Debug.Log($"[Targetable] OnEnable for {name}");

			_wasPooled = true;

			// Only add to cell if partition is already available (not a pooled object or injection already occurred)
			// Pooled objects will add to cell in OnReset() after injection
			if (_cellSpacePartition != null && _cellIndex != -1)
				AddThisTargetToCell();

			if (_targetProcessor != null)
				_targetProcessor.AddTarget(this);
		}

        /// <summary>
        /// Removes this target from the cell and processor when disabled.
        /// </summary>
		protected void OnDisable()
		{
			RemoveThisTarget();

			if (_targetProcessor != null)
				_targetProcessor.RemoveTarget(this);
		}

        /// <summary>
        /// Initializes the GUID component and calculates size squared.
        /// </summary>
		private void Awake()
		{
			_gUIDComponent = GetComponent<GUIDComponent>();
			Init();

			CalculateSizeSquared();
		}

        /// <summary>
        /// Start is called during instantiation before injection, so we do nothing here.
        /// </summary>
		protected void Start()
		{
			// Start is called during instantiation before injection, so we do nothing here
			// Use OnReset instead for initialization that depends on injected fields
		}

        /// <summary>
        /// Resets the pooled object.
        /// </summary>
		public void OnReset()
		{
			_transform = transform;
			_cellIndex = _cellSpacePartition.PositionToIndex(transform.position);

			Debug.Log($"[Targetable] OnReset for {name} - partition={(_cellSpacePartition != null)}, cellIndex={_cellIndex}, position={transform.position}, targetType={_targetType}");

			AddThisTargetToCell();
		}
	}
}
