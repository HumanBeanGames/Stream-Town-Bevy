using Character;
using GUIDSystem;
using Target;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects
{
    /// <summary>
    /// References used by SaveProcessor to capture and restore a player.
    /// </summary>
	public class SaveablePlayer : SaveableObject
	{
        /// <summary>
        /// The role handler.
        /// </summary>
		public RoleHandler RoleHandler;

        /// <summary>
        /// Sets the player variables.
        /// </summary>
        /// <param name="target">The targetable object.</param>
        /// <param name="component">The GUID component.</param>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
        /// <param name="roleHandler">The role handler.</param>
		public void SetVariables(Targetable target, GUIDComponent component, string poolName, PoolableObject poolableObject, RoleHandler roleHandler )
		{
			RoleHandler = roleHandler;
			base.SetVariables(target, component, poolName, poolableObject);
		}
	}
}
