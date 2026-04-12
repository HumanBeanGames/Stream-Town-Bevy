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

	/// <summary>
	/// Saveable object for the ResourceManager that handles saving/loading ResourceData arrays.
	/// Replaces per-object resource saving with batch saving of all resource data.
	/// </summary>
	public class SaveableResourceManager : SaveableObject
	{
		[Inject] private ResourceManager _resourceManager;

		public override object SaveData()
		{
			return new ResourceManagerSaveData(
				_resourceManager.GetWoodResources(),
				_resourceManager.GetOreResources(),
				_resourceManager.GetFoodResources(),
				_resourceManager.GetGoldResources(),
				_resourceManager.GetRecruitResources()
			);
		}

		public override void LoadData(object data)
		{
			ResourceManagerSaveData saveData = (ResourceManagerSaveData)data;

			ResourceData[] woodResources = saveData.GetWoodResources();
			ResourceData[] oreResources = saveData.GetOreResources();
			ResourceData[] foodResources = saveData.GetFoodResources();
			ResourceData[] goldResources = saveData.GetGoldResources();
			ResourceData[] recruitResources = saveData.GetRecruitResources();

			// Note: Mesh and material lists need to be restored from the generation settings
			// This should be handled by the generation system before calling this load
			// For now, we'll pass null and the ResourceManager will use empty lists
			_resourceManager.SetWoodResources(woodResources, null, null);
			_resourceManager.SetOreResources(oreResources, null, null);
			_resourceManager.SetFoodResources(foodResources, null, null);
			_resourceManager.SetGoldResources(goldResources, null, null);
			_resourceManager.SetRecruitResources(recruitResources, null, null);

			// Update A* graph after loading
			_resourceManager.UpdateAllGraphBounds();
		}

		public void SetVariables(ResourceManager resourceManager)
		{
			_resourceManager = resourceManager;
		}
	}
}