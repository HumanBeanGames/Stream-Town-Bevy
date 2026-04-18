using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores player input system settings for the game.
	/// Currently has no editor-configurable settings but can be expanded in the future.
	/// </summary>
	[CreateAssetMenu(fileName = "PlayerInputSettings", menuName = "Scriptables/PlayerInput Settings")]
	public class PlayerInputSettingsScriptable : ScriptableObject, IDataScriptable
	{
		// No editor-configurable settings currently
		// This can be expanded if needed in the future
	}
}
