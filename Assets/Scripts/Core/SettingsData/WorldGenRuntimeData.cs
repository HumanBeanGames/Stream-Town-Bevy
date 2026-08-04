using System;
using UnityEngine;
using Buildings;
using GameResources;
using System.Collections.Generic;
using Utils.Pooling;
using Utils;
using ScriptablesProcessorInfrastructure;
using Enemies;

namespace Processors
{
	/// <summary>
	/// Generation state for Process-based execution.
	/// </summary>
	public enum GenerationState
	{
		Idle,
		InitializingPooling,
		PrewarmingPooling,
		GeneratingTerrain,
		SpawningTownhall,
		GeneratingObjects,
		GeneratingNavmesh,
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
	public class WorldGenRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Dictionary of pooled objects by pool name.
		/// </summary>
		private Dictionary<string, List<PoolableObject>> _pooledObjects;

		/// <summary>
		/// Dictionary of buildings by type.
		/// </summary>
		private Dictionary<BuildingType, List<Buildings.BuildingBase>> _buildings;

		/// <summary>
		/// List of generated enemy camp spawners.
		/// </summary>
		private List<EnemySpawner> _enemyCampSpawners;

		/// <summary>
		/// List of wood resources.
		/// </summary>
		private List<GameResources.ResourceData> _woodResources;

		/// <summary>
		/// List of ore resources.
		/// </summary>
		private List<GameResources.ResourceData> _oreResources;

		/// <summary>
		/// List of food resources.
		/// </summary>
		private List<GameResources.ResourceData> _foodResources;

		/// <summary>
		/// List of gold resources.
		/// </summary>
		private List<GameResources.ResourceData> _goldResources;

		/// <summary>
		/// List of recruit resources.
		/// </summary>
		private List<GameResources.ResourceData> _recruitResources;

		/// <summary>
		/// List of on-land foliage.
		/// </summary>
		private List<GameResources.FoliageData> _onLandFoliage;

		/// <summary>
		/// List of underwater foliage.
		/// </summary>
		private List<GameResources.FoliageData> _underWaterFoliage;

		/// <summary>
		/// Whether the world has been generated.
		/// </summary>
		private bool _worldGenerated;

		/// <summary>
		/// The generated terrain mesh.
		/// </summary>
		private Mesh _generatedMesh;
		private bool _terrainCanRegenerateFromSeed;

		private GameObject _terrainHost;

		/// <summary>
		/// Whether terrain check passed.
		/// </summary>
		private bool _terrainCheckPassed;

		/// <summary>
		/// Whether editor is regenerating.
		/// </summary>
		private bool _isEditorRegenerating;

		/// <summary>
		/// Generation state for Process-based execution.
		/// </summary>
		private GenerationState _generationState;

		/// <summary>
		/// Number of generation attempts.
		/// </summary>
		private int _generationAttempts;

		// Async generation state tracking
		private int _currentGenerationSettingsIndex;
		private int _currentY;
		private int _currentX;
		private int _totalChecks;
		private int _checksProcessed;
		private float _frameStartTime;
		private float[,] _generatedNoiseMap;
		private int _parallelTaskCount;
		private int _completedParallelTasks;
		private bool _forceGenerate;
		private Action<float, string> _progressReporterCallback;

		// Properties for accessing the data
		public Dictionary<string, List<PoolableObject>> PooledObjects => _pooledObjects;
		public Dictionary<BuildingType, List<Buildings.BuildingBase>> Buildings => _buildings;
		public List<EnemySpawner> EnemyCampSpawners => _enemyCampSpawners;
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
		public bool TerrainCanRegenerateFromSeed
		{
			get => _terrainCanRegenerateFromSeed;
			set => _terrainCanRegenerateFromSeed = value;
		}
		public GameObject TerrainHost
		{
			get => _terrainHost;
			set => _terrainHost = value;
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

		/// <summary>
		/// Initializes the world generation runtime data with default values.
		/// </summary>
		public WorldGenRuntimeData()
		{
			_pooledObjects = new Dictionary<string, List<PoolableObject>>();
			_buildings = new Dictionary<BuildingType, List<Buildings.BuildingBase>>();
			_enemyCampSpawners = new List<EnemySpawner>();
			_woodResources = new List<GameResources.ResourceData>();
			_oreResources = new List<GameResources.ResourceData>();
			_foodResources = new List<GameResources.ResourceData>();
			_goldResources = new List<GameResources.ResourceData>();
			_recruitResources = new List<GameResources.ResourceData>();
			_onLandFoliage = new List<GameResources.FoliageData>();
			_underWaterFoliage = new List<GameResources.FoliageData>();
			_worldGenerated = false;
			_generatedMesh = null;
			_terrainCanRegenerateFromSeed = false;
			_terrainHost = null;
			_terrainCheckPassed = false;
			_isEditorRegenerating = false;
			_generationState = GenerationState.Idle;
			_generationAttempts = 0;
			_currentGenerationSettingsIndex = 0;
			_currentY = 0;
			_currentX = 0;
			_totalChecks = 0;
			_checksProcessed = 0;
			_frameStartTime = 0f;
			_generatedNoiseMap = null;
			_parallelTaskCount = 0;
			_completedParallelTasks = 0;
			_forceGenerate = false;
			_progressReporterCallback = null;
		}
	}
}
