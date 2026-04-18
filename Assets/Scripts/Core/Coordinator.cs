using Buildings;
using Character;
using GameEventSystem;
using GridSystem.Partitioning;
using TownGoal;
using System.Collections.Generic;
using TechTree;
using UnityEngine;
using UnityEngine.InputSystem;
using Utils;
using World;
using World.Generation;
using SavingAndLoading;
using GUIDSystem;
using Enemies;
using PlayerControls;
using UnityEngine.EventSystems;
using UserInterface.MainMenu;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using System;
using System.Collections;
using System.Linq;
using System.Reflection;
using Environment;
using Utils.Pooling;
using Twitch;
using Audio;
using Data.Containers;
using GameResources;
using GridSystem;
using Sensors;
using Settings;
using Processors;

namespace Core
{
	[DefaultExecutionOrder(-1000)]
	public class Coordinator : MonoBehaviour, IInstaller
	{
		private List<IProcessor> _processors;
		private List<IDataScriptable> _dataScriptables;
		private int _frameCounter = 0;
		private const int WARNING_FRAME_INTERVAL = 120;
		private bool _initializationComplete = false;

		private void Awake()
		{
			_processors = new List<IProcessor>();
			_dataScriptables = new List<IDataScriptable>();
			StartCoroutine(StartupSequence());
		}

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			// Instantiate a copy of Coordinator into the scene
			GameObject coordinatorInstance = new GameObject("Coordinator");
			coordinatorInstance.AddComponent<Coordinator>();
		}

		private IEnumerator StartupSequence()
		{
			// Find all processors via reflection
			CacheProcessorsViaReflection();

			// Check if all processors are available - throw error immediately if not
			if (!AllProcessorsAvailable())
			{
				Debug.LogError("[COORDINATOR] Not all processors are available from ProjectScope. Some processors may not be registered correctly.");
				yield break;
			}

			// Cache data scriptables via reflection
			CacheDataScriptablesViaReflection();

			// Wait for all data scriptables to be available (from SceneScope)
			while (!AllDataScriptablesAvailable())
			{
				_frameCounter++;
				if (_frameCounter % WARNING_FRAME_INTERVAL == 0)
				{
					Debug.LogWarning("[COORDINATOR] Waiting for data scriptables to be available from SceneScope...");
				}
				yield return null;
			}

			// Initialize all processors using IProcessor interface
			InitializeAllProcessors();

			// Mark initialization as complete so Process() can run
			_initializationComplete = true;
		}

		private void CacheProcessorsViaReflection()
		{
			// Find all types that implement IProcessor
			var processorTypes = AppDomain.CurrentDomain.GetAssemblies()
				.SelectMany(assembly => assembly.GetTypes())
				.Where(type => typeof(IProcessor).IsAssignableFrom(type) && !type.IsInterface && !type.IsAbstract);

			foreach (var processorType in processorTypes)
			{
				// Try to get the processor from the container via reflection
				try
				{
					var processor = GameObject.FindObjectOfType(processorType) as IProcessor;
					if (processor != null)
					{
						_processors.Add(processor);
					}
				}
				catch (Exception ex)
				{
					Debug.LogWarning($"[COORDINATOR] Could not find processor of type {processorType.Name}: {ex.Message}");
				}
			}
		}

		private void CacheDataScriptablesViaReflection()
		{
			// Find all types that implement IDataScriptable
			var dataScriptableTypes = AppDomain.CurrentDomain.GetAssemblies()
				.SelectMany(assembly => assembly.GetTypes())
				.Where(type => typeof(IDataScriptable).IsAssignableFrom(type) && !type.IsInterface && !type.IsAbstract);

			foreach (var scriptableType in dataScriptableTypes)
			{
				// Try to get the scriptable from Resources
				try
				{
					var scriptables = Resources.LoadAll(scriptableType.Name);
					foreach (var scriptable in scriptables)
					{
						if (scriptable is IDataScriptable dataScriptable)
						{
							_dataScriptables.Add(dataScriptable);
						}
					}
				}
				catch (Exception ex)
				{
					Debug.LogWarning($"[COORDINATOR] Could not find data scriptable of type {scriptableType.Name}: {ex.Message}");
				}
			}
		}

		private bool AllProcessorsAvailable()
		{
			return _processors.Count > 0;
		}

		private bool AllDataScriptablesAvailable()
		{
			foreach (var scriptable in _dataScriptables)
			{
				if (scriptable == null)
					return false;
			}
			return true;
		}

		private void InitializeAllProcessors()
		{
			foreach (var processor in _processors)
			{
				processor.Initialize();
			}
		}

		private void Update()
		{
			// Only call Process() after initialization is complete
			if (!_initializationComplete)
				return;

			// Call Process() on all processors every frame
			foreach (var processor in _processors)
			{
				processor.Process();
			}
		}
	}
}
