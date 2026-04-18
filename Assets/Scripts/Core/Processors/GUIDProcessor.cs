using SavingAndLoading.SavableObjects;
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
		/// Runtime data ScriptableObject for GUID data.
		/// Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private GUIDRuntimeData _guidRuntimeData;

		/// <summary>

		public void Initialize()
		{
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
				Debug.Log("GUIDProcessor: Is already initialized");

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
			// Generate a new unique GUID for this object type
			uint gUID = GenerateNewGUID(comp.PoolType.ToString());
			
			// Assign the GUID to the object's GUID component
			((SaveableObject)comp.SaveableObject).GUIDComponent.SetGUID(gUID);
			
			// Add the object to the dictionary if the type exists and GUID is not already used
			if (_guidRuntimeData.WorldObjects.ContainsKey((comp.PoolType).ToString()) && !_guidRuntimeData.WorldObjects[comp.PoolType.ToString()].ContainsKey(gUID))
				_guidRuntimeData.WorldObjects[comp.PoolType.ToString()].Add(((SaveableObject)comp.SaveableObject).GUIDComponent.GUID, comp);

			return gUID;
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
			// Get all existing GUIDs for this pool type (not used but kept for potential validation)
			List<uint> keys = _guidRuntimeData.WorldObjects[comp.PoolType.ToString()].Keys.ToList();
			
			// Add the object if its GUID is not already in the dictionary
			if (!_guidRuntimeData.WorldObjects[comp.PoolType.ToString()].ContainsKey(((SaveableObject)comp.SaveableObject).GUIDComponent.GUID))
				_guidRuntimeData.WorldObjects[comp.PoolType.ToString()].Add(((SaveableObject)comp.SaveableObject).GUIDComponent.GUID, comp);
			// Log warning if GUID is zero (uninitialized)
			else if (((SaveableObject)comp.SaveableObject).GUIDComponent.GUID == 0)
				Debug.Log("GUID == 0");
			// Log warning if GUID is already in use (duplicate)
			else
				Debug.Log("Duplicate GUID detected");
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
			GUIDRuntimeData guidRuntimeData = ScriptableObject.CreateInstance<GUIDRuntimeData>();
			containerBuilder.AddSingleton(guidRuntimeData);
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
				gUID = (uint)Random.Range(uint.MinValue, uint.MaxValue);

				// Check if this GUID is not already in use and is not zero
				if (!_guidRuntimeData.WorldObjects[type].ContainsKey(gUID) && gUID != 0)
					newGUIDFound = true;
			}
			return gUID;
		}
	}
}
