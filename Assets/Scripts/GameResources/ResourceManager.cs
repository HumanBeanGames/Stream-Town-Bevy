using System.Collections.Generic;
using Pathfinding;
using Reflex.Core;
using UnityEngine;
using Utils;

namespace GameResources
{
	/// <summary>
	/// Virtual target representation for data-driven resources.
	/// Used by units to target resources without GameObject references.
	/// </summary>
	public struct ResourceTarget
	{
		public uint GUID;
		public Vector3 Position;
		public global::Utils.Resource ResourceType;
		public int CurrentAmount;
		public int AssignedCount;
		public float SizeSqr;

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
	/// Data-driven resource manager for managing world resources.
	/// Replaces object-based ResourceHolders with array-based data structures.
	/// </summary>
	public class ResourceManager : MonoBehaviour, IInstaller
	{
		private Dictionary<(int meshIndex, int materialIndex), ResourceData[]> _woodResources;
		private Dictionary<(int meshIndex, int materialIndex), ResourceData[]> _oreResources;
		private Dictionary<(int meshIndex, int materialIndex), ResourceData[]> _foodResources;
		private Dictionary<(int meshIndex, int materialIndex), ResourceData[]> _goldResources;
		private Dictionary<(int meshIndex, int materialIndex), ResourceData[]> _recruitResources;

		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _woodMatrices;
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _oreMatrices;
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _foodMatrices;
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _goldMatrices;
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _recruitMatrices;

		// Store mesh and material lists from settings for lookup by index
		private List<Mesh> _woodMeshes;
		private List<Material> _woodMaterials;
		private List<Mesh> _oreMeshes;
		private List<Material> _oreMaterials;
		private List<Mesh> _foodMeshes;
		private List<Material> _foodMaterials;
		private List<Mesh> _goldMeshes;
		private List<Material> _goldMaterials;
		private List<Mesh> _recruitMeshes;
		private List<Material> _recruitMaterials;

		private Dictionary<uint, int> _resourceAssignmentCounts = new Dictionary<uint, int>();
		private float _resourceSize = 1f;

		[SerializeField]
		private Vector3 _resourceBounds = new Vector3(1, 5, 1);

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}

		public void SetWoodResources(ResourceData[] resources, List<Mesh> meshes, List<Material> materials)
		{
			_woodMeshes = meshes;
			_woodMaterials = materials;
			_woodResources = GroupByIndices(resources);
			_woodMatrices = BuildMatricesDictionary(_woodResources);
		}

		public void SetOreResources(ResourceData[] resources, List<Mesh> meshes, List<Material> materials)
		{
			_oreMeshes = meshes;
			_oreMaterials = materials;
			_oreResources = GroupByIndices(resources);
			_oreMatrices = BuildMatricesDictionary(_oreResources);
		}

		public void SetFoodResources(ResourceData[] resources, List<Mesh> meshes, List<Material> materials)
		{
			_foodMeshes = meshes;
			_foodMaterials = materials;
			_foodResources = GroupByIndices(resources);
			_foodMatrices = BuildMatricesDictionary(_foodResources);
		}

		public void SetGoldResources(ResourceData[] resources, List<Mesh> meshes, List<Material> materials)
		{
			_goldMeshes = meshes;
			_goldMaterials = materials;
			_goldResources = GroupByIndices(resources);
			_goldMatrices = BuildMatricesDictionary(_goldResources);
		}

		public void SetRecruitResources(ResourceData[] resources, List<Mesh> meshes, List<Material> materials)
		{
			_recruitMeshes = meshes;
			_recruitMaterials = materials;
			_recruitResources = GroupByIndices(resources);
			_recruitMatrices = BuildMatricesDictionary(_recruitResources);
		}

		private Dictionary<(int meshIndex, int materialIndex), ResourceData[]> GroupByIndices(ResourceData[] resources)
		{
			Dictionary<(int, int), ResourceData[]> grouped = new Dictionary<(int, int), ResourceData[]>();
			if (resources == null || resources.Length == 0)
				return grouped;

			Dictionary<(int, int), List<ResourceData>> temp = new Dictionary<(int, int), List<ResourceData>>();

			foreach (ResourceData resource in resources)
			{
				var key = (resource.MeshIndex, resource.MaterialIndex);

				if (!temp.ContainsKey(key))
					temp[key] = new List<ResourceData>();

				temp[key].Add(resource);
			}

			foreach (var kvp in temp)
			{
				grouped[kvp.Key] = kvp.Value.ToArray();
			}

			return grouped;
		}

		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> BuildMatricesDictionary(Dictionary<(int meshIndex, int materialIndex), ResourceData[]> resourcesDict)
		{
			Dictionary<(int, int), Matrix4x4[]> matricesDict = new Dictionary<(int, int), Matrix4x4[]>();

			foreach (var kvp in resourcesDict)
			{
				matricesDict[kvp.Key] = BuildMatrices(kvp.Value);
			}

			return matricesDict;
		}

		private Matrix4x4[] BuildMatrices(ResourceData[] resources)
		{
			if (resources == null || resources.Length == 0)
				return null;

			Matrix4x4[] matrices = new Matrix4x4[resources.Length];
			for (int i = 0; i < resources.Length; i++)
			{
				matrices[i] = resources[i].Matrix;
			}
			return matrices;
		}

		public Dictionary<(int meshIndex, int materialIndex), ResourceData[]> GetWoodResources() => _woodResources;
		public Dictionary<(int meshIndex, int materialIndex), ResourceData[]> GetOreResources() => _oreResources;
		public Dictionary<(int meshIndex, int materialIndex), ResourceData[]> GetFoodResources() => _foodResources;
		public Dictionary<(int meshIndex, int materialIndex), ResourceData[]> GetGoldResources() => _goldResources;
		public Dictionary<(int meshIndex, int materialIndex), ResourceData[]> GetRecruitResources() => _recruitResources;

		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetWoodMatrices() => _woodMatrices;
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetOreMatrices() => _oreMatrices;
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetFoodMatrices() => _foodMatrices;
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetGoldMatrices() => _goldMatrices;
		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GetRecruitMatrices() => _recruitMatrices;

		public (List<Mesh> meshes, List<Material> materials) GetWoodMeshMaterials() => (_woodMeshes, _woodMaterials);
		public (List<Mesh> meshes, List<Material> materials) GetOreMeshMaterials() => (_oreMeshes, _oreMaterials);
		public (List<Mesh> meshes, List<Material> materials) GetFoodMeshMaterials() => (_foodMeshes, _foodMaterials);
		public (List<Mesh> meshes, List<Material> materials) GetGoldMeshMaterials() => (_goldMeshes, _goldMaterials);
		public (List<Mesh> meshes, List<Material> materials) GetRecruitMeshMaterials() => (_recruitMeshes, _recruitMaterials);

		/// <summary>
		/// Takes resources from a specific resource instance by GUID.
		/// Returns the amount taken, and removes the resource if depleted.
		/// </summary>
		public int TakeResource(uint guid, int amount)
		{
			ResourceData[] resources = FindResourceByGUID(guid);
			if (resources == null)
				return 0;

			int index = FindResourceIndex(resources, guid);
			if (index == -1)
				return 0;

			ResourceData resource = resources[index];
			if (resource.IsUnlimited)
				return amount;

			int taken = Mathf.Min(amount, resource.CurrentAmount);
			int newAmount = resource.CurrentAmount - taken;

			if (newAmount <= 0)
			{
				// Remove depleted resource
				RemoveResourceByGUID(guid);
				// Clear A* graph at this position
				ClearGraphBounds(resource.Position);
			}
			else
			{
				// Update amount
				UpdateResourceAmount(guid, newAmount);
			}

			return taken;
		}

		/// <summary>
		/// Updates the A* graph for all resource positions after generation.
		/// </summary>
		public void UpdateAllGraphBounds()
		{
			if (AstarPath.active == null)
				return;

			UpdateGraphBoundsForResourceDictionary(_woodResources);
			UpdateGraphBoundsForResourceDictionary(_oreResources);
			UpdateGraphBoundsForResourceDictionary(_foodResources);
			UpdateGraphBoundsForResourceDictionary(_goldResources);
			UpdateGraphBoundsForResourceDictionary(_recruitResources);
		}

		/// <summary>
		/// Clears the A* graph at a specific position (for resource depletion).
		/// </summary>
		private void ClearGraphBounds(Vector3 position)
		{
			if (AstarPath.active == null)
				return;

			Bounds bounds = new Bounds(position, _resourceBounds);
			var guo = new GraphUpdateObject(bounds);
			guo.modifyWalkability = true;
			guo.setWalkability = true; // Make walkable again
			guo.modifyTag = true;
			guo.setTag = 0;
			AstarPath.active.UpdateGraphs(guo);
		}

		private void UpdateGraphBoundsForResourceArray(ResourceData[] resources)
		{
			if (resources == null || resources.Length == 0)
				return;

			foreach (ResourceData resource in resources)
			{
				Bounds bounds = new Bounds(resource.Position, _resourceBounds);
				var guo = new GraphUpdateObject(bounds);
				guo.modifyWalkability = true;
				guo.setWalkability = false; // Make unwalkable
				guo.modifyTag = true;
				guo.setTag = 0;
				AstarPath.active.UpdateGraphs(guo);
			}
		}

		private void UpdateGraphBoundsForResourceDictionary(Dictionary<(int meshIndex, int materialIndex), ResourceData[]> resourcesDict)
		{
			if (resourcesDict == null)
				return;

			foreach (var kvp in resourcesDict)
			{
				UpdateGraphBoundsForResourceArray(kvp.Value);
			}
		}

		private ResourceData[] FindResourceByGUID(uint guid)
		{
			ResourceData[] resources = FindResourceInDictionary(_woodResources, guid);
			if (resources != null) return resources;

			resources = FindResourceInDictionary(_oreResources, guid);
			if (resources != null) return resources;

			resources = FindResourceInDictionary(_foodResources, guid);
			if (resources != null) return resources;

			resources = FindResourceInDictionary(_goldResources, guid);
			if (resources != null) return resources;

			resources = FindResourceInDictionary(_recruitResources, guid);
			if (resources != null) return resources;

			return null;
		}

		private ResourceData[] FindResourceInDictionary(Dictionary<(int meshIndex, int materialIndex), ResourceData[]> resourcesDict, uint guid)
		{
			if (resourcesDict == null)
				return null;

			foreach (var kvp in resourcesDict)
			{
				int index = FindResourceIndex(kvp.Value, guid);
				if (index != -1)
					return kvp.Value;
			}
			return null;
		}

		private int FindResourceIndex(ResourceData[] resources, uint guid)
		{
			if (resources == null)
				return -1;

			for (int i = 0; i < resources.Length; i++)
			{
				if (resources[i].GUID == guid)
					return i;
			}
			return -1;
		}

		private void RemoveResourceByGUID(uint guid)
		{
			if (RemoveFromDictionary(ref _woodResources, guid))
			{
				_woodMatrices = BuildMatricesDictionary(_woodResources);
				return;
			}
			if (RemoveFromDictionary(ref _oreResources, guid))
			{
				_oreMatrices = BuildMatricesDictionary(_oreResources);
				return;
			}
			if (RemoveFromDictionary(ref _foodResources, guid))
			{
				_foodMatrices = BuildMatricesDictionary(_foodResources);
				return;
			}
			if (RemoveFromDictionary(ref _goldResources, guid))
			{
				_goldMatrices = BuildMatricesDictionary(_goldResources);
				return;
			}
			if (RemoveFromDictionary(ref _recruitResources, guid))
			{
				_recruitMatrices = BuildMatricesDictionary(_recruitResources);
				return;
			}
		}

		private bool RemoveFromDictionary(ref Dictionary<(int meshIndex, int materialIndex), ResourceData[]> resourcesDict, uint guid)
		{
			if (resourcesDict == null)
				return false;

			foreach (var kvp in resourcesDict)
			{
				int index = FindResourceIndex(kvp.Value, guid);
				if (index != -1)
				{
					ResourceData[] newArray = new ResourceData[kvp.Value.Length - 1];
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

		private void UpdateResourceAmount(uint guid, int newAmount)
		{
			ResourceData[] resources = FindResourceByGUID(guid);
			if (resources == null)
				return;

			int index = FindResourceIndex(resources, guid);
			if (index == -1)
				return;

			ResourceData resource = resources[index];
			ResourceData updatedResource = new ResourceData(resource.Position, resource.ResourceType, newAmount, resource.IsUnlimited, resource.Matrix, resource.GUID, resource.MeshIndex, resource.MaterialIndex);
			resources[index] = updatedResource;
		}

		/// <summary>
		/// Gets all resources of a specific type within range of a position.
		/// </summary>
		public List<ResourceTarget> GetResourcesInRange(Vector3 position, float range, global::Utils.Resource resourceType)
		{
			List<ResourceTarget> targets = new List<ResourceTarget>();
			ResourceData[] resources = GetResourcesByType(resourceType);

			if (resources == null)
				return targets;

			float rangeSqr = range * range;

			foreach (ResourceData resource in resources)
			{
				float distSqr = (resource.Position - position).sqrMagnitude;
				if (distSqr <= rangeSqr)
				{
					int assignedCount = _resourceAssignmentCounts.ContainsKey(resource.GUID) ? _resourceAssignmentCounts[resource.GUID] : 0;
					targets.Add(new ResourceTarget(resource.GUID, resource.Position, resource.ResourceType, resource.CurrentAmount, _resourceSize * _resourceSize));
				}
			}

			return targets;
		}

		/// <summary>
		/// Gets a ResourceTarget by GUID.
		/// </summary>
		public ResourceTarget? GetResourceTarget(uint guid)
		{
			ResourceData[] resources = FindResourceByGUID(guid);
			if (resources == null)
				return null;

			int index = FindResourceIndex(resources, guid);
			if (index == -1)
				return null;

			ResourceData resource = resources[index];
			int assignedCount = _resourceAssignmentCounts.ContainsKey(resource.GUID) ? _resourceAssignmentCounts[resource.GUID] : 0;
			return new ResourceTarget(resource.GUID, resource.Position, resource.ResourceType, resource.CurrentAmount, _resourceSize * _resourceSize);
		}

		/// <summary>
		/// Calculates the targeting score for a resource.
		/// </summary>
		public float CalculateTargetScore(uint guid, Vector3 fromPosition, float distancePenaltyMod = 0.5f, float assignmentPenaltyMod = 15f)
		{
			ResourceTarget? target = GetResourceTarget(guid);
			if (!target.HasValue)
				return float.MaxValue;

			ResourceTarget t = target.Value;
			float distance = Vector3.Distance(fromPosition, t.Position);
			int assignedCount = _resourceAssignmentCounts.ContainsKey(t.GUID) ? _resourceAssignmentCounts[t.GUID] : 0;

			return (distance * distancePenaltyMod) + (assignedCount * assignmentPenaltyMod);
		}

		/// <summary>
		/// Assigns a unit to a resource target.
		/// </summary>
		public void AssignToTarget(uint guid)
		{
			if (_resourceAssignmentCounts.ContainsKey(guid))
				_resourceAssignmentCounts[guid]++;
			else
				_resourceAssignmentCounts[guid] = 1;
		}

		/// <summary>
		/// Unassigns a unit from a resource target.
		/// </summary>
		public void UnassignFromTarget(uint guid)
		{
			if (_resourceAssignmentCounts.ContainsKey(guid))
			{
				_resourceAssignmentCounts[guid]--;
				if (_resourceAssignmentCounts[guid] <= 0)
					_resourceAssignmentCounts.Remove(guid);
			}
		}

		private ResourceData[] GetResourcesByType(global::Utils.Resource resourceType)
		{
			Dictionary<(int, int), ResourceData[]> dict = null;
			switch (resourceType)
			{
				case global::Utils.Resource.Wood: dict = _woodResources; break;
				case global::Utils.Resource.Ore: dict = _oreResources; break;
				case global::Utils.Resource.Food: dict = _foodResources; break;
				case global::Utils.Resource.Gold: dict = _goldResources; break;
				case global::Utils.Resource.Recruit: dict = _recruitResources; break;
				default: return null;
			}

			return FlattenDictionary(dict);
		}

		private ResourceData[] FlattenDictionary(Dictionary<(int, int), ResourceData[]> dict)
		{
			if (dict == null || dict.Count == 0)
				return new ResourceData[0];

			int total = 0;
			foreach (var kvp in dict)
			{
				total += kvp.Value.Length;
			}

			ResourceData[] flattened = new ResourceData[total];
			int index = 0;
			foreach (var kvp in dict)
			{
				System.Array.Copy(kvp.Value, 0, flattened, index, kvp.Value.Length);
				index += kvp.Value.Length;
			}

			return flattened;
		}
	}
}
