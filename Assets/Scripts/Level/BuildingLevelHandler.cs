using Buildings;
using Processors;
using Reflex.Attributes;
using System;

namespace Level
{
	/// <summary>
	/// Handles the leveling for any building unit.
	/// </summary>
	public class BuildingLevelHandler : LevelHandler
	{
        /// <summary>
        /// The building processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] protected BuildingProcessor _buildingProcessor;

        /// <summary>
        /// The building base component.
        /// </summary>
		protected BuildingBase _buildingBase;

        /// <summary>
        /// Event triggered when the building levels up.
        /// </summary>
		public event Action<LevelHandler> OnLeveledUp;

        /// <summary>
        /// Checks if the building can level up.
        /// </summary>
        /// <returns>True if the building can level up.</returns>
		public override bool CanLevel()
		{
			return CanLevel();
		}

		/// <summary>
		/// Returns true if the building can level up.
		/// </summary>
        /// <param name="skipCostCheck">Whether to skip the cost check.</param>
        /// <returns>True if the building can level up.</returns>
		public bool CanLevel(bool skipCostCheck = false)
		{
			if (base.CanLevel() && (skipCostCheck || _buildingProcessor.CanAffordToLevel(_buildingBase.BuildingType, _currentLevel)))
			{
				return true;
			}

			return false;
		}

		/// <summary>
		/// Called when building levels up.
		/// </summary>
		public override void OnLevelUp()
		{
			_buildingProcessor.OnLevelBuilding(_buildingBase.BuildingType, _currentLevel);
			base.OnLevelUp();
			OnLeveledUp?.Invoke(this);
		}

		/// <summary>
		/// Initializes all required references and data.
		/// </summary>
		protected override void Init()
		{
			base.Init();

			_buildingBase = GetComponent<BuildingBase>();
		}
	}
}
