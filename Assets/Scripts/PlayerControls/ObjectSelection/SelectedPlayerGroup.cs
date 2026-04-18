using UnityEngine;
using TMPro;
using Character;
using Processors;
using Units;
using UnityEngine.Events;
using System.Collections.Generic;
using Reflex.Attributes;

namespace PlayerControls.ObjectSelection
{
    /// <summary>
    /// Handles the display and interaction for selected player groups.
    /// </summary>
	public class SelectedPlayerGroup : SelectedObject
	{
        /// <summary>
        /// The outline game objects.
        /// </summary>
		private List<GameObject> _outlines;

        /// <summary>
        /// The role processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private RoleProcessor _roleProcessor;

        /// <summary>
        /// The player processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// The object pooling processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingProcessor _poolingProcessor;

        /// <summary>
        /// Sets the display for the selected player group.
        /// </summary>
        /// <param name="data">The player group data.</param>
		public override void SetDisplay(object data)
		{
			base.SetDisplay(data);
			EnableDisplay();
			UpdateDisplay();
			AttachEvents();
		}

        /// <summary>
        /// Called when the recruit role is changed.
        /// </summary>
        /// <param name="index">The role index.</param>
		public void RecruitChange(int index)
		{
			List<RoleHandler> players = ((List<RoleHandler>)_selectedObject);
			for (int i = 0; i < players.Count; i++)
				if (players[i] != null)
					players[i].TrySetRole(_roleProcessor.GetAvailableRoleFromIndex(index));
		}

        /// <summary>
        /// Attaches event handlers to the player group.
        /// </summary>
		protected override void AttachEvents()
		{
			OnDropDownChange += RecruitChange;
			_selectedObjectTypeUI.SelectionDropdown.onValueChanged.AddListener(OnDropDownChange);

			_selectedObjectTypeUI.SelectionDropdown.ClearOptions();

			_selectedObjectTypeUI.SelectionDropdown.AddOptions(_roleProcessor.GetAvailableRolesAsString());
			_selectedObjectTypeUI.SelectionDropdown.SetValueWithoutNotify(_roleProcessor.GetRoleIndex(Utils.PlayerRole.Builder));

			OnButtonClick += OnDismissButtonClick;
			_selectedObjectTypeUI.SelectionButton.onClick.AddListener(OnButtonClick);

			OnCheckConfirm += DisableCheck;
			OnCheckConfirm += DismissRecruits;
			OnCheckDeny += DisableCheck;
		}


        /// <summary>
        /// Called when the dismiss button is clicked.
        /// </summary>
		public void OnDismissButtonClick()
		{
			_selectedObjectTypeUI.ConfirmCheck.SetConfirmCheck(OnCheckConfirm, OnCheckDeny, "Do you wish to mass remove these recruits?", "This action is irreversable and will delete all selected recruits!");
		}
		

        /// <summary>
        /// Dismisses the recruits.
        /// </summary>
		public void DismissRecruits()
		{
			List<RoleHandler> players = ((List<RoleHandler>)_selectedObject);
			for (int i = 0; i < players.Count; i++)
				if (players[i] != null)
					_playerProcessor.DismissRecruit(players[i].Player);
			_selectedObjectTypeUI.HideContext();
		}

        /// <summary>
        /// Detaches the current events.
        /// </summary>
		public void DetachCurrentEvents() { DetachEvents(); }

        /// <summary>
        /// Detaches event handlers from the player group.
        /// </summary>
		protected override void DetachEvents()
		{
			OnDropDownChange -= RecruitChange;
			_selectedObjectTypeUI.SelectionDropdown.onValueChanged.RemoveAllListeners();

			OnButtonClick -= OnDismissButtonClick;
			_selectedObjectTypeUI.SelectionButton.onClick.RemoveAllListeners();

			for (int i = 0; i < _outlines.Count; i++)
			{
				_outlines[i].SetActive(false);
			}
			_outlines.Clear();

			OnCheckConfirm -= DisableCheck;
			OnCheckConfirm -= DismissRecruits;
			OnCheckDeny -= DisableCheck;
			_selectedObjectTypeUI.ConfirmCheck.RemoveListeners();
		}

        /// <summary>
        /// Enables the display.
        /// </summary>
		protected override void EnableDisplay()
		{
			_selectedObjectTypeUI.DropdownHolder.gameObject.SetActive(true);
			_selectedObjectTypeUI.Description.gameObject.SetActive(true);
			_selectedObjectTypeUI.ButtonHolder.gameObject.SetActive(true);
		}

        /// <summary>
        /// Updates the display.
        /// </summary>
		public override void UpdateDisplay()
		{
			List<RoleHandler> players = ((List<RoleHandler>)_selectedObject);
			_selectedObjectTypeUI.Description.text = $"Mass Selection: {players.Count}";
			_selectedObjectTypeUI.SelectionButtonText.text = "Mass Dismiss";

			_outlines = new List<GameObject>();
			for (int i = 0; i < players.Count; i++)
			{
				_outlines.Add(_poolingProcessor.GetPooledObject("UI_Selection_Outline").gameObject);
				BoxCollider collider = players[i].GetComponent<BoxCollider>();
				_outlines[i].transform.position = new Vector3(collider.transform.position.x, 0.15f, collider.transform.position.z);
				_outlines[i].transform.rotation = collider.transform.rotation;
				_outlines[i].transform.parent = collider.transform;
				_outlines[i].transform.localScale = new Vector3(collider.size.x * 1.25f, 1, collider.size.z * 1.25f);
				_outlines[i].SetActive(true);
			}
		}
	}
}
