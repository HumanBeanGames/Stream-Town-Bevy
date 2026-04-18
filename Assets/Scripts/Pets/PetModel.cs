using Pets.Enumerations;
using UnityEngine;

namespace Pets 
{
    /// <summary>
    /// Represents the visual model of a pet.
    /// </summary>
    public class PetModel : MonoBehaviour
	{
        /// <summary>
        /// The pet type.
        /// </summary>
		[SerializeField]
		private PetType _petType;

        /// <summary>
        /// The animator.
        /// </summary>
        private Animator _animator;

        /// <summary>
        /// The move speed hash.
        /// </summary>
		private static int _moveSpeedHash = Animator.StringToHash("MoveSpeed");

        /// <summary>
        /// Gets the pet type.
        /// </summary>
		public PetType PetType => _petType;

        /// <summary>
        /// Gets whether the pet model has an animator.
        /// </summary>
		public bool HasAnimator => _animator != null;

        /// <summary>
        /// Initializes the pet model.
        /// </summary>
		private void Awake()
		{
			TryGetComponent(out _animator);
		}

        /// <summary>
        /// Sets an animation trigger.
        /// </summary>
        /// <param name="trigger">The trigger name.</param>
		public void SetAnimationTrigger(string trigger)
		{
			_animator.SetTrigger(trigger);
		}

        /// <summary>
        /// Sets the movement speed for the animator.
        /// </summary>
        /// <param name="speed">The movement speed.</param>
		public void SetMovementSpeed(float speed)
		{
			if (!HasAnimator)
				return;

			_animator.SetFloat(_moveSpeedHash, speed);
		}
	}
}
