using UnityEngine.Events;

using ScriptablesProcessorInfrastructure;
using Utils;

namespace Processors
{
	/// <summary>
	/// Runtime data for DebugProcessor.
	/// Manages object selection events and the currently selected debug object.
	/// </summary>
	public class DebugRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Event fired when an object is selected for debugging.
		/// Passes the selected object and associated data.
		/// </summary>
		private UnityEvent<SelectableObject, object> _onObjectSelected;

		/// <summary>
		/// The currently selected object for debugging.
		/// Tuple containing the selectable object and associated data.
		/// </summary>
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
		public DebugRuntimeData()
		{
			_onObjectSelected = new UnityEvent<SelectableObject, object>();
			_selectedObject = (null, null);
		}
	}
}
