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
    /// Handles the display and interaction for selected players.
    /// </summary>
	public class SelectedPlayer : SelectedObject
	{
        /// <summary>
        /// The role processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private RoleProcessor _roleProcessor;

        /// <summary>
        /// The player processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// Sets the display for the selected player.
        /// </summary>
        /// <param name="data">The player data.</param>
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
			RoleHandler roleHandler = ((RoleHandler)_selectedObject);
			roleHandler.TrySetRole(_roleProcessor.GetAvailableRoleFromIndex(index));
			UpdateExperience(roleHandler);
		}

        /// <summary>
        /// Attaches event handlers to the player.
        /// </summary>
		protected override void AttachEvents()
		{
			RoleHandler roleHandler = ((RoleHandler)_selectedObject);
			roleHandler.PlayerRoleData.OnExperienceChange += UpdateExperience;
			roleHandler.PlayerRoleData.HealthHandler.OnHealthChange += UpdateHealth;
			roleHandler.OnRoleChanged += UpdateRole;

			// Is player a recruit?
			if (string.IsNullOrEmpty(roleHandler.Player.TwitchUser.Username) || roleHandler.Player.TwitchUser.TwitchUserType == TwitchLib.Client.Enums.UserType.Broadcaster)
			{
				_selectedObjectTypeUI.DropdownHolder.SetActive(true);
				OnDropDownChange += RecruitChange;
				_selectedObjectTypeUI.SelectionDropdown.onValueChanged.AddListener(OnDropDownChange);

				_selectedObjectTypeUI.SelectionDropdown.ClearOptions();

				_selectedObjectTypeUI.SelectionDropdown.AddOptions(_roleProcessor.GetAvailableRolesAsString());
				_selectedObjectTypeUI.SelectionDropdown.SetValueWithoutNotify(_roleProcessor.GetRoleIndex(roleHandler.CurrentRole));

				if (string.IsNullOrEmpty(roleHandler.Player.TwitchUser.Username))
				{
					OnButtonClick += OnDismissButtonClick;
					_selectedObjectTypeUI.SelectionButton.onClick.AddListener(OnButtonClick);
					_selectedObjectTypeUI.SelectionButtonText.text = "Dismiss";
					_selectedObjectTypeUI.ButtonHolder.SetActive(true);

					OnCheckConfirm += DisableCheck;
					OnCheckConfirm += DismissRecruits;
					OnCheckDeny += DisableCheck;
;				}
			}

			UpdateExperience(roleHandler);
			roleHandler.PlayerRoleData.HealthHandler.OnDeath += OnPlayerDeathOrRemove;
		}

        /// <summary>
        /// Called when the dismiss button is clicked.
        /// </summary>
		public void OnDismissButtonClick()
		{
			_selectedObjectTypeUI.ConfirmCheck.SetConfirmCheck(OnCheckConfirm, OnCheckDeny, "Do you wish to Dismiss this recruit?", "This action is irreversable and will delete this recruit!");	
		}

        /// <summary>
        /// Dismisses the recruit.
        /// </summary>
		public void DismissRecruits()
		{
			RoleHandler roleHandler = ((RoleHandler)_selectedObject);
			OnPlayerDeathOrRemove(true);
			_playerProcessor.DismissRecruit(roleHandler.Player);
		}

        /// <summary>
        /// Called when the player dies or is removed.
        /// </summary>
        /// <param name="died">Whether the player died.</param>
		public void OnPlayerDeathOrRemove(bool died)
		{
			DetachCurrentEvents();
			_selectedObjectTypeUI.HideContext();
		}

        /// <summary>
        /// Detaches the current events.
        /// </summary>
		public void DetachCurrentEvents() { DetachEvents(); }

        /// <summary>
        /// Detaches event handlers from the player.
        /// </summary>
		protected override void DetachEvents()
		{
			RoleHandler roleHandler = ((RoleHandler)_selectedObject);
			roleHandler.PlayerRoleData.OnExperienceChange -= UpdateExperience;
			roleHandler.PlayerRoleData.HealthHandler.OnHealthChange -= UpdateHealth;
			roleHandler.OnRoleChanged -= UpdateRole;

			// Is player a recruit
			if (string.IsNullOrEmpty(roleHandler.Player.TwitchUser.Username) || roleHandler.Player.TwitchUser.TwitchUserType == TwitchLib.Client.Enums.UserType.Broadcaster)
			{
				OnDropDownChange -= RecruitChange;
				_selectedObjectTypeUI.SelectionDropdown.onValueChanged.RemoveAllListeners();

				if (string.IsNullOrEmpty(roleHandler.Player.TwitchUser.Username))
				{
					OnButtonClick -= OnDismissButtonClick;
					_selectedObjectTypeUI.SelectionButton.onClick.RemoveAllListeners();

					OnCheckConfirm -= DisableCheck;
					OnCheckConfirm -= DismissRecruits;
					OnCheckDeny -= DisableCheck;
					_selectedObjectTypeUI.ConfirmCheck.RemoveListeners();
				}
			}
		}

        /// <summary>
        /// Enables the display.
        /// </summary>
		protected override void EnableDisplay()
		{
			_selectedObjectTypeUI.Title.gameObject.SetActive(true);
			_selectedObjectTypeUI.RoleContainer.gameObject.SetActive(true);
			_selectedObjectTypeUI.Description.gameObject.SetActive(true);
			_selectedObjectTypeUI.RedSliderContainer.gameObject.SetActive(true);
			_selectedObjectTypeUI.GreenSliderContainer.gameObject.SetActive(true);
			_selectedObjectTypeUI.ID.gameObject.SetActive(true);
		}

        /// <summary>
        /// Updates the display.
        /// </summary>
		public override void UpdateDisplay()
		{
			RoleHandler roleHandler = ((RoleHandler)_selectedObject);
			UpdatePlayerName(roleHandler);
			UpdateHealth(roleHandler.PlayerRoleData.HealthHandler);
			UpdateRole(roleHandler);
			UpdateExperience(roleHandler);
			UpdateActivity(roleHandler);
		}

        /// <summary>
        /// Updates the player name display.
        /// </summary>
        /// <param name="roleHandler">The role handler.</param>
		public void UpdatePlayerName(RoleHandler roleHandler)
		{
			if (string.IsNullOrEmpty(roleHandler.Player.TwitchUser.Username))
				_selectedObjectTypeUI.Title.text = "RECRUIT";
			else
				_selectedObjectTypeUI.Title.text = (roleHandler.Player.TwitchUser.Username).ToUpper();
		}

        /// <summary>
        /// Updates the health display.
        /// </summary>
        /// <param name="health">The health handler.</param>
		public void UpdateHealth(HealthHandler health)
		{
			_selectedObjectTypeUI.RedSliderValue.text = health.Health + " / " + health.MaxHealth;
			if (health.Health != 0.0f)
				_selectedObjectTypeUI.RedSlider.value = health.HealthPercentage;
			else
				_selectedObjectTypeUI.RedSlider.value = 0.0f;
		}

        /// <summary>
        /// Updates the role display.
        /// </summary>
        /// <param name="roleHandler">The role handler.</param>
		public void UpdateRole(RoleHandler roleHandler)
		{
			_selectedObjectTypeUI.Role.text = "Role: " + roleHandler.CurrentRole.ToString();
			_selectedObjectTypeUI.RoleImage.sprite = _roleProcessor.AllRoleData.RoleDataDictionary[roleHandler.CurrentRole].DisplayIcon;
		}

        /// <summary>
        /// Updates the experience display.
        /// </summary>
        /// <param name="roleHandler">The role handler.</param>
		public void UpdateExperience(RoleHandler roleHandler)
		{
			_selectedObjectTypeUI.Description.text = "Lvl. " + roleHandler.PlayerRoleData.CurrentLevel + " /  Lvl. " + RoleProcessor.MAX_ROLE_LEVEL;
			_selectedObjectTypeUI.GreenSliderValue.text = roleHandler.PlayerRoleData.CurrentExp + " / " + roleHandler.PlayerRoleData.RequiredExp;
			if (roleHandler.PlayerRoleData.CurrentExp != 0.0f)
				_selectedObjectTypeUI.GreenSlider.value = ((float)roleHandler.PlayerRoleData.CurrentExp / roleHandler.PlayerRoleData.RequiredExp);
			else
				_selectedObjectTypeUI.GreenSlider.value = 0.0f;
		}

        /// <summary>
        /// Updates the activity display.
        /// </summary>
        /// <param name="roleHandler">The role handler.</param>
		public void UpdateActivity(RoleHandler roleHandler)
		{
			// Get if the player is currently active and  enable role switching is inactive
		}
	}
}
