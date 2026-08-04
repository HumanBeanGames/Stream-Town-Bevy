using UnityEngine;
using UnityEngine.Events;
using Processors;
using System;
using Target;
using Reflex.Attributes;
using UserInterface;

namespace Units
{
	/// <summary>
	/// A component that handles the health and death of an entity.
	/// </summary>
	public class HealthHandler : MonoBehaviour
	{
		[SerializeField]
		private int _maxHealth;
		[SerializeField]
		private float _healthRegen;
		[SerializeField]
		private bool _regenRequiresFood = false;

		[SerializeField]
		private int _healthGainOnLevel = 25;

		private float _healthRegenAccumulated;
		private int _health;

		[SerializeField]
		private UnityEvent _onDeath;
		[SerializeField]
		private UnityEvent _onDamaged;
		[SerializeField]
		private UnityEvent _onHealthChanged;
		[SerializeField]
		private UnityEvent _onRevived;

		private int _baseMaxHealth;

		[Inject] private TownResourceProcessor _resourceProcessor;
		private bool _deathInvoked = false;
		public bool Dead => _health <= 0 ? true : false;
		public int Health => _health;
		public int MaxHealth => _maxHealth;
		public float HealthPercentage => _maxHealth > 0 ? Mathf.Clamp01((float)_health / _maxHealth) : 0f;
		public bool RegenRequiresFood => _regenRequiresFood;
		public void SetRegenRequiresFood(bool value) => _regenRequiresFood = value;
		public Action<bool> OnDeath { get; set; }
		public Action<object> OnDeathObject { get; set; }
		public event Action<HealthHandler> OnHealthChange;
		public event Action<Targetable> OnTookDamage;
		public int BaseMaxHealth => _baseMaxHealth;

		public Action OnRevived { get; set; }

		/// <summary>
		/// Sets the Health to the given value, capped to Max Health.
		/// </summary>
		/// <param name="value"></param>
		public void SetHealth(int value)
		{
			_health = value;
			OnHealthChanged(false);
		}

		/// <summary>
		/// Sets the Max Health to the given value.
		/// </summary>
		/// <param name="value"></param>
		public void SetMaxHealth(int value)
		{
			_maxHealth = value;
			OnHealthChanged(false);
		}

		/// <summary>
		/// Adds the value to the current Health.
		/// </summary>
		/// <param name="value"></param>
		public void ModHealth(int value)
		{
			_health += value;
			OnHealthChanged(false);
		}

		/// <summary>
		/// Subtracs the value from the current health and calls the OnDamaged event.
		/// </summary>
		/// <param name="value"></param>
		public void TakeDamage(int value, Targetable targetable)
		{
			_health -= value;
			_onDamaged?.Invoke();
			OnTookDamage?.Invoke(targetable);
			OnHealthChanged(true);
		}

		/// <summary>
		/// Revives the entity and sets it's Health to its Max Health.
		/// </summary>
		public void Revive()
		{
			SetHealth(_maxHealth);
			_onRevived.Invoke();
			OnRevived?.Invoke();
		}

		public bool TryRevive(Utils.ReviveType type)
		{
			if(_resourceProcessor.TryTakeReviveCost(type))
			{
				Revive();
				return true;
			}
			else
				return false;
		}

		/// <summary>
		/// Sets the value of the entity's Health Regen.
		/// </summary>
		/// <param name="value"></param>
		public void SetHealthRegen(float value)
		{
			_healthRegen = value;
		}

		public void IncreaseHealthByLevel()
		{
			SetMaxHealth(_maxHealth + _healthGainOnLevel);
			ModHealth(_healthGainOnLevel);
		}

		/// <summary>
		/// Called when the current Health is changed.
		/// </summary>
		private void OnHealthChanged(bool damaged)
		{
			if (_health <= 0)
			{
				_health = 0;
				if (!_deathInvoked)
				{
					OnDeath?.Invoke(damaged);
					_onDeath.Invoke();
					_deathInvoked = true;
				}
			}
			else
				_deathInvoked = false;

			if (_health > _maxHealth)
				_health = _maxHealth;

			_onHealthChanged.Invoke();
			OnHealthChange?.Invoke(this);
		}

		/// <summary>
		/// Handles the Health Regen of the entity, accumulating Health Regen overtime.
		/// </summary>
		private void HandleHealthRegen()
		{
			if (!Dead)
				_healthRegenAccumulated += _healthRegen * Time.deltaTime;

			// If enough health regen was accumulated, add it to the current health.
			if (_healthRegenAccumulated >= 1)
			{

				// Round the health regen so that any overflow remains.
				int rounded = Mathf.FloorToInt(_healthRegenAccumulated);
				_healthRegenAccumulated -= rounded;

				if (!_regenRequiresFood)
					ModHealth(rounded);
				else
				{
					if (_resourceProcessor.MoreThanEqualComparison(Utils.Resource.Food, rounded))
					{
						ModHealth(rounded);
						_resourceProcessor.RemoveResource(Utils.Resource.Food, rounded);
					}
				}
			}
		}

		private void Awake()
		{
			_baseMaxHealth = _maxHealth;

			// Unit health bars are runtime presentation attached to the shared health
			// boundary. This ensures players and every enemy type receive the same
			// behaviour without requiring the component to be duplicated across every
			// prefab. Buildings already own their purpose-built, settings-aware display.
			if (GetComponentInChildren<UnitHealthBar>(true) == null &&
				GetComponentInChildren<UserInterface_BuildingHealthBar>(true) == null)
			{
				gameObject.AddComponent<UnitHealthBar>();
			}

			// Don't set health here - let BuildingBase handle it based on construction state
		}

		private void Update()
		{
			HandleHealthRegen();
		}

		private void OnDisable()
		{
			_health = 1;
			_deathInvoked = false;
			SetMaxHealth(_baseMaxHealth);
		}
	}
}
