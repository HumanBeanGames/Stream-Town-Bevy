using Character;
using Pets.Enumerations;
using System.Collections.Generic;
using UnityEngine;
using Utils;

namespace Pets
{
    /// <summary>
    /// Represents a pet that follows the player.
    /// </summary>
	public class Pet : MonoBehaviour
	{
        /// <summary>
        /// The closest distance to the player.
        /// </summary>
		[SerializeField]
		private float _closestDistanceToPlayer = 1.0f;

        /// <summary>
        /// The maximum distance from the player.
        /// </summary>
		[SerializeField]
		private float _maxDistanceFromPlayer = 5.0f;

        /// <summary>
        /// The minimum move speed.
        /// </summary>
		[SerializeField]
		private float _minMoveSpeed = 0.5f;

        /// <summary>
        /// The maximum move speed.
        /// </summary>
		[SerializeField]
		private float _maxMoveSpeed = 10.0f;

        /// <summary>
        /// The rotation speed.
        /// </summary>
		[SerializeField]
		private float _rotationSpeed = 5.0f;

        /// <summary>
        /// The squared closest distance.
        /// </summary>
		private float _closestDistanceSqrd;

        /// <summary>
        /// The squared maximum distance.
        /// </summary>
		private float _maxDistanceSqrd;

        /// <summary>
        /// The active pet type.
        /// </summary>
		private PetType _activePetType;

        /// <summary>
        /// Dictionary of pet models.
        /// </summary>
		private Dictionary<PetType, PetModel> _petModels = new Dictionary<PetType, PetModel>();

        /// <summary>
        /// The active pet model.
        /// </summary>
		private PetModel _activePetModel;

        /// <summary>
        /// The owner player.
        /// </summary>
		[SerializeField]
		private Player _owner;

        /// <summary>
        /// The owner transform.
        /// </summary>
		private Transform _ownerTransform;

        /// <summary>
        /// Gets the active pet type.
        /// </summary>
		public PetType ActivePetType => _activePetType;

        /// <summary>
        /// Gets the active pet model.
        /// </summary>
		public PetModel ActivePet => _activePetModel;

        /// <summary>
        /// Whether the pet is active.
        /// </summary>
		public bool IsActive;

        /// <summary>
        /// Sets the owner of the pet.
        /// </summary>
        /// <param name="owner">The owner transform.</param>
        /// <param name="player">The owner player.</param>
		public void SetOwner(Transform owner, Player player)
		{
			_owner = player;
			_ownerTransform = owner;
			transform.position = _ownerTransform.position;
		}

        /// <summary>
        /// Activates the pet.
        /// </summary>
		public void ActivatePet()
		{
			gameObject.SetActive(true);
			IsActive = true;
		}

        /// <summary>
        /// Deactivates the pet.
        /// </summary>
		public void DeactivatePet()
		{
			gameObject.SetActive(false);
			IsActive = false;
		}

        /// <summary>
        /// Tries to set the active pet type.
        /// </summary>
        /// <param name="petType">The pet type to activate.</param>
		public void TrySetActivePet(PetType petType)
		{
			if (petType == PetType.None)
			{
				if (_activePetModel != null)
					_activePetModel.gameObject.SetActive(false);
				_activePetModel = _petModels[petType];
				_activePetType = PetType.None;
			}
			else if (_petModels.ContainsKey(petType))
			{
				if (_activePetModel != null)
					_activePetModel.gameObject.SetActive(false);
				_activePetModel = _petModels[petType];
				_activePetModel.gameObject.SetActive(true);
				_activePetType = petType;
				IsActive = true;
			}
		}

        /// <summary>
        /// Updates the pet movement.
        /// </summary>
		private void Update()
		{
			if (_ownerTransform == null)
				return;

			Vector3 dir = _ownerTransform.position - transform.position;

			float sqDist = dir.sqrMagnitude;
			Vector3 lookDir = dir;
			lookDir.y = 0;
			dir.Normalize();

			float scalar = MathExtended.RemapValue(sqDist, _closestDistanceSqrd, _maxDistanceSqrd, _minMoveSpeed, _maxMoveSpeed);
			if (scalar < _minMoveSpeed)
				scalar = _minMoveSpeed;

			if (scalar > _maxMoveSpeed)
				scalar = _maxMoveSpeed;
			transform.position += dir * scalar * Time.deltaTime;

			if (_activePetModel && _activePetModel.HasAnimator)
				_activePetModel.SetMovementSpeed(scalar);

			if (lookDir == Vector3.zero)
				return;

			Quaternion rotation = Quaternion.LookRotation(lookDir);
			transform.rotation = Quaternion.Slerp(transform.rotation, rotation, Time.deltaTime * _rotationSpeed);


		}

        /// <summary>
        /// Initializes the pet.
        /// </summary>
		private void Awake()
		{
			_closestDistanceSqrd = _closestDistanceToPlayer * _closestDistanceToPlayer;
			_maxDistanceSqrd = _maxDistanceFromPlayer * _maxDistanceFromPlayer;

			for (int i = 0; i < transform.childCount; i++)
			{
				GameObject go = transform.GetChild(i).gameObject;

				if (go.TryGetComponent(out PetModel petModel))
				{
					_petModels.Add(petModel.PetType, petModel);
				}

				go.SetActive(false);
			}
			_petModels.Add(PetType.None, null);
		}
	}
}
