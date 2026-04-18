using GUIDSystem;
using Processors;
using Reflex.Attributes;
using SavingAndLoading.Structs;
using Target;
using Units;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects 
{
    /// <summary>
    /// Handles saving and loading for enemy camps.
    /// </summary>
    public class SaveableEnemyCamp : SaveableObject
	{
        /// <summary>
        /// The health handler.
        /// </summary>
		public HealthHandler HealthHandler;

        /// <summary>
        /// The GUID processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GUIDProcessor _guidProcessor;

        /// <summary>
        /// Saves the enemy camp data.
        /// </summary>
        /// <returns>The enemy camp save data.</returns>
		public override object SaveData()
		{
			return (object)new EnemyCampSaveData(HealthHandler.transform, HealthHandler.Health, GUIDComponent.GUID);
		}

        /// <summary>
        /// Loads the enemy camp data.
        /// </summary>
        /// <param name="data">The enemy camp save data.</param>
		public override void LoadData(object data)
		{
			EnemyCampSaveData enemyCampData = (EnemyCampSaveData)data;
			HealthHandler.transform.position = Vector3SaveData.ToUnityVec3(enemyCampData.Transform.Position);
			HealthHandler.transform.eulerAngles = Vector3SaveData.ToUnityVec3(enemyCampData.Transform.Rotation);
			HealthHandler.transform.localScale = Vector3SaveData.ToUnityVec3(enemyCampData.Transform.LossyScale);
			HealthHandler.gameObject.SetActive(true);
			HealthHandler.SetHealth(enemyCampData.Health);

			GUIDComponent.SetGUID(enemyCampData.GUID);
			_guidProcessor.AddToDictionary(PoolableObject);
		}

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
