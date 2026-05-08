using System;
using Buildings;
using Character;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.InputSystem;
using UserInterface;
using Utils;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using Processors;
using Sirenix.OdinInspector;

namespace Processors
{
	/// <summary>
	/// Categories for debug log filtering.
	/// </summary>
	public enum DebugLogCategory
	{
		Targetable,
		ObjectPoolingProcessor,
		CellSpacePartitioning,
		ResourceGathering,
		GoToLocation,
		StateMachine,
		Building,
		Combat,
		Pathfinding,
		General,
		DebugUI,
		MainMenuManager,
		LoadingManager,
		TwitchClient,
		TwitchMessage,
		DebugCommand,
		STSM_Action_Build,
		STSM_Action_GatherResource,
		STSM_Helper_Build,
		STSM_Idle_Player,
		HealerRole,
		BuilderSearch,
		WorldGen,
		WorldGenProcessor,
		FoliageProcessor,
		BuildingModelHandler,
		GridProcessor,
		TL_Secrets,
		Coordinator,
		RoleHandler,
		PlacementProbe,
		GlobalAudioController,
		PlayerAudioHandler,
		PlacementProbeHandler,
		CameraController,
		VfxAnimationController,
		SimpleScreenShot,
		ObjectSelectionProcessor,
		RoleProcessor,
		SeasonProcessor,
		VoteEvent,
		BuildingPlacer,
		GameIO,
		TechTreeProcessor,
		TechnologyTree,
		TownGoalProcessor,
		Objective,
		Goal,
		PlayerInputProcessor,
		UserInterface_TownVote,
		UserInterface_TownGoal,
		SaveableResource,
		ScriptablesEditor,
		BuildingDataContainer,
		RoleDataContainer,
		SeasonDataContainer,
		RoleSlot
	}

	/// <summary>
	/// Processor that manages debug functionality for the game.
	/// Handles object selection for debugging and inspector display.
	/// </summary>
	public partial class DebugProcessor : MonoBehaviour, IInstaller, IProcessor, IMainThreadInitializableProcessor
	{
        /// <summary>
        /// ScriptableObject containing debug settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private DebugSettings _debugSettings;

        /// <summary>
        /// Runtime data ScriptableObject for debug data.
        /// Created and bound in InjectRuntimeData().
        /// </summary>
        private DebugRuntimeData _debugRuntimeData;

		[Header("Log Filtering")]
		[TitleGroup("Core Systems")]
		[SerializeField] private bool _enableTargetableLogs = true;
		[SerializeField] private bool _enableObjectPoolingLogs = true;
		[SerializeField] private bool _enableCellSpacePartitioningLogs = true;
		[SerializeField] private bool _enableGridProcessorLogs = true;
		[SerializeField] private bool _enableCoordinatorLogs = true;
		[SerializeField] private bool _enableGameIOLogs = true;

		[TitleGroup("World Generation")]
		[SerializeField] private bool _enableWorldGenLogs = true;
		[SerializeField] private bool _enableWorldGenProcessorLogs = true;
		[SerializeField] private bool _enableFoliageProcessorLogs = true;

		[TitleGroup("State Machine")]
		[SerializeField] private bool _enableStateMachineLogs = true;
		[SerializeField] private bool _enableGoToLocationLogs = true;
		[SerializeField] private bool _enableResourceGatheringLogs = true;
		[SerializeField] private bool _enableSTSM_Action_BuildLogs = true;
		[SerializeField] private bool _enableSTSM_Action_GatherResourceLogs = true;
		[SerializeField] private bool _enableSTSM_Helper_BuildLogs = true;
		[SerializeField] private bool _enableSTSM_Idle_PlayerLogs = true;
		[SerializeField] private bool _enableHealerRoleLogs = true;
		[SerializeField] private bool _enableBuilderSearchLogs = true;

		[TitleGroup("Building")]
		[SerializeField] private bool _enableBuildingLogs = true;
		[SerializeField] private bool _enableBuildingModelHandlerLogs = true;
		[SerializeField] private bool _enableBuildingPlacerLogs = true;
		[SerializeField] private bool _enablePlacementProbeLogs = true;
		[SerializeField] private bool _enablePlacementProbeHandlerLogs = true;

		[TitleGroup("Combat")]
		[SerializeField] private bool _enableCombatLogs = true;
		[SerializeField] private bool _enablePathfindingLogs = true;

		[TitleGroup("Character")]
		[SerializeField] private bool _enableRoleHandlerLogs = true;
		[SerializeField] private bool _enableRoleProcessorLogs = true;
		[SerializeField] private bool _enableRoleSlotLogs = true;
		[SerializeField] private bool _enableRoleDataContainerLogs = true;

		[TitleGroup("Audio")]
		[SerializeField] private bool _enableGlobalAudioControllerLogs = true;
		[SerializeField] private bool _enablePlayerAudioHandlerLogs = true;

		[TitleGroup("UI")]
		[SerializeField] private bool _enableDebugUILogs = true;
		[SerializeField] private bool _enableMainMenuManagerLogs = true;
		[SerializeField] private bool _enableLoadingManagerLogs = true;
		[SerializeField] private bool _enableUserInterface_TownVoteLogs = true;
		[SerializeField] private bool _enableUserInterface_TownGoalLogs = true;
		[SerializeField] private bool _enableObjectSelectionProcessorLogs = true;

		[TitleGroup("Twitch")]
		[SerializeField] private bool _enableTwitchClientLogs = true;
		[SerializeField] private bool _enableTwitchMessageLogs = true;
		[SerializeField] private bool _enableDebugCommandLogs = true;
		[SerializeField] private bool _enableTL_SecretsLogs = true;

		[TitleGroup("Game Systems")]
		[SerializeField] private bool _enableTechTreeProcessorLogs = true;
		[SerializeField] private bool _enableTechnologyTreeLogs = true;
		[SerializeField] private bool _enableTownGoalProcessorLogs = true;
		[SerializeField] private bool _enableObjectiveLogs = true;
		[SerializeField] private bool _enableGoalLogs = true;
		[SerializeField] private bool _enableSeasonProcessorLogs = true;
		[SerializeField] private bool _enableSeasonDataContainerLogs = true;
		[SerializeField] private bool _enableVoteEventLogs = true;

		[TitleGroup("Player")]
		[SerializeField] private bool _enablePlayerInputProcessorLogs = true;
		[SerializeField] private bool _enableCameraControllerLogs = true;

		[TitleGroup("Misc")]
		[SerializeField] private bool _enableVfxAnimationControllerLogs = true;
		[SerializeField] private bool _enableSimpleScreenShotLogs = true;
		[SerializeField] private bool _enableSaveableResourceLogs = true;
		[SerializeField] private bool _enableScriptablesEditorLogs = true;
		[SerializeField] private bool _enableBuildingDataContainerLogs = true;
		[SerializeField] private bool _enableGeneralLogs = true;

        public void Initialize()
        {
            if (_debugRuntimeData == null)
                throw new InvalidOperationException("DebugProcessor: DebugRuntimeData has not been installed.");

            _debugRuntimeData.OnObjectSelected.AddListener(ObjectSelected);
        }

        public void Process()
        {
            if (Keyboard.current.escapeKey.wasReleasedThisFrame)
            {
                _debugSettings.DebugUI.HideBuildingContext();
                _debugSettings.DebugUI.HideCharacterContext();
            }
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // DebugProcessor does not have scene-specific settings to refresh
        }

        public void InstallBindings(ContainerBuilder containerBuilder)
        {
            containerBuilder.AddSingleton(this);
            InjectRuntimeData(containerBuilder);
        }

        public void InjectRuntimeData(ContainerBuilder containerBuilder)
        {
            if (_debugRuntimeData != null)
                throw new InvalidOperationException("DebugProcessor: DebugRuntimeData has already been installed.");

            _debugRuntimeData = new DebugRuntimeData();
            containerBuilder.AddSingleton(_debugRuntimeData);
        }

        // Handles object selection events and updates the selected object in debug data.
        private void ObjectSelected(SelectableObject selected, object data)
        {
            _debugRuntimeData.SelectedObject = (selected, data);

            Debug.Log($"Object Selected: {selected.gameObject.transform.parent.name}, {selected.SelectableType}");
        }

		/// <summary>
		/// Logs a message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		public void Log(DebugLogCategory category, string message)
		{
			if (!IsCategoryEnabled(category))
				return;

			Debug.Log($"[{category}] - {message}");
		}

		/// <summary>
		/// Logs a message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		/// <param name="context">Object to which the message applies.</param>
		public void Log(DebugLogCategory category, string message, UnityEngine.Object context)
		{
			if (!IsCategoryEnabled(category))
				return;

			Debug.Log($"[{category}] - {message}", context);
		}

		/// <summary>
		/// Logs a warning message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		public void LogWarning(DebugLogCategory category, string message)
		{
			if (!IsCategoryEnabled(category))
				return;

			Debug.LogWarning($"[{category}] - {message}");
		}

		/// <summary>
		/// Logs a warning message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		/// <param name="context">Object to which the message applies.</param>
		public void LogWarning(DebugLogCategory category, string message, UnityEngine.Object context)
		{
			if (!IsCategoryEnabled(category))
				return;

			Debug.LogWarning($"[{category}] - {message}", context);
		}

		/// <summary>
		/// Logs an error message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		public void LogError(DebugLogCategory category, string message)
		{
			if (!IsCategoryEnabled(category))
				return;

			Debug.LogError($"[{category}] - {message}");
		}

		/// <summary>
		/// Logs an error message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		/// <param name="context">Object to which the message applies.</param>
		public void LogError(DebugLogCategory category, string message, UnityEngine.Object context)
		{
			if (!IsCategoryEnabled(category))
				return;

			Debug.LogError($"[{category}] - {message}", context);
		}

		/// <summary>
		/// Checks if a log category is enabled.
		/// </summary>
		/// <param name="category">The category to check.</param>
		/// <returns>True if the category is enabled, false otherwise.</returns>
		private bool IsCategoryEnabled(DebugLogCategory category)
		{
			switch (category)
			{
				case DebugLogCategory.Targetable:
					return _enableTargetableLogs;
				case DebugLogCategory.ObjectPoolingProcessor:
					return _enableObjectPoolingLogs;
				case DebugLogCategory.CellSpacePartitioning:
					return _enableCellSpacePartitioningLogs;
				case DebugLogCategory.ResourceGathering:
					return _enableResourceGatheringLogs;
				case DebugLogCategory.GoToLocation:
					return _enableGoToLocationLogs;
				case DebugLogCategory.StateMachine:
					return _enableStateMachineLogs;
				case DebugLogCategory.Building:
					return _enableBuildingLogs;
				case DebugLogCategory.Combat:
					return _enableCombatLogs;
				case DebugLogCategory.Pathfinding:
					return _enablePathfindingLogs;
				case DebugLogCategory.General:
					return _enableGeneralLogs;
				case DebugLogCategory.DebugUI:
					return _enableDebugUILogs;
				case DebugLogCategory.MainMenuManager:
					return _enableMainMenuManagerLogs;
				case DebugLogCategory.LoadingManager:
					return _enableLoadingManagerLogs;
				case DebugLogCategory.TwitchClient:
					return _enableTwitchClientLogs;
				case DebugLogCategory.TwitchMessage:
					return _enableTwitchMessageLogs;
				case DebugLogCategory.DebugCommand:
					return _enableDebugCommandLogs;
				case DebugLogCategory.STSM_Action_Build:
					return _enableSTSM_Action_BuildLogs;
				case DebugLogCategory.STSM_Action_GatherResource:
					return _enableSTSM_Action_GatherResourceLogs;
				case DebugLogCategory.STSM_Helper_Build:
					return _enableSTSM_Helper_BuildLogs;
				case DebugLogCategory.STSM_Idle_Player:
					return _enableSTSM_Idle_PlayerLogs;
				case DebugLogCategory.HealerRole:
					return _enableHealerRoleLogs;
				case DebugLogCategory.BuilderSearch:
					return _enableBuilderSearchLogs;
				case DebugLogCategory.WorldGen:
					return _enableWorldGenLogs;
				case DebugLogCategory.WorldGenProcessor:
					return _enableWorldGenProcessorLogs;
				case DebugLogCategory.FoliageProcessor:
					return _enableFoliageProcessorLogs;
				case DebugLogCategory.BuildingModelHandler:
					return _enableBuildingModelHandlerLogs;
				case DebugLogCategory.GridProcessor:
					return _enableGridProcessorLogs;
				case DebugLogCategory.TL_Secrets:
					return _enableTL_SecretsLogs;
				case DebugLogCategory.Coordinator:
					return _enableCoordinatorLogs;
				case DebugLogCategory.RoleHandler:
					return _enableRoleHandlerLogs;
				case DebugLogCategory.PlacementProbe:
					return _enablePlacementProbeLogs;
				case DebugLogCategory.GlobalAudioController:
					return _enableGlobalAudioControllerLogs;
				case DebugLogCategory.PlayerAudioHandler:
					return _enablePlayerAudioHandlerLogs;
				case DebugLogCategory.PlacementProbeHandler:
					return _enablePlacementProbeHandlerLogs;
				case DebugLogCategory.CameraController:
					return _enableCameraControllerLogs;
				case DebugLogCategory.VfxAnimationController:
					return _enableVfxAnimationControllerLogs;
				case DebugLogCategory.SimpleScreenShot:
					return _enableSimpleScreenShotLogs;
				case DebugLogCategory.ObjectSelectionProcessor:
					return _enableObjectSelectionProcessorLogs;
				case DebugLogCategory.RoleProcessor:
					return _enableRoleProcessorLogs;
				case DebugLogCategory.SeasonProcessor:
					return _enableSeasonProcessorLogs;
				case DebugLogCategory.VoteEvent:
					return _enableVoteEventLogs;
				case DebugLogCategory.BuildingPlacer:
					return _enableBuildingPlacerLogs;
				case DebugLogCategory.GameIO:
					return _enableGameIOLogs;
				case DebugLogCategory.TechTreeProcessor:
					return _enableTechTreeProcessorLogs;
				case DebugLogCategory.TechnologyTree:
					return _enableTechnologyTreeLogs;
				case DebugLogCategory.TownGoalProcessor:
					return _enableTownGoalProcessorLogs;
				case DebugLogCategory.Objective:
					return _enableObjectiveLogs;
				case DebugLogCategory.Goal:
					return _enableGoalLogs;
				case DebugLogCategory.PlayerInputProcessor:
					return _enablePlayerInputProcessorLogs;
				case DebugLogCategory.UserInterface_TownVote:
					return _enableUserInterface_TownVoteLogs;
				case DebugLogCategory.UserInterface_TownGoal:
					return _enableUserInterface_TownGoalLogs;
				case DebugLogCategory.SaveableResource:
					return _enableSaveableResourceLogs;
				case DebugLogCategory.ScriptablesEditor:
					return _enableScriptablesEditorLogs;
				case DebugLogCategory.BuildingDataContainer:
					return _enableBuildingDataContainerLogs;
				case DebugLogCategory.RoleDataContainer:
					return _enableRoleDataContainerLogs;
				case DebugLogCategory.SeasonDataContainer:
					return _enableSeasonDataContainerLogs;
				case DebugLogCategory.RoleSlot:
					return _enableRoleSlotLogs;
				default:
					return false;
			}
		}
    }
}
