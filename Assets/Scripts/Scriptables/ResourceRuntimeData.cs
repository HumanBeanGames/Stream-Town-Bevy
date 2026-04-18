using System.Collections.Generic;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for ResourceProcessor.
	/// Stores all runtime state for the ResourceProcessor including caches and tracking dictionaries.
	/// </summary>
	public class ResourceRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// List of wood resources.
		/// </summary>
		[SerializeField]
		private List<GameResources.ResourceData> _woodResources = new List<GameResources.ResourceData>();

		/// <summary>
		/// List of ore resources.
		/// </summary>
		[SerializeField]
		private List<GameResources.ResourceData> _oreResources = new List<GameResources.ResourceData>();

		/// <summary>
		/// List of food resources.
		/// </summary>
		[SerializeField]
		private List<GameResources.ResourceData> _foodResources = new List<GameResources.ResourceData>();

		/// <summary>
		/// List of gold resources.
		/// </summary>
		[SerializeField]
		private List<GameResources.ResourceData> _goldResources = new List<GameResources.ResourceData>();

		/// <summary>
		/// List of recruit resources.
		/// </summary>
		[SerializeField]
		private List<GameResources.ResourceData> _recruitResources = new List<GameResources.ResourceData>();

		/// <summary>
		/// List of wood meshes.
		/// </summary>
		[SerializeField]
		private List<Mesh> _woodMeshes = new List<Mesh>();

		/// <summary>
		/// List of wood materials.
		/// </summary>
		[SerializeField]
		private List<Material> _woodMaterials = new List<Material>();

		/// <summary>
		/// List of ore meshes.
		/// </summary>
		[SerializeField]
		private List<Mesh> _oreMeshes = new List<Mesh>();

		/// <summary>
		/// List of ore materials.
		/// </summary>
		[SerializeField]
		private List<Material> _oreMaterials = new List<Material>();

		/// <summary>
		/// List of food meshes.
		/// </summary>
		[SerializeField]
		private List<Mesh> _foodMeshes = new List<Mesh>();

		/// <summary>
		/// List of food materials.
		/// </summary>
		[SerializeField]
		private List<Material> _foodMaterials = new List<Material>();

		/// <summary>
		/// List of gold meshes.
		/// </summary>
		[SerializeField]
		private List<Mesh> _goldMeshes = new List<Mesh>();

		/// <summary>
		/// List of gold materials.
		/// </summary>
		[SerializeField]
		private List<Material> _goldMaterials = new List<Material>();

		/// <summary>
		/// List of recruit meshes.
		/// </summary>
		[SerializeField]
		private List<Mesh> _recruitMeshes = new List<Mesh>();

		/// <summary>
		/// List of recruit materials.
		/// </summary>
		[SerializeField]
		private List<Material> _recruitMaterials = new List<Material>();

		/// <summary>
		/// Dictionary tracking assignment counts for each resource GUID.
		/// </summary>
		[SerializeField]
		private Dictionary<uint, int> _resourceAssignmentCounts = new Dictionary<uint, int>();

		/// <summary>
		/// Dictionary tracking current amounts for each resource GUID.
		/// </summary>
		[SerializeField]
		private Dictionary<uint, int> _resourceCurrentAmounts = new Dictionary<uint, int>();

		/// <summary>
		/// Cache of wood resources grouped by mesh and material indices.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _woodResourcesCache;

		/// <summary>
		/// Cache of ore resources grouped by mesh and material indices.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _oreResourcesCache;

		/// <summary>
		/// Cache of food resources grouped by mesh and material indices.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _foodResourcesCache;

		/// <summary>
		/// Cache of gold resources grouped by mesh and material indices.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _goldResourcesCache;

		/// <summary>
		/// Cache of recruit resources grouped by mesh and material indices.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _recruitResourcesCache;

		/// <summary>
		/// Cache of transformation matrices for wood resources.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _woodMatricesCache;

		/// <summary>
		/// Cache of transformation matrices for ore resources.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _oreMatricesCache;

		/// <summary>
		/// Cache of transformation matrices for food resources.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _foodMatricesCache;

		/// <summary>
		/// Cache of transformation matrices for gold resources.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _goldMatricesCache;

		/// <summary>
		/// Cache of transformation matrices for recruit resources.
		/// </summary>
		[SerializeField]
		private Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> _recruitMatricesCache;

		/// <summary>
		/// Size of resources for targeting calculations.
		/// </summary>
		[SerializeField]
		private float _resourceSize = 1f;

		// Properties for resource lists
		public List<GameResources.ResourceData> WoodResources
		{
			get { return _woodResources; }
			set { _woodResources = value; }
		}

		public List<GameResources.ResourceData> OreResources
		{
			get { return _oreResources; }
			set { _oreResources = value; }
		}

		public List<GameResources.ResourceData> FoodResources
		{
			get { return _foodResources; }
			set { _foodResources = value; }
		}

		public List<GameResources.ResourceData> GoldResources
		{
			get { return _goldResources; }
			set { _goldResources = value; }
		}

		public List<GameResources.ResourceData> RecruitResources
		{
			get { return _recruitResources; }
			set { _recruitResources = value; }
		}

		public List<Mesh> WoodMeshes
		{
			get { return _woodMeshes; }
			set { _woodMeshes = value; }
		}

		public List<Material> WoodMaterials
		{
			get { return _woodMaterials; }
			set { _woodMaterials = value; }
		}

		public List<Mesh> OreMeshes
		{
			get { return _oreMeshes; }
			set { _oreMeshes = value; }
		}

		public List<Material> OreMaterials
		{
			get { return _oreMaterials; }
			set { _oreMaterials = value; }
		}

		public List<Mesh> FoodMeshes
		{
			get { return _foodMeshes; }
			set { _foodMeshes = value; }
		}

		public List<Material> FoodMaterials
		{
			get { return _foodMaterials; }
			set { _foodMaterials = value; }
		}

		public List<Mesh> GoldMeshes
		{
			get { return _goldMeshes; }
			set { _goldMeshes = value; }
		}

		public List<Material> GoldMaterials
		{
			get { return _goldMaterials; }
			set { _goldMaterials = value; }
		}

		public List<Mesh> RecruitMeshes
		{
			get { return _recruitMeshes; }
			set { _recruitMeshes = value; }
		}

		public List<Material> RecruitMaterials
		{
			get { return _recruitMaterials; }
			set { _recruitMaterials = value; }
		}

		public Dictionary<uint, int> ResourceAssignmentCounts
		{
			get { return _resourceAssignmentCounts; }
			set { _resourceAssignmentCounts = value; }
		}

		public Dictionary<uint, int> ResourceCurrentAmounts
		{
			get { return _resourceCurrentAmounts; }
			set { _resourceCurrentAmounts = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> WoodResourcesCache
		{
			get { return _woodResourcesCache; }
			set { _woodResourcesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> OreResourcesCache
		{
			get { return _oreResourcesCache; }
			set { _oreResourcesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> FoodResourcesCache
		{
			get { return _foodResourcesCache; }
			set { _foodResourcesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> GoldResourcesCache
		{
			get { return _goldResourcesCache; }
			set { _goldResourcesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> RecruitResourcesCache
		{
			get { return _recruitResourcesCache; }
			set { _recruitResourcesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> WoodMatricesCache
		{
			get { return _woodMatricesCache; }
			set { _woodMatricesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> OreMatricesCache
		{
			get { return _oreMatricesCache; }
			set { _oreMatricesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> FoodMatricesCache
		{
			get { return _foodMatricesCache; }
			set { _foodMatricesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> GoldMatricesCache
		{
			get { return _goldMatricesCache; }
			set { _goldMatricesCache = value; }
		}

		public Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> RecruitMatricesCache
		{
			get { return _recruitMatricesCache; }
			set { _recruitMatricesCache = value; }
		}

		public float ResourceSize
		{
			get { return _resourceSize; }
			set { _resourceSize = value; }
		}

		/// <summary>
		/// Gets the meshes and materials for wood resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetWoodMeshMaterials()
		{
			return (_woodMeshes, _woodMaterials);
		}

		/// <summary>
		/// Gets the meshes and materials for ore resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetOreMeshMaterials()
		{
			return (_oreMeshes, _oreMaterials);
		}

		/// <summary>
		/// Gets the meshes and materials for food resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetFoodMeshMaterials()
		{
			return (_foodMeshes, _foodMaterials);
		}

		/// <summary>
		/// Gets the meshes and materials for gold resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetGoldMeshMaterials()
		{
			return (_goldMeshes, _goldMaterials);
		}

		/// <summary>
		/// Gets the meshes and materials for recruit resources.
		/// </summary>
		/// <returns>Tuple of meshes and materials lists.</returns>
		public (List<Mesh> meshes, List<Material> materials) GetRecruitMeshMaterials()
		{
			return (_recruitMeshes, _recruitMaterials);
		}

		/// <summary>
		/// Initializes the resource runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
