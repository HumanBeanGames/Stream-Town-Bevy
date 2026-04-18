using UnityEngine;
using UnityEngine.Events;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for DebugProcessor.
	/// Manages object selection events and the currently selected debug object.
	/// </summary>
	public class DebugRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Event fired when an object is selected for debugging.
		/// Passes the selected object and associated data.
		/// </summary>
		[SerializeField]
		private UnityEvent<SelectableObject, object> _onObjectSelected = new UnityEvent<SelectableObject, object>();

		/// <summary>
		/// The currently selected object for debugging.
		/// Tuple containing the selectable object and associated data.
		/// </summary>
		[SerializeField]
		private (SelectableObject, object) _selectedObject;

		/// <summary>
		/// Gets the event fired when an object is selected.
		/// </summary>
		public UnityEvent<SelectableObject, object> OnObjectSelected => _onObjectSelected;

		/// <summary>
		/// Gets or sets the currently selected object.
		/// </summary>
		public (SelectableObject, object) SelectedObject
		{
			get => _selectedObject;
			set => _selectedObject = value;
		}

		/// <summary>
		/// Initializes the debug runtime state.
		/// </summary>
		public void Initialize()
		{
			_selectedObject = (null, null);
		}
	}
}
