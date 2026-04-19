using ScriptablesProcessorInfrastructure;
using UnityEngine;
using UnityEngine.VFX;
using Utils;
using Reflex.Attributes;
using Reflex.Core;
using Data.Containers;

namespace Processors
{
    /// <summary>
    /// Processor that manages weather system for the game.
    /// Handles weather effects, transitions, and visual effects.
    /// </summary>
	public partial class WeatherProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// Container for season data definitions.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private SeasonDataContainer _seasonDataContainer;

        /// <summary>
        /// ScriptableObject containing season settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private SeasonSettings _seasonSettings;

        /// <summary>
        /// Runtime data ScriptableObject for weather data.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private WeatherRuntimeData _weatherRuntimeData;

        /// <summary>
        /// ScriptableObject containing weather settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private WeatherSettings _weatherSettings;

        /// <summary>
        /// Processor that manages season logic for the game.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private SeasonProcessor _seasonProcessor;

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
			_weatherRuntimeData.RemainingRunTime = Random.Range(seasonData.MinRunTime, seasonData.MaxRunTime);
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
        /// Sets up VFX references for each season.
        /// </summary>
		public void Initialize()
		{
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
			WeatherRuntimeData weatherRuntimeData = ScriptableObject.CreateInstance<WeatherRuntimeData>();
			containerBuilder.AddSingleton(weatherRuntimeData);
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
    }
}
