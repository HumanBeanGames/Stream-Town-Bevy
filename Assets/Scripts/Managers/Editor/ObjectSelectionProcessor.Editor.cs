#if UNITY_EDITOR
using UnityEngine;
using Processors;

namespace Processors.Editor
{
	/// <summary>
	/// Editor-only visualization for ObjectSelectionProcessor.
	/// Draws gizmos for debugging group selection in the Unity editor scene view.
	/// </summary>
	public partial class ObjectSelectionProcessor
	{
		/// <summary>
		/// Draws gizmos for debugging group selection.
		/// </summary>
		private void OnDrawGizmos()
		{
			// Editor visualization disabled during ScriptableObject migration
			// Object selection data is now in private fields and settings are injected via DI
			// TODO: Re-enable visualization after adding Editor-specific data access
		}
	}
}
#endif
