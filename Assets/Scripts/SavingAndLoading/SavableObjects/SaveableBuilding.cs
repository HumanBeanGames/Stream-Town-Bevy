using Buildings;
using GUIDSystem;
using Processors;
using Reflex.Attributes;
using SavingAndLoading.Structs;
using Target;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects
{
    /// <summary>
    /// Handles saving and loading for buildings.
    /// </summary>
	public class SaveableBuilding : SaveableObject
	{
        /// <summary>
        /// The building base.
        /// </summary>
		public BuildingBase BuildingBase;

        /// <summary>
        /// The GUID processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GUIDProcessor _guidProcessor;

        /// <summary>
        /// The building processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private BuildingProcessor _buildingProcessor;

        /// <summary>
        /// Saves the building data.
        /// </summary>
        /// <returns>The building save data.</returns>
		public override object SaveData()
		{
			return (object)new BuildingSaveData(BuildingBase.transform, PoolName, BuildingBase.HealthHandler.Health, GUIDComponent.GUID, BuildingBase.BuildingState, BuildingBase.GetRemovedFoliageData());
		}

        /// <summary>
        /// Loads the building data.
        /// </summary>
        /// <param name="data">The building save data.</param>
		public override void LoadData(object data)
		{
			BuildingSaveData buildingData = (BuildingSaveData)data;
			BuildingBase.HealthHandler.transform.position = Vector3SaveData.ToUnityVec3(buildingData.BuildingTranform.Position);
			BuildingBase.HealthHandler.transform.eulerAngles = Vector3SaveData.ToUnityVec3(buildingData.BuildingTranform.Rotation);
			BuildingBase.HealthHandler.transform.localScale = Vector3SaveData.ToUnityVec3(buildingData.BuildingTranform.LossyScale);
			BuildingBase.HealthHandler.gameObject.SetActive(true);
			BuildingBase.HealthHandler.SetHealth(buildingData.BuildingHealth);
			BuildingBase.BuildingState = buildingData.BuildingState;
			BuildingBase.SetRemovedFoliage(buildingData.DestroyedFoliage);
			GUIDComponent.SetGUID(buildingData.GUID);
			_guidProcessor.AddToDictionary(PoolableObject);
			_buildingProcessor.AddLoadedBuilding(BuildingBase);

			if (BuildingBase.BuildingState == Utils.BuildingState.Building)
				BuildingBase.OnLoadedBuiltBuilding();

			if (BuildingBase.DamageHandler != null)
				BuildingBase.DamageHandler.OnHealthChanged(BuildingBase.HealthHandler);

		}

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
