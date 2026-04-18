using GUIDSystem;
using Target;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects 
{
    /// <summary>
    /// Base class for objects that can be saved and loaded.
    /// </summary>
    public class SaveableObject 
	{
        /// <summary>
        /// The targetable object.
        /// </summary>
        public Targetable Target;

        /// <summary>
        /// The GUID component.
        /// </summary>
        public GUIDComponent GUIDComponent;

        /// <summary>
        /// The pool name.
        /// </summary>
        public string PoolName;

        /// <summary>
        /// The poolable object.
        /// </summary>
        public PoolableObject PoolableObject;

        /// <summary>
        /// Saves the object data.
        /// </summary>
        /// <returns>The saved data.</returns>
        public virtual object SaveData() {return new object(); }

        /// <summary>
        /// Loads the object data.
        /// </summary>
        /// <param name="data">The data to load.</param>
        public virtual void LoadData(object data) {}

        /// <summary>
        /// Sets the object variables.
        /// </summary>
        /// <param name="target">The targetable object.</param>
        /// <param name="component">The GUID component.</param>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
        public void SetVariables(Targetable target, GUIDComponent component, string poolName, PoolableObject poolableObject)
        {
            Target = target;
            GUIDComponent = component;
            PoolName = poolName;
            PoolableObject = poolableObject;
        }
    }
}
