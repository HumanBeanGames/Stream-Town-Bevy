using UnityEngine;
using TMPro;
using Enemies;
using Units;

namespace PlayerControls.ObjectSelection 
{
    /// <summary>
    /// Handles the display and interaction for selected enemies.
    /// </summary>
    public class SelectedEnemy : SelectedObject
	{
        /// <summary>
        /// Sets the display for the selected enemy.
        /// </summary>
        /// <param name="data">The enemy data.</param>
		public override void SetDisplay(object data)
		{
			base.SetDisplay(data);
			AttachEvents();
			EnableDisplay();
			SetEnemyName();
			UpdateDisplay();
		}

        /// <summary>
        /// Attaches event handlers to the enemy.
        /// </summary>
		protected override void AttachEvents()
		{
			Enemy enemy = ((Enemy)_selectedObject);
			enemy.HealthHandler.OnHealthChange += UpdateHealth;
			enemy.HealthHandler.OnDeath += OnDeath;
		}

        /// <summary>
        /// Called when the enemy dies.
        /// </summary>
        /// <param name="died">Whether the enemy died.</param>
		public void OnDeath(bool died)
		{
			DetachCurrentEvents();
			_selectedObjectTypeUI.HideContext();
		}

        /// <summary>
        /// Detaches event handlers from the enemy.
        /// </summary>
		protected override void DetachEvents()
		{
			Enemy enemy = ((Enemy)_selectedObject);
			enemy.HealthHandler.OnHealthChange -= UpdateHealth;
			enemy.HealthHandler.OnDeath -= OnDeath;
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
			_selectedObjectTypeUI.RedSliderContainer.gameObject.SetActive(true);
			_selectedObjectTypeUI.ID.gameObject.SetActive(true);
		}

        /// <summary>
        /// Updates the display.
        /// </summary>
		public override void UpdateDisplay()
		{
			Enemy enemy = ((Enemy)_selectedObject);
			UpdateHealth(enemy.HealthHandler);	
		}

        /// <summary>
        /// Sets the enemy name in the UI.
        /// </summary>
		public void SetEnemyName()
		{
			_selectedObjectTypeUI.Title.text = (((Enemy)_selectedObject).EnemyType.ToString()).ToUpper(); ;
		}

        /// <summary>
        /// Updates the health display.
        /// </summary>
        /// <param name="health">The health handler.</param>
		public void UpdateHealth(HealthHandler health)
		{
			_selectedObjectTypeUI.RedSliderValue.text = health.Health + " / " + health.MaxHealth;
			if (health.Health != 0)
				_selectedObjectTypeUI.RedSlider.value = health.HealthPercentage;
			else
				_selectedObjectTypeUI.RedSlider.value = 0;
		}
	}
}
