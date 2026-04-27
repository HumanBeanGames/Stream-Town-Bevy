using UnityEngine;
using World.Generation;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject for terrain generation settings.
	/// 
	/// Existing values to copy:
	/// - MeshHeightMultiplier: 1f
	/// - MeshHeightCurve: AnimationCurve(new Keyframe(0, 0), new Keyframe(1, 1))
	/// - EnableIslandBias: false
	/// - IslandBiasCurve: AnimationCurve(new Keyframe(0, 1), new Keyframe(1, 0))
	/// - IslandSize: 150f
	/// - IslandMultiplier: 1f
	/// - IslandAddition: 0f
	/// - QuantizationFactor: 0.1f
	/// - TopFaceProportion: 1f
	/// </summary>
	[CreateAssetMenu(fileName = "TerrainGenSettings", menuName = "Scriptables/World Generation/Terrain Generation Settings")]
	public class TerrainGenSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private GenerationSettings _generationSettings;

		[SerializeField]
		private float _meshHeightMultiplier = 1f;
		[SerializeField]
		private AnimationCurve _meshHeightCurve = new AnimationCurve(new Keyframe(0, 0), new Keyframe(1, 1));

		[Header("Island Bias")]
		[SerializeField]
		private bool _enableIslandBias = false;
		[SerializeField]
		private AnimationCurve _islandBiasCurve = new AnimationCurve(new Keyframe(0, 1), new Keyframe(1, 0));
		[SerializeField]
		private float _islandSize = 150f;
		[SerializeField]
		private float _islandMultiplier = 1f;
		[SerializeField]
		private float _islandAddition = 0f;

		[Header("Height Quantization")]
		[SerializeField]
		private float _quantizationFactor = 0.1f;

		[Header("Mesh Settings")]
		[SerializeField]
		[Range(0, 1)]
		private float _topFaceProportion = 1f;
		[SerializeField]
		private float _cellSize = 1f;

		[Header("Rendering")]
		[SerializeField]
		private GameObject _terrainPrefab;
		[SerializeField]
		private Material _terrainMaterial;

		public GenerationSettings GenerationSettings => _generationSettings;
		public float MeshHeightMultiplier => _meshHeightMultiplier;
		public AnimationCurve MeshHeightCurve => _meshHeightCurve;
		public bool EnableIslandBias => _enableIslandBias;
		public AnimationCurve IslandBiasCurve => _islandBiasCurve;
		public float IslandSize => _islandSize;
		public float IslandMultiplier => _islandMultiplier;
		public float IslandAddition => _islandAddition;
		public float QuantizationFactor => _quantizationFactor;
		public float TopFaceProportion => _topFaceProportion;
		public float CellSize => _cellSize;
		public GameObject TerrainPrefab => _terrainPrefab;
		public Material TerrainMaterial => _terrainMaterial;
	}
}
