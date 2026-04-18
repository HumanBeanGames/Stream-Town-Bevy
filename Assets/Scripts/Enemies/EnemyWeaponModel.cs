using Animation;
using UnityEngine;
using Utils;

namespace Enemies
{
	/// <summary>
	/// Handles an enemy character's weapon model.
	/// </summary>
	[System.Serializable]
	public class EnemyWeaponModel
	{
        /// <summary>
        /// The main weapon model GameObject.
        /// </summary>
		[SerializeField]
		private GameObject _mainWeaponModel;

        /// <summary>
        /// Array of off-hand weapon model GameObjects.
        /// </summary>
		[SerializeField]
		private GameObject[] _offHandModels;

        /// <summary>
        /// The weapon animation name.
        /// </summary>
		[SerializeField]
		private AnimationName _weaponAnimationName = AnimationName.GenericAction;

        /// <summary>
        /// The number of animation variants.
        /// </summary>
		[SerializeField]
		private int _animationVariants = 1;

        /// <summary>
        /// The run animation type.
        /// </summary>
		[SerializeField]
		private RunAnimation _runAnimation = RunAnimation.Generic;

        /// <summary>
        /// The animation handler.
        /// </summary>
		private AnimationHandler _animationHandler;

        /// <summary>
        /// Gets the run animation type.
        /// </summary>
		public RunAnimation RunAnimation => _runAnimation;

        /// <summary>
        /// Gets the number of animation variants.
        /// </summary>
		public int AnimationVariants => _animationVariants;

        /// <summary>
        /// Gets the weapon animation name.
        /// </summary>
		public AnimationName WeaponAnimation => _weaponAnimationName;

        /// <summary>
        /// Sets up the reference for the Animation Handler.
        /// </summary>
        /// <param name="animationHandler">The animation handler.</param>
		public void SetAnimationHandler(AnimationHandler animationHandler)
		{
			_animationHandler = animationHandler;
		}

        /// <summary>
        /// Activates or deactivates the weapon models.
        /// </summary>
        /// <param name="value">Whether to activate the weapon models.</param>
		public void SetActive(bool value)
		{
			if (value)
				Activate();
			else
				Deactivate();
		}

        /// <summary>
        /// Enables the weapon models.
        /// </summary>
		private void Activate()
		{
			_mainWeaponModel.SetActive(true);
			if (_offHandModels != null)
			{
				for (int i = 0; i < _offHandModels.Length; i++)
				{
					_offHandModels[i].SetActive(true);
				}
			}
			_animationHandler.SetRunAnimationIndex((int)_runAnimation);
		}

        /// <summary>
        /// Disables the weapon models.
        /// </summary>
		private void Deactivate()
		{
			_mainWeaponModel.SetActive(false);
			if (_offHandModels != null)
			{
				for (int i = 0; i < _offHandModels.Length; i++)
				{
					_offHandModels[i].SetActive(false);
				}
			}
		}
	}
}
