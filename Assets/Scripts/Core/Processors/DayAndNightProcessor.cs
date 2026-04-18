using UnityEngine;
using Utils;
using UnityEngine.Rendering.Universal;
using UnityEngine.Rendering;
using System;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
	/// <summary>
	/// Processor that manages the day/night cycle for the game.
	/// Controls lighting transitions, material emissions, and day/night events.
	/// </summary>
	public partial class DayAndNightProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// Time processor for accessing time data.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private TimeProcessor _timeProcessor;

		/// <summary>
		/// ScriptableObject containing day/night settings (lighting, materials, etc.).
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private DayAndNightSettingsScriptable _dayAndNightSettings;

		/// <summary>
		/// Runtime data ScriptableObject for day/night data.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private DayAndNightRuntimeData _dayAndNightRuntimeData;

		/// <summary>
		/// Gets whether it is currently daytime.
		/// </summary>
		public bool IsDayTime => _dayAndNightRuntimeData.IsDayTime;

		/// <summary>
		/// Event fired when the daytime period has begun.
		/// </summary>
		public event Action OnDayStarted { add => _dayAndNightRuntimeData.OnDayStarted += value; remove => _dayAndNightRuntimeData.OnDayStarted -= value; }

		/// <summary>
		/// Event fired when the nighttime period has begun.
		/// </summary>
		public event Action OnNightStarted { add => _dayAndNightRuntimeData.OnNightStarted += value; remove => _dayAndNightRuntimeData.OnNightStarted -= value; }

		/// <summary>
		/// Event fired when the daytime period is about to start.
		/// </summary>
		public event Action OnDayStarting { add => _dayAndNightRuntimeData.OnDayStarting += value; remove => _dayAndNightRuntimeData.OnDayStarting -= value; }

		/// <summary>
		/// Event fired when the nighttime period is about to start.
		/// </summary>
		public event Action OnNightStarting { add => _dayAndNightRuntimeData.OnNightStarting += value; remove => _dayAndNightRuntimeData.OnNightStarting -= value; }

		/// <summary>
		/// Initializes the day/night processor.
		/// Resets visual settings to daytime defaults and calculates day/night durations.
		/// </summary>
		public void Initialize()
		{
			// Reset building emission to zero (no glow at start)
			_dayAndNightSettings.BuildingMaterial.SetFloat("_EmissionStrength", 0);
			// Reset post-processing volume to zero (no effects at start)
			_dayAndNightSettings.PostProcessVolume.weight = 0;

			// Calculate day length: total day time * day percentage minus transition time
			_dayAndNightRuntimeData.DayLength = _timeProcessor.SecondsPerDay * _dayAndNightSettings.DayPercentage - _dayAndNightSettings.TransitionLength;
			// Calculate night length: total day time * night percentage minus transition time
			_dayAndNightRuntimeData.NightLength = _timeProcessor.SecondsPerDay * (1 - _dayAndNightSettings.DayPercentage) - _dayAndNightSettings.TransitionLength;
			
			_dayAndNightRuntimeData.TransitionLength = _dayAndNightSettings.TransitionLength;
		}

		/// <summary>
		/// Processes day/night logic every frame.
		/// Called every frame by the Coordinator.
		/// </summary>
		public void Process()
		{
			if (_dayAndNightRuntimeData.IsTransitioning)
			{
				AdvanceTransition();
				return;
			}

			_dayAndNightRuntimeData.TimeTillTransition += Time.deltaTime;

			if (_dayAndNightRuntimeData.IsDayTime)
			{
				if (_dayAndNightRuntimeData.TimeTillTransition > _dayAndNightRuntimeData.DayLength)
					BeginTransition(false);
			}
			else
			{
				if (_dayAndNightRuntimeData.TimeTillTransition > _dayAndNightRuntimeData.NightLength)
					BeginTransition(true);
			}
		}

		/// <summary>
		/// Registers this processor as a singleton in the dependency injection container.
		/// Called by Reflex during container initialization.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		/// <summary>
		/// Injects the DayAndNightRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			DayAndNightRuntimeData dayAndNightRuntimeData = ScriptableObject.CreateInstance<DayAndNightRuntimeData>();
			containerBuilder.AddSingleton(dayAndNightRuntimeData);
		}

		private void BeginTransition(bool transitionToDay)
		{
			_dayAndNightRuntimeData.IsTransitioning = true;
			_dayAndNightRuntimeData.TransitionToDay = transitionToDay;
			_dayAndNightRuntimeData.TransitionTime = 0f;
			_dayAndNightRuntimeData.TimeTillTransition = 0f;

			if (transitionToDay)
				_dayAndNightRuntimeData.InvokeDayStarting();
			else
				_dayAndNightRuntimeData.InvokeNightStarting();
		}

		private void AdvanceTransition()
		{
			_dayAndNightRuntimeData.TransitionTime += Time.deltaTime;
			float progress = _dayAndNightRuntimeData.TransitionLength <= 0f
				? 1f
				: Mathf.Clamp01(_dayAndNightRuntimeData.TransitionTime / _dayAndNightRuntimeData.TransitionLength);
			float t = EaseInOutCubic(progress);

			if (_dayAndNightRuntimeData.TransitionToDay)
			{
				_dayAndNightSettings.MainLightSource.intensity = Mathf.Lerp(_dayAndNightSettings.NightLightIntensity, _dayAndNightSettings.DayLightIntensity, t);
				_dayAndNightSettings.MainLightSource.transform.parent.eulerAngles = new Vector3(0, Mathf.Lerp(-120, 0, t), 0);
				_dayAndNightSettings.BuildingMaterial.SetFloat("_EmissionStrength", Mathf.Lerp(_dayAndNightSettings.MaxEmissionStrength, 0, t));
				_dayAndNightSettings.PostProcessVolume.weight = Mathf.Lerp(1.0f, 0.0f, t);
			}
			else
			{
				_dayAndNightSettings.MainLightSource.intensity = Mathf.Lerp(_dayAndNightSettings.DayLightIntensity, _dayAndNightSettings.NightLightIntensity, t);
				_dayAndNightSettings.MainLightSource.transform.parent.eulerAngles = new Vector3(0, Mathf.Lerp(0, -120, t), 0);
				_dayAndNightSettings.BuildingMaterial.SetFloat("_EmissionStrength", Mathf.Lerp(0, _dayAndNightSettings.MaxEmissionStrength, t));
				_dayAndNightSettings.PostProcessVolume.weight = Mathf.Lerp(0.0f, 1.0f, t);
			}

			if (progress >= 1f)
			{
				_dayAndNightRuntimeData.IsDayTime = _dayAndNightRuntimeData.TransitionToDay;
				_dayAndNightRuntimeData.IsTransitioning = false;
				_dayAndNightRuntimeData.TransitionTime = 0f;
				OnTimeChanged();
			}
		}

		/// <summary>
		/// Fires the appropriate event when day/night changes.
		/// Invokes OnDayStarted or OnNightStarted based on current state.
		/// </summary>
		private void OnTimeChanged()
		{
			if (_dayAndNightRuntimeData.IsDayTime)
				_dayAndNightRuntimeData.InvokeDayStarted();
			else
				_dayAndNightRuntimeData.InvokeNightStarted();
		}

		/// <summary>
		/// Cubic easing function for smooth transitions.
		/// Provides slow-in, slow-out interpolation for natural-looking transitions.
		/// </summary>
		/// <param name="x">Progress value from 0 to 1.</param>
		/// <returns>Eased progress value from 0 to 1.</returns>
		private float EaseInOutCubic(float x)
		{
			// Cubic easing: slow start, fast middle, slow end
			return x < 0.5 ? 4 * x * x * x : 1 - Mathf.Pow(-2 * x + 2, 3) / 2;
		}
	}
}
