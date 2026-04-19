using UnityEngine;
using UnityEngine.Rendering.Universal;
using UnityEngine.Rendering;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores day/night cycle settings for the game.
	/// Contains lighting parameters, material references, and transition configuration.
	/// </summary>
	[CreateAssetMenu(fileName = "DayAndNightSettings", menuName = "Scriptables/Day And Night Settings")]
	public class DayAndNightSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// The main directional light source for the scene.
		/// Its intensity and rotation are adjusted during day/night transitions.
		/// </summary>
		[SerializeField]
		private Light _mainLightSource;

		/// <summary>
		/// Percentage of the day cycle that should be daytime.
		/// 0.666f means 66.6% of the cycle is day, 33.3% is night.
		/// </summary>
		[SerializeField]
		private float _dayPercentage = 0.666f;

		/// <summary>
		/// Duration of the daytime period in seconds.
		/// Used to calculate when day-to-night transitions should occur.
		/// </summary>
		[SerializeField]
		private float _dayLength;

		/// <summary>
		/// Duration of the nighttime period in seconds.
		/// Used to calculate when night-to-day transitions should occur.
		/// </summary>
		[SerializeField]
		private float _nightLength;

		/// <summary>
		/// Duration of the day/night transition period in seconds.
		/// Determines how long lighting and material changes take to complete.
		/// </summary>
		[SerializeField]
		private float _transitionLength;

		/// <summary>
		/// Light intensity during nighttime.
		/// Applied to the main light source during the night period.
		/// </summary>
		[SerializeField]
		private float _nightLightIntensity;

		/// <summary>
		/// Light intensity during daytime.
		/// Applied to the main light source during the day period.
		/// </summary>
		[SerializeField]
		private float _dayLightIntensity;

		/// <summary>
		/// Material used on buildings that supports emission changes.
		/// Emission strength is adjusted during day/night transitions.
		/// </summary>
		[SerializeField]
		private Material _buildingMaterial;

		/// <summary>
		/// Maximum emission strength for building materials during nighttime.
		/// Used to make buildings glow at night.
		/// </summary>
		[SerializeField]
		private float _maxEmissionStrength;

		/// <summary>
		/// Universal Render Pipeline post-processing volume.
		/// Used to apply visual effects during day/night transitions (e.g., color grading, bloom).
		/// </summary>
		[SerializeField]
		private Volume _postProcessVolume;

		/// <summary>
		/// Gets the main directional light source.
		/// </summary>
		public Light MainLightSource => _mainLightSource;

		/// <summary>
		/// Gets the percentage of the day cycle that should be daytime.
		/// </summary>
		public float DayPercentage => _dayPercentage;

		/// <summary>
		/// Gets the duration of the daytime period.
		/// </summary>
		public float DayLength => _dayLength;

		/// <summary>
		/// Gets the duration of the nighttime period.
		/// </summary>
		public float NightLength => _nightLength;

		/// <summary>
		/// Gets the duration of the transition period.
		/// </summary>
		public float TransitionLength => _transitionLength;

		/// <summary>
		/// Gets the light intensity during nighttime.
		/// </summary>
		public float NightLightIntensity => _nightLightIntensity;

		/// <summary>
		/// Gets the light intensity during daytime.
		/// </summary>
		public float DayLightIntensity => _dayLightIntensity;

		/// <summary>
		/// Gets the building material for emission changes.
		/// </summary>
		public Material BuildingMaterial => _buildingMaterial;

		/// <summary>
		/// Gets the maximum emission strength for nighttime.
		/// </summary>
		public float MaxEmissionStrength => _maxEmissionStrength;

		/// <summary>
		/// Gets the post-processing volume for visual effects.
		/// </summary>
		public Volume PostProcessVolume => _postProcessVolume;
	}
}
