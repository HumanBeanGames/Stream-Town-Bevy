using Buildings;
using GUIDSystem;
using Target;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects
{
    /// <summary>
    /// References used by SaveProcessor to capture and restore a building.
    /// </summary>
	public class SaveableBuilding : SaveableObject
	{
        /// <summary>
        /// The building base.
        /// </summary>
		public BuildingBase BuildingBase;

        /// <summary>
        /// Sets the building variables.
        /// </summary>
        /// <param name="target">The targetable object.</param>
        /// <param name="component">The GUID component.</param>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
        /// <param name="buildingBase">The building base.</param>
		public void SetVariables(Targetable target, GUIDComponent component, string poolName, PoolableObject poolableObject, BuildingBase buildingBase)
		{
			BuildingBase = buildingBase;
			base.SetVariables(target, component, poolName, poolableObject);
		}
	}
}
