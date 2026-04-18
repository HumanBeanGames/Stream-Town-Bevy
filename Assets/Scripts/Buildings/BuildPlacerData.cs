using UnityEngine;
using Utils;

namespace Buildings
{
	/// <summary>
	/// Stores data used by the building placer to determine certain attributes of a building.
	/// </summary>
	[System.Serializable]
	public class BuildPlacerData
	{
        /// <summary>
        /// The name of the building.
        /// </summary>
		public string BuildingName;

        /// <summary>
        /// The type of the building.
        /// </summary>
		public BuildingType BuildingType;

        /// <summary>
        /// The 3D model for the building.
        /// </summary>
		public GameObject BuildingModel;

        /// <summary>
        /// The size of the building in grid units.
        /// </summary>
		public Vector2 BuildingSize;

        /// <summary>
        /// The prefab to spawn for the building.
        /// TODO:: Use pooling to spawn.
        /// </summary>
		public GameObject Prefab;

        /// <summary>
        /// The renderer component for the building model.
        /// </summary>
		public Renderer Renderer;

        /// <summary>
        /// The placement probe handler for the building.
        /// </summary>
		public PlacementProbeHandler ProbeProcessor;
	}
}
