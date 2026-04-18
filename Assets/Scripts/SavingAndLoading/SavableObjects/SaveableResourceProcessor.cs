using System.Collections.Generic;
using System.Linq;
using GameResources;
using Processors;
using Reflex.Attributes;
using SavingAndLoading.Structs;
using UnityEngine;

namespace SavingAndLoading.SavableObjects
{
	/// <summary>
	/// Saveable object for the ResourceProcessor that handles saving/loading ResourceData arrays.
	/// Replaces per-object resource saving with batch saving of all resource data.
	/// </summary>
	public class SaveableResourceProcessor : SaveableObject
	{
        /// <summary>
        /// The resource runtime data. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ResourceProcessor _resourceProcessor;

        /// <summary>
        /// Saves the resource processor data.
        /// </summary>
        /// <returns>The resource processor save data.</returns>
		public override object SaveData()
		{
			return new ResourceProcessorSaveData(
				_resourceProcessor.WoodResources.ToArray(),
				_resourceProcessor.OreResources.ToArray(),
				_resourceProcessor.FoodResources.ToArray(),
				_resourceProcessor.GoldResources.ToArray(),
				_resourceProcessor.RecruitResources.ToArray()
			);
		}

        /// <summary>
        /// Loads the resource processor data.
        /// </summary>
        /// <param name="data">The resource processor save data.</param>
		public override void LoadData(object data)
		{
			ResourceProcessorSaveData saveData = (ResourceProcessorSaveData)data;

			List<ResourceData> woodResources = saveData.GetWoodResources().ToList();
			List<ResourceData> oreResources = saveData.GetOreResources().ToList();
			List<ResourceData> foodResources = saveData.GetFoodResources().ToList();
			List<ResourceData> goldResources = saveData.GetGoldResources().ToList();
			List<ResourceData> recruitResources = saveData.GetRecruitResources().ToList();

			// Note: Mesh and material lists need to be restored from the generation settings
			// This should be handled by the generation system before calling this load
			// For now, we'll pass null and the ScriptableObject will use empty lists
			_resourceProcessor.WoodResources.Clear();
			_resourceProcessor.WoodResources.AddRange(woodResources);

			_resourceProcessor.OreResources.Clear();
			_resourceProcessor.OreResources.AddRange(oreResources);

			_resourceProcessor.FoodResources.Clear();
			_resourceProcessor.FoodResources.AddRange(foodResources);

			_resourceProcessor.GoldResources.Clear();
			_resourceProcessor.GoldResources.AddRange(goldResources);

			_resourceProcessor.RecruitResources.Clear();
			_resourceProcessor.RecruitResources.AddRange(recruitResources);

			// Note: UpdateAllGraphBounds() should be called by ResourceProcessor after observing changes to ResourceRuntimeData
			// This class should not reference processors directly
		}
	}
}
