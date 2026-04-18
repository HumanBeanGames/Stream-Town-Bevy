using UnityEngine;
using UnityEngine.Events;

namespace Level
{
	/// <summary>
	/// Base Level Handler class that handles base functionality of anything that can level up.
	/// </summary>
	public class LevelHandler : MonoBehaviour
	{
        /// <summary>
        /// The current level.
        /// </summary>
		[SerializeField]
		protected int _currentLevel = 1;

        /// <summary>
        /// The maximum level.
        /// </summary>
		[SerializeField]
		protected int _maxLevel = 10;

        /// <summary>
        /// Unity event triggered on level up.
        /// </summary>
		[SerializeField]
		protected UnityEvent _onLevelUp;

        /// <summary>
        /// Gets the current level.
        /// </summary>
		public int Level => _currentLevel;

        /// <summary>
        /// Gets or sets the maximum level.
        /// </summary>
		public int MaxLevel
		{
			get { return _maxLevel; }
			set { _maxLevel = value; }
		}

		/// <summary>
		/// Called when leveling up.
		/// </summary>
		public virtual void OnLevelUp()
		{
			if (_currentLevel >= _maxLevel)
				return;

			_currentLevel++;
			_onLevelUp.Invoke();
		}

		/// <summary>
		/// Returns true if leveling is possible.
		/// </summary>
        /// <returns>True if leveling is possible.</returns>
		public virtual bool CanLevel()
		{
			if (_currentLevel < _maxLevel)
				return true;
			else
				return false;
		}

		/// <summary>
		/// Attempts to level up and returns the result.
		/// </summary>
        /// <returns>True if the level up was successful.</returns>
		public virtual bool TryLevel()
		{
			if (!CanLevel())
				return false;

			OnLevelUp();

			return true;
		}

        /// <summary>
        /// Initializes the level handler.
        /// </summary>
		protected virtual void Init()
		{

		}

		// Unity Functions.
        /// <summary>
        /// Initializes the level handler on awake.
        /// </summary>
		private void Awake()
		{
			Init();
		}

        /// <summary>
        /// Resets the level when disabled.
        /// </summary>
		private void OnDisable()
		{
			_currentLevel = 1;
		}
	}
}
