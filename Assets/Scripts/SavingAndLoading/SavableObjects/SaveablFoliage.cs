using Utils.Pooling;

namespace SavingAndLoading.SavableObjects 
{
    /// <summary>
    /// References used by SaveProcessor for legacy pooled foliage descriptors.
    /// </summary>
    public class SaveablFoliage : SaveableObject
	{
        /// <summary>
        /// Sets the foliage variables.
        /// </summary>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
		public void SetVariables(string poolName, PoolableObject poolableObject)
		{
			PoolName = poolName;
			PoolableObject = poolableObject;
		}
	}
}
