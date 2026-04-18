using System;
using System.Collections;
using Target;
using Units;
using UnityEngine;

namespace Buildings 
{
    /// <summary>
    /// Handles building damage visualization through material property changes.
    /// Updates the destruction value on the building material based on health percentage.
    /// </summary>
    public class BuildingDamageMaterialHandler : MonoBehaviour 
	{
		private BuildingBase _building;
		private HealthHandler _healthHandler;
		private Renderer _renderer;
		private MaterialPropertyBlock _materialPropertyBlock;
		private bool _initialized = false;

		/// <summary>
		/// Initializes components and sets up health change event subscription.
		/// </summary>
		private void Awake()
		{
			_renderer = GetComponent<Renderer>();
			_building = GetComponentInParent<BuildingBase>();
			_building.DamageHandler = this;
			_healthHandler = GetComponentInParent<HealthHandler>();
			_healthHandler.OnHealthChange += OnHealthChanged;
			_materialPropertyBlock = new MaterialPropertyBlock();
			_initialized = true;
			// Defer material update to reduce synchronous overhead during scene activation
			StartCoroutine(DeferredMaterialUpdate());
		}

		/// <summary>
		/// Defers material update to reduce synchronous overhead during scene activation.
		/// </summary>
		private void OnEnable()
		{
			if (!_initialized)
				return;

			// Defer material update to reduce synchronous overhead during scene activation
			StartCoroutine(DeferredMaterialUpdate());
		}

		/// <summary>
		/// Coroutine that defers material update by one frame.
		/// </summary>
		private IEnumerator DeferredMaterialUpdate()
		{
			yield return null; // Wait one frame to defer the update
			SetDamageByPercentage(_healthHandler.HealthPercentage);
		}

		/// <summary>
		/// Called when the building's health changes.
		/// Updates the damage material based on health percentage.
		/// </summary>
		/// <param name="obj">The health handler that changed.</param>
		public void OnHealthChanged(HealthHandler obj)
		{
			if (_building.BuildingState == Utils.BuildingState.Construction)
				return;

			SetDamageByPercentage(obj.HealthPercentage);
		}

		/// <summary>
		/// Sets the destruction value on the material based on health percentage.
		/// </summary>
		/// <param name="percentage">The health percentage.</param>
		private void SetDamageByPercentage(float percentage)
		{
			//percentage *= 2;
			if (_renderer == null)
				_renderer = GetComponent<Renderer>();
			_renderer.GetPropertyBlock(_materialPropertyBlock);

			_materialPropertyBlock.SetFloat("_DestructionValue", percentage);

			_renderer.SetPropertyBlock(_materialPropertyBlock);
		}
    }
}
