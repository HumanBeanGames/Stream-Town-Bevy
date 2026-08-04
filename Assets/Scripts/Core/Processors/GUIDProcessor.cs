using SavingAndLoading.SavableObjects;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Xml;
using UnityEditor;
using UnityEngine;
using UnityEngine.Rendering;
using Utils.Pooling;
using Reflex.Core;
using Reflex.Attributes;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
	/// <summary>
	/// Processor that manages GUID (Globally Unique Identifier) assignment and tracking.
	/// Provides GUID generation, dictionary management, and object lookup for save/load functionality.
	/// </summary>
	public partial class GUIDProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		/// <summary>
		/// Runtime data for GUID data.
		/// Assigned in InjectRuntimeData.
		/// </summary>
		private GUIDRuntimeData _guidRuntimeData;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		/// <summary>

		public void Initialize()
		{
			if (_guidRuntimeData == null)
				throw new InvalidOperationException("GUIDProcessor runtime data has not been installed.");

			if (_guidRuntimeData.IsInitialized)
				throw new InvalidOperationException("GUIDProcessor runtime data has already been initialized.");

			// Only initialize if not already done
			if (!_guidRuntimeData.IsInitialized)
			{
				// Create the outer dictionary mapping scene names to GUID dictionaries
				Dictionary<string, Dictionary<uint, PoolableObject>> worldObjects = new Dictionary<string, Dictionary<uint, PoolableObject>>();
				
				// Create a dictionary for each pool type
				for (int i = 0; i < (int)PoolType.Count; i++)
				{
					Dictionary<uint, PoolableObject> dic = new Dictionary<uint, PoolableObject>();
					worldObjects.Add(((PoolType)i).ToString(), dic);
				}

				_guidRuntimeData.InitializeWorldObjects(worldObjects);
			}
			else
				_debugProcessor.Log(DebugLogCategory.General, "GUIDProcessor: Is already initialized");

			// Mark as initialized regardless of whether we just initialized or it was already done
			_guidRuntimeData.IsInitialized = true;
		}

		/// <summary>
		/// Creates a new GUID for a poolable object and adds it to the dictionary.
		/// Generates a unique GUID and assigns it to the object's GUID component.
		/// </summary>
		/// <param name="comp">The poolable object to assign a GUID to.</param>
		/// <returns>The newly generated GUID.</returns>
		public uint CreateGUIDandAddToDictionary(PoolableObject comp)
		{
			if (comp == null || !(comp.SaveableObject is SaveableObject saveable) || saveable.GUIDComponent == null)
				throw new ArgumentException("A poolable object with a GUID component is required.", nameof(comp));

			string type = comp.PoolType.ToString();
			Dictionary<uint, PoolableObject> objects = _guidRuntimeData.WorldObjects[type];
			uint existingGuid = saveable.GUIDComponent.GUID;
			if (existingGuid != 0)
			{
				if (!objects.TryGetValue(existingGuid, out PoolableObject existingObject))
				{
					objects.Add(existingGuid, comp);
					return existingGuid;
				}

				if (ReferenceEquals(existingObject, comp))
					return existingGuid;
			}

			uint guid = GenerateNewGUID(type);
			saveable.GUIDComponent.SetGUID(guid);
			objects.Add(guid, comp);
			return guid;
		}

		/// <summary>
		/// Replaces any checkout-time GUID with the identity stored in a save file.
		/// Stale dictionary entries for the same pooled instance are removed first.
		/// </summary>
		public void RegisterLoadedGUID(PoolableObject comp, uint guid)
		{
			if (guid == 0)
				throw new ArgumentOutOfRangeException(nameof(guid), "Loaded GUIDs cannot be zero.");
			if (comp == null || !(comp.SaveableObject is SaveableObject saveable) || saveable.GUIDComponent == null)
				throw new ArgumentException("A poolable object with a GUID component is required.", nameof(comp));

			Dictionary<uint, PoolableObject> objects = _guidRuntimeData.WorldObjects[comp.PoolType.ToString()];
			List<uint> staleKeys = objects
				.Where(pair => ReferenceEquals(pair.Value, comp) && pair.Key != guid)
				.Select(pair => pair.Key)
				.ToList();
			for (int i = 0; i < staleKeys.Count; i++)
				objects.Remove(staleKeys[i]);

			if (objects.TryGetValue(guid, out PoolableObject collision) && !ReferenceEquals(collision, comp))
				throw new InvalidOperationException($"Loaded GUID {guid} is already assigned to another {comp.PoolType} object.");

			saveable.GUIDComponent.SetGUID(guid);
			objects[guid] = comp;
		}

		/// <summary>
		/// Removes an object from the GUID dictionary.
		/// Used when objects are destroyed or removed from the scene.
		/// </summary>
		/// <param name="type">The pool type of the object.</param>
		/// <param name="gUID">The GUID of the object to remove.</param>
		public void RemoveFromGUID(PoolType type, uint gUID)
		{
			// Check if the pool type exists in the dictionary
			if (_guidRuntimeData.WorldObjects.ContainsKey(type.ToString()))
			{
				// Check if the specific GUID exists and remove it
				if (_guidRuntimeData.WorldObjects[type.ToString()].ContainsKey(gUID))
					_guidRuntimeData.WorldObjects[type.ToString()].Remove(gUID);
			}
		}

		/// <summary>
		/// Adds an existing GUID component to the dictionary.
		/// Used when loading saved objects that already have GUIDs assigned.
		/// </summary>
		/// <param name="comp">The poolable object to add to the dictionary.</param>
		public void AddToDictionary(PoolableObject comp)
		{
			if (comp == null || !(comp.SaveableObject is SaveableObject saveable) || saveable.GUIDComponent == null)
				return;

			uint guid = saveable.GUIDComponent.GUID;
			if (guid == 0)
			{
				_debugProcessor.LogWarning(DebugLogCategory.General, "Cannot register GUID 0.");
				return;
			}

			RegisterLoadedGUID(comp, guid);
		}

		/// <summary>
		/// Retrieves a poolable object by its GUID.
		/// Used during save/load operations to locate objects by their unique identifiers.
		/// </summary>
		/// <param name="gUID">The GUID to search for.</param>
		/// <param name="type">The pool type of the object.</param>
		/// <returns>The poolable object with the matching GUID, or null if not found.</returns>
		public PoolableObject GetComponentFromID(uint gUID, string type)
		{
			// Attempt to retrieve the object by GUID
			if (_guidRuntimeData.WorldObjects[type].TryGetValue(gUID, out PoolableObject comp))
				return comp;
			else
				return null;
		}

		public bool TryGetComponentFromID(uint guid, string type, out PoolableObject component)
		{
			component = null;
			return !string.IsNullOrWhiteSpace(type)
				&& _guidRuntimeData.WorldObjects.TryGetValue(type, out Dictionary<uint, PoolableObject> objects)
				&& objects.TryGetValue(guid, out component);
		}

		/// <summary>Clears identities from the previous scene before a saved world is restored.</summary>
		public void ResetWorldState()
		{
			foreach (Dictionary<uint, PoolableObject> objects in _guidRuntimeData.WorldObjects.Values)
				objects.Clear();
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
		/// Injects the GUIDRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_guidRuntimeData != null)
				throw new InvalidOperationException("GUIDProcessor runtime data has already been installed.");

			_guidRuntimeData = new GUIDRuntimeData();
			containerBuilder.AddSingleton(_guidRuntimeData);
		}

		/// <summary>
		/// Processes GUID logic every frame.
		/// Called every frame by the Coordinator.
		/// GUIDProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			// GUIDProcessor does not require per-frame updates
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// GUIDProcessor does not have scene-specific settings to refresh
		}

		/// <summary>
		/// Generates a new unique GUID for a specific pool type.
		/// Uses random number generation and collision detection to ensure uniqueness.
		/// </summary>
		/// <param name="type">The pool type to generate a GUID for.</param>
		/// <returns>A unique GUID that doesn't conflict with existing GUIDs.</returns>
		private uint GenerateNewGUID(string type)
		{
			uint gUID = 0;
			bool newGUIDFound = false;

			// Keep generating random GUIDs until we find one that's not already in use
			while (!newGUIDFound)
			{
				// Generate a random uint as a potential GUID
				gUID = (uint)UnityEngine.Random.Range(uint.MinValue, uint.MaxValue);

				// Check if this GUID is not already in use and is not zero
				if (!_guidRuntimeData.WorldObjects[type].ContainsKey(gUID) && gUID != 0)
					newGUIDFound = true;
			}
			return gUID;
		}

		/// <summary>
		/// Generates a new unique GUID for data-driven resources (non-GameObject).
		/// Uses random number generation and collision detection against existing GUIDs.
		/// </summary>
		/// <param name="existingGUIDs">Set of existing GUIDs to check for collisions.</param>
		/// <returns>A unique GUID that doesn't conflict with existing GUIDs.</returns>
		public uint GenerateResourceGUID(HashSet<uint> existingGUIDs)
		{
			uint gUID = 0;
			bool newGUIDFound = false;

			// Keep generating random GUIDs until we find one that's not already in use
			while (!newGUIDFound)
			{
				// Generate a random uint as a potential GUID
				gUID = (uint)UnityEngine.Random.Range(uint.MinValue, uint.MaxValue);

				// Check if this GUID is not already in use and is not zero
				if (!existingGUIDs.Contains(gUID) && gUID != 0)
					newGUIDFound = true;
			}
			return gUID;
		}
	}
}
