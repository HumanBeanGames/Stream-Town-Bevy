using UnityEngine;
using UnityEngine.Events;
using Character;
using System.Collections.Generic;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for ObjectSelectionProcessor.
	/// Manages single object selection, group selection, and selection events.
	/// </summary>
	public class ObjectSelectionRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Event fired when an object is selected.
		/// Passes the selected object and associated data.
		/// </summary>
		[SerializeField]
		private UnityEvent<SelectableObject, object> _onObjectSelected = new UnityEvent<SelectableObject, object>();

		/// <summary>
		/// Whether a group selection operation is in progress.
		/// Set to true when group selection starts, false when it ends.
		/// </summary>
		[SerializeField]
		private bool _startedGroupSelection = false;

		/// <summary>
		/// World position where group selection started.
		/// Used to calculate the selection box area.
		/// </summary>
		[SerializeField]
		private Vector3 _startedSelectionPosition = Vector3.zero;

		/// <summary>
		/// World position where group selection ended.
		/// Used to calculate the selection box area.
		/// </summary>
		[SerializeField]
		private Vector3 _endedSelectionPosition = Vector3.zero;

		/// <summary>
		/// The currently selected object and its associated data.
		/// Tuple containing the selectable object and associated data.
		/// </summary>
		[SerializeField]
		private (SelectableObject, object) _selectedObject;

		/// <summary>
		/// Whether an object is currently selected.
		/// Set to true when an object is selected, false when deselected.
		/// </summary>
		[SerializeField]
		private bool _objectSelected = false;

		/// <summary>
		/// List of selected player characters in the group.
		/// Contains all RoleHandlers that were selected in the group selection.
		/// </summary>
		[SerializeField]
		private List<RoleHandler> _selectedPlayerGroup;

		/// <summary>
		/// Whether a group of players is currently selected.
		/// Set to true when group selection completes, false when cleared.
		/// </summary>
		[SerializeField]
		private bool _groupSelected = false;

		/// <summary>
		/// Gets or sets the event fired when an object is selected.
		/// </summary>
		public UnityEvent<SelectableObject, object> OnObjectSelected { get => _onObjectSelected; set => _onObjectSelected = value; }

		/// <summary>
		/// Gets or sets whether group selection is in progress.
		/// </summary>
		public bool StartedGroupSelection
		{
			get { return _startedGroupSelection; }
			set { _startedGroupSelection = value; }
		}

		/// <summary>
		/// Gets or sets the position where group selection started.
		/// </summary>
		public Vector3 StartedSelectionPosition
		{
			get { return _startedSelectionPosition; }
			set { _startedSelectionPosition = value; }
		}

		/// <summary>
		/// Gets or sets the position where group selection ended.
		/// </summary>
		public Vector3 EndedSelectionPosition
		{
			get { return _endedSelectionPosition; }
			set { _endedSelectionPosition = value; }
		}

		/// <summary>
		/// Gets or sets the currently selected object.
		/// </summary>
		public (SelectableObject, object) SelectedObject
		{
			get { return _selectedObject; }
			set { _selectedObject = value; }
		}

		/// <summary>
		/// Gets or sets whether an object is currently selected.
		/// </summary>
		public bool ObjectSelected
		{
			get { return _objectSelected; }
			set { _objectSelected = value; }
		}

		/// <summary>
		/// Gets or sets the list of selected players in the group.
		/// </summary>
		public List<RoleHandler> SelectedPlayerGroup
		{
			get { return _selectedPlayerGroup; }
			set { _selectedPlayerGroup = value; }
		}

		/// <summary>
		/// Gets or sets whether a group is currently selected.
		/// </summary>
		public bool GroupSelected
		{
			get { return _groupSelected; }
			set { _groupSelected = value; }
		}

		/// <summary>
		/// Initializes the object selection runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
