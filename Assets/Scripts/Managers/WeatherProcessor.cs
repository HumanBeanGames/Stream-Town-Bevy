using ScriptablesProcessorInfrastructure;
using System;
using UnityEngine;
using UnityEngine.VFX;
using Reflex.Core;
using Reflex.Attributes;
using Utils;

namespace Processors
{
    /// <summary>
    /// Processor that manages weather system for the game.
    /// Handles weather effects, transitions, and visual effects.
    /// </summary>
	public partial class WeatherProcessor : MonoBehaviour, IInstaller, IProcessor, IMainThreadInitializableProcessor
	{
        /// <summary>
        /// Container for season data definitions.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private SeasonProcessor _seasonProcessor;
        /// <summary>
        /// The season data container. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private AllSeasonSettings _seasonDataContainer;

        /// <summary>
        /// Runtime data ScriptableObject for weather data.
        /// Created and bound in InjectRuntimeData().
        /// </summary>
		private WeatherRuntimeData _weatherRuntimeData;

        /// <summary>
        /// ScriptableObject containing weather settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private WeatherSettings _weatherSettings;

        /// <summary>
        /// Starts the weather VFX for a specific season.
        /// </summary>
        /// <param name="season">The season to start weather for.</param>
		public void StartWeather(Season season)
		{
			SeasonDataSettings seasonData = _seasonDataContainer.GetSeasonData(season);
			if (seasonData == null)
				return;

			StopWeather();

			_weatherRuntimeData.ActiveSeasonData = seasonData;
			_weatherRuntimeData.CurrentVFX = seasonData.VFX;
			_weatherRuntimeData.RemainingRunTime = UnityEngine.Random.Range(seasonData.MinRunTime, seasonData.MaxRunTime);
			_weatherRuntimeData.ParticleLerpValue = 0f;
			_weatherRuntimeData.WeatherRunning = true;

			if (_weatherRuntimeData.CurrentVFX != null)
			{
				_weatherRuntimeData.CurrentVFX.Play();
				_weatherRuntimeData.CurrentVFX.SetFloat("AmountOfParticles", 0f);
			}
		}

        /// <summary>
        /// Stops all weather effects.
        /// </summary>
		public void StopWeather()
		{
			_weatherRuntimeData.WeatherRunning = false;
			_weatherRuntimeData.RemainingRunTime = 0f;
			_weatherRuntimeData.ParticleLerpValue = 0f;
			_weatherRuntimeData.ActiveSeasonData = null;
			if (_weatherRuntimeData.CurrentVFX != null)
				_weatherRuntimeData.CurrentVFX.Stop();
			_weatherRuntimeData.CurrentVFX = null;
		}

        // Sets the VFX for each season from the weather settings.
		private void SetDataVFX()
		{
			_seasonDataContainer.GetSeasonData(Utils.Season.Summer).VFX = _weatherSettings.SummerVFX;
			_seasonDataContainer.GetSeasonData(Utils.Season.Autumn).VFX = _weatherSettings.AutumnVFX;
			_seasonDataContainer.GetSeasonData(Utils.Season.Winter).VFX = _weatherSettings.WinterVFX;
			_seasonDataContainer.GetSeasonData(Utils.Season.Spring).VFX = _weatherSettings.SpringVFX;
		}

        /// <summary>
        /// Initializes the weather processor.
        /// Creates RuntimeData after all processors are confirmed ready.
        /// Sets up VFX references for each season.
        /// </summary>
		public void Initialize()
		{
			if (_weatherRuntimeData == null)
				throw new InvalidOperationException("WeatherProcessor: WeatherRuntimeData has not been installed.");

			SetDataVFX();
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
		/// Injects the WeatherRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_weatherRuntimeData != null)
				throw new InvalidOperationException("WeatherProcessor: WeatherRuntimeData has already been installed.");

			_weatherRuntimeData = new WeatherRuntimeData();
			containerBuilder.AddSingleton(_weatherRuntimeData);
		}

        /// <summary>
        /// Processes weather logic every frame.
        /// Called every frame by the Coordinator.
        /// </summary>
        public void Process()
        {
            if (!_weatherRuntimeData.WeatherRunning || _weatherRuntimeData.ActiveSeasonData == null)
                return;

            if (_weatherRuntimeData.ParticleLerpValue < 1f)
            {
                float lerpSpeed = _weatherRuntimeData.ActiveSeasonData.ParticleLerpTime <= 0f
                    ? 1f
                    : Time.deltaTime / _weatherRuntimeData.ActiveSeasonData.ParticleLerpTime;
                _weatherRuntimeData.ParticleLerpValue = Mathf.Min(1f, _weatherRuntimeData.ParticleLerpValue + lerpSpeed);
            }

            if (_weatherRuntimeData.CurrentVFX != null)
            {
                _weatherRuntimeData.CurrentVFX.SetFloat(
                    "AmountOfParticles",
                    Mathf.Lerp(0f, _weatherRuntimeData.ActiveSeasonData.MaxParticleCount, _weatherRuntimeData.ParticleLerpValue));
            }

            _weatherRuntimeData.RemainingRunTime -= Time.deltaTime;
            if (_weatherRuntimeData.RemainingRunTime <= 0f)
                StopWeather();
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // WeatherProcessor does not have scene-specific settings to refresh
        }
    }
}
