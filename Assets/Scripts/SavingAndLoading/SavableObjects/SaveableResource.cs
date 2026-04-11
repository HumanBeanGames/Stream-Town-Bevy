using GameResources;
using GUIDSystem;
using Managers;
using Reflex.Attributes;
using SavingAndLoading.Structs;
using Target;
using UnityEngine;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects
{
	public class SaveableResource : SaveableObject
	{
		public ResourceHolder ResourceHolder;
		[Inject] private GUIDManager _guidManager;
		public override object SaveData()
		{
			return (object)new ResourceSaveData(ResourceHolder.transform, PoolName, ResourceHolder.Amount, _guidManager.CreateGUIDandAddToDictionary(PoolableObject));
		}

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
			_guidManager.AddToDictionary(PoolableObject);
		}

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