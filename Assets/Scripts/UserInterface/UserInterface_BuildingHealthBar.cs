using Units;
using UnityEngine;
using UnityEngine.UI;

namespace UserInterface
{
	public class UserInterface_BuildingHealthBar : MonoBehaviour
	{
		private GameObject _displayUI;

		private bool _initialized = false;

		private HealthHandler _healthHandler;

		private Slider _healthBar;

		private BuildingHealthDisplayOption _displayOption;

		private static SettingsData CurrentSettings;

		public bool IsConfigured => _initialized;
		public bool IsVisible => _displayUI != null && _displayUI.activeSelf;
		public float DisplayedHealth => _healthBar != null ? _healthBar.value : 0f;

		private void Start()
		{
			CurrentSettings = SettingsIO.LoadOrCreate();
			UpdateHealthBar();
			CheckWhatDisplayOption();
		}

		/// <summary>
		/// Checks and changes the display option
		/// </summary>
		private void CheckWhatDisplayOption()
		{
			if (!_initialized || CurrentSettings == null)
				return;

			switch (CurrentSettings.displayBuildingDamage)
			{
				case 1:
					_displayOption = BuildingHealthDisplayOption.Damaged;
					break;
				case 2:
					_displayOption = BuildingHealthDisplayOption.Always;
					break;
				default:
					_displayOption = BuildingHealthDisplayOption.None;
					break;
			}

			switch (_displayOption)
			{
				case BuildingHealthDisplayOption.Damaged:
					if (_healthHandler.HealthPercentage < 1)
						_displayUI.SetActive(true);
					else
						_displayUI.SetActive(false);
					break;
				case BuildingHealthDisplayOption.Always:
					_displayUI.SetActive(true);
					break;
				default:
					_displayUI.SetActive(false);
					break;
			}
		}

		/// <summary>
		/// Updates the health bar to equal the health percentage
		/// </summary>
		public void UpdateHealthBar()
		{
			if (!_initialized || _healthBar == null || _healthHandler == null)
				return;

			_healthBar.value = _healthHandler.HealthPercentage;
		}
		
		private void Awake()
		{
			_healthHandler = GetComponentInParent<HealthHandler>();
			_healthBar = GetComponentInChildren<Slider>();
			_displayUI = transform.childCount > 0 ? transform.GetChild(0).gameObject : null;
			_initialized = _healthHandler != null && _healthBar != null && _displayUI != null;

			if (!_initialized)
				Debug.LogWarning($"Building health bar on '{name}' is missing its HealthHandler, Slider, or display root.", this);
		}

		private void Update()
		{
			CheckWhatDisplayOption();
		}
	}

	public enum BuildingHealthDisplayOption
	{
		None,
		Damaged,
		Always
	}
}
