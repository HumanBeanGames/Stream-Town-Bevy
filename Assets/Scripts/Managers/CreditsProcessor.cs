using UnityEngine;
using UnityEngine.InputSystem;
using UserInterface.MainMenu;
using Reflex.Core;
using Reflex.Attributes;
using ScriptablesProcessorInfrastructure;
using Processors;

namespace Processors 
{
    /// <summary>
    /// Processor that manages credits screen functionality.
    /// Handles credits skipping and scene transitions.
    /// </summary>
    public partial class CreditsProcessor : MonoBehaviour, IInstaller, IProcessor 
	{
        /// <summary>
        /// Runtime data ScriptableObject for credits data.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private CreditsRuntimeData _creditsRuntimeData;

        /// <summary>
        /// Loading processor for scene load requests.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private LoadingProcessor _loadingProcessor;

        /// <summary>
        /// Skips the credits screen and requests scene load.
        /// </summary>
        public void SkipCredits()
		{
			_loadingProcessor.LoadWorldScene(1);
		}

        /// <summary>
        /// Initializes the credits processor.
        /// No initialization logic required for this processor.
        /// </summary>
        public void Initialize()
		{
			// CreditsProcessor doesn't require initialization logic
		}

        /// <summary>
        /// Checks for escape key press to skip credits.
        /// Called every frame by the Coordinator.
        /// </summary>
        public void Process()
        {
            if(Keyboard.current.escapeKey.wasReleasedThisFrame)
                SkipCredits();
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
		/// Injects the CreditsRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			CreditsRuntimeData creditsRuntimeData = ScriptableObject.CreateInstance<CreditsRuntimeData>();
			containerBuilder.AddSingleton(creditsRuntimeData);
		}
	}
}
