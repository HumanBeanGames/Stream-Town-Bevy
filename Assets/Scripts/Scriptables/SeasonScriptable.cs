using UnityEngine;
using UnityEngine.VFX;
using Utils;

namespace ScriptablesProcessorInfrastructure 
{
	/// <summary>
	/// ScriptableObject that stores configuration data for a specific season.
	/// Contains color palettes, visual effects, and particle settings for seasonal changes.
	/// </summary>
    [CreateAssetMenu(fileName = "SeasonDataSettings", menuName = "ScriptableObjects/SeasonDataSettings", order = 1)]
    public class SeasonDataSettings : ScriptableObject, IDataScriptable 
	{
		/// <summary>
		/// The season this data represents.
		/// Used for season identification and lookup.
		/// </summary>
		public Season Season;

		[Header("Grass Colors")]
		/// <summary>
		/// First grass grid color for this season.
		/// Used for grass texture variation.
		/// </summary>
		public Color GrassGridColor1;

		/// <summary>
		/// Second grass grid color for this season.
		/// Used for grass texture variation.
		/// </summary>
		public Color GrassGridColor2;

		/// <summary>
		/// Top grass color for this season.
		/// Applied to the upper portion of grass meshes.
		/// </summary>
		public Color GrassTopColor;

		/// <summary>
		/// Wind-affected grass color for this season.
		/// Applied to grass when affected by wind simulation.
		/// </summary>
		public Color GrassWindColor;

		[Header("Terrain Colors")]
		/// <summary>
		/// First terrain color for this season.
		/// Used for ground texture variation.
		/// </summary>
		public Color TerrainColor1;

		/// <summary>
		/// Second terrain color for this season.
		/// Used for ground texture variation.
		/// </summary>
		public Color TerrainColor2;

		[Header("Tree Colors")]
		/// <summary>
		/// Gradient for tree foliage colors in this season.
		/// Applied to tree meshes to create seasonal color transitions.
		/// </summary>
		public Gradient TreeColorGradient;

		[Header("VFX")]
		/// <summary>
		/// Visual effect for this season.
		/// Used for seasonal particle effects (snow, falling leaves, etc.).
		/// </summary>
		[HideInInspector]
		public VisualEffect VFX;

		/// <summary>
		/// Maximum number of particles for the seasonal VFX.
		/// Limits particle count for performance.
		/// </summary>
		public int MaxParticleCount;

		/// <summary>
		/// Time in seconds to interpolate between seasonal particle effects.
		/// Controls the transition smoothness between seasons.
		/// </summary>
		public float ParticleLerpTime = 3;

		/// <summary>
		/// Minimum run time for the seasonal VFX.
		/// Minimum duration before the effect can be stopped.
		/// </summary>
		public float MinRunTime;

		/// <summary>
		/// Maximum run time for the seasonal VFX.
		/// Maximum duration before the effect must be stopped.
		/// </summary>
		public float MaxRunTime;
    }
}
