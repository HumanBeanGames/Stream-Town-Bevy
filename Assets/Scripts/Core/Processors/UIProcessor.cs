using System;
using System.Collections;
using System.Collections.Generic;
using TechTree;
using TMPro;
using UnityEngine;
using UnityEngine.UI;
using UserInterface;
using Utils;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
    /// <summary>
    /// Manages the game's user interface updates.
    /// Handles resource displays, time display, season slider, and count texts.
    /// </summary>
    public class UIProcessor : MonoBehaviour, IInstaller, IProcessor
    {
        /// <summary>
        /// Town resource processor for accessing town resource data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TownResourceProcessor _townResourceProcessor;

        /// <summary>
        /// Runtime UI data ScriptableObject.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private UIRuntimeData _uiRuntimeData;

        /// <summary>
        /// Building processor for accessing building data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private BuildingProcessor _buildingProcessor;

        /// <summary>
        /// Tech tree processor for accessing tech tree data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TechTreeProcessor _techTreeProcessor;

        /// <summary>
        /// Time processor for accessing time data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TimeProcessor _timeProcessor;

        /// <summary>
        /// Season processor for accessing season data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private SeasonProcessor _seasonProcessor;

        /// <summary>
        /// Player processor for accessing player data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private PlayerProcessor _playerProcessor;

        /// <summary>
        /// ScriptableObject containing UI settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private UISettings _uiSettings;

        /// <summary>
        /// Gets or sets the ruler vote interface.
        /// </summary>
        public UserInterface_RulerVote RulerVoteInterface
        {
            get => _uiRuntimeData.RulerVoteInterface;
            set => _uiRuntimeData.RulerVoteInterface = value;
        }

        /// <summary>
        /// Gets or sets the town vote interface.
        /// </summary>
        public UserInterface_TownVote TownVoteInterface
        {
            get => _uiRuntimeData.TownVoteInterface;
            set => _uiRuntimeData.TownVoteInterface = value;
        }

        /// <summary>
        /// Gets or sets the town goal interface.
        /// </summary>
        public UserInterface_TownGoal TownGoalInterface
        {
            get => _uiRuntimeData.TownGoalInterface;
            set => _uiRuntimeData.TownGoalInterface = value;
        }

        /// <summary>
        /// Gets or sets the event interface.
        /// </summary>
        public UserInterface_Event EventInterface
        {
            get => _uiRuntimeData.EventInterface;
            set => _uiRuntimeData.EventInterface = value;
        }

        // Updates resource display texts with current amounts.
        private void UpdateResourcesDisplay()
        {
            _uiSettings.WoodDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Wood), _townResourceProcessor.MaxResourceAmount(Resource.Wood));
            _uiSettings.FoodDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Food), _townResourceProcessor.MaxResourceAmount(Resource.Food));
            _uiSettings.OreDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Ore), _townResourceProcessor.MaxResourceAmount(Resource.Ore));
            _uiSettings.GoldDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Gold));
        }

        // Updates resource rate of change texts.
        private void UpdateResourcesRateOfChange()
        {
            _uiSettings.WoodRateOfChangeText.text = FormattedRateOfChangeString(_townResourceProcessor.RateOfChangeForResource(Resource.Wood));
            _uiSettings.FoodRateOfChangeText.text = FormattedRateOfChangeString(_townResourceProcessor.RateOfChangeForResource(Resource.Food));
            _uiSettings.OreRateOfChangeText.text = FormattedRateOfChangeString(_townResourceProcessor.RateOfChangeForResource(Resource.Ore));
            _uiSettings.GoldRateOfChangeText.text = FormattedRateOfChangeString(rateOfChange: _townResourceProcessor.RateOfChangeForResource(Resource.Gold));
        }

        // Updates building and player count texts.
        private void UpdateCountTexts()
        {
            _uiSettings.BuildingCountText.text = _buildingProcessor.NumberOfBuildings.ToString();
            _uiSettings.PlayerCountText.text = _playerProcessor.PlayerCount().ToString();
        }

        // Updates the season slider based on time passed.
        private void UpdateSeasonSlider()
        {
            float newValue = ((_timeProcessor.WorldTimePassed + _uiSettings.SeaonSliderStartOffset) / (float)_timeProcessor.SecondsPerDay / (float)_seasonProcessor.DaysPerSeason) / 4f;
            int roundedDown = (int)Mathf.Floor(newValue);
            _uiSettings.SeasonalSlider.value = newValue - roundedDown;
        }

        // Updates the time of day display.
        private void UpdateTimeOfDay()
        {
            TimeSpan t = TimeSpan.FromSeconds(_timeProcessor.WorldTimePassed);
            string newString = "";
            string formatted = string.Format("{0:D1}", t.Days);
            newString += $"<size=48>{formatted}</size><size=32><color=#958450>D</color></size> ";
            formatted = string.Format("{0:D2}", t.Hours);
            newString += $"<size=48>{formatted}</size><size=32><color=#958450>H</color></size> ";
            formatted = string.Format("{0:D2}", t.Minutes);
            newString += $"<size=48>{formatted}</size><size=32><color=#958450>M</color></size> ";
            formatted = string.Format("{0:D2}", t.Seconds);
            newString += $"<size=48>{formatted}</size><size=32><color=#958450>S</color></size>";
            _uiSettings.TimeDisplayText.text = newString;
        }

        // Formats a resource amount string with optional max amount.
        private string FormattedResourceString(int currentAmount, int maxAmount = -1)
        {
            string newString = $"<size=48>{StringUtils.GetShortenedNumberAsString(currentAmount)}</size>";

            if (maxAmount != -1)
                newString += $"<size=32><color=#958450> / {StringUtils.GetShortenedNumberAsString(maxAmount)}</color></size>";

            return newString;
        }

        // Formats a rate of change string with sign.
        private string FormattedRateOfChangeString(int rateOfChange = 0)
        {
            string newString = $"<size=27><color=#3af826>";

            newString += rateOfChange >= 0 ? "+" : "-";
            newString += $"{StringUtils.GetShortenedNumberAsString(rateOfChange)}/HR</size></color>";

            return newString;
        }

        /// <summary>
        /// Initializes the UI processor.
        /// Gets UI component references.
        /// </summary>
        public void Initialize()
        {
            _uiRuntimeData.TownGoalInterface = GetComponent<UserInterface_TownGoal>();
            _uiRuntimeData.RulerVoteInterface = GetComponent<UserInterface_RulerVote>();
            _uiRuntimeData.TownVoteInterface = GetComponent<UserInterface_TownVote>();
            _uiRuntimeData.EventInterface = GetComponent<UserInterface_Event>();
        }

        /// <summary>
        /// Updates all UI elements every frame.
        /// Called every frame by the Coordinator.
        /// </summary>
        public void Process()
        {
            UpdateResourcesDisplay();
            UpdateResourcesRateOfChange();
            UpdateCountTexts();
            UpdateSeasonSlider();
            UpdateTimeOfDay();
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
            // Instantiate and register UIRuntimeData ScriptableObject
            UIRuntimeData uiRuntimeData = ScriptableObject.CreateInstance<UIRuntimeData>();
            containerBuilder.AddSingleton(uiRuntimeData);
        }
    }
}
