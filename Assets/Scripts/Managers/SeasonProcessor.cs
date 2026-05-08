using System;
using UnityEngine;
using Utils;
using ScriptablesProcessorInfrastructure;
using Reflex.Core;
using Reflex.Attributes;

namespace Processors
{
    /// <summary>
    /// Processor that manages the game's season system.
    /// Handles season transitions, visual effects, and season-related events.
    /// </summary>
    public partial class SeasonProcessor : MonoBehaviour, IInstaller, IProcessor, IPostInitializeProcessor, IMainThreadInitializableProcessor
    {
        /// <summary>
        /// Container for season data definitions.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TimeProcessor _timeProcessor;
        /// <summary>
        /// The season data container. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private AllSeasonSettings _seasonDataContainer;

        /// <summary>
        /// The debug processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Processors.DebugProcessor _debugProcessor;

        /// <summary>
        /// Nested runtime data class for season data.
        /// Created and bound in InjectRuntimeData().
        /// </summary>
        private SeasonRuntimeData _seasonRuntimeData;

        /// <summary>
        /// Gets or sets the days per season.
        /// </summary>
        public int DaysPerSeason
        {
            get => _seasonRuntimeData.DaysPerSeason;
            set => _seasonRuntimeData.DaysPerSeason = value;
        }

        /// <summary>
        /// Event invoked when the season has changed.
        /// </summary>
		public event Action<Season> OnSeasonChanged
		{
			add => _seasonRuntimeData.OnSeasonChanged += value;
			remove => _seasonRuntimeData.OnSeasonChanged -= value;
		}

        /// <summary>
        /// Event invoked when the season is about to change.
        /// </summary>
		public event Action<Season> OnSeasonChanging
		{
			add => _seasonRuntimeData.OnSeasonChanging += value;
			remove => _seasonRuntimeData.OnSeasonChanging -= value;
		}

        /// <summary>
        /// Gets the number of days per season.
        /// </summary>
        /// <returns>The number of days per season.</returns>
		public int GetDaysPerSeason()
		{
			return _seasonDataContainer.DaysPerSeason;
		}

        /// <summary>
        /// Gets the current season.
        /// </summary>
        /// <returns>The current season.</returns>
		public Season GetCurrentSeason()
		{
			return _seasonRuntimeData.CurrentSeason;
		}

        /// <summary>
        /// Gets all season data.
        /// </summary>
        /// <returns>Array of all season data configurations.</returns>
		public SeasonDataSettings[] GetAllSeasonsData()
		{
			return _seasonDataContainer.SeasonSettingsArray;
		}

		public void CallNextSeason()
		{
			NextSeason(0);
		}

		public void UpdateCurrentSeason()
		{
			BeginTransition(_seasonRuntimeData.CurrentSeason, 0f, false);
		}

		/// <summary>
		/// Called when a day has passed.
		/// Checks if it's time to transition to the next season.
		/// </summary>
		private void OnDayPassed()
		{
			if ((_timeProcessor.DayCount) % _seasonDataContainer.DaysPerSeason == 0)
				NextSeason();
		}

		/// <summary>
		/// Starts transition to next season.
		/// </summary>
		/// <param name="_transitionTime">Optional override for transition time.</param>
		private void NextSeason(float _transitionTime = -1)
		{
			_debugProcessor.Log(DebugLogCategory.SeasonProcessor, "Next Season Called");
			Season nextSeason = _seasonRuntimeData.CurrentSeason + 1;

			if (nextSeason == Season.Count)
				nextSeason = 0;

			if (!_seasonRuntimeData.SeasonChanging)
				BeginTransition(nextSeason, _transitionTime == -1 ? _seasonDataContainer.SeasonTransitionTime : _transitionTime, true);
		}

		/// <summary>
		/// Forces an immediate transition to the next season without animation.
		/// Used for editor testing or instant season changes.
		/// </summary>
		public void ForceSetNextSeason()
		{
			Season nextSeason = _seasonRuntimeData.CurrentSeason + 1;

			if (nextSeason == Season.Count)
				nextSeason = 0;
			SeasonDataSettings currentSeasonData = _seasonDataContainer.GetSeasonData(_seasonRuntimeData.CurrentSeason);
			SeasonDataSettings nextSeasonData = _seasonDataContainer.GetSeasonData(nextSeason);
			// Grass Values.
			if (_seasonDataContainer.GrassMaterial)
			{
				_seasonDataContainer.GrassMaterial.SetColor("_GridColor1", Color.Lerp(currentSeasonData.GrassGridColor1, nextSeasonData.GrassGridColor1, 1));
				_seasonDataContainer.GrassMaterial.SetColor("_GridColor2", Color.Lerp(currentSeasonData.GrassGridColor2, nextSeasonData.GrassGridColor2, 1));
				_seasonDataContainer.GrassMaterial.SetColor("_TopColor", Color.Lerp(currentSeasonData.GrassTopColor, nextSeasonData.GrassTopColor, 1));
				_seasonDataContainer.GrassMaterial.SetColor("_WindColor", Color.Lerp(currentSeasonData.GrassWindColor, nextSeasonData.GrassWindColor, 1));
			}

			// Terrain Values.
			if (_seasonDataContainer.TerrainMaterial)
			{
				_seasonDataContainer.TerrainMaterial.SetColor("_color1", Color.Lerp(currentSeasonData.TerrainColor1, nextSeasonData.TerrainColor1, 1));
				_seasonDataContainer.TerrainMaterial.SetColor("_color2", Color.Lerp(currentSeasonData.TerrainColor2, nextSeasonData.TerrainColor2, 1));
			}

			// Tree Values.
			SetSeasonMaterial(nextSeason, 1, _seasonDataContainer);
			_seasonRuntimeData.CurrentSeason = nextSeason;
		}

		/// <summary>
		/// Sets the current season to the specified season without transition.
		/// </summary>
		/// <param name="selectedSeason">The season to set.</param>
		public void SetSeason(Season selectedSeason)
		{
			SeasonDataSettings selectedSeasonData = _seasonDataContainer.GetSeasonData(selectedSeason);
			// Grass Values.
			if (_seasonDataContainer.GrassMaterial)
			{
				_seasonDataContainer.GrassMaterial.SetColor("_GridColor1", selectedSeasonData.GrassGridColor1);
				_seasonDataContainer.GrassMaterial.SetColor("_GridColor2", selectedSeasonData.GrassGridColor2);
				_seasonDataContainer.GrassMaterial.SetColor("_TopColor", selectedSeasonData.GrassTopColor);
				_seasonDataContainer.GrassMaterial.SetColor("_WindColor", selectedSeasonData.GrassWindColor);
			}

			// Terrain Values.
			if (_seasonDataContainer.TerrainMaterial)
			{
				_seasonDataContainer.TerrainMaterial.SetColor("_color1", selectedSeasonData.TerrainColor1);
				_seasonDataContainer.TerrainMaterial.SetColor("_color2", selectedSeasonData.TerrainColor2);
			}
		}

		/// <summary>
		/// Sets the season based on the time passed (day count).
		/// </summary>
		public void SetSeasonByTimePassed()
		{
			_seasonRuntimeData.CurrentSeason = (Season)(_timeProcessor.DayCount % _seasonDataContainer.DaysPerSeason);
			SetSeasonMaterial(_seasonRuntimeData.CurrentSeason, 1.0f, _seasonDataContainer);
		}

		public void SetSeasonByTimePassed(float timePassed)
		{
			int totalDays = _timeProcessor.CalculateDayCount(timePassed);
			_seasonRuntimeData.CurrentSeason = (Season)((totalDays / _seasonDataContainer.DaysPerSeason) % (int)Season.Count);
			SetSeasonMaterial(_seasonRuntimeData.CurrentSeason, 1.0f, _seasonDataContainer);
		}

        /// <summary>
        /// Initializes the season processor.
        /// Creates RuntimeData after all processors are confirmed ready.
        /// Sets starting season and subscribes to day passed event.
        /// </summary>
        		public void Initialize()
		{
			if (_seasonRuntimeData == null)
				throw new InvalidOperationException("SeasonProcessor: SeasonRuntimeData has not been installed.");

			_seasonRuntimeData.CurrentSeason = _seasonDataContainer.StartingSeason;
            BeginTransition(_seasonRuntimeData.CurrentSeason, 0f, false);
        }

        public void Activate()
        {
            _timeProcessor.DayPassed += OnDayPassed;
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

        		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_seasonRuntimeData != null)
				throw new InvalidOperationException("SeasonProcessor: SeasonRuntimeData has already been installed.");

			_seasonRuntimeData = new SeasonRuntimeData();
			containerBuilder.AddSingleton(_seasonRuntimeData);
		}

        /// <summary>
        /// Processes season logic every frame.
        /// Called every frame by the Coordinator.
        /// SeasonProcessor does not require per-frame updates.
        /// </summary>
        public void Process()
        {
            AdvanceTransition();
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // SeasonProcessor does not have scene-specific settings to refresh
        }

		private void BeginTransition(Season nextSeason, float transitionTime, bool triggerEvent)
		{
			_seasonRuntimeData.TransitionFromSeason = _seasonRuntimeData.CurrentSeason;
			_seasonRuntimeData.TransitionToSeason = nextSeason;
			_seasonRuntimeData.TransitionElapsed = 0f;
			_seasonRuntimeData.TransitionDuration = Mathf.Max(0f, transitionTime);
			_seasonRuntimeData.TransitionTriggerEvent = triggerEvent;
			_seasonRuntimeData.SeasonChanging = true;

			if (triggerEvent)
				_seasonRuntimeData.InvokeSeasonChanging(nextSeason);

			if (_seasonRuntimeData.TransitionDuration <= 0f)
			{
				ApplyTransition(1f);
				CompleteTransition();
			}
		}

		private void AdvanceTransition()
		{
			if (!_seasonRuntimeData.SeasonChanging)
				return;

			if (_seasonRuntimeData.TransitionDuration <= 0f)
				return;

			_seasonRuntimeData.TransitionElapsed += Time.deltaTime;
			float transition = Mathf.Clamp01(_seasonRuntimeData.TransitionElapsed / _seasonRuntimeData.TransitionDuration);
			ApplyTransition(transition);

			if (transition >= 1f)
				CompleteTransition();
		}

		private void ApplyTransition(float transition)
		{
			SeasonDataSettings currentSeasonData = _seasonDataContainer.GetSeasonData(_seasonRuntimeData.TransitionFromSeason);
			SeasonDataSettings nextSeasonData = _seasonDataContainer.GetSeasonData(_seasonRuntimeData.TransitionToSeason);

			if (_seasonDataContainer.GrassMaterial)
			{
				_seasonDataContainer.GrassMaterial.SetColor("_GridColor1", Color.Lerp(currentSeasonData.GrassGridColor1, nextSeasonData.GrassGridColor1, transition));
				_seasonDataContainer.GrassMaterial.SetColor("_GridColor2", Color.Lerp(currentSeasonData.GrassGridColor2, nextSeasonData.GrassGridColor2, transition));
				_seasonDataContainer.GrassMaterial.SetColor("_TopColor", Color.Lerp(currentSeasonData.GrassTopColor, nextSeasonData.GrassTopColor, transition));
				_seasonDataContainer.GrassMaterial.SetColor("_WindColor", Color.Lerp(currentSeasonData.GrassWindColor, nextSeasonData.GrassWindColor, transition));
			}

			if (_seasonDataContainer.TerrainMaterial)
			{
				_seasonDataContainer.TerrainMaterial.SetColor("_color1", Color.Lerp(currentSeasonData.TerrainColor1, nextSeasonData.TerrainColor1, transition));
				_seasonDataContainer.TerrainMaterial.SetColor("_color2", Color.Lerp(currentSeasonData.TerrainColor2, nextSeasonData.TerrainColor2, transition));
			}

			SetSeasonMaterial(_seasonRuntimeData.TransitionToSeason, transition, _seasonDataContainer);
		}

		private void CompleteTransition()
		{
			_seasonRuntimeData.CurrentSeason = _seasonRuntimeData.TransitionToSeason;
			_seasonRuntimeData.SeasonChanging = false;
			_seasonRuntimeData.InvokeSeasonChanged(_seasonRuntimeData.CurrentSeason);
		}

		// Sets material shader properties based on season and transition progress.
		private void SetSeasonMaterial(Season season, float transition, ScriptablesProcessorInfrastructure.AllSeasonSettings container)
		{
			if (season == Season.Autumn)
			{
				container.TreeMaterial.SetFloat("_AutumnPower", transition * 0.3f);
				container.TreeMaterial.SetFloat("_SnowPower", 0);
				container.BuildingMaterial.SetFloat("_SnowPower", 0);
				container.BuildingMaterial.SetFloat("_SnowNoiseLevels", 0);
			}
			else if (season == Season.Winter)
			{
				container.TreeMaterial.SetFloat("_AutumnPower", (1 - transition) * 0.5f);
				container.TreeMaterial.SetFloat("_SnowPower", transition * 0.5f);
				container.BuildingMaterial.SetFloat("_SnowPower", transition * 1f);
				container.BuildingMaterial.SetFloat("_SnowNoiseLevels", transition);
				container.WaterMaterial.SetFloat("_IceStrength", transition);
				container.TerrainMaterial.SetFloat("_Tint", transition * container.WinterTint);
				container.GrassMaterial.SetFloat("_Tint", transition * container.WinterTint);
			}
			else if (season == Season.Spring)
			{
				container.TreeMaterial.SetFloat("_SnowPower", (1 - transition) * 0.5f);
				container.BuildingMaterial.SetFloat("_SnowPower", (1 - transition) * 0.5f);
				container.TreeMaterial.SetFloat("_Spring", transition * 0.1f);
				container.TreeMaterial.SetFloat("_AutumnPower", 0);
				container.BuildingMaterial.SetFloat("_SnowNoiseLevels", (1 - transition));
				container.WaterMaterial.SetFloat("_IceStrength", 1 - transition);
				container.TerrainMaterial.SetFloat("_Tint", (1 - transition) * container.RestTint);
				container.GrassMaterial.SetFloat("_Tint", (1 - transition) * container.RestTint);
			}
			else
			{
				container.TreeMaterial.SetFloat("_Spring", (1 - transition) * 0.1f);
				container.TreeMaterial.SetFloat("_SnowPower", 0);
				container.TreeMaterial.SetFloat("_AutumnPower", 0);
				container.BuildingMaterial.SetFloat("_SnowPower", 0);
				container.BuildingMaterial.SetFloat("_SnowNoiseLevels", 0);
			}
		}

	}
}
