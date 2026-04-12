using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for terrain generation settings.
	/// </summary>
	public class TerrainGenerationSettings : MonoBehaviour
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

		public GenerationSettings GenerationSettings => _generationSettings;
		public float MeshHeightMultiplier => _meshHeightMultiplier;
		public AnimationCurve MeshHeightCurve => _meshHeightCurve;
		public bool EnableIslandBias => _enableIslandBias;
		public AnimationCurve IslandBiasCurve => _islandBiasCurve;
		public float IslandSize => _islandSize;
	}
}
