using UnityEngine;
using UserInterface;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores debug system settings for the game.
	/// Contains references to debug UI components.
	/// </summary>
	[CreateAssetMenu(fileName = "DebugSettings", menuName = "Scriptables/Debug Settings")]
	public class DebugSettingsScriptable : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Reference to the debug user interface component.
		/// Used to display debug information and controls.
		/// </summary>
		[SerializeField]
		private UserInterface_Debug _debugUI;

		/// <summary>
		/// Gets the debug user interface component.
		/// </summary>
		public UserInterface_Debug DebugUI => _debugUI;
	}
}
