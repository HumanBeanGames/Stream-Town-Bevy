using UnityEngine;
using TMPro;
using GameResources;

namespace PlayerControls.ObjectSelection
{
    /// <summary>
    /// Handles the display and interaction for selected resources.
    /// </summary>
	public class SelectedResource : SelectedObject
	{
        /// <summary>
        /// Sets the display for the selected resource.
        /// </summary>
        /// <param name="data">The resource data.</param>
		public override void SetDisplay(object data)
		{
			base.SetDisplay(data);
			EnableDisplay();
			SetResourceName();
			UpdateDisplay();
			AttachEvents();
		}

        /// <summary>
        /// Called when the resource is depleted.
        /// </summary>
		public void OnResourceDepleted()
		{

		}

        /// <summary>
        /// Attaches event handlers to the resource.
        /// </summary>
		protected override void AttachEvents()
		{
			ResourceHolder resourceHolder = ((ResourceHolder)_selectedObject);
			resourceHolder.OnAmountChange += UpdateResourceAmount;
		}

        /// <summary>
        /// Detaches event handlers from the resource.
        /// </summary>
		protected override void DetachEvents()
		{
			ResourceHolder resourceHolder = ((ResourceHolder)_selectedObject);
			resourceHolder.OnAmountChange -= UpdateResourceAmount;
		}

        /// <summary>
        /// Detaches the current events.
        /// </summary>
		public void DetachCurrentEvents() { DetachEvents(); }

        /// <summary>
        /// Enables the display.
        /// </summary>
		protected override void EnableDisplay()
		{
			_selectedObjectTypeUI.Title.gameObject.SetActive(true);
			_selectedObjectTypeUI.Description.gameObject.SetActive(true);
			//_selectedObjectTypeUI.RedSlider.gameObject.SetActive(true);
			_selectedObjectTypeUI.ID.gameObject.SetActive(true);
		}

        /// <summary>
        /// Updates the display.
        /// </summary>
		public override void UpdateDisplay()
		{
			ResourceHolder resourceHolder = ((ResourceHolder)_selectedObject);
			UpdateResourceAmount(resourceHolder);
		}

        /// <summary>
        /// Sets the resource name in the UI.
        /// </summary>
		public void SetResourceName()
		{
			_selectedObjectTypeUI.Title.text = (((ResourceHolder)_selectedObject).ResourceType.ToString()).ToUpper();
		}

        /// <summary>
        /// Updates the resource amount display.
        /// </summary>
        /// <param name="resource">The resource holder.</param>
		public void UpdateResourceAmount(ResourceHolder resource)
		{
			_selectedObjectTypeUI.Description.text = "Remaining: " + resource.Amount.ToString();
			// set slider value here (currently doesnt have a max vale)
		}
	}
}
