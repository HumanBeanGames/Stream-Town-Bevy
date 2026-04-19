using UnityEngine;
using UnityEngine.UI;
using TMPro;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores loading screen settings for the game.
	/// Contains loading speed, wait times, and tooltip text configuration.
	/// </summary>
	[CreateAssetMenu(fileName = "LoadingSettings", menuName = "Scriptables/Loading Settings")]
	public class LoadingSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Speed at which the loading bar progresses.
		/// Higher values make the loading bar fill faster.
		/// </summary>
		public float LoadingSpeed = 0.5f;

		/// <summary>
		/// Time to wait after loading completes before transitioning.
		/// Used to ensure the loading screen is visible for a minimum duration.
		/// </summary>
		public float WaitTime = 0.5f;

		/// <summary>
		/// Array of tooltip strings to display during loading.
		/// Randomly selected tips are shown to engage players while waiting.
		/// </summary>
		public string[] ToolTips;
	}
}
