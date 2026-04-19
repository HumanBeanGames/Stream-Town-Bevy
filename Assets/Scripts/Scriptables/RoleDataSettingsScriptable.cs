using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores configuration data for a specific player role.
	/// Contains role type, stats, scaling values, and resource requirements.
	/// </summary>
	[CreateAssetMenu(fileName = "RoleDataSettings", menuName = "ScriptableObjects/RoleDataSettings", order = 1)]
	public class RoleDataSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// The specific role this data represents.
		/// Used for role identification and lookup.
		/// </summary>
		public PlayerRole Role;

		/// <summary>
		/// Flags indicating the type/category of the role.
		/// Used for role grouping and filtering.
		/// </summary>
		public PlayerRoleType RoleFlags;

		/// <summary>
		/// Flags indicating what targets this role can interact with.
		/// Used for targeting logic and interaction validation.
		/// </summary>
		public TargetMask TargetFlags;

		/// <summary>
		/// The resource type this role produces or consumes.
		/// Set to None if the role doesn't handle resources.
		/// </summary>
		public Utils.Resource Resource = Utils.Resource.None;

		/// <summary>
		/// Animation to play when the role performs its action.
		/// Used for visual feedback during role activities.
		/// </summary>
		public AnimationName ActionAnimationName;

		/// <summary>
		/// Number of animation variants available for the action.
		/// Used to add variety to role animations.
		/// </summary>
		public int ActionAnimationVariants = 1;

		/// <summary>
		/// Flags indicating which stations this role can use.
		/// Used for station assignment and interaction.
		/// </summary>
		public StationMask StationFlags;

		/// <summary>
		/// Base amount of resource produced/consumed per action.
		/// Scales with level and global modifiers.
		/// </summary>
		public int BaseActionAmount;

		/// <summary>
		/// Base speed at which the role performs actions.
		/// Higher values mean faster action completion.
		/// </summary>
		public float BaseActionSpeed;

		/// <summary>
		/// Base range for role actions.
		/// Determines how far the character can interact.
		/// </summary>
		public float BaseActionRange;

		/// <summary>
		/// Starting level for this role.
		/// Used for initial role strength balancing.
		/// </summary>
		public int BaseRoleLevel;

		/// <summary>
		/// Base food upkeep cost for this role.
		/// Consumed per game tick or time period.
		/// </summary>
		public float BaseFoodUpkeep;

		/// <summary>
		/// Base gold upkeep cost for this role.
		/// Consumed per game tick or time period.
		/// </summary>
		public float BaseGoldUpkeep;

		/// <summary>
		/// Base health points for this role.
		/// Starting health value at level 1.
		/// </summary>
		public int BaseHealth;

		/// <summary>
		/// Base health regeneration per tick.
		/// Amount of health recovered over time.
		/// </summary>
		public int BaseHealthRegen;

		/// <summary>
		/// Base damage reduction percentage.
		/// Reduces incoming damage by this amount.
		/// </summary>
		public int BaseDamageReduction;

		/// <summary>
		/// Base movement speed for this role.
		/// Determines how fast the character moves.
		/// </summary>
		public int BaseMovementSpeed;

		/// <summary>
		/// Base maximum resource capacity for this role.
		/// Maximum amount of resources the character can carry.
		/// </summary>
		public int BaseMaxResource;

		/// <summary>
		/// Additional max resource capacity per level.
		/// Scales resource capacity with role level.
		/// </summary>
		public float MaxResourcePerLevel;

		/// <summary>
		/// Additional movement speed per level.
		/// Scales movement speed with role level.
		/// </summary>
		public float MovementSpeedPerLevel;

		/// <summary>
		/// Additional action amount per level.
		/// Scales resource production/consumption with role level.
		/// </summary>
		public float ActionAmountPerLevel;

		/// <summary>
		/// Additional action speed per level.
		/// Scales action speed with role level.
		/// </summary>
		public float ActionSpeedPerLevel;

		/// <summary>
		/// Additional action range per level.
		/// Scales interaction range with role level.
		/// </summary>
		public float ActionRangePerLevel;

		/// <summary>
		/// Additional health per level.
		/// Scales health with role level.
		/// </summary>
		public float HealthPerLevel;

		/// <summary>
		/// Additional health regeneration per level.
		/// Scales health regen with role level.
		/// </summary>
		public float HealthRegenPerLevel;

		/// <summary>
		/// Additional damage reduction per level.
		/// Scales damage reduction with role level.
		/// </summary>
		public float DamageReductionPerLevel;

		/// <summary>
		/// Global action amount increase per level.
		/// Applied to all roles when leveled up.
		/// </summary>
		public float GlobalActionPerLevel;

		/// <summary>
		/// Global action speed increase per level.
		/// Applied to all roles when leveled up.
		/// </summary>
		public float GlobalActionSpeedPerLevel;

		/// <summary>
		/// Global action range increase per level.
		/// Applied to all roles when leveled up.
		/// </summary>
		public float GlobalActionRangePerLevel;

		/// <summary>
		/// Global movement speed increase per level.
		/// Applied to all roles when leveled up.
		/// </summary>
		public float GlobalMovementSpeedPerLevel;

		/// <summary>
		/// Global health increase per level.
		/// Applied to all roles when leveled up.
		/// </summary>
		public float GlobalHealthPerLevel;

		/// <summary>
		/// Global health regeneration increase per level.
		/// Applied to all roles when leveled up.
		/// </summary>
		public float GlobalHealthRegenPerLevel;

		/// <summary>
		/// Global resource capacity increase per level.
		/// Applied to all roles when leveled up.
		/// </summary>
		public float GlobalResourceCarryPerLevel;

		/// <summary>
		/// Global damage reduction increase per level.
		/// Applied to all roles when leveled up.
		/// </summary>
		public float GlobalDamageReductionPerLevel;

		/// <summary>
		/// Whether this role has a user limit.
		/// If true, the number of players with this role is restricted.
		/// </summary>
		public bool HasUserLimit;

		/// <summary>
		/// Base maximum number of players allowed for this role.
		/// Only applies if HasUserLimit is true.
		/// </summary>
		public int BaseMaxUserLimit;

		/// <summary>
		/// Experience gain modifier for this role.
		/// Multiplier applied to experience earned.
		/// </summary>
		public float ExpModifier = 1;

		/// <summary>
		/// Icon displayed for this role in the UI.
		/// Used for role identification in menus and HUD.
		/// </summary>
		public Sprite DisplayIcon;

		/// <summary>
		/// Audio clips played when the role performs actions.
		/// Randomly selected to add audio variety.
		/// </summary>
		public AudioClip[] ActionClips;
	}
}
