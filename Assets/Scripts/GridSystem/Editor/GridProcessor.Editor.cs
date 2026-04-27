#if UNITY_EDITOR
using UnityEngine;

namespace GridSystem
{
	/// <summary>
	/// Editor-only visualization for GridProcessor.
	/// Draws gizmos to visualize the grid in the Unity editor scene view.
	/// </summary>
	public class GridProcessorEditor
	{
		/// <summary>
		/// Called by Unity editor to draw gizmos for visualization.
		/// Draws the grid in the scene view if enabled.
		/// </summary>
		private void OnDrawGizmos()
		{
			// Editor visualization disabled during ScriptableObject migration
			// Grid settings are now injected via DI and not accessible in Editor mode
			// TODO: Re-enable visualization after adding Editor-specific data access
		}
	}
}
#endif
