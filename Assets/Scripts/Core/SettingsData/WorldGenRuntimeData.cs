using System;
using UnityEngine;
using Buildings;
using GameResources;
using System.Collections.Generic;
using Utils.Pooling;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Generation state for Process-based execution.
	/// </summary>
	public enum GenerationState
	{
		Idle,
		GeneratingTerrain,
		SpawningTownhall,
		GeneratingObjects,
		Complete,
		// Async generation states
		GeneratingResources,
		GeneratingFoliage,
		GeneratingNoiseMap,
		ParallelResourceSpawning,
		ParallelFoliageSpawning,
		TerrainCheck
	}

	/// <summary>
	/// Runtime data for WorldGenProcessor.
	/// Stores world generation state including pooled objects, buildings, resources, and foliage.
	/// </summary>
	[CreateAssetMenu(fileName = "WorldGenRuntimeData", menuName = "Scriptables/WorldGen Runtime Data")]
	public class WorldGenRuntimeData : ScriptableObject
	{
		/// <summary>
		/// Dictionary of pooled objects by pool name.
		/// </summary>
		[SerializeField]
		private Dictionary<string, List<PoolableObject>> _pooledObjects = new Dictionary<string, List<PoolableObject>>();

		/// <summary>
		/// Dictionary of buildings by type.
		/// </summary>
		[SerializeField]
		private Dictionary<BuildingType, List<Buildings.BuildingBase>> _buildings = new Dictionary<BuildingType, List<Buildings.BuildingBase>>();

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
		/// List of on-land foliage.
		/// </summary>
		[SerializeField]
		private List<GameResources.FoliageData> _onLandFoliage = new List<GameResources.FoliageData>();

		/// <summary>
		/// List of underwater foliage.
		/// </summary>
		[SerializeField]
		private List<GameResources.FoliageData> _underWaterFoliage = new List<GameResources.FoliageData>();

		/// <summary>
		/// Whether the world has been generated.
		/// </summary>
		[SerializeField]
		private bool _worldGenerated = false;

		/// <summary>
		/// The generated terrain mesh.
		/// </summary>
		[SerializeField]
		private Mesh _generatedMesh;

		/// <summary>
		/// Whether terrain check passed.
		/// </summary>
		[SerializeField]
		private bool _terrainCheckPassed = false;

		/// <summary>
		/// Whether editor is regenerating.
		/// </summary>
		[SerializeField]
		private bool _isEditorRegenerating = false;

		/// <summary>
		/// Generation state for Process-based execution.
		/// </summary>
		[SerializeField]
		private GenerationState _generationState = GenerationState.Idle;

		/// <summary>
		/// Number of generation attempts.
		/// </summary>
		[SerializeField]
		private int _generationAttempts = 0;

		// Async generation state tracking
		[SerializeField]
		private int _currentGenerationSettingsIndex = 0;
		[SerializeField]
		private int _currentY = 0;
		[SerializeField]
		private int _currentX = 0;
		[SerializeField]
		private int _totalChecks = 0;
		[SerializeField]
		private int _checksProcessed = 0;
		[SerializeField]
		private float _frameStartTime = 0f;
		[SerializeField]
		private float[,] _generatedNoiseMap = null;
		[SerializeField]
		private int _parallelTaskCount = 0;
		[SerializeField]
		private int _completedParallelTasks = 0;
		[SerializeField]
		private bool _forceGenerate = false;
		[SerializeField]
		private Action<float, string> _progressReporterCallback;

		// Properties for accessing the data
		public Dictionary<string, List<PoolableObject>> PooledObjects => _pooledObjects;
		public Dictionary<BuildingType, List<Buildings.BuildingBase>> Buildings => _buildings;
		public List<GameResources.ResourceData> WoodResources
		{
			get => _woodResources;
			set => _woodResources = value;
		}
		public List<GameResources.ResourceData> OreResources
		{
			get => _oreResources;
			set => _oreResources = value;
		}
		public List<GameResources.ResourceData> FoodResources
		{
			get => _foodResources;
			set => _foodResources = value;
		}
		public List<GameResources.ResourceData> GoldResources
		{
			get => _goldResources;
			set => _goldResources = value;
		}
		public List<GameResources.ResourceData> RecruitResources
		{
			get => _recruitResources;
			set => _recruitResources = value;
		}
		public List<GameResources.FoliageData> OnLandFoliage
		{
			get => _onLandFoliage;
			set => _onLandFoliage = value;
		}
		public List<GameResources.FoliageData> UnderWaterFoliage
		{
			get => _underWaterFoliage;
			set => _underWaterFoliage = value;
		}
		public bool WorldGenerated
		{
			get => _worldGenerated;
			set => _worldGenerated = value;
		}
		public Mesh GeneratedMesh
		{
			get => _generatedMesh;
			set => _generatedMesh = value;
		}
		public bool TerrainCheckPassed
		{
			get => _terrainCheckPassed;
			set => _terrainCheckPassed = value;
		}
		public bool IsEditorRegenerating
		{
			get => _isEditorRegenerating;
			set => _isEditorRegenerating = value;
		}
		public GenerationState State
		{
			get => _generationState;
			set => _generationState = value;
		}
		public int GenerationAttempts
		{
			get => _generationAttempts;
			set => _generationAttempts = value;
		}
		public int CurrentGenerationSettingsIndex
		{
			get => _currentGenerationSettingsIndex;
			set => _currentGenerationSettingsIndex = value;
		}
		public int CurrentY
		{
			get => _currentY;
			set => _currentY = value;
		}
		public int CurrentX
		{
			get => _currentX;
			set => _currentX = value;
		}
		public int TotalChecks
		{
			get => _totalChecks;
			set => _totalChecks = value;
		}
		public int ChecksProcessed
		{
			get => _checksProcessed;
			set => _checksProcessed = value;
		}
		public float FrameStartTime
		{
			get => _frameStartTime;
			set => _frameStartTime = value;
		}
		public float[,] GeneratedNoiseMap
		{
			get => _generatedNoiseMap;
			set => _generatedNoiseMap = value;
		}
		public int ParallelTaskCount
		{
			get => _parallelTaskCount;
			set => _parallelTaskCount = value;
		}
		public int CompletedParallelTasks
		{
			get => _completedParallelTasks;
			set => _completedParallelTasks = value;
		}
		public bool ForceGenerate
		{
			get => _forceGenerate;
			set => _forceGenerate = value;
		}
		public Action<float, string> ProgressReporterCallback
		{
			get => _progressReporterCallback;
			set => _progressReporterCallback = value;
		}
	}
}
