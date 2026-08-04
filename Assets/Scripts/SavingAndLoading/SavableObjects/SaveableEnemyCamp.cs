using GUIDSystem;
using Target;
using Units;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects 
{
    /// <summary>
    /// References used by SaveProcessor to capture and restore an enemy camp.
    /// </summary>
    public class SaveableEnemyCamp : SaveableObject
	{
        /// <summary>
        /// The health handler.
        /// </summary>
		public HealthHandler HealthHandler;

        /// <summary>
        /// Sets the enemy camp variables.
        /// </summary>
        /// <param name="target">The targetable object.</param>
        /// <param name="component">The GUID component.</param>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
        /// <param name="healthHandler">The health handler.</param>
		public void SetVariables(Targetable target, GUIDComponent component, string poolName, PoolableObject poolableObject, HealthHandler healthHandler)
		{
			HealthHandler = healthHandler;
			base.SetVariables(target, component, poolName, poolableObject);
		}

	}
}
