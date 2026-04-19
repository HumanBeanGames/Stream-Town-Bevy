using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "SeasonMaterialsSettings", menuName = "Scriptables/SeasonMaterialsSettings")]
	public class SeasonMaterialsSettings : ScriptableObject, IDataScriptable
	{
		[Header("Materials")]
		public Material GrassMaterial;
		public Material TerrainMaterial;
		public Material TreeMaterial;
		public Material BuildingMaterial;
		public Material WaterMaterial;
		
		[Header("Settings")]
#if UNITY_EDITOR
		[Tooltip("If set to true, seasons will be driven by the game's time processor.")]
		public bool DriveSeasonsByTime = false;
#endif
	}
}
