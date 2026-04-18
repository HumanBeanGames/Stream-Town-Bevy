using System;
using Utils;
using Character;
using Reflex.Core;
using UnityEngine;

namespace Processors
{
    /// <summary>
    /// Processor that manages game events through action-based event system.
    /// Provides centralized events for enemy kills, building construction, resource changes, and player death.
    /// </summary>
	public class EventProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// Event fired when an enemy is killed.
        /// </summary>
        public Action<EnemyType> EnemyKilled;

        /// <summary>
        /// Event fired when a building is constructed.
        /// </summary>
        public Action<BuildingType> BuildingBuilt;

        /// <summary>
        /// Event fired when resources are gained (e.g., from harvesting or trade).
        /// </summary>
        public Action<Resource, int> ResourceGained;

        /// <summary>
        /// Event fired when resources are sold.
        /// </summary>
        public Action<Resource, int> ResourceSold;

        /// <summary>
        /// Event fired when resources are bought.
        /// </summary>
        public Action<Resource, int> ResourceBought;

        /// <summary>
        /// Event fired when a player dies.
        /// </summary>
        public Action<Player> PlayerDied;

        /// <summary>
        /// Initializes the event processor.
        /// No initialization logic required for this processor.
        /// </summary>
        public void Initialize()
        {
            // EventProcessor doesn't require initialization logic
        }

        /// <summary>
        /// Processes event logic every frame.
        /// Called every frame by the Coordinator.
        /// EventProcessor does not require per-frame updates.
        /// </summary>
        public void Process()
        {
            // EventProcessor does not require per-frame updates
        }

        /// <summary>
        /// Registers this processor as a singleton in the dependency injection container.
        /// Called by Reflex during container initialization.
        /// </summary>
        /// <param name="containerBuilder">The container builder to register bindings with.</param>
        public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
