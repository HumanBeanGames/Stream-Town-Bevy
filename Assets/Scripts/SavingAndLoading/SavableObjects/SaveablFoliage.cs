using GUIDSystem;
using SavingAndLoading.Structs;
using Target;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects 
{
    /// <summary>
    /// Handles saving and loading for foliage.
    /// </summary>
    public class SaveablFoliage : SaveableObject
	{
        /// <summary>
        /// Saves the foliage data.
        /// </summary>
        /// <returns>The foliage save data.</returns>
		public override object SaveData()
		{
			return (object)new FoliageSaveData(PoolableObject.transform, PoolName);
		}

        /// <summary>
        /// Loads the foliage data.
        /// </summary>
        /// <param name="data">The foliage save data.</param>
		public override void LoadData(object data)
		{
			FoliageSaveData foliageData = (FoliageSaveData)data;
			PoolableObject.transform.position = Vector3SaveData.ToUnityVec3(foliageData.FoliageTransform.Position);
			PoolableObject.transform.eulerAngles = Vector3SaveData.ToUnityVec3(foliageData.FoliageTransform.Rotation);
			PoolableObject.transform.localScale = Vector3SaveData.ToUnityVec3(foliageData.FoliageTransform.LossyScale);
			PoolableObject.gameObject.SetActive(true);
		}

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
