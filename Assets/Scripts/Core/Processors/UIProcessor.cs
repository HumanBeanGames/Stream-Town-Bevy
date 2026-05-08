using System;
using System.Collections;
using System.Collections.Generic;
using TechTree;
using TechTree.Data;
using TMPro;
using UnityEngine;
using UnityEngine.UI;
using UserInterface;
using TownGoal.Data;
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
    public class UIProcessor : MonoBehaviour, IInstaller, IProcessor, IPostInitializeProcessor
    {
        /// <summary>
        /// Town resource processor for accessing town resource data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TownResourceProcessor _townResourceProcessor;

        /// <summary>
        /// Runtime UI data.
        /// Assigned in InjectRuntimeData.
        /// </summary>
        private UIRuntimeData _uiRuntimeData;

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

        public void RegisterResourceDisplay(TextMeshProUGUI woodDisplayText, TextMeshProUGUI foodDisplayText, TextMeshProUGUI oreDisplayText, TextMeshProUGUI goldDisplayText, TextMeshProUGUI recruitDisplayText = null)
        {
            _uiRuntimeData.WoodDisplayText = woodDisplayText;
            _uiRuntimeData.FoodDisplayText = foodDisplayText;
            _uiRuntimeData.OreDisplayText = oreDisplayText;
            _uiRuntimeData.GoldDisplayText = goldDisplayText;
            _uiRuntimeData.RecruitDisplayText = recruitDisplayText;
        }

        public void RegisterResourceRateOfChangeDisplay(TextMeshProUGUI woodRateOfChangeText, TextMeshProUGUI foodRateOfChangeText, TextMeshProUGUI oreRateOfChangeText, TextMeshProUGUI goldRateOfChangeText, TextMeshProUGUI recruitRateOfChangeText = null)
        {
            _uiRuntimeData.WoodRateOfChangeText = woodRateOfChangeText;
            _uiRuntimeData.FoodRateOfChangeText = foodRateOfChangeText;
            _uiRuntimeData.OreRateOfChangeText = oreRateOfChangeText;
            _uiRuntimeData.GoldRateOfChangeText = goldRateOfChangeText;
            _uiRuntimeData.RecruitRateOfChangeText = recruitRateOfChangeText;
        }

        public void RegisterHudCounters(TextMeshProUGUI playerCountText, TextMeshProUGUI buildingCountText, TextMeshProUGUI timeDisplayText, Slider seasonalSlider)
        {
            _uiRuntimeData.PlayerCountText = playerCountText;
            _uiRuntimeData.BuildingCountText = buildingCountText;
            _uiRuntimeData.TimeDisplayText = timeDisplayText;
            _uiRuntimeData.SeasonalSlider = seasonalSlider;
        }

        // Updates resource display texts with current amounts.
        private void UpdateResourcesDisplay()
        {
            if (_uiRuntimeData.WoodDisplayText == null || _uiRuntimeData.FoodDisplayText == null || _uiRuntimeData.OreDisplayText == null || _uiRuntimeData.GoldDisplayText == null)
                return;

            _uiRuntimeData.WoodDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Wood), _townResourceProcessor.MaxResourceAmount(Resource.Wood));
            _uiRuntimeData.FoodDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Food), _townResourceProcessor.MaxResourceAmount(Resource.Food));
            _uiRuntimeData.OreDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Ore), _townResourceProcessor.MaxResourceAmount(Resource.Ore));
            _uiRuntimeData.GoldDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Gold));

            if (_uiRuntimeData.RecruitDisplayText != null)
            {
                _uiRuntimeData.RecruitDisplayText.text = FormattedResourceString(_townResourceProcessor.CurrentResourceAmount(Resource.Recruit), _townResourceProcessor.MaxResourceAmount(Resource.Recruit));
            }
        }

        // Updates resource rate of change texts.
        private void UpdateResourcesRateOfChange()
        {
            if (_uiRuntimeData.WoodRateOfChangeText == null || _uiRuntimeData.FoodRateOfChangeText == null || _uiRuntimeData.OreRateOfChangeText == null || _uiRuntimeData.GoldRateOfChangeText == null)
                return;

            _uiRuntimeData.WoodRateOfChangeText.text = FormattedRateOfChangeString(_townResourceProcessor.RateOfChangeForResource(Resource.Wood));
            _uiRuntimeData.FoodRateOfChangeText.text = FormattedRateOfChangeString(_townResourceProcessor.RateOfChangeForResource(Resource.Food));
            _uiRuntimeData.OreRateOfChangeText.text = FormattedRateOfChangeString(_townResourceProcessor.RateOfChangeForResource(Resource.Ore));
            _uiRuntimeData.GoldRateOfChangeText.text = FormattedRateOfChangeString(rateOfChange: _townResourceProcessor.RateOfChangeForResource(Resource.Gold));

            if (_uiRuntimeData.RecruitRateOfChangeText != null)
            {
                _uiRuntimeData.RecruitRateOfChangeText.text = FormattedRateOfChangeString(_townResourceProcessor.RateOfChangeForResource(Resource.Recruit));
            }
        }

        // Updates building and player count texts.
        private void UpdateCountTexts()
        {
            if (_uiRuntimeData.BuildingCountText == null || _uiRuntimeData.PlayerCountText == null)
                return;

            int buildingCount = _buildingProcessor.NumberOfBuildings;
            int playerCount = _playerProcessor.PlayerCount();
            int recruitCount = _playerProcessor.RecruitCount();

            // Exclude debugger from player count (debugger is a special case)
            foreach (var player in _playerProcessor.Players)
            {
                if (player != null && player.TwitchUser != null && player.TwitchUser.Username == "Debugger")
                {
                    playerCount--;
                    break;
                }
            }

            _uiRuntimeData.BuildingCountText.text = buildingCount.ToString();
            _uiRuntimeData.PlayerCountText.text = playerCount.ToString();

            if (_uiRuntimeData.RecruitCountText != null)
            {
                _uiRuntimeData.RecruitCountText.text = recruitCount.ToString();
            }
        }

        // Updates the season slider based on time passed.
        private void UpdateSeasonSlider()
        {
            if (_uiRuntimeData.SeasonalSlider == null)
                return;

            float newValue = ((_timeProcessor.WorldTimePassed + _uiSettings.SeaonSliderStartOffset) / (float)_timeProcessor.SecondsPerDay / (float)_seasonProcessor.DaysPerSeason) / 4f;
            int roundedDown = (int)Mathf.Floor(newValue);
            _uiRuntimeData.SeasonalSlider.value = newValue - roundedDown;
        }

        // Updates the time of day display.
        private void UpdateTimeOfDay()
        {
            if (_uiRuntimeData.TimeDisplayText == null)
                return;

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
            _uiRuntimeData.TimeDisplayText.text = newString;
        }

        private void UpdateTownGoalDisplay()
        {
            if (_uiRuntimeData.TownGoalInterface == null)
                return;

            if (_techTreeProcessor == null || !_techTreeProcessor.TryGetCurrentGoal(out Goal currentGoal, out TechNodeData nodeData))
            {
                if (_uiRuntimeData.RenderedTownGoal != null)
                {
                    _uiRuntimeData.TownGoalInterface.DisableTownGoalContainer();
                    _uiRuntimeData.RenderedTownGoal = null;
                }

                return;
            }

            if (_uiRuntimeData.RenderedTownGoal == currentGoal)
                return;

            _uiRuntimeData.TownGoalInterface.DisableTownGoalContainer();
            _uiRuntimeData.TownGoalInterface.AddGoal(currentGoal, nodeData);
            _uiRuntimeData.RenderedTownGoal = currentGoal;
        }

        // Formats a resource amount string with optional max amount.
        private string FormattedResourceString(int currentAmount, int maxAmount = -1)
        {
            string newString = $"<size=48>{StringUtils.GetShortenedNumberAsString(currentAmount)}</size>";

            if (maxAmount != -1)
	            newString += $" <size=32><color=#958450>/ {StringUtils.GetShortenedNumberAsString(maxAmount)}</color></size>";

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

        public void Initialize()
        {
            if (_uiRuntimeData == null)
                throw new InvalidOperationException("UIProcessor: UIRuntimeData has not been installed.");
        }

        /// <summary>
        /// Activates the UI processor on the main thread after initialization.
        /// Binds scene-local UI components via GetComponent.
        /// </summary>
        public void Activate()
        {
            // TODO(Architecture): Remove scene lookups - these should be injected or configured
            _uiRuntimeData.TownGoalInterface = GetComponent<UserInterface_TownGoal>();
        }

        /// <summary>
        /// Processes UI logic every frame.
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
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // UIProcessor does not have scene-specific settings to refresh
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
            if (_uiRuntimeData != null)
                throw new InvalidOperationException("UIProcessor: UIRuntimeData has already been installed.");

            _uiRuntimeData = new UIRuntimeData();
            containerBuilder.AddSingleton(_uiRuntimeData);
        }
    }
}
