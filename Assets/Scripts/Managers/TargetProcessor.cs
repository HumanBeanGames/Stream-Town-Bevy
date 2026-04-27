using System;
using System.Collections.Generic;
using Target;
using UnityEngine;
using Utils;
using Reflex.Core;
using Reflex.Attributes;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
    /// <summary>
    /// Manages all targets in the game.
    /// Handles target registration, removal, and querying by type flags.
    /// </summary>
    //TODO:: Check if this is still required after BSP implementation
    public partial class TargetProcessor : MonoBehaviour, IInstaller, IProcessor
    {
        /// <summary>
        /// ScriptableObject containing target settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TargetSettings _targetSettings;

        /// <summary>
        /// Runtime data for target data.
        /// Assigned in InjectRuntimeData.
        /// </summary>
        private TargetRuntimeData _targetRuntimeData;


        /// <summary>
        /// Gets the update type for a target mask.
        /// </summary>
        /// <param name="type">The target mask to get update type for.</param>
        /// <returns>The station update type for the target.</returns>
        public StationUpdate GetUpdateType(TargetMask type)
        {
            return _targetSettings.TargetableData[TargetFlagHelper.GetIndexByFlag(type)].UpdateType;
        }

        /// <summary>
        /// Gets all targets defined by the flag into one list.
        /// </summary>
        /// <param name="flag">The target flags to filter by.</param>
        /// <returns>List of targets matching the flags.</returns>
        public List<Targetable> GetTargetsByFlag(TargetMask flag)
        {
            List<Targetable> targets = new List<Targetable>();

            foreach (int i in System.Enum.GetValues(typeof(TargetMask)))
            {
                TargetMask t = (TargetMask)i;

                if (t == TargetMask.Nothing)
                    continue;

                if (!flag.HasFlag(t) || !_targetRuntimeData.TargetDictionary.ContainsKey(t))
                    continue;

                targets.AddRange(_targetRuntimeData.TargetDictionary[t]);
            }

            return targets;
        }

        /// <summary>
        /// Adds a target to the target dictionary based on its type flags.
        /// </summary>
        /// <param name="target">The target to add.</param>
        public void AddTarget(Targetable target)
        {
            // Add to each flag type
            foreach (int i in Enum.GetValues(typeof(TargetMask)))
            {
                TargetMask t = (TargetMask)i;

                if (t == TargetMask.Nothing)
                    continue;

                if (target.TargetType.HasFlag(t))
                {
                    AddTarget(t, target);
                }
            }
        }

        /// <summary>
        /// Removes a target from the target dictionary based on its type flags.
        /// </summary>
        /// <param name="target">The target to remove.</param>
        public void RemoveTarget(Targetable target)
        {
            foreach (int i in Enum.GetValues(typeof(TargetMask)))
            {
                TargetMask t = (TargetMask)i;

                if (t == TargetMask.Nothing)
                    continue;

                if (target.TargetType.HasFlag(t))
                {
                    RemoveTarget(t, target);
                }
            }
        }

        /// <summary>
        /// Initializes the target processor.
        /// Creates RuntimeData after all processors are confirmed ready.
        /// </summary>
        public void Initialize()
        {
            if (_targetRuntimeData == null)
                throw new InvalidOperationException("TargetProcessor: TargetRuntimeData has not been installed.");
        }

        /// <summary>
        /// Processes target logic every frame.
        /// Called every frame by the Coordinator.
        /// TargetProcessor does not require per-frame updates.
        /// </summary>
        public void Process()
        {
            // TargetProcessor does not require per-frame updates
        }

        /// <summary>
        /// Refreshes scene-specific data when a new scene loads.
        /// Called by the Coordinator after scene container is available.
        /// </summary>
        public void RefreshSceneData(Container sceneContainer)
        {
            // TargetProcessor does not have scene-specific settings to refresh
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
        /// Injects the TargetRuntimeData ScriptableObject into the DI container.
        /// </summary>
        /// <param name="containerBuilder">The container builder to register bindings with.</param>
        public void InjectRuntimeData(ContainerBuilder containerBuilder)
        {
            if (_targetRuntimeData != null)
                throw new InvalidOperationException("TargetProcessor: TargetRuntimeData has already been installed.");

            _targetRuntimeData = new TargetRuntimeData();
            containerBuilder.AddSingleton(_targetRuntimeData);
        }

        /// <summary>
        /// Adds a target to the dictionary under a specific type mask.
        /// </summary>
        /// <param name="type">The type mask to add the target under.</param>
        /// <param name="target">The target to add.</param>
        private void AddTarget(TargetMask type, Targetable target)
        {
            if (!_targetRuntimeData.TargetDictionary.ContainsKey(type))
                _targetRuntimeData.TargetDictionary[type] = new List<Targetable>();

            if (_targetRuntimeData.TargetDictionary[type].Contains(target))
                return;

            _targetRuntimeData.TargetDictionary[type].Add(target);
        }

        /// <summary>
        /// Removes a target from the dictionary under a specific type mask.
        /// </summary>
        /// <param name="type">The type mask to remove the target from.</param>
        /// <param name="target">The target to remove.</param>
        private void RemoveTarget(TargetMask type, Targetable target)
        {
            if (!_targetRuntimeData.TargetDictionary.ContainsKey(type))
                return;

            if (!_targetRuntimeData.TargetDictionary[type].Contains(target))
                return;

            _targetRuntimeData.TargetDictionary[type].Remove(target);
        }
    }
}
