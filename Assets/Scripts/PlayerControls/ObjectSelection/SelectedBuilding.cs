using UnityEngine;
using TMPro;
using Buildings;
using Units;
using Level;
using Processors;
using GameResources;
using Reflex.Attributes;

namespace PlayerControls.ObjectSelection
{
    /// <summary>
    /// Handles the display and interaction for selected buildings.
    /// </summary>
	public class SelectedBuilding : SelectedObject
	{
        /// <summary>
        /// The town resource processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private TownResourceProcessor _townResourceProcessor;

        /// <summary>
        /// The building processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private BuildingProcessor _buildingProcessor;

        /// <summary>
        /// Sets the display for the selected building.
        /// </summary>
        /// <param name="data">The building data.</param>
		public override void SetDisplay(object data)
		{
			base.SetDisplay(data);
			EnableDisplay();
			UpdateDisplay();
			SetBuildingName();
			AttachEvents();
			BuildingBase building = (BuildingBase)_selectedObject;
			if (building.BuildingType != Utils.BuildingType.Townhall)
			{
				_selectedObjectTypeUI.ButtonHolder.gameObject.SetActive(true);
				_selectedObjectTypeUI.SelectionButtonText.text = "Remove";
			}

			if (building.LevelHandler != null)
			{
				if (building.LevelHandler.Level == building.LevelHandler.MaxLevel)
				{
					_selectedObjectTypeUI.SelectionButtonTextTwo.text = "Max Level";
					_selectedObjectTypeUI.SelectionButtonTwo.interactable = false;
				}

				else
					_selectedObjectTypeUI.SelectionButtonTextTwo.text = "Level Up";

				if (_buildingProcessor.GetBuildingData(building.BuildingType).CanLevel && building.LevelHandler.CanLevel())
					_selectedObjectTypeUI.SelectionButtonTwo.interactable = true;
				else
					_selectedObjectTypeUI.SelectionButtonTwo.interactable = false;
			}
			else
				_selectedObjectTypeUI.ButtonHolderTwo.SetActive(false);
			//_selectedObjectTypeUI.Description.gameObject.SetActive(false);
		}

        /// <summary>
        /// Attaches event handlers to the building.
        /// </summary>
		protected override void AttachEvents()
		{
			BuildingBase building = (BuildingBase)_selectedObject;
			if (building.HealthHandler != null)
				building.HealthHandler.OnHealthChange += UpdateBuildingHealth;
			if (building.LevelHandler != null)
				building.LevelHandler.OnLeveledUp += UpdateBuildingLevel;

			if (building.BuildingType != Utils.BuildingType.Townhall)
			{
				OnButtonClick += OnRemoveButtonClick;
				_selectedObjectTypeUI.SelectionButton.onClick.AddListener(OnButtonClick);
				OnCheckConfirm += DisableCheck;
				OnCheckConfirm += RemoveBuilding;
				OnCheckDeny += DisableCheck;
			}
			if (building.BuildingType == Utils.BuildingType.Marketplace)
			{
				_selectedObjectTypeUI.RoleContainer.SetActive(true);
				PassiveResourceIncrementer incrementer = building.GetComponent<PassiveResourceIncrementer>();
				UpdateMarketPlaceRate(incrementer);
				incrementer.OnRateChange += UpdateMarketPlaceRate;
				_selectedObjectTypeUI.RoleImage.gameObject.SetActive(false);
				// Currently does not update to represent changing market values
			}

			OnButtonTwoClick += OnBuildingLevelUp;
			_selectedObjectTypeUI.SelectionButtonTwo.onClick.AddListener(OnButtonTwoClick);
			_townResourceProcessor.OnAnyResourceChangeEvent += OnResourcesAdded;
		}

        /// <summary>
        /// Called when resources are added.
        /// </summary>
        /// <param name="resource">The resource type.</param>
        /// <param name="amount">The amount added.</param>
		private void OnResourcesAdded(Utils.Resource resource, int amount, bool purchase)
		{
			BuildingBase building = (BuildingBase)_selectedObject;
			if (building.LevelHandler != null)
				if (building.LevelHandler.CanLevel())
					_selectedObjectTypeUI.SelectionButtonTwo.interactable = true;
		}

        /// <summary>
        /// Detaches event handlers from the building.
        /// </summary>
		protected override void DetachEvents()
		{
			BuildingBase building = (BuildingBase)_selectedObject;
			if (building.HealthHandler != null)
				building.HealthHandler.OnHealthChange -= UpdateBuildingHealth;
			if (building.LevelHandler != null)
				building.LevelHandler.OnLeveledUp -= UpdateBuildingLevel;

			if (building.BuildingType != Utils.BuildingType.Townhall)
			{
				OnButtonClick -= OnRemoveButtonClick;
				_selectedObjectTypeUI.SelectionButton.onClick.RemoveAllListeners();

			}
			else
			{
				OnCheckConfirm -= DisableCheck;
				OnCheckConfirm -= RemoveBuilding;
				OnCheckDeny -= DisableCheck;
			}
				_selectedObjectTypeUI.ConfirmCheck.RemoveListeners();
			if(building.BuildingType == Utils.BuildingType.Marketplace)
				building.GetComponent<PassiveResourceIncrementer>().OnRateChange -= UpdateMarketPlaceRate;

			_selectedObjectTypeUI.RoleImage.gameObject.SetActive(true);
			OnButtonTwoClick -= OnBuildingLevelUp;
			_selectedObjectTypeUI.SelectionButtonTwo.onClick.RemoveAllListeners();
			_townResourceProcessor.OnAnyResourceChangeEvent -= OnResourcesAdded;
		}

        /// <summary>
        /// Called when the remove button is clicked.
        /// </summary>
		public void OnRemoveButtonClick()
		{
			_selectedObjectTypeUI.ConfirmCheck.SetConfirmCheck(OnCheckConfirm, OnCheckDeny, "Do you wish to remove this building?", "This action is irreversable and will delete this building!");
		}

        /// <summary>
        /// Removes the building.
        /// </summary>
		public void RemoveBuilding()
		{
			BuildingBase building = (BuildingBase)_selectedObject;
			if (_buildingProcessor.TryRemoveBuilding(building))
				_selectedObjectTypeUI.HideContext();
		}

        /// <summary>
        /// Called when the building levels up.
        /// </summary>
		public void OnBuildingLevelUp()
		{
			BuildingBase building = (BuildingBase)_selectedObject;
			if (!_buildingProcessor.CanLevelBuilding(building))
				_selectedObjectTypeUI.SelectionButtonTwo.interactable = false;
			else if (building.LevelHandler.Level == building.LevelHandler.MaxLevel)
			{
				_selectedObjectTypeUI.SelectionButtonTextTwo.text = "Max Level";
				_selectedObjectTypeUI.SelectionButtonTwo.interactable = false;
			}
		}

        /// <summary>
        /// Detaches the current events.
        /// </summary>
		public void DetachCurrentEvents() { DetachEvents(); }

        /// <summary>
        /// Updates the display.
        /// </summary>
		public override void UpdateDisplay()
		{
			BuildingBase buildingBase = (BuildingBase)_selectedObject;
			if (buildingBase.HealthHandler != null)
				UpdateBuildingHealth(buildingBase.HealthHandler);
			UpdateBuildingLevel(buildingBase.LevelHandler);
		}

        /// <summary>
        /// Enables the display.
        /// </summary>
		protected override void EnableDisplay()
		{
			_selectedObjectTypeUI.Title.gameObject.SetActive(true);
			_selectedObjectTypeUI.Description.gameObject.SetActive(true);
			_selectedObjectTypeUI.RedSliderContainer.gameObject.SetActive(true);
			_selectedObjectTypeUI.ID.gameObject.SetActive(true);
			_selectedObjectTypeUI.ButtonHolderTwo.gameObject.SetActive(true);
		}

        /// <summary>
        /// Sets the building name in the UI.
        /// </summary>
		private void SetBuildingName()
		{
			_selectedObjectTypeUI.Title.text = (((BuildingBase)_selectedObject).BuildingType.ToString()).ToUpper();
		}

        /// <summary>
        /// Updates the building health display.
        /// </summary>
        /// <param name="health">The health handler.</param>
		public void UpdateBuildingHealth(HealthHandler health)
		{
			_selectedObjectTypeUI.RedSliderValue.text = health.Health + " / " + health.MaxHealth;
			if (health.Health != 0)
				_selectedObjectTypeUI.RedSlider.value = health.HealthPercentage;
			else
				_selectedObjectTypeUI.RedSlider.value = 0;
		}

        /// <summary>
        /// Updates the building level display.
        /// </summary>
        /// <param name="level">The level handler.</param>
		public void UpdateBuildingLevel(LevelHandler level)
		{
			if (level != null)
				_selectedObjectTypeUI.Description.text = "Lvl. " + level.Level + " / Lvl. " + level.MaxLevel;
			else
				_selectedObjectTypeUI.Description.text = "";

		}

        /// <summary>
        /// Updates the marketplace rate display.
        /// </summary>
        /// <param name="incrementor">The passive resource incrementer.</param>
		public void UpdateMarketPlaceRate(PassiveResourceIncrementer incrementor)
		{
			_selectedObjectTypeUI.Role.text = incrementor.GetInformation();
		}
	}
}
