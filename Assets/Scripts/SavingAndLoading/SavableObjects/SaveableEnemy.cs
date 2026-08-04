using Enemies;
using GUIDSystem;
using Target;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects 
{
    /// <summary>
    /// References used by SaveProcessor to capture and restore an enemy.
    /// </summary>
    public class SaveableEnemy : SaveableObject
	{
        /// <summary>
        /// Gets or sets the enemy.
        /// </summary>
        public Enemy Enemy { get; set; }

        /// <summary>
        /// Sets the enemy variables.
        /// </summary>
        /// <param name="target">The targetable object.</param>
        /// <param name="component">The GUID component.</param>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
        /// <param name="enemy">The enemy.</param>
		public void SetVariables(Targetable target, GUIDComponent component, string poolName, PoolableObject poolableObject, Enemy enemy)
		{
			Enemy = enemy;
			base.SetVariables(target, component, poolName, poolableObject);
		}
	}
}
