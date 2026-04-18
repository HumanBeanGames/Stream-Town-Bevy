using UnityEngine;
using UnityEngine.UI;
using TMPro;
using System;
using UnityEngine.Events;

namespace PlayerControls
{
    /// <summary>
    /// UI component for displaying a confirmation dialog.
    /// </summary>
	public class ConfirmCheck : MonoBehaviour
	{
        /// <summary>
        /// The confirm button.
        /// </summary>
		[SerializeField]
		private Button _confirmButton;

        /// <summary>
        /// The deny button.
        /// </summary>
		[SerializeField]
		private Button _denyButton;

        /// <summary>
        /// The title text.
        /// </summary>
		[SerializeField]
		private TMP_Text _title;

        /// <summary>
        /// The description text.
        /// </summary>
		[SerializeField]
		private TMP_Text _description;

        /// <summary>
        /// The UI object.
        /// </summary>
		[SerializeField]
		private GameObject _uIObject;

        /// <summary>
        /// Gets the confirm button.
        /// </summary>
		public Button ConfirmButton => _confirmButton;

        /// <summary>
        /// Gets the deny button.
        /// </summary>
		public Button DenyButton => _denyButton;

        /// <summary>
        /// Enables the confirmation check.
        /// </summary>
		public void EnableCheck()
		{
			_uIObject.SetActive(true);
		}

        /// <summary>
        /// Disables the confirmation check.
        /// </summary>
		public void DisableCheck()
		{
			_uIObject.SetActive(false);
		}

        /// <summary>
        /// Sets the prompt text.
        /// </summary>
        /// <param name="prompt">The title text.</param>
        /// <param name="description">The description text.</param>
		private void SetPrompt(string prompt, string description)
		{
			_title.text = prompt;
			_description.text = description;
		}

        /// <summary>
        /// Sets the confirm button action.
        /// </summary>
        /// <param name="confirm">The confirm action.</param>
		private void SetConfirmFunction(UnityAction confirm)
		{
			_confirmButton?.onClick.AddListener(confirm);
		}

        /// <summary>
        /// Sets the deny button action.
        /// </summary>
        /// <param name="deny">The deny action.</param>
		private void SetDenyFunction(UnityAction deny)
		{
			_denyButton?.onClick.AddListener(deny);
		}

        /// <summary>
        /// Removes all listeners from the buttons.
        /// </summary>
		public void RemoveListeners()
		{
			_confirmButton.onClick.RemoveAllListeners();
			_denyButton.onClick.RemoveAllListeners();
		}

        /// <summary>
        /// Sets up the confirmation check with actions and text.
        /// </summary>
        /// <param name="confirm">The confirm action.</param>
        /// <param name="deny">The deny action.</param>
        /// <param name="title">The title text.</param>
        /// <param name="description">The description text.</param>
		public void SetConfirmCheck(UnityAction confirm, UnityAction deny, string title, string description)
		{
			SetPrompt(title, description);
			SetConfirmFunction(confirm);
			SetDenyFunction(deny);
			EnableCheck();
		}
	}
}
