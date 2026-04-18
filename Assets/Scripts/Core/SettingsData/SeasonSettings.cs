using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "SeasonSettings", menuName = "Scriptables/Season Settings")]
	public class SeasonSettingsScriptable : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private Material _grassMaterial;
		[SerializeField]
		private Material _terrainMaterial;
		[SerializeField]
		private Material _treeMaterial;
		[SerializeField]
		private Material _buildingMaterial;
		[SerializeField]
		private Material _waterMaterial;

		/// <summary>
		/// Tint value for winter season materials.
		/// </summary>
		[SerializeField]
		private float _winterTint = 0.42f;

		/// <summary>
		/// Tint value for non-winter season materials.
		/// </summary>
		[SerializeField]
		private float _restTint = -0.08f;

		public Material GrassMaterial => _grassMaterial;
		public Material TerrainMaterial => _terrainMaterial;
		public Material TreeMaterial => _treeMaterial;
		public Material BuildingMaterial => _buildingMaterial;
		public Material WaterMaterial => _waterMaterial;
		public float WinterTint => _winterTint;
		public float RestTint => _restTint;
	}
}
