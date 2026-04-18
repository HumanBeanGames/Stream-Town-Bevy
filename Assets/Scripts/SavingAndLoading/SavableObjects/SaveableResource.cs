using GameResources;
using GUIDSystem;
using Processors;
using Reflex.Attributes;
using SavingAndLoading.Structs;
using Data.Containers;
using Target;
using UnityEngine;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects
{
    /// <summary>
    /// Handles saving and loading for individual resources.
    /// </summary>
	public class SaveableResource : SaveableObject
	{
        /// <summary>
        /// The resource holder.
        /// </summary>
		public ResourceHolder ResourceHolder;

        /// <summary>
        /// The GUID processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GUIDProcessor _guidProcessor;

        /// <summary>
        /// The resource runtime data. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ResourceProcessor _resourceProcessor;

        /// <summary>
        /// Saves the resource data.
        /// </summary>
        /// <returns>The resource save data.</returns>
		public override object SaveData()
		{
			return (object)new ResourceSaveData(ResourceHolder.transform, PoolName, ResourceHolder.Amount, _guidProcessor.CreateGUIDandAddToDictionary(PoolableObject));
		}

        /// <summary>
        /// Loads the resource data.
        /// </summary>
        /// <param name="data">The resource save data.</param>
		public override void LoadData(object data)
		{
			ResourceSaveData resourceData = (ResourceSaveData)data;
			if (resourceData.ResourceType == "Wood")
			{
				Debug.Log("lol");
			}
			ResourceHolder.transform.position = Vector3SaveData.ToUnityVec3(resourceData.ResourceTransform.Position);
			ResourceHolder.transform.eulerAngles = Vector3SaveData.ToUnityVec3(resourceData.ResourceTransform.Rotation);
			ResourceHolder.transform.localScale = Vector3SaveData.ToUnityVec3(resourceData.ResourceTransform.LossyScale);
			ResourceHolder.gameObject.SetActive(true);
			GUIDComponent.SetGUID(resourceData.GUID);
			_guidProcessor.AddToDictionary(PoolableObject);
		}

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
			if (poolName == "Wood")
			{
				Debug.Log("lol");
			}
			ResourceHolder = resourceHolder;
			base.SetVariables(target, component, poolName, poolableObject);
		}
	}
}
