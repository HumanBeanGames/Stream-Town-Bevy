using GameResources;
using GUIDSystem;
using Target;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects
{
    /// <summary>
    /// References used by SaveProcessor to capture and restore a pooled resource.
    /// </summary>
	public class SaveableResource : SaveableObject
	{
        /// <summary>
        /// The resource holder.
        /// </summary>
		public ResourceHolder ResourceHolder;

        /// <summary>
        /// Sets the resource variables.
        /// </summary>
        /// <param name="target">The targetable object.</param>
        /// <param name="component">The GUID component.</param>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
        /// <param name="resourceHolder">The resource holder.</param>
		public void SetVariables(Targetable target, GUIDComponent component, string poolName, PoolableObject poolableObject, ResourceHolder resourceHolder)
		{
			ResourceHolder = resourceHolder;
			base.SetVariables(target, component, poolName, poolableObject);
		}
	}
}
