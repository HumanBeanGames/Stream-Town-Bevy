using UnityEngine;
using Utils;

namespace Character
{
	/// <summary>
	/// Holds all current role data for a player character's role.
	/// </summary>
	[System.Serializable]
	public class RoleData
	{
        /// <summary>
        /// The name of the role.
        /// </summary>
		public string RoleName;

        /// <summary>
        /// Experience gain modifier for this role.
        /// </summary>
		public float ExpModifier;

        /// <summary>
        /// The player role.
        /// </summary>
		public PlayerRole Role;

        /// <summary>
        /// The role type flags.
        /// </summary>
		public PlayerRoleType RoleFlags;

        /// <summary>
        /// The target flags.
        /// </summary>
		public TargetMask TargetFlags;

        /// <summary>
        /// The resource type.
        /// </summary>
		public Utils.Resource Resource;

        /// <summary>
        /// The action animation name.
        /// </summary>
		public AnimationName ActionAnimationName;

        /// <summary>
        /// The number of action animation variants.
        /// </summary>
		public int ActionAnimationVariants;

        /// <summary>
        /// The station flags.
        /// </summary>
		public StationMask StationFlags;

        /// <summary>
        /// The base action amount.
        /// </summary>
		public int BaseActionAmount;

        /// <summary>
        /// The base action speed.
        /// </summary>
		public float BaseActionSpeed;

        /// <summary>
        /// The base action range.
        /// </summary>
		public float BaseActionRange;

        /// <summary>
        /// The base role level.
        /// </summary>
		public int BaseRoleLevel;

        /// <summary>
        /// The base food upkeep.
        /// </summary>
		public float BaseFoodUpkeep;

        /// <summary>
        /// The base gold upkeep.
        /// </summary>
		public float BaseGoldUpkeep;

        /// <summary>
        /// The base health.
        /// </summary>
		public int BaseHealth;

        /// <summary>
        /// The base health regeneration.
        /// </summary>
		public int BaseHealthRegen;

        /// <summary>
        /// The base damage reduction.
        /// </summary>
		public int BaseDamageReduction;

        /// <summary>
        /// The base movement speed.
        /// </summary>
		public int BaseMovementSpeed;

        /// <summary>
        /// The base maximum resource capacity.
        /// </summary>
		public int BaseMaxResource;

        /// <summary>
        /// The maximum resource capacity per level.
        /// </summary>
		public float MaxResourcePerLevel;

        /// <summary>
        /// The movement speed per level.
        /// </summary>
		public float MovementSpeedPerLevel;

        /// <summary>
        /// The action amount per level.
        /// </summary>
		public float ActionAmountPerLevel;

        /// <summary>
        /// The action speed per level.
        /// </summary>
		public float ActionSpeedPerLevel;

        /// <summary>
        /// The action range per level.
        /// </summary>
		public float ActionRangePerLevel;

        /// <summary>
        /// The health per level.
        /// </summary>
		public float HealthPerLevel;

        /// <summary>
        /// The health regeneration per level.
        /// </summary>
		public float HealthRegenPerLevel;

        /// <summary>
        /// The damage reduction per level.
        /// </summary>
		public float DamageReductionPerLevel;

        /// <summary>
        /// The global action per level.
        /// </summary>
		public float GlobalActionPerLevel;

        /// <summary>
        /// The global action speed per level.
        /// </summary>
		public float GlobalActionSpeedPerLevel;

        /// <summary>
        /// The global action range per level.
        /// </summary>
		public float GlobalActionRangePerLevel;

        /// <summary>
        /// The global movement speed per level.
        /// </summary>
		public float GlobalMovementSpeedPerLevel;

        /// <summary>
        /// The global health per level.
        /// </summary>
		public float GlobalHealthPerLevel;

        /// <summary>
        /// The global health regeneration per level.
        /// </summary>
		public float GlobalHealthRegenPerLevel;

        /// <summary>
        /// The global resource carry per level.
        /// </summary>
		public float GlobalResourceCarryPerLevel;

        /// <summary>
        /// The global damage reduction per level.
        /// </summary>
		public float GlobalDamageReductionPerLevel;

        /// <summary>
        /// Whether the role has a user limit.
        /// </summary>
		public bool HasUserLimit;

        /// <summary>
        /// The base maximum user limit.
        /// </summary>
		public int BaseMaxUserLimit;

        /// <summary>
        /// The current role count.
        /// </summary>
		public int CurrentRoleCount;

        /// <summary>
        /// The display icon.
        /// </summary>
		public Sprite DisplayIcon;

        /// <summary>
        /// The action audio clips.
        /// </summary>
		public AudioClip[] ActionClips;

        /// <summary>
        /// Initializes a new role data instance from a scriptable object.
        /// </summary>
        /// <param name="e">The role data scriptable object.</param>
		public RoleData(ScriptablesProcessorInfrastructure.RoleDataSettings e)
		{
			Role = e.Role;
			RoleFlags = e.RoleFlags;
			TargetFlags = e.TargetFlags;
			StationFlags = e.StationFlags;
			ActionAnimationName = e.ActionAnimationName;
			ActionAnimationVariants = e.ActionAnimationVariants;
			Resource = e.Resource;
			BaseActionAmount = e.BaseActionAmount;
			BaseActionSpeed = e.BaseActionSpeed;
			BaseActionRange = e.BaseActionRange;
			BaseRoleLevel = e.BaseRoleLevel;
			BaseFoodUpkeep = e.BaseFoodUpkeep;
			BaseGoldUpkeep = e.BaseGoldUpkeep;
			BaseMaxResource = e.BaseMaxResource;
			MaxResourcePerLevel = e.MaxResourcePerLevel;
			ActionAmountPerLevel = e.ActionAmountPerLevel;
			ActionSpeedPerLevel = e.ActionSpeedPerLevel;
			ActionRangePerLevel = e.ActionRangePerLevel;
			HealthPerLevel = e.HealthPerLevel;
			HealthRegenPerLevel = e.HealthRegenPerLevel;
			DamageReductionPerLevel = e.DamageReductionPerLevel;
			GlobalActionPerLevel = e.GlobalActionPerLevel;
			GlobalActionSpeedPerLevel = e.GlobalActionSpeedPerLevel;
			GlobalActionRangePerLevel = e.GlobalActionRangePerLevel;
			GlobalMovementSpeedPerLevel = e.GlobalMovementSpeedPerLevel;
			GlobalHealthPerLevel = e.GlobalHealthPerLevel;
			GlobalHealthRegenPerLevel = e.GlobalHealthRegenPerLevel;
			GlobalResourceCarryPerLevel = e.GlobalResourceCarryPerLevel;
			GlobalDamageReductionPerLevel = e.GlobalDamageReductionPerLevel;
			HasUserLimit = e.HasUserLimit;
			BaseMaxUserLimit = e.BaseMaxUserLimit;
			BaseHealth = e.BaseHealth;
			BaseHealthRegen = e.BaseHealthRegen;
			BaseDamageReduction = e.BaseDamageReduction;
			BaseMovementSpeed = e.BaseMovementSpeed;
			MovementSpeedPerLevel = e.MovementSpeedPerLevel;
			CurrentRoleCount = 0;
			RoleName = e.Role.ToString();
			ExpModifier = e.ExpModifier;
			DisplayIcon = e.DisplayIcon;
			ActionClips = e.ActionClips;
	}
	}
}
