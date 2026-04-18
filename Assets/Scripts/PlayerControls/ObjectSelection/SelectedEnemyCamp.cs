using UnityEngine;
using TMPro;
using Buildings;
using Units;

namespace PlayerControls.ObjectSelection 
{
    /// <summary>
    /// Handles the display and interaction for selected enemy camps.
    /// </summary>
    public class SelectedEnemyCamp : SelectedObject
	{
        /// <summary>
        /// Sets the display for the selected enemy camp.
        /// </summary>
        /// <param name="data">The enemy camp data.</param>
		public override void SetDisplay(object data)
		{
			base.SetDisplay(data);
			EnableDisplay();
			SetEnemyCampName();
			//SetEnemyCampID();
			UpdateDisplay();
		}

        /// <summary>
        /// Enables the display.
        /// </summary>
		protected override void EnableDisplay()
		{
			_selectedObjectTypeUI.RedSliderContainer.gameObject.SetActive(true);
			_selectedObjectTypeUI.Title.gameObject.SetActive(true);
			_selectedObjectTypeUI.ID.gameObject.SetActive(true);
		}

        /// <summary>
        /// Updates the display.
        /// </summary>
		public override void UpdateDisplay()
		{
			Station station = ((Station)_selectedObject);
			UpdateHealth(station);
		}

        /// <summary>
        /// Updates the health display.
        /// </summary>
        /// <param name="station">The station.</param>
		public void UpdateHealth(Station station)
		{
			HealthHandler health = station.transform.GetComponent<HealthHandler>();
			_selectedObjectTypeUI.RedSliderValue.text = health.Health + " / " + health.MaxHealth;
			if (health.Health != 0)
				_selectedObjectTypeUI.RedSlider.value = (health.Health / health.MaxHealth);
			else
				_selectedObjectTypeUI.RedSlider.value = 0;
		}

        /// <summary>
        /// Sets the enemy camp name in the UI.
        /// </summary>
		private void SetEnemyCampName()
		{
			_selectedObjectTypeUI.Title.text = (((Station)_selectedObject).Flags.ToString()).ToUpper();
		}

        /// <summary>
        /// Sets the enemy camp ID in the UI.
        /// </summary>
		private void SetEnemyCampID()
		{
			_selectedObjectTypeUI.Title.text = ((Station)_selectedObject).Flags.ToString();
		}
	}
}
