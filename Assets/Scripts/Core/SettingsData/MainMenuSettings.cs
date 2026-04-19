using MetaData;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores main menu settings for the game.
	/// Contains the scene index for the game world.
	/// </summary>
	[CreateAssetMenu(fileName = "MainMenuSettings", menuName = "Scriptables/Main Menu Settings")]
	public class MainMenuSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Index of the game world scene in the Unity build settings.
		/// Used to load the correct scene when starting the game from the main menu.
		/// </summary>
		public int SceneIndex = 0;
	}
}
