using System;
using Buildings;
using Reflex.Attributes;
using Reflex.Core;
using System.Collections.Generic;
using UnityEngine;
using Utils;
using Character;
using ScriptablesProcessorInfrastructure;
using Utils.Pooling;
using Data.Containers;
using Processors;

#if UNITY_EDITOR
using UnityEditor;
#endif

namespace Processors
{
    /// <summary>
    /// Processor that manages all buildings in the game.
    /// Handles building placement, leveling, removal, and resource cost calculations.
    /// </summary>
    [System.Serializable]
    public class BuildingProcessor : MonoBehaviour, IInstaller, IProcessor
    {
        #region Dependencies (Settings)

        /// <summary>
        /// Container for building data ScriptableObjects.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Data.Containers.BuildingDataContainer _buildingDataContainer;

        /// <summary>
        /// ScriptableObject containing object pooling settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private ObjectPoolingSettings _poolingSettings;

        /// <summary>
        /// ScriptableObject containing building settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private BuildingSettings _buildingSettings;

        #endregion

        #region Dependencies (Runtime Data)

        /// <summary>
        /// Runtime building data ScriptableObject.
        /// Created and bound in InjectRuntimeData().
        /// </summary>
        private BuildingRuntimeData _buildingRuntimeData;

        #endregion

        #region Dependencies (Processors)

        /// <summary>
        /// Town resource processor for accessing town resource data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TownResourceProcessor _townResourceProcessor;

        /// <summary>
        /// Object pooling processor for accessing pooled objects.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingProcessor _poolingProcessor;
		[Inject] private GUIDProcessor _guidProcessor;

        #endregion

        #region Public API (Properties)

        /// <summary>
        /// Gets the building counts dictionary from runtime data.
        /// </summary>
        public Dictionary<BuildingType, int> BuildingCounts => _buildingRuntimeData.BuildingCounts;

        /// <summary>
        /// Gets the buildings unlocked dictionary from runtime data.
        /// </summary>
        public Dictionary<BuildingType, bool> BuildingsUnlocked => _buildingRuntimeData.BuildingsUnlocked;

        /// <summary>
        /// Gets or sets the total number of buildings.
        /// </summary>
        public int NumberOfBuildings
        {
            get => _buildingRuntimeData.NumberOfBuildings;
            set => _buildingRuntimeData.NumberOfBuildings = value;
        }

        /// <summary>
        /// Gets or sets the last building type selected by the user player.
        /// </summary>
        public BuildingType LastBuildingType
        {
            get => _buildingRuntimeData.LastBuildingType;
            set => _buildingRuntimeData.LastBuildingType = value;
        }

		public bool BuildingsCostResourcesEnabled => _buildingSettings.BuildingsCostResourcesEnabled;

        #endregion

        #region IProcessor

        /// <summary>
        /// Initializes the building processor.
        /// Creates RuntimeData after all processors are confirmed ready.
        /// Sets up runtime state dictionaries and building data.
        /// </summary>
        public void Initialize()
        {
            if (_buildingRuntimeData == null)
                throw new InvalidOperationException("BuildingProcessor: BuildingRuntimeData has not been installed.");

            InitializeRuntimeState();
        }

		/// <summary>Clears world-instance and derived building state before a restore.</summary>
		public void ResetWorldState()
		{
			InitializeRuntimeState();
		}

        /// <summary>
        /// Processes building logic every frame.
        /// Called every frame by the Coordinator.
        /// BuildingProcessor does not require per-frame updates.
        /// </summary>
        public void Process()
        {
            // BuildingProcessor does not require per-frame updates
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // BuildingProcessor does not have scene-specific settings to refresh
        }

        #endregion

        #region IInstaller

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

        #endregion

        #region Private Methods

        public void InjectRuntimeData(ContainerBuilder containerBuilder)
        {
            if (_buildingRuntimeData != null)
                throw new InvalidOperationException("BuildingProcessor: BuildingRuntimeData has already been installed.");

            _buildingRuntimeData = new BuildingRuntimeData();
            containerBuilder.AddSingleton(_buildingRuntimeData);
        }

        #endregion

        #region Public API (Methods)

        /// <summary>
        /// Returns true if the building can be afforded.
        /// Checks if the town has sufficient resources to build the specified building type.
        /// </summary>
        /// <param name="type">The building type to check.</param>
        /// <returns>True if the building can be afforded, false otherwise.</returns>
		public bool ToggleBuildingsCostResourcesEnabled()
		{
			bool enabled = !_buildingSettings.BuildingsCostResourcesEnabled;
			_buildingSettings.SetBuildingsCostResourcesEnabled(enabled);
			return enabled;
		}

        public bool CanAffordToBuild(BuildingType type)
        {
            // Buildings are always affordable if resource costs are disabled
            if (!_buildingSettings.BuildingsCostResourcesEnabled)
                return true;

            BuildingDataSettings data = _buildingDataContainer.BuildingDataDictionary[type];
            // Calculate costs with scaling based on existing building count
            int woodCost = data.BuildResourceCost.WoodCost + (int)((float)(data.BuildResourceCost.WoodCost * _buildingRuntimeData.BuildingCounts[type]) * data.CostIncreasePerBuildingMultiplier);
            int oreCost = data.BuildResourceCost.OreCost + (int)((float)(data.BuildResourceCost.OreCost * _buildingRuntimeData.BuildingCounts[type]) * data.CostIncreasePerBuildingMultiplier);
            int foodCost = data.BuildResourceCost.FoodCost + (int)((float)(data.BuildResourceCost.FoodCost * _buildingRuntimeData.BuildingCounts[type]) * data.CostIncreasePerBuildingMultiplier);
            int goldCost = data.BuildResourceCost.GoldCost + (int)((float)(data.BuildResourceCost.GoldCost * _buildingRuntimeData.BuildingCounts[type]) * data.CostIncreasePerBuildingMultiplier);
            
            // Check if town has sufficient resources after applying cost reduction
            if (_townResourceProcessor.MoreThanEqualComparison(Resource.Wood, woodCost - CalculateCostReduction(type, woodCost))
                && _townResourceProcessor.MoreThanEqualComparison(Resource.Ore, oreCost - CalculateCostReduction(type, oreCost))
                && _townResourceProcessor.MoreThanEqualComparison(Resource.Food, foodCost - CalculateCostReduction(type, foodCost))
                && _townResourceProcessor.MoreThanEqualComparison(Resource.Gold, goldCost - CalculateCostReduction(type, goldCost)))
                return true;
            else
                return false;
        }
		/// <summary>
		/// Calculates the cost reduction for a building type based on modifiers.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <param name="baseValue">The base cost value.</param>
		/// <returns>The reduced cost amount.</returns>
		public int CalculateCostReduction(BuildingType type, int baseValue)
		{
			return (int)(_buildingSettings.BuildingCostModifiers[type] * (baseValue / 100.0f));
		}

		/// <summary>
		/// Called when a new building has been built.
		/// Deducts build costs and adds the building to the building list.
		/// </summary>
		/// <param name="building">The building that was built.</param>
		public void OnBuiltNewBuilding(BuildingBase building)
		{
			BuildingDataSettings data = _buildingDataContainer.BuildingDataDictionary[building.BuildingType];

			if (_buildingSettings.BuildingsCostResourcesEnabled)
			{
				int woodCost = data.BuildResourceCost.WoodCost + (int)((float)(data.BuildResourceCost.WoodCost * _buildingRuntimeData.BuildingCounts[building.BuildingType]) * data.CostIncreasePerBuildingMultiplier);
				int oreCost = data.BuildResourceCost.OreCost + (int)((float)(data.BuildResourceCost.OreCost * _buildingRuntimeData.BuildingCounts[building.BuildingType]) * data.CostIncreasePerBuildingMultiplier);
				int foodCost = data.BuildResourceCost.FoodCost + (int)((float)(data.BuildResourceCost.FoodCost * _buildingRuntimeData.BuildingCounts[building.BuildingType]) * data.CostIncreasePerBuildingMultiplier);
				int goldCost = data.BuildResourceCost.GoldCost + (int)((float)(data.BuildResourceCost.GoldCost * _buildingRuntimeData.BuildingCounts[building.BuildingType]) * data.CostIncreasePerBuildingMultiplier);
				_townResourceProcessor.RemoveResource(Resource.Wood, woodCost - CalculateCostReduction(building.BuildingType, woodCost), true);
				_townResourceProcessor.RemoveResource(Resource.Ore, oreCost - CalculateCostReduction(building.BuildingType, oreCost), true);
				_townResourceProcessor.RemoveResource(Resource.Food, foodCost - CalculateCostReduction(building.BuildingType, foodCost), true);
				_townResourceProcessor.RemoveResource(Resource.Gold, goldCost - CalculateCostReduction(building.BuildingType, goldCost), true);
			}

			AddBuilding(building);
		}

		/// <summary>
		/// Adds a building that was loaded from a save file.
		/// </summary>
		/// <param name="building">The building to add.</param>
		public void AddLoadedBuilding(BuildingBase building)
		{
			AddBuilding(building);
		}

		/// <summary>
		/// Called when a building has been removed.
		/// Removes the building from the building list.
		/// </summary>
		/// <param name="building">The building that was removed.</param>
		public void OnBuildingRemoved(BuildingBase building)
		{
			RemoveBuilding(building);
		}

		/// <summary>
		/// Returns true if the building can be leveled up.
		/// Checks if the town has sufficient resources to level the building.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <param name="currentLevel">The current level of the building.</param>
		/// <returns>True if the building can be leveled, false otherwise.</returns>
		public bool CanAffordToLevel(BuildingType type, int currentLevel)
		{
			if (!_buildingSettings.BuildingsCostResourcesEnabled)
				return true;

			BuildingDataSettings data = _buildingDataContainer.BuildingDataDictionary[type];

			int woodCost = (int)(data.LevelResourceCost.WoodCost * currentLevel * currentLevel * data.CostIncreasePerLevelMultiplier);
			woodCost -= CalculateCostReduction(type, woodCost);
			int oreCost = (int)(data.LevelResourceCost.OreCost * currentLevel * currentLevel * data.CostIncreasePerLevelMultiplier);
			oreCost -= CalculateCostReduction(type, oreCost);
			int goldCost = (int)(data.LevelResourceCost.GoldCost * currentLevel * currentLevel * data.CostIncreasePerLevelMultiplier);
			goldCost -= CalculateCostReduction(type, goldCost);
			int foodCost = (int)(data.LevelResourceCost.FoodCost * currentLevel * currentLevel * data.CostIncreasePerLevelMultiplier);
			foodCost -= CalculateCostReduction(type, foodCost);

			// Store this calculation in a better way rather.
			if (_townResourceProcessor.MoreThanEqualComparison(Resource.Wood, woodCost)
				&& _townResourceProcessor.MoreThanEqualComparison(Resource.Ore, oreCost)
				&& _townResourceProcessor.MoreThanEqualComparison(Resource.Gold, goldCost)
				&& _townResourceProcessor.MoreThanEqualComparison(Resource.Food, foodCost))
				return true;
			else
				return false;
		}

		/// <summary>
		/// Called when a building is leveled.
		/// Deducts level costs from town resources.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <param name="currentLevel">The current level of the building.</param>
		public void OnLevelBuilding(BuildingType type, int currentLevel)
		{
			if (!_buildingSettings.BuildingsCostResourcesEnabled)
				return;

			BuildingDataSettings data = _buildingDataContainer.BuildingDataDictionary[type];
			int woodCost = (int)(data.LevelResourceCost.WoodCost * currentLevel * currentLevel * data.CostIncreasePerLevelMultiplier);
			woodCost -= CalculateCostReduction(type, woodCost);
			int oreCost = (int)(data.LevelResourceCost.OreCost * currentLevel * currentLevel * data.CostIncreasePerLevelMultiplier);
			oreCost -= CalculateCostReduction(type, oreCost);
			int goldCost = (int)(data.LevelResourceCost.GoldCost * currentLevel * currentLevel * data.CostIncreasePerLevelMultiplier);
			goldCost -= CalculateCostReduction(type, goldCost);
			int foodCost = (int)(data.LevelResourceCost.FoodCost * currentLevel * currentLevel * data.CostIncreasePerLevelMultiplier);
			foodCost -= CalculateCostReduction(type, foodCost);

			_townResourceProcessor.RemoveResource(Resource.Wood, woodCost);
			_townResourceProcessor.RemoveResource(Resource.Ore, oreCost);
			_townResourceProcessor.RemoveResource(Resource.Gold, goldCost);
			_townResourceProcessor.RemoveResource(Resource.Food, foodCost);
		}

		/// <summary>
		/// Returns true if the type of building exists in the data dictionary.
		/// </summary>
		/// <param name="type">The building type to check.</param>
		/// <returns>True if the building type exists, false otherwise.</returns>
		public bool CheckBuildingTypeExists(BuildingType type)
		{
			return _buildingDataContainer.BuildingDataDictionary.ContainsKey(type);
		}

		/// <summary>
		/// Checks if the building type is placeable by the player.
		/// </summary>
		/// <param name="type">The building type to check.</param>
		/// <returns>True if the building is placeable, false otherwise.</returns>
		public bool CheckBuildingIsPlaceable(BuildingType type)
		{
			//TODO:: Add check for build permissions as well as building requirements
			if (!CheckBuildingTypeExists(type))
				return false;
			else
				return _buildingDataContainer.BuildingDataDictionary[type].Placeable;
		}

		/// <summary>
		/// Attempts to start a new building placer.
		/// Returns <b>false</b> if player is already building, or building type doesn't exist / isn't placeable
		/// </summary>
		/// <param name="player">The player starting the building placement.</param>
		/// <param name="type">The building type to place.</param>
		/// <param name="errorMessage">Output error message if placement fails.</param>
		/// <returns>True if placement started successfully, false otherwise.</returns>
		public bool TryStartNewBuildingPlacer(Player player, BuildingType type, out string errorMessage)
		{
			if (player == null)
				throw new InvalidOperationException($"BuildingProcessor: Cannot start building placer for '{type}' without a valid player. Ensure PlayerProcessor.UserPlayer is established before UI building actions are enabled.");

			// Check if player is already placing a building.
			if (_buildingRuntimeData.Placers.ContainsKey(player))
			{
				errorMessage = "Already Placing Building";
				return false;
			}

			if (!_buildingRuntimeData.BuildingsUnlocked[type] && !_buildingSettings.IgnoreTechUnlocks)
			{
				errorMessage = "Building Not Unlocked Yet!";
				return false;
			}

			// Check if the building type is valid.
			if (!CheckBuildingIsPlaceable(type))
			{
				errorMessage = "Building Not Placeable";
				return false;
			}

			// Check if the building can be afforded.
			if (!CanAffordToBuild(type) && _buildingSettings.BuildingsCostResourcesEnabled)
			{
				errorMessage = "Can't Afford to Build";
				return false;
			}

    // Get Building Placer from pool and set it's position to the player's last succesful building position;
    		BuildingPlacer obj = _poolingProcessor.GetPooledObject("BuildingPlacer").GetComponent<BuildingPlacer>();
    obj.OnPooled(player);
    obj.transform.position = player.LastBuildingPlacement;
    obj.RotatePlacer(amount: player.TotalBuildingRotation);
    			// Add players placer to the list.
    	AddPlacer(player, obj);

			// Set up building placer to use proper building model
			obj.gameObject.SetActive(true);
			obj.SetBuildingByType(type);
			obj.UpdateCollision();

			errorMessage = "";
			return true;
		}

		/// <summary>
		/// Attempts to move a player's active building placer and returns whether it succeeded.
		/// </summary>
		/// <param name="player">The player whose placer to move.</param>
		/// <param name="moveInput">The movement direction vector.</param>
		/// <returns>True if the move succeeded, false otherwise.</returns>
		public bool TryMoveBuilding(Player player, Vector3 moveInput)
		{
			if (!_buildingRuntimeData.Placers.ContainsKey(player))
				return false;

			_buildingRuntimeData.Placers[player].MovePlacer(moveInput);
			return true;
		}

		/// <summary>
		/// Attempts to rotate a player's active building placer and returns whether it succeeded.
		/// </summary>
		/// <param name="player">The player whose placer to rotate.</param>
		/// <param name="rotationAmount">The amount to rotate in degrees.</param>
		/// <returns>True if the rotation succeeded, false otherwise.</returns>
		public bool TryRotateBuilding(Player player, int rotationAmount)
		{
			if (!_buildingRuntimeData.Placers.ContainsKey(player))
				return false;

			_buildingRuntimeData.Placers[player].RotatePlacer(amount: rotationAmount);
			player.TotalBuildingRotation += rotationAmount;
			return true;
		}

		/// <summary>
		/// Updates the collision state of a player's building placer.
		/// </summary>
		/// <param name="player">The player whose placer to update.</param>
		public void UpdatePlacerCollision(Player player)
		{
			if (!_buildingRuntimeData.Placers.ContainsKey(player))
				return;

			_buildingRuntimeData.Placers[player].UpdateCollision();
		}

		/// <summary>
		/// Attempts to place a building and returns whether it succeeded, passing out an error message to inform the user why it failed.
		/// </summary>
		/// <param name="player">The player attempting to place the building.</param>
		/// <param name="errorMessage">Output error message if placement fails.</param>
		/// <returns>True if placement succeeded, false otherwise.</returns>
		public bool TryPlaceBuilding(Player player, out string errorMessage)
		{
			// Check that the player is in build mode and has a building placer down.
			if (!_buildingRuntimeData.Placers.ContainsKey(player))
			{
				errorMessage = " Not In Build Mode!";
				return false;
			}

			// If the player can place the building, remove their placer and clear the error message.
			if (_buildingRuntimeData.Placers[player].TrySpawnBuilding(out Vector3 placementPos, out errorMessage))
			{
				RemovePlacer(player);
				player.LastBuildingPlacement = placementPos;
				errorMessage = "";
				return true;
			}

			return false;
		}

		/// <summary>
		/// Attempts to cancel a building placement and returns whether it succeeded.
		/// </summary>
		/// <param name="player">The player to cancel building for.</param>
		/// <returns>True if cancellation succeeded, false otherwise.</returns>
		public bool TryCancelBuilding(Player player)
		{
			// If the player does not have a building placer active, then there is nothing to cancel.
			if (player != null)
			{
				if (!_buildingRuntimeData.Placers.ContainsKey(player))
					return false;

				_buildingRuntimeData.Placers[player].gameObject.SetActive(false);
				RemovePlacer(player);
				return true;
			}

			return false;
		}

		/// <summary>
		/// Attempts to cancel a building placement with an object parameter.
		/// Used for UI event callbacks.
		/// </summary>
		/// <param name="obj">The player object to cancel building for.</param>
		public void TryCancelBuilding(object obj)
		{
			TryCancelBuilding((Player)obj);
		}

		/// <summary>
		/// Checks if a building type is unlocked.
		/// </summary>
		/// <param name="buildingType">The building type to check.</param>
		/// <returns>True if the building is unlocked, false otherwise.</returns>
		public bool IsBuildingUnlocked(BuildingType buildingType)
		{
			return _buildingRuntimeData.BuildingsUnlocked[buildingType];
		}

		/// <summary>
		/// Unlocks all buildings for debug/testing purposes.
		/// Bypasses tech tree requirements.
		/// </summary>
		public void UnlockAllBuildingsForDebug()
		{
			_buildingSettings.UnlockAllBuildings();
		}

		/// <summary>
		/// Attempts to level up a building.
		/// Checks if the building can be leveled, if not at max level, and if affordable.
		/// </summary>
		/// <param name="building">The building to level up.</param>
		/// <param name="errorMessage">Output error message if leveling fails.</param>
		/// <returns>True if leveling succeeded, false otherwise.</returns>
		public bool TryLevelBuilding(BuildingBase building, out string errorMessage)
		{

			// Check if the building can be leveled up.
			if (!building.BuildingData.CanLevel || building.LevelHandler == null || building.BuildingState == BuildingState.Construction)
			{
				errorMessage = "Building Can't Be Leveld Up";
				return false;
			}

			// Check the building is not at max level.
			if (!building.LevelHandler.CanLevel(true))
			{
				errorMessage = "Building Already at Max Level";
				return false;
			}

			// Check that the town can afford to level the building.
			if (!CanAffordToLevel(building.BuildingType, building.LevelHandler.Level))
			{
				errorMessage = "Can't Afford To Level";
				return false;
			}

			// Finally, attempt to level the building.
			if (building.LevelHandler.TryLevel())
			{
				errorMessage = "";
				OnLevelBuilding(building.BuildingType, building.LevelHandler.Level - 1);
				return true;
			}

			errorMessage = "Unknown Error";
			return false;
		}

		/// <summary>
		/// Attempts to level up a building by type and index.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <param name="index">The index of the building in the building list.</param>
		/// <param name="errorMessage">Output error message if leveling fails.</param>
		/// <returns>True if leveling succeeded, false otherwise.</returns>
		public bool TryLevelBuilding(BuildingType type, int index, out string errorMessage)
		{
			// Check that this type of building exists.
			if (!_buildingRuntimeData.Buildings.ContainsKey(type))
			{
				errorMessage = "Building Not Found";
				return false;
			}

			// Check that they are within the bounds of the building array.
			List<BuildingBase> buildings = _buildingRuntimeData.Buildings[type];
			if (buildings.Count <= index || index < 0)
			{
				errorMessage = "Building Not Found";
				return false;
			}

			BuildingBase building = buildings[index];

			return TryLevelBuilding(building, out errorMessage);

		}

		/// <summary>
		/// Attempts to remove a building from the game by type and index.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <param name="index">The index of the building in the building list.</param>
		/// <param name="errorMessage">Output error message if removal fails.</param>
		/// <returns>True if removal succeeded, false otherwise.</returns>
		public bool TryRemoveBuilding(BuildingType type, int index, out string errorMessage)
		{
			// Check that the type of building exists already.
			if (!_buildingRuntimeData.Buildings.ContainsKey(type))
			{
				errorMessage = "Building Does Not Exist";
				return false;
			}

			// Check that the index is within the bounds of the array.
			List<BuildingBase> buildings = _buildingRuntimeData.Buildings[type];
			if (buildings.Count <= index || index < 0)
			{
				errorMessage = "Building Does Not Exist";
				return false;
			}

			// Remove the building.
			errorMessage = "";
			BuildingBase building = buildings[index];
			building.RemoveBuilding();
			RemoveBuilding(building);
			building.RestoreFoliage(false);
			return true;
		}

		/// <summary>
		/// Attempts to get a building by type and index.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <param name="index">The index of the building in the building list.</param>
		/// <param name="buildingBase">Output parameter for the building.</param>
		/// <param name="errorMessage">Output error message if retrieval fails.</param>
		/// <returns>True if retrieval succeeded, false otherwise.</returns>
		public bool TryGetBuilding(BuildingType type, int index, out BuildingBase buildingBase, out string errorMessage)
		{
			// Check that the type of building exists already.
			if (!_buildingRuntimeData.Buildings.ContainsKey(type))
			{
				errorMessage = "Building Does Not Exist";
				buildingBase = null;
				return false;
			}

			// Check that the index is within the bounds of the array.
			List<BuildingBase> buildings = _buildingRuntimeData.Buildings[type];
			if (buildings.Count <= index || index < 0)
			{
				errorMessage = "Building Does Not Exist";
				buildingBase = null;
				return false;
			}

			// Get the building.
			errorMessage = "";
			buildingBase = buildings[index];
			return true;
		}

		/// <summary>
		/// Attempts to remove a building directly by reference.
		/// </summary>
		/// <param name="building">The building to remove.</param>
		/// <returns>True if removal succeeded, false otherwise.</returns>
		public bool TryRemoveBuilding(BuildingBase building)
		{
			RemoveBuilding(building);
			return true;
		}

		/// <summary>
		/// Displays the building ID for a given amount of time.
		/// Currently not implemented.
		/// </summary>
		/// <param name="type">The building type to display IDs for.</param>
		/// <returns>True (placeholder).</returns>
		public bool DisplayBuildingIdsOfType(BuildingType type)
		{
			return true;
		}

		/// <summary>
		/// Gets all buildings of a specific type as an array.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <returns>Array of all buildings of the specified type.</returns>
		public BuildingBase[] GetAllBuildingsOfType(BuildingType type)
		{
			if (!_buildingRuntimeData.Buildings.ContainsKey(type))
				return new BuildingBase[0];

			return _buildingRuntimeData.Buildings[type].ToArray();
		}

		/// <summary>
		/// Gets the dictionary of all buildings grouped by type.
		/// </summary>
		/// <returns>Dictionary mapping building types to lists of buildings.</returns>
		public Dictionary<BuildingType, List<BuildingBase>> GetAllBuildingsDictionary()
		{
			return _buildingRuntimeData.Buildings;
		}

		/// <summary>
		/// Gets the list of buildings of a specific type.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <returns>List of all buildings of the specified type.</returns>
		public List<BuildingBase> GetBuildingsByType(BuildingType type)
		{
			if (!_buildingRuntimeData.Buildings.ContainsKey(type))
				return new List<BuildingBase>();

			return _buildingRuntimeData.Buildings[type];
		}

		/// <summary>
		/// Unlocks a building type for construction.
		/// </summary>
		/// <param name="type">The building type to unlock.</param>
		public void UnlockBuilding(BuildingType type)
		{
			_buildingRuntimeData.BuildingsUnlocked[type] = true;
		}

		/// <summary>
		/// Gets the cost modifier for a building type.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <returns>The cost modifier value.</returns>
		public int GetBuildCostModifier(BuildingType type) => _buildingSettings.BuildingCostModifiers[type];

		/// <summary>
		/// Gets the max level for a building type.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <returns>The max level value.</returns>
		public int GetBuildingsMaxLevel(BuildingType type) => _buildingSettings.BuildingsMaxLevel[type];

		/// <summary>
		/// Gets the max level value for a building type.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <returns>The max level value.</returns>
		public int GetBuildingMaxLevel(BuildingType type) => _buildingSettings.BuildingsMaxLevel[type];

		public (int woodCost, int oreCost, int foodCost, int goldCost, int maxLevel) GetBuildingCostSummary(BuildingType type)
		{
			BuildingDataSettings data = _buildingDataContainer.BuildingDataDictionary[type];
			int woodCost = data.BuildResourceCost.WoodCost + (int)((float)(data.BuildResourceCost.WoodCost * _buildingRuntimeData.BuildingCounts[type]) * data.CostIncreasePerBuildingMultiplier);
			int oreCost = data.BuildResourceCost.OreCost + (int)((float)(data.BuildResourceCost.OreCost * _buildingRuntimeData.BuildingCounts[type]) * data.CostIncreasePerBuildingMultiplier);
			int foodCost = data.BuildResourceCost.FoodCost + (int)((float)(data.BuildResourceCost.FoodCost * _buildingRuntimeData.BuildingCounts[type]) * data.CostIncreasePerBuildingMultiplier);
			int goldCost = data.BuildResourceCost.GoldCost + (int)((float)(data.BuildResourceCost.GoldCost * _buildingRuntimeData.BuildingCounts[type]) * data.CostIncreasePerBuildingMultiplier);

			woodCost -= CalculateCostReduction(type, woodCost);
			oreCost -= CalculateCostReduction(type, oreCost);
			foodCost -= CalculateCostReduction(type, foodCost);
			goldCost -= CalculateCostReduction(type, goldCost);

			return (woodCost, oreCost, foodCost, goldCost, GetBuildingMaxLevel(type));
		}

		/// <summary>
		/// Gets the age for a building type.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <returns>The age value.</returns>
		public Age GetBuildingAge(BuildingType type) => _buildingSettings.BuildingAges[type];

		/// <summary>
		/// Initializes runtime state dictionaries and building settings.
		/// Sets up default values for all building types.
		/// </summary>
		private void InitializeRuntimeState()
		{
			_buildingSettings.BuildingsMaxLevel = new Dictionary<BuildingType, int>();
			_buildingSettings.BuildingCostModifiers = new Dictionary<BuildingType, int>();
			_buildingSettings.BuildingAges = new Dictionary<BuildingType, Age>();
			_buildingSettings.GlobalBuildCostModifier = 0;

			for (int i = 0; i < (int)BuildingType.Count; i++)
			{
				BuildingType buildingType = (BuildingType)i;
				var buildingData = _buildingDataContainer.BuildingDataDictionary[buildingType];
				_buildingSettings.BuildingsMaxLevel.Add(buildingType, 1);
				_buildingSettings.BuildingCostModifiers.Add(buildingType, 0);
				_buildingSettings.BuildingAges.Add(buildingType, buildingData.StartingAge);
			}

			// Initialize building data dictionaries
			_buildingRuntimeData.Placers = new Dictionary<Player, BuildingPlacer>();
			_buildingRuntimeData.Buildings = new Dictionary<BuildingType, List<BuildingBase>>();
			_buildingRuntimeData.BuildingCounts = new Dictionary<BuildingType, int>();
			_buildingRuntimeData.BuildingsUnlocked = new Dictionary<BuildingType, bool>();
			_buildingRuntimeData.NumberOfBuildings = 0;

			for (int i = 0; i < (int)BuildingType.Count; i++)
			{
				BuildingType buildingType = (BuildingType)i;
				_buildingRuntimeData.Buildings.Add(buildingType, new List<BuildingBase>());
				_buildingRuntimeData.BuildingCounts.Add(buildingType, 0);
				_buildingRuntimeData.BuildingsUnlocked.Add(buildingType, false);
			}
		}

		/// <summary>
		/// Checks if a building can be leveled without actually leveling it.
		/// </summary>
		/// <param name="building">The building to check.</param>
		/// <returns>True if the building can be leveled, false otherwise.</returns>
		public bool CanLevelBuilding(BuildingBase building)
		{
			// Check if the building can be leveled up.
			if (!building.BuildingData.CanLevel || building.LevelHandler == null || building.BuildingState == BuildingState.Construction)
				return false;

			// Check the building is not at max level.
			if (!building.LevelHandler.CanLevel(true))
				return false;

			// Check that the town can afford to level the building.
			if (!CanAffordToLevel(building.BuildingType, building.LevelHandler.Level))
				return false;

			// Finally, attempt to level the building.
			if (building.LevelHandler.TryLevel())
				return true;

			return false;
		}

		/// <summary>
		/// Resets all buildings of a specific type.
		/// Saves current building data, disables objects, and reloads them.
		/// </summary>
		/// <param name="type">The building type to reset.</param>
		public void ResetBuilding(BuildingType type)
		{
			List<(string poolName, Vector3 position, Quaternion rotation, Vector3 scale,
				int health, uint guid, BuildingState state, int level, List<PoolableObject> foliage)> buildings =
				new List<(string, Vector3, Quaternion, Vector3, int, uint, BuildingState, int, List<PoolableObject>)>();

			List<PoolableObject> objs = _poolingProcessor.GetAllActivePooledObjectsOfType(type.ToString());
			if (objs != null)
			{
				for (int o = 0; o < objs.Count; o++)
				{
					BuildingBase current = objs[o].GetComponent<BuildingBase>();
					GUIDSystem.GUIDComponent guid = objs[o].GetComponent<GUIDSystem.GUIDComponent>();
					if (current == null || guid == null)
						continue;

					buildings.Add((
						objs[o].PoolName,
						current.transform.position,
						current.transform.rotation,
						current.transform.localScale,
						current.HealthHandler.Health,
						guid.GUID,
						current.BuildingState,
						current.LevelHandler != null ? current.LevelHandler.Level : 1,
						current.FoliageRemoved == null
							? new List<PoolableObject>()
							: new List<PoolableObject>(current.FoliageRemoved)));
				}
			}

			ResetBuildingType(type);
			_poolingProcessor.DisableObjectsInPool(type.ToString());

			for(int i = 0; i < buildings.Count; i++)
			{
				var data = buildings[i];
				PoolableObject pooled = _poolingProcessor.GetPooledObject(data.poolName, data.position, data.rotation, false);
				BuildingBase building = pooled != null ? pooled.GetComponent<BuildingBase>() : null;
				if (building == null)
					continue;

				building.transform.localScale = data.scale;
				building.BuildingState = data.state;
				building.HealthHandler.SetHealth(data.health);
				building.FoliageRemoved = data.foliage;
				_guidProcessor.RegisterLoadedGUID(pooled, data.guid);
				AddLoadedBuilding(building);
				if (building.BuildingState == BuildingState.Building)
					building.OnLoadedBuiltBuilding();
				if (building.LevelHandler != null)
					building.LevelHandler.RestoreLevel(Mathf.Max(1, data.level));

				building.HealthHandler.SetHealth(data.health);
				building.DamageHandler?.OnHealthChanged(building.HealthHandler);
			}
		}

		// Adds a building to the building list and updates counts.
		private void AddBuilding(BuildingBase building)
		{
			BuildingType type = building.BuildingType;
			_buildingRuntimeData.Buildings[type].Add(building);
			_buildingRuntimeData.BuildingCounts[type]++;
			_buildingRuntimeData.NumberOfBuildings++;
		}

		// Removes a building from the building list and updates counts.
		private void RemoveBuilding(BuildingBase building)
		{
			BuildingType type = building.BuildingType;
			_buildingRuntimeData.Buildings[type].Remove(building);
			_buildingRuntimeData.BuildingCounts[type]--;
			_buildingRuntimeData.NumberOfBuildings--;
		}

		// Adds a building placer for a player to track active placements.
		private void AddPlacer(Player player, BuildingPlacer placer)
		{
			_buildingRuntimeData.Placers.Add(player, placer);
		}

		// Removes a building placer for a player when placement is complete or cancelled.
		private void RemovePlacer(Player player)
		{
			_buildingRuntimeData.Placers.Remove(player);
		}

		// Clears all buildings of a specific type and resets their count to zero.
		private void ResetBuildingType(BuildingType type)
		{
			_buildingRuntimeData.Buildings[type].Clear();
			_buildingRuntimeData.BuildingCounts[type] = 0;
		}

		/// <summary>
		/// Unlocks all buildings for debug purposes.
		/// Sets the IgnoreTechUnlocks flag to true to bypass tech unlock checks.
		/// </summary>
		public void UnlockAllBuildings()
		{
			_buildingSettings.UnlockAllBuildings();
		}

		#endregion

		/// <summary>
		/// Gets building data for a specific building type.
		/// </summary>
		/// <param name="type">The building type.</param>
		/// <returns>The building data scriptable object.</returns>
		public BuildingDataSettings GetBuildingData(BuildingType type)
		{
			return _buildingDataContainer.BuildingDataDictionary[type];
		}
	}
}
