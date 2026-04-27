using System.Collections.Generic;
using Pathfinding;
using Reflex.Core;
using Reflex.Attributes;
using UnityEngine;
using Utils;
using Data.Containers;
using ScriptablesProcessorInfrastructure;
namespace Processors
{
	/// <summary>
	/// Virtual target representation for data-driven resources.
	/// Used by units to target resources without GameObject references.
	/// </summary>
	public struct ResourceTarget
	{
		/// <summary>
		/// Globally unique identifier for the resource.
		/// </summary>
		public uint GUID;

		/// <summary>
		/// World position of the resource.
		/// </summary>
		public Vector3 Position;

		/// <summary>
		/// Type of resource (Wood, Ore, Food, Gold, Recruit).
		/// </summary>
		public global::Utils.Resource ResourceType;

		/// <summary>
		/// Current amount of resource available.
		/// </summary>
		public int CurrentAmount;

		/// <summary>
		/// Number of units currently assigned to this resource.
		/// </summary>
		public int AssignedCount;

		/// <summary>
		/// Size squared of the resource for targeting calculations.
		/// </summary>
		public float SizeSqr;

		/// <summary>
		/// Creates a new resource target.
		/// </summary>
		/// <param name="guid">GUID of the resource.</param>
		/// <param name="position">World position of the resource.</param>
		/// <param name="resourceType">Type of resource.</param>
		/// <param name="currentAmount">Current amount available.</param>
		/// <param name="sizeSqr">Size squared for targeting.</param>
		public ResourceTarget(uint guid, Vector3 position, global::Utils.Resource resourceType, int currentAmount, float sizeSqr)
		{
			GUID = guid;
			Position = position;
			ResourceType = resourceType;
			CurrentAmount = currentAmount;
			AssignedCount = 0;
			SizeSqr = sizeSqr;
		}
	}

	/// <summary>
	/// Data-driven resource processor for managing world resources.
	/// Replaces object-based ResourceHolders with array-based data structures.
	/// All runtime state is stored in ResourceRuntimeData.
	/// </summary>
	public partial class ResourceProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// Runtime data for resource data.
		/// Assigned in InjectRuntimeData.
		/// </summary>
		private ResourceRuntimeData _resourceData;

		/// <summary>
		/// Gets the list of wood resources.
		/// </summary>
		public List<GameResources.ResourceData> WoodResources => _resourceData.WoodResources;

		/// <summary>
		/// Gets the list of ore resources.
		/// </summary>
		public List<GameResources.ResourceData> OreResources => _resourceData.OreResources;

		/// <summary>
		/// Gets the list of food resources.
		/// </summary>
		public List<GameResources.ResourceData> FoodResources => _resourceData.FoodResources;

		/// <summary>
		/// Gets the list of gold resources.
		/// </summary>
		public List<GameResources.ResourceData> GoldResources => _resourceData.GoldResources;

		/// <summary>
		/// Gets the list of recruit resources.
		/// </summary>
		public List<GameResources.ResourceData> RecruitResources => _resourceData.RecruitResources;

		/// <summary>
		/// Registers this processor as a singleton in the dependency injection container.
		/// Called by Reflex during container initialization.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		/// <summary>
		/// Injects the ResourceRuntimeData into the DI container and assigns it to the processor.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_resourceData != null)
				throw new System.InvalidOperationException("ResourceProcessor runtime data has already been installed.");

			_resourceData = new ResourceRuntimeData();
			containerBuilder.AddSingleton(_resourceData);
		}

		public void Initialize()
		{
			if (_resourceData == null)
				throw new System.InvalidOperationException("ResourceProcessor runtime data has not been installed.");

			InitializeFromScriptableObject();
		}


		/// <summary>
		/// Initializes the processor by loading and caching resource data from the ScriptableObject.
		/// Groups resources by mesh/material indices and pre-calculates transformation matrices.
		/// </summary>
		private void InitializeFromScriptableObject()
		{
			// Initialize dictionaries in ResourceRuntimeData
			_resourceData.ResourceAssignmentCounts = new Dictionary<uint, int>();
			_resourceData.ResourceCurrentAmounts = new Dictionary<uint, int>();
			_resourceData.ResourceSize = 1f;
			
			// Check if resource runtime data is available
			if (_resourceData != null)
			{
				// Group and cache wood resources
				_resourceData.WoodResourcesCache = GroupByIndices(_resourceData.WoodResources.ToArray());
				_resourceData.WoodMatricesCache = BuildMatricesDictionary(_resourceData.WoodResourcesCache);
				
				// Group and cache ore resources
				_resourceData.OreResourcesCache = GroupByIndices(_resourceData.OreResources.ToArray());
				_resourceData.OreMatricesCache = BuildMatricesDictionary(_resourceData.OreResourcesCache);
				
				// Group and cache food resources
				_resourceData.FoodResourcesCache = GroupByIndices(_resourceData.FoodResources.ToArray());
				_resourceData.FoodMatricesCache = BuildMatricesDictionary(_resourceData.FoodResourcesCache);
				
				// Group and cache gold resources
				_resourceData.GoldResourcesCache = GroupByIndices(_resourceData.GoldResources.ToArray());
				_resourceData.GoldMatricesCache = BuildMatricesDictionary(_resourceData.GoldResourcesCache);
				
				// Group and cache recruit resources
				_resourceData.RecruitResourcesCache = GroupByIndices(_resourceData.RecruitResources.ToArray());
				_resourceData.RecruitMatricesCache = BuildMatricesDictionary(_resourceData.RecruitResourcesCache);

				// Initialize runtime state with initial amounts for all resource types
				InitializeRuntimeState(_resourceData.WoodResources);
				InitializeRuntimeState(_resourceData.OreResources);
				InitializeRuntimeState(_resourceData.FoodResources);
				InitializeRuntimeState(_resourceData.GoldResources);
				InitializeRuntimeState(_resourceData.RecruitResources);
			}
		}

		/// <summary>
		/// Initializes runtime state with initial resource amounts.
		/// </summary>
		/// <param name="resources">List of resources to initialize state for.</param>
		private void InitializeRuntimeState(List<GameResources.ResourceData> resources)
		{
			foreach (var resource in resources)
			{
				// Set initial amount for each resource
				_resourceData.ResourceCurrentAmounts[resource.GUID] = resource.CurrentAmount;
			}
		}

		/// <summary>
		/// Groups resource data by mesh and material indices for efficient GPU instancing.
		/// Resources with the same mesh and material can be rendered in a single draw call.
		/// </summary>
		/// <param name="resources">Array of resource data to group.</param>
		/// <returns>Dictionary mapping (meshIndex, materialIndex) tuples to arrays of resource data.</returns>
		private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> GroupByIndices(GameResources.ResourceData[] resources)
		{
			Dictionary<(int, int), GameResources.ResourceData[]> grouped = new Dictionary<(int, int), GameResources.ResourceData[]>();
			// Return empty dictionary if no resources
			if (resources == null || resources.Length == 0)
				return grouped;

			Dictionary<(int, int), List<GameResources.ResourceData>> temp = new Dictionary<(int, int), List<GameResources.ResourceData>>();

			// Group resources by mesh and material indices
			foreach (GameResources.ResourceData resource in resources)
			{
				var key = (resource.MeshIndex, resource.MaterialIndex);
				if (!temp.ContainsKey(key))
					temp[key] = new List<GameResources.ResourceData>();
				temp[key].Add(resource);
			}

			// Convert lists to arrays
			foreach (var kvp in temp)
				grouped[kvp.Key] = kvp.Value.ToArray();

			return grouped;
		}

		/// <summary>
		/// Builds a dictionary of transformation matrices for GPU instancing.
		/// Uses the pre-calculated Matrix4x4 from each resource's data.
		/// </summary>
		/// <param name="resourcesDict">Dictionary of resource data grouped by mesh and material indices.</param>
		/// <returns>Dictionary mapping (meshIndex, materialIndex) tuples to arrays of transformation matrices.</returns>
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> BuildMatricesDictionary(Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> resourcesDict)
		{
			Dictionary<(int, int), Matrix4x4[]> matricesDict = new Dictionary<(int, int), Matrix4x4[]>();

			// Build transformation matrices for each resource group
			foreach (var kvp in resourcesDict)
			{
				var key = kvp.Key;
				GameResources.ResourceData[] resources = kvp.Value;

				// Create array of matrices for this resource group
				Matrix4x4[] matrices = new Matrix4x4[resources.Length];
				for (int i = 0; i < resources.Length; i++)
					// Use the pre-calculated matrix from resource data
					matrices[i] = resources[i].Matrix;

				matricesDict[key] = matrices;
			}

			return matricesDict;
		}

		/// <summary>
		/// Builds a dictionary of transformation matrices for GPU instancing.
		/// <param name="resources">Array of resource data.</param>
		/// <returns>Array of transformation matrices.</returns>
		private Matrix4x4[] BuildMatrices(GameResources.ResourceData[] resources)
		{
			// Return null if no resources
			if (resources == null || resources.Length == 0)
				return null;

			// Create array of matrices
			Matrix4x4[] matrices = new Matrix4x4[resources.Length];
			for (int i = 0; i < resources.Length; i++)
			{
				// Copy the pre-calculated matrix from resource data
				matrices[i] = resources[i].Matrix;
			}
			return matrices;
		}

		/// <summary>
		/// Gets the cached wood resources grouped by mesh and material.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> GetWoodResources() => _resourceData.WoodResourcesCache;

		/// <summary>
		/// Gets the cached ore resources grouped by mesh and material.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> GetOreResources() => _resourceData.OreResourcesCache;

		/// <summary>
		/// Gets the cached food resources grouped by mesh and material.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> GetFoodResources() => _resourceData.FoodResourcesCache;

		/// <summary>
		/// Gets the cached gold resources grouped by mesh and material.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> GetGoldResources() => _resourceData.GoldResourcesCache;

		/// <summary>
		/// Gets the cached recruit resources grouped by mesh and material.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> GetRecruitResources() => _resourceData.RecruitResourcesCache;

		/// <summary>
		/// Gets the cached transformation matrices for wood resources.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetWoodMatrices()
		{
			if (_resourceData != null)
				return _resourceData.WoodMatricesCache;
			return new Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]>();
		}

		/// <summary>
		/// Gets the cached transformation matrices for ore resources.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetOreMatrices()
		{
			if (_resourceData != null)
				return _resourceData.OreMatricesCache;
			return new Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]>();
		}

		/// <summary>
		/// Gets the cached transformation matrices for food resources.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetFoodMatrices()
		{
			if (_resourceData != null)
				return _resourceData.FoodMatricesCache;
			return new Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]>();
		}

		/// <summary>
		/// Gets the cached transformation matrices for gold resources.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetGoldMatrices()
		{
			if (_resourceData != null)
				return _resourceData.GoldMatricesCache;
			return new Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]>();
		}

		/// <summary>
		/// Gets the cached transformation matrices for recruit resources.
		/// </summary>
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetRecruitMatrices()
		{
			if (_resourceData != null)
				return _resourceData.RecruitMatricesCache;
			return new Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]>();
		}

		public void SetGeneratedResources(global::Utils.Resource resourceType, List<GameResources.ResourceData> resources, List<Mesh> meshes, List<Material> materials)
		{
			switch (resourceType)
			{
				case global::Utils.Resource.Wood:
					_resourceData.WoodResources = resources ?? new List<GameResources.ResourceData>();
					_resourceData.WoodMeshes = meshes ?? new List<Mesh>();
					_resourceData.WoodMaterials = materials ?? new List<Material>();
					_resourceData.WoodResourcesCache = GroupByIndices(_resourceData.WoodResources.ToArray());
					_resourceData.WoodMatricesCache = BuildMatricesDictionary(_resourceData.WoodResourcesCache);
					InitializeRuntimeState(_resourceData.WoodResources);
					break;
				case global::Utils.Resource.Ore:
					_resourceData.OreResources = resources ?? new List<GameResources.ResourceData>();
					_resourceData.OreMeshes = meshes ?? new List<Mesh>();
					_resourceData.OreMaterials = materials ?? new List<Material>();
					_resourceData.OreResourcesCache = GroupByIndices(_resourceData.OreResources.ToArray());
					_resourceData.OreMatricesCache = BuildMatricesDictionary(_resourceData.OreResourcesCache);
					InitializeRuntimeState(_resourceData.OreResources);
					break;
				case global::Utils.Resource.Food:
					_resourceData.FoodResources = resources ?? new List<GameResources.ResourceData>();
					_resourceData.FoodMeshes = meshes ?? new List<Mesh>();
					_resourceData.FoodMaterials = materials ?? new List<Material>();
					_resourceData.FoodResourcesCache = GroupByIndices(_resourceData.FoodResources.ToArray());
					_resourceData.FoodMatricesCache = BuildMatricesDictionary(_resourceData.FoodResourcesCache);
					InitializeRuntimeState(_resourceData.FoodResources);
					break;
				case global::Utils.Resource.Gold:
					_resourceData.GoldResources = resources ?? new List<GameResources.ResourceData>();
					_resourceData.GoldMeshes = meshes ?? new List<Mesh>();
					_resourceData.GoldMaterials = materials ?? new List<Material>();
					_resourceData.GoldResourcesCache = GroupByIndices(_resourceData.GoldResources.ToArray());
					_resourceData.GoldMatricesCache = BuildMatricesDictionary(_resourceData.GoldResourcesCache);
					InitializeRuntimeState(_resourceData.GoldResources);
					break;
				case global::Utils.Resource.Recruit:
					_resourceData.RecruitResources = resources ?? new List<GameResources.ResourceData>();
					_resourceData.RecruitMeshes = meshes ?? new List<Mesh>();
					_resourceData.RecruitMaterials = materials ?? new List<Material>();
					_resourceData.RecruitResourcesCache = GroupByIndices(_resourceData.RecruitResources.ToArray());
					_resourceData.RecruitMatricesCache = BuildMatricesDictionary(_resourceData.RecruitResourcesCache);
					InitializeRuntimeState(_resourceData.RecruitResources);
					break;
			}
		}

		/// <summary>
		/// Gets the meshes and materials for wood resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetWoodMeshMaterials()
		{
			if (_resourceData != null)
				return _resourceData.GetWoodMeshMaterials();
			return (new List<Mesh>(), new List<Material>());
		}

		/// <summary>
		/// Gets the meshes and materials for ore resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetOreMeshMaterials()
		{
			if (_resourceData != null)
				return _resourceData.GetOreMeshMaterials();
			return (new List<Mesh>(), new List<Material>());
		}

		/// <summary>
		/// Gets the meshes and materials for food resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetFoodMeshMaterials()
		{
			if (_resourceData != null)
				return _resourceData.GetFoodMeshMaterials();
			return (new List<Mesh>(), new List<Material>());
		}

		/// <summary>
		/// Gets the meshes and materials for gold resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetGoldMeshMaterials()
		{
			if (_resourceData != null)
				return _resourceData.GetGoldMeshMaterials();
			return (new List<Mesh>(), new List<Material>());
		}

		/// <summary>
		/// Gets the meshes and materials for recruit resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetRecruitMeshMaterials()
		{
			if (_resourceData != null)
				return _resourceData.GetRecruitMeshMaterials();
			return (new List<Mesh>(), new List<Material>());
		}

		/// <summary>
		/// Takes resources from a specific resource instance by GUID.
		/// Returns the amount taken, and removes the resource if depleted.
		/// </summary>
		/// <param name="guid">GUID of the resource to take from.</param>
		/// <param name="amount">Amount of resource to take.</param>
		/// <returns>Actual amount taken.</returns>
		public int TakeResource(uint guid, int amount)
		{
			GameResources.ResourceData[] resources = FindResourceByGUID(guid);
			if (resources == null)
				return 0;

			int index = FindResourceIndex(resources, guid);
			if (index == -1)
				return 0;

			GameResources.ResourceData resource = resources[index];
			// Unlimited resources can always return the requested amount
			if (resource.IsUnlimited)
				return amount;

			// Check if resource exists in runtime state
			if (!_resourceData.ResourceCurrentAmounts.ContainsKey(guid))
				return 0;

			int currentAmount = _resourceData.ResourceCurrentAmounts[guid];
			int taken = Mathf.Min(amount, currentAmount);
			int newAmount = currentAmount - taken;

			if (newAmount <= 0)
			{
				// Remove depleted resource
				RemoveResourceByGUID(guid);
				// Clear A* graph at this position to make it walkable again
				ClearGraphBounds(resource.Position);
			}
			else
			{
				// Update amount in runtime state
				_resourceData.ResourceCurrentAmounts[guid] = newAmount;
			}

			return taken;
		}

		/// <summary>
		/// Updates the A* graph for all resource positions after generation.
		/// Marks resource locations as unwalkable for pathfinding.
		/// </summary>
		public void UpdateAllGraphBounds()
		{
			if (AstarPath.active == null)
				return;

			// Update graph bounds for all resource types
			UpdateGraphBoundsForResourceDictionary(_resourceData.WoodResourcesCache);
			UpdateGraphBoundsForResourceDictionary(_resourceData.OreResourcesCache);
			UpdateGraphBoundsForResourceDictionary(_resourceData.FoodResourcesCache);
			UpdateGraphBoundsForResourceDictionary(_resourceData.GoldResourcesCache);
			UpdateGraphBoundsForResourceDictionary(_resourceData.RecruitResourcesCache);
		}

		/// <summary>
		/// Clears the A* graph at a specific position (for resource depletion).
		/// Makes the position walkable again after resource is removed.
		/// </summary>
		/// <param name="position">Position to clear graph bounds at.</param>
		private void ClearGraphBounds(Vector3 position)
		{
			if (AstarPath.active == null)
				return;

			// Create bounds at resource position
			Bounds bounds = new Bounds(position, Vector3.one);
			var guo = new GraphUpdateObject(bounds);
			guo.modifyWalkability = true;
			guo.setWalkability = true; // Make walkable again
			guo.modifyTag = true;
			guo.setTag = 0;
			AstarPath.active.UpdateGraphs(guo);
		}

		/// <summary>
		/// Updates the A* graph for an array of resources.
		/// Marks resource locations as unwalkable.
		/// </summary>
		/// <param name="resources">Array of resources to update graph bounds for.</param>
		private void UpdateGraphBoundsForResourceArray(GameResources.ResourceData[] resources)
		{
			if (resources == null || resources.Length == 0)
				return;

			// Mark each resource position as unwalkable
			foreach (GameResources.ResourceData resource in resources)
			{
				Bounds bounds = new Bounds(resource.Position, Vector3.one);
				var guo = new GraphUpdateObject(bounds);
				guo.modifyWalkability = true;
				guo.setWalkability = false; // Make unwalkable
				guo.modifyTag = true;
				guo.setTag = 0;
				AstarPath.active.UpdateGraphs(guo);
			}
		}

		/// <summary>
		/// Updates the A* graph for a dictionary of resource groups.
		/// </summary>
		/// <param name="resourcesDict">Dictionary of resource groups to update.</param>
		private void UpdateGraphBoundsForResourceDictionary(Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> resourcesDict)
		{
			if (resourcesDict == null)
				return;

			// Update graph bounds for each resource group
			foreach (var kvp in resourcesDict)
			{
				UpdateGraphBoundsForResourceArray(kvp.Value);
			}
		}

		/// <summary>
		/// Finds resource data by GUID across all resource types.
		/// </summary>
		/// <param name="guid">GUID to search for.</param>
		/// <returns>Array containing the resource, or null if not found.</returns>
		private GameResources.ResourceData[] FindResourceByGUID(uint guid)
		{
			// Search in wood resources
			GameResources.ResourceData[] resources = FindResourceInDictionary(_resourceData.WoodResourcesCache, guid);
			if (resources != null) return resources;

			// Search in ore resources
			resources = FindResourceInDictionary(_resourceData.OreResourcesCache, guid);
			if (resources != null) return resources;

			// Search in food resources
			resources = FindResourceInDictionary(_resourceData.FoodResourcesCache, guid);
			if (resources != null) return resources;

			// Search in gold resources
			resources = FindResourceInDictionary(_resourceData.GoldResourcesCache, guid);
			if (resources != null) return resources;

			// Search in recruit resources
			resources = FindResourceInDictionary(_resourceData.RecruitResourcesCache, guid);
			if (resources != null) return resources;

			return null;
		}

		/// <summary>
		/// Finds resource data by GUID in a specific resource dictionary.
		/// </summary>
		/// <param name="resourcesDict">Dictionary to search in.</param>
		/// <param name="guid">GUID to search for.</param>
		/// <returns>Array containing the resource, or null if not found.</returns>
		private GameResources.ResourceData[] FindResourceInDictionary(Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> resourcesDict, uint guid)
		{
			if (resourcesDict == null)
				return null;

			// Search each resource group
			foreach (var kvp in resourcesDict)
			{
				int index = FindResourceIndex(kvp.Value, guid);
				if (index != -1)
					return kvp.Value;
			}
			return null;
		}

		/// <summary>
		/// Finds the index of a resource in an array by GUID.
		/// </summary>
		/// <param name="resources">Array of resources to search.</param>
		/// <param name="guid">GUID to search for.</param>
		/// <returns>Index of the resource, or -1 if not found.</returns>
		private int FindResourceIndex(GameResources.ResourceData[] resources, uint guid)
		{
			if (resources == null)
				return -1;

			// Linear search for the resource by GUID
			for (int i = 0; i < resources.Length; i++)
			{
				if (resources[i].GUID == guid)
					return i;
			}
			return -1;
		}

		/// <summary>
		/// Removes a resource from all caches by GUID.
		/// Also removes from runtime state and rebuilds matrices cache.
		/// </summary>
		/// <param name="guid">GUID of resource to remove.</param>
		private void RemoveResourceByGUID(uint guid)
		{
			// Remove from runtime state
			_resourceData.ResourceCurrentAmounts.Remove(guid);
			_resourceData.ResourceAssignmentCounts.Remove(guid);

			// Try to remove from each resource type cache
			if (RemoveFromDictionary(_resourceData.WoodResourcesCache, guid))
			{
				_resourceData.WoodMatricesCache = BuildMatricesDictionary(_resourceData.WoodResourcesCache);
				return;
			}
			if (RemoveFromDictionary(_resourceData.OreResourcesCache, guid))
			{
				_resourceData.OreMatricesCache = BuildMatricesDictionary(_resourceData.OreResourcesCache);
				return;
			}
			if (RemoveFromDictionary(_resourceData.FoodResourcesCache, guid))
			{
				_resourceData.FoodMatricesCache = BuildMatricesDictionary(_resourceData.FoodResourcesCache);
				return;
			}
			if (RemoveFromDictionary(_resourceData.GoldResourcesCache, guid))
			{
				_resourceData.GoldMatricesCache = BuildMatricesDictionary(_resourceData.GoldResourcesCache);
				return;
			}
			if (RemoveFromDictionary(_resourceData.RecruitResourcesCache, guid))
			{
				_resourceData.RecruitMatricesCache = BuildMatricesDictionary(_resourceData.RecruitResourcesCache);
				return;
			}
		}

		/// <summary>
		/// Removes a resource from a dictionary by GUID.
		/// Rebuilds the array without the removed resource.
		/// </summary>
		/// <param name="resourcesDict">Dictionary to remove from.</param>
		/// <param name="guid">GUID of resource to remove.</param>
		/// <returns>True if resource was removed, false if not found.</returns>
		private bool RemoveFromDictionary(Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> resourcesDict, uint guid)
		{
			if (resourcesDict == null)
				return false;

			// Search each resource group
			foreach (var kvp in resourcesDict)
			{
				int index = FindResourceIndex(kvp.Value, guid);
				if (index != -1)
				{
					// Create new array without the removed resource
					GameResources.ResourceData[] newArray = new GameResources.ResourceData[kvp.Value.Length - 1];
					System.Array.Copy(kvp.Value, 0, newArray, 0, index);
					System.Array.Copy(kvp.Value, index + 1, newArray, index, kvp.Value.Length - index - 1);
					resourcesDict[kvp.Key] = newArray;

					// If the array is now empty, remove the entry
					if (newArray.Length == 0)
						resourcesDict.Remove(kvp.Key);

					return true;
				}
			}
			return false;
		}

		/// <summary>
		/// Updates the current amount of a resource.
		/// </summary>
		/// <param name="guid">GUID of resource to update.</param>
		/// <param name="newAmount">New amount value.</param>
		private void UpdateResourceAmount(uint guid, int newAmount)
		{
			if (_resourceData.ResourceCurrentAmounts.ContainsKey(guid))
			{
				_resourceData.ResourceCurrentAmounts[guid] = newAmount;
			}
		}

		/// <summary>
		/// Gets resources by type as a flattened array.
		/// </summary>
		/// <param name="resourceType">Type of resource to get.</param>
		/// <returns>Array of resources of the specified type.</returns>
		private GameResources.ResourceData[] GetResourcesByType(global::Utils.Resource resourceType)
		{
			Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> cache;
			switch (resourceType)
			{
				case global::Utils.Resource.Wood:
					cache = _resourceData.WoodResourcesCache;
					break;
				case global::Utils.Resource.Ore:
					cache = _resourceData.OreResourcesCache;
					break;
				case global::Utils.Resource.Food:
					cache = _resourceData.FoodResourcesCache;
					break;
				case global::Utils.Resource.Gold:
					cache = _resourceData.GoldResourcesCache;
					break;
				case global::Utils.Resource.Recruit:
					cache = _resourceData.RecruitResourcesCache;
					break;
				default:
					return null;
			}

			if (cache == null)
				return null;

			// Flatten the dictionary values into a single array
			List<GameResources.ResourceData> allResources = new List<GameResources.ResourceData>();
			foreach (var kvp in cache)
			{
				allResources.AddRange(kvp.Value);
			}
			return allResources.ToArray();
		}

		/// <summary>
		/// Gets all resources of a specific type within range of a position.
		/// </summary>
		/// <param name="position">Center position to search around.</param>
		/// <param name="range">Search radius.</param>
		/// <param name="resourceType">Type of resource to search for.</param>
		/// <returns>List of resource targets in range.</returns>
		public List<ResourceTarget> GetResourcesInRange(Vector3 position, float range, global::Utils.Resource resourceType)
		{
			List<ResourceTarget> targets = new List<ResourceTarget>();
			GameResources.ResourceData[] resources = GetResourcesByType(resourceType);

			if (resources == null)
				return targets;

			float rangeSqr = range * range;

			// Check each resource for range
			foreach (GameResources.ResourceData resource in resources)
			{
				float distSqr = (resource.Position - position).sqrMagnitude;
				if (distSqr <= rangeSqr)
				{
					// Get assignment count and current amount from runtime state
					int assignedCount = _resourceData.ResourceAssignmentCounts.ContainsKey(resource.GUID) ? _resourceData.ResourceAssignmentCounts[resource.GUID] : 0;
					int currentAmount = _resourceData.ResourceCurrentAmounts.ContainsKey(resource.GUID) ? _resourceData.ResourceCurrentAmounts[resource.GUID] : resource.CurrentAmount;
					targets.Add(new ResourceTarget(resource.GUID, resource.Position, resource.ResourceType, currentAmount, _resourceData.ResourceSize * _resourceData.ResourceSize));
				}
			}

			return targets;
		}

		/// <summary>
		/// Gets a ResourceTarget by GUID.
		/// </summary>
		/// <param name="guid">GUID of resource to get target for.</param>
		/// <returns>ResourceTarget if found, null otherwise.</returns>
		public ResourceTarget? GetResourceTarget(uint guid)
		{
			GameResources.ResourceData[] resources = FindResourceByGUID(guid);
			if (resources == null)
				return null;

			int index = FindResourceIndex(resources, guid);
			if (index == -1)
				return null;

			GameResources.ResourceData resource = resources[index];
			// Get assignment count and current amount from runtime state
			int assignedCount = _resourceData.ResourceAssignmentCounts.ContainsKey(resource.GUID) ? _resourceData.ResourceAssignmentCounts[resource.GUID] : 0;
			int currentAmount = _resourceData.ResourceCurrentAmounts.ContainsKey(resource.GUID) ? _resourceData.ResourceCurrentAmounts[resource.GUID] : resource.CurrentAmount;
			return new ResourceTarget(resource.GUID, resource.Position, resource.ResourceType, currentAmount, _resourceData.ResourceSize * _resourceData.ResourceSize);
		}

		/// <summary>
		/// Calculates the targeting score for a resource.
		/// Lower scores are better (closer and less assigned).
		/// </summary>
		/// <param name="guid">GUID of resource to score.</param>
		/// <param name="fromPosition">Position to calculate distance from.</param>
		/// <param name="distancePenaltyMod">Multiplier for distance penalty.</param>
		/// <param name="assignmentPenaltyMod">Multiplier for assignment penalty.</param>
		/// <returns>Targeting score (lower is better).</returns>
		public float CalculateTargetScore(uint guid, Vector3 fromPosition, float distancePenaltyMod = 0.5f, float assignmentPenaltyMod = 15f)
		{
			ResourceTarget? target = GetResourceTarget(guid);
			if (!target.HasValue)
				return float.MaxValue;

			ResourceTarget t = target.Value;
			float distance = Vector3.Distance(fromPosition, t.Position);
			int assignedCount = _resourceData.ResourceAssignmentCounts.ContainsKey(t.GUID) ? _resourceData.ResourceAssignmentCounts[t.GUID] : 0;

			// Score combines distance and assignment count
			return (distance * distancePenaltyMod) + (assignedCount * assignmentPenaltyMod);
		}

		/// <summary>
		/// Assigns a unit to a resource target.
		/// Increments the assignment count for the resource.
		/// </summary>
		/// <param name="guid">GUID of resource to assign to.</param>
		public void AssignToTarget(uint guid)
		{
			if (_resourceData.ResourceAssignmentCounts.ContainsKey(guid))
				_resourceData.ResourceAssignmentCounts[guid]++;
			else
				_resourceData.ResourceAssignmentCounts[guid] = 1;
		}

		/// <summary>
		/// Unassigns a unit from a resource target.
		/// Decrements the assignment count for the resource.
		/// </summary>
		/// <param name="guid">GUID of resource to unassign from.</param>
		public void UnassignFromTarget(uint guid)
		{
			if (_resourceData.ResourceAssignmentCounts.ContainsKey(guid))
			{
				_resourceData.ResourceAssignmentCounts[guid]--;
				// Remove entry if count reaches zero
				if (_resourceData.ResourceAssignmentCounts[guid] <= 0)
					_resourceData.ResourceAssignmentCounts.Remove(guid);
			}
		}

		/// <summary>
		/// Gets all resources of a specific type.
		/// <summary>
		/// Processes resource logic every frame.
		/// Called every frame by the Coordinator.
		/// ResourceProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			// ResourceProcessor does not require per-frame updates
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// ResourceProcessor does not have scene-specific settings to refresh
		}

	}
}
