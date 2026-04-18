using System.Collections.Generic;
using UnityEngine;
using GameResources;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for FoliageProcessor.
	/// Contains lists of foliage data for on-land and underwater foliage types.
	/// Also contains cached data for efficient GPU instancing.
	/// </summary>
	public class FoliageRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// List of foliage data for on-land vegetation.
		/// Contains position, rotation, and type data for all land-based foliage.
		/// </summary>
		[SerializeField]
		private List<FoliageData> _onLandFoliage = new List<FoliageData>();

		/// <summary>
		/// List of foliage data for underwater vegetation.
		/// Contains position, rotation, and type data for all underwater foliage.
		/// </summary>
		[SerializeField]
		private List<FoliageData> _underWaterFoliage = new List<FoliageData>();

		/// <summary>
		/// Cache of on-land foliage grouped by mesh and material.
		/// Used for efficient GPU instancing of land-based foliage.
		/// </summary>
		[SerializeField]
		private Dictionary<(Mesh mesh, Material material), FoliageData[]> _onLandFoliageCache = new Dictionary<(Mesh, Material material), FoliageData[]>();

		/// <summary>
		/// Cache of underwater foliage grouped by mesh and material.
		/// Used for efficient GPU instancing of water-based foliage.
		/// </summary>
		[SerializeField]
		private Dictionary<(Mesh mesh, Material material), FoliageData[]> _underWaterFoliageCache = new Dictionary<(Mesh, Material material), FoliageData[]>();

		/// <summary>
		/// Cache of transformation matrices for on-land foliage.
		/// Pre-calculated for GPU instancing performance.
		/// </summary>
		[SerializeField]
		private Dictionary<(Mesh mesh, Material material), Matrix4x4[]> _onLandMatricesCache = new Dictionary<(Mesh, Material material), Matrix4x4[]>();

		/// <summary>
		/// Cache of transformation matrices for underwater foliage.
		/// Pre-calculated for GPU instancing performance.
		/// </summary>
		[SerializeField]
		private Dictionary<(Mesh mesh, Material material), Matrix4x4[]> _underWaterMatricesCache = new Dictionary<(Mesh, Material material), Matrix4x4[]>();

		/// <summary>
		/// Gets or sets the list of on-land foliage data.
		/// </summary>
		public List<FoliageData> OnLandFoliage
		{
			get { return _onLandFoliage; }
			set { _onLandFoliage = value; }
		}

		/// <summary>
		/// Gets or sets the list of underwater foliage data.
		/// </summary>
		public List<FoliageData> UnderWaterFoliage
		{
			get { return _underWaterFoliage; }
			set { _underWaterFoliage = value; }
		}

		/// <summary>
		/// Gets or sets the cache of on-land foliage grouped by mesh and material.
		/// </summary>
		public Dictionary<(Mesh mesh, Material material), FoliageData[]> OnLandFoliageCache
		{
			get { return _onLandFoliageCache; }
			set { _onLandFoliageCache = value; }
		}

		/// <summary>
		/// Gets or sets the cache of underwater foliage grouped by mesh and material.
		/// </summary>
		public Dictionary<(Mesh mesh, Material material), FoliageData[]> UnderWaterFoliageCache
		{
			get { return _underWaterFoliageCache; }
			set { _underWaterFoliageCache = value; }
		}

		/// <summary>
		/// Gets or sets the cache of transformation matrices for on-land foliage.
		/// </summary>
		public Dictionary<(Mesh mesh, Material material), Matrix4x4[]> OnLandMatricesCache
		{
			get { return _onLandMatricesCache; }
			set { _onLandMatricesCache = value; }
		}

		/// <summary>
		/// Gets or sets the cache of transformation matrices for underwater foliage.
		/// </summary>
		public Dictionary<(Mesh mesh, Material material), Matrix4x4[]> UnderWaterMatricesCache
		{
			get { return _underWaterMatricesCache; }
			set { _underWaterMatricesCache = value; }
		}

		/// <summary>
		/// Initializes the foliage runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
