using UnityEngine;
using UnityEngine.Events;
using UserInterface;

namespace PlayerControls.ObjectSelection 
{
    /// <summary>
    /// Base class for handling selected object display.
    /// </summary>
    public class SelectedObject
	{
        /// <summary>
        /// The selected object type UI.
        /// </summary>
		[SerializeField]
		protected UserInterface_ObjectSelection _selectedObjectTypeUI;

        /// <summary>
        /// The selected object.
        /// </summary>
        protected object _selectedObject;

        /// <summary>
        /// Action for button click.
        /// </summary>
        protected UnityAction OnButtonClick;

        /// <summary>
        /// Action for second button click.
        /// </summary>
        protected UnityAction OnButtonTwoClick;

        /// <summary>
        /// Action for check confirm.
        /// </summary>
        protected UnityAction OnCheckConfirm;

        /// <summary>
        /// Action for check deny.
        /// </summary>
        protected UnityAction OnCheckDeny;

        /// <summary>
        /// Action for dropdown change.
        /// </summary>
        public UnityAction<int> OnDropDownChange;

        /// <summary>
        /// Gets or sets the selectable object UI.
        /// </summary>
        public UserInterface_ObjectSelection SelectableObjectUI
		{
            get { return _selectedObjectTypeUI; }
            set { _selectedObjectTypeUI = value; }
        }

        /// <summary>
        /// Gets whether the display is enabled.
        /// </summary>
		public bool DisplayEnabled { get { return _selectedObjectTypeUI.gameObject.activeInHierarchy; } }

        /// <summary>
        /// Enables the display.
        /// </summary>
        protected virtual void EnableDisplay(){}

        /// <summary>
        /// Attaches events.
        /// </summary>
        protected virtual void AttachEvents(){}

        /// <summary>
        /// Detaches events.
        /// </summary>
        protected virtual void DetachEvents(){}

        /// <summary>
        /// Disables the confirmation check.
        /// </summary>
        protected void DisableCheck()
        {
            _selectedObjectTypeUI.ConfirmCheck.DisableCheck();
        }

        /// <summary>
        /// Enables the confirmation check.
        /// </summary>
        protected void EnableCheck()
        {
            _selectedObjectTypeUI.ConfirmCheck.EnableCheck();
        }

        /// <summary>
        /// Toggles the display.
        /// </summary>
        public void ToggleDisplay()
        {
            _selectedObjectTypeUI.gameObject.gameObject.SetActive(!_selectedObjectTypeUI.gameObject.activeInHierarchy);
        }

        /// <summary>
        /// Sets the display data.
        /// </summary>
        /// <param name="data">The data to display.</param>
        public virtual void SetDisplay(object data) 
        {
            _selectedObject = data;
        }

        /// <summary>
        /// Updates the display.
        /// </summary>
        public virtual void UpdateDisplay() {}

        /// <summary>
        /// Hides the display.
        /// </summary>
        public virtual void HideDisplay() {}
    }
}
