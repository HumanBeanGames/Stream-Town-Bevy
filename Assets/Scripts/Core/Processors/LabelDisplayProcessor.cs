using Character;
using Reflex.Attributes;
using System.Collections.Generic;
using Target;
using System;
using UnityEngine;
using UserInterface;
using VFX;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
	/// <summary>
	/// Used to manage the display text over targetable objects.
	/// This used as the game will run poorly if everything has it's own text component.
	/// </summary>
	public partial class LabelDisplayProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// Object pooling processor for managing pooled objects.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingProcessor _objectPoolingProcessor;

        /// <summary>
        /// Runtime data for util display.
        /// Assigned in InjectRuntimeData.
        /// </summary>
        private UtilDisplayRuntimeData _utilDisplayRuntimeData;

        /// <summary>
        /// ScriptableObject containing object pooling settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingSettings _objectPoolingSettings;


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
		/// Injects the UtilDisplayRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_utilDisplayRuntimeData != null)
				throw new InvalidOperationException("LabelDisplayProcessor: UtilDisplayRuntimeData has already been installed.");

			_utilDisplayRuntimeData = new UtilDisplayRuntimeData();
			containerBuilder.AddSingleton(_utilDisplayRuntimeData);
		}

		/// <summary>
		/// Adds a target and it's display to the dictionary.
		/// </summary>
		/// <param name="target">The targetable object to display text over.</param>
		/// <param name="text">The text to display.</param>
		/// <param name="time">The duration to display the text.</param>
		public void AddTextDisplay(Targetable target, string text, float time = 15.0f)
		{
			UnitTextDisplay display = default;

			if (!_utilDisplayRuntimeData.ActiveTextDisplays.ContainsKey(target))
			{
				var textDisplay = _objectPoolingProcessor.GetPooledObject("TextDisplay");
				textDisplay.gameObject.SetActive(true);
				var rectTransform = textDisplay.GetComponent<RectTransform>();
				rectTransform.SetParent(target.TextDisplayTransform, false);
				rectTransform.localPosition = target.TextDisplayTransform.localPosition;

				display = textDisplay.GetComponent<UnitTextDisplay>();
				display.Targetable = target;

				_utilDisplayRuntimeData.ActiveTextDisplays.Add(target, display);
			}
			else
				display = _utilDisplayRuntimeData.ActiveTextDisplays[target];

			if (!display.gameObject.activeInHierarchy)
				display.gameObject.SetActive(true);

			display.SetDisplayText(text);
			display.SetDisplayTextAfterTime($"", time);
		}

		/// <summary>
		/// Removes the target and its display from the dictionary.
		/// </summary>
		/// <param name="target">The targetable object to remove.</param>
		public void RemoveTextDisplay(Targetable target)
		{
			if (target != null && _utilDisplayRuntimeData.ActiveTextDisplays.ContainsKey(target))
				_utilDisplayRuntimeData.ActiveTextDisplays.Remove(target);
		}

		/// <summary>
		/// Adds a ping VFX object to a player.
		/// </summary>
		/// <param name="player">The player to add the ping to.</param>
		public void AddPingObject(Player player)
		{
			if (_utilDisplayRuntimeData.PingObjects.ContainsKey(player))
				return;

			VFXArrowPointer pingObject = _objectPoolingProcessor.GetPooledObject("VFXPing").GetComponent<VFXArrowPointer>();

			pingObject.transform.parent = player.Character.transform;
			pingObject.transform.localPosition = Vector3.zero;
			pingObject.SetPlayer(player);
			pingObject.gameObject.SetActive(true);

			_utilDisplayRuntimeData.PingObjects.Add(player, pingObject.gameObject);
		}

		/// <summary>
		/// Removes a ping VFX object from a player.
		/// </summary>
		/// <param name="player">The player to remove the ping from.</param>
		public void RemovePingObject(Player player)
		{
			if (player != null && !_utilDisplayRuntimeData.PingObjects.ContainsKey(player))
				return;

			_utilDisplayRuntimeData.PingObjects.Remove(player);
		}

		public void Initialize()
		{
			if (_utilDisplayRuntimeData == null)
				throw new InvalidOperationException("LabelDisplayProcessor: UtilDisplayRuntimeData has not been installed.");
		}

		public void Process()
		{
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// LabelDisplayProcessor does not have scene-specific settings to refresh
		}
	}
}
