using UnityEngine;
using UnityEngine.InputSystem;
using UserInterface.MainMenu;
using Reflex.Core;

namespace Processors 
{
    /// <summary>
    /// Processor that manages credits screen functionality.
    /// Handles credits skipping and scene transitions.
    /// </summary>
	public partial class CreditsProcessor : MonoBehaviour, IInstaller, IProcessor 
	{
		[SerializeField]
        private LoadingManager _loadingProcessor;

        /// <summary>
        /// Skips the credits screen and requests scene load.
        /// </summary>
        public void SkipCredits()
		{
			_loadingProcessor.LoadWorldScene(1);
		}

		public void Initialize()
		{
			// No initialization logic required
		}

		public void Process()
		{
			if (Keyboard.current.escapeKey.wasReleasedThisFrame)
				SkipCredits();
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// CreditsProcessor does not have scene-specific settings to refresh
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// No runtime data for CreditsProcessor
		}
	}
}
