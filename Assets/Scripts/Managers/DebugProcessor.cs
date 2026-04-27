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

namespace Processors
{
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
    }
}
