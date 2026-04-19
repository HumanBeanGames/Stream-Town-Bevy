using Buildings;
using Character;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.InputSystem;
using UserInterface;
using Utils;
using Reflex.Core;
using Reflex.Attributes;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
	/// <summary>
	/// Processor that manages debug functionality for the game.
	/// Handles object selection for debugging and inspector display.
	/// </summary>
	public partial class DebugProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// ScriptableObject containing debug settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private DebugSettings _debugSettings;

        /// <summary>
        /// Runtime data ScriptableObject for debug data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private DebugRuntimeData _debugRuntimeData;

        public void Initialize()
		{
			_debugRuntimeData.OnObjectSelected.AddListener(ObjectSelected);
		}

        /// <summary>
        /// Checks for escape key press to hide debug context menus.
        /// Called every frame by the Coordinator.
        /// </summary>
        public void Process()
        {
            if (Keyboard.current.escapeKey.wasReleasedThisFrame)
            {
                _debugSettings.DebugUI.HideBuildingContext();
                _debugSettings.DebugUI.HideCharacterContext();
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
		/// Injects the DebugRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			DebugRuntimeData debugRuntimeData = ScriptableObject.CreateInstance<DebugRuntimeData>();
			containerBuilder.AddSingleton(debugRuntimeData);
		}

        // Handles object selection events and updates the selected object in debug data.
		private void ObjectSelected(SelectableObject selected, object data)
		{
			_debugRuntimeData.SelectedObject = (selected, data);

			Debug.Log($"Object Selected: {selected.gameObject.transform.parent.name}, {selected.SelectableType}");
		}
	}
}
