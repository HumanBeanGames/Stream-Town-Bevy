using System;
using System.Runtime.CompilerServices;
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
		[HideInCallstack]
		public void Log(DebugLogCategory category, string message, [CallerFilePath] string callerFilePath = "", [CallerMemberName] string callerMemberName = "")
		{
			SetCategory(category, callerFilePath, callerMemberName);
			Debug.Log(message);
		}

		/// <summary>
		/// Logs a message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		/// <param name="context">Object to which the message applies.</param>
		[HideInCallstack]
		public void Log(DebugLogCategory category, string message, UnityEngine.Object context, [CallerFilePath] string callerFilePath = "", [CallerMemberName] string callerMemberName = "")
		{
			SetCategory(category, callerFilePath, callerMemberName);
			Debug.Log(message, context);
		}

		/// <summary>
		/// Logs a warning message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		[HideInCallstack]
		public void LogWarning(DebugLogCategory category, string message, [CallerFilePath] string callerFilePath = "", [CallerMemberName] string callerMemberName = "")
		{
			SetCategory(category, callerFilePath, callerMemberName);
			Debug.LogWarning(message);
		}

		/// <summary>
		/// Logs a warning message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		/// <param name="context">Object to which the message applies.</param>
		[HideInCallstack]
		public void LogWarning(DebugLogCategory category, string message, UnityEngine.Object context, [CallerFilePath] string callerFilePath = "", [CallerMemberName] string callerMemberName = "")
		{
			SetCategory(category, callerFilePath, callerMemberName);
			Debug.LogWarning(message, context);
		}

		/// <summary>
		/// Logs an error message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		[HideInCallstack]
		public void LogError(DebugLogCategory category, string message, [CallerFilePath] string callerFilePath = "", [CallerMemberName] string callerMemberName = "")
		{
			SetCategory(category, callerFilePath, callerMemberName);
			Debug.LogError(message);
		}

		/// <summary>
		/// Logs an error message with the specified category if that category is enabled.
		/// </summary>
		/// <param name="category">The category of the log message.</param>
		/// <param name="message">The message to log.</param>
		/// <param name="context">Object to which the message applies.</param>
		[HideInCallstack]
		public void LogError(DebugLogCategory category, string message, UnityEngine.Object context, [CallerFilePath] string callerFilePath = "", [CallerMemberName] string callerMemberName = "")
		{
			SetCategory(category, callerFilePath, callerMemberName);
			Debug.LogError(message, context);
		}

		private static void SetCategory(DebugLogCategory category, string callerFilePath, string callerMemberName)
		{
		#if UNITY_EDITOR
			Core.Coordinator.CustomLogHandler.SetNextCategory(category, callerFilePath, callerMemberName);
		#endif
		}
    }
}
