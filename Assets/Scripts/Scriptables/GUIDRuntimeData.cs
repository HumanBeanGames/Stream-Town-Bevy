using GUIDSystem;
using SavingAndLoading.SavableObjects;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using Utils.Pooling;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// Runtime data for GUIDProcessor.
	/// Manages the mapping of GUIDs to poolable objects for save/load functionality.
	/// </summary>
	public class GUIDRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Dictionary mapping scene names to GUID-to-object mappings.
		/// Outer dictionary key is scene name, inner dictionary maps GUID to PoolableObject.
		/// Used to track all objects by their GUIDs for save/load operations.
		/// </summary>
		[SerializeField]
		private Dictionary<string, Dictionary<uint, PoolableObject>> _worldObjects;

		/// <summary>
		/// Whether the GUID system has been initialized.
		/// Set to true after the GUID mapping has been populated.
		/// </summary>
		[SerializeField]
		private bool _initialized = false;

		/// <summary>
		/// Gets the dictionary mapping scenes to GUID-to-object mappings.
		/// </summary>
		public Dictionary<string, Dictionary<uint, PoolableObject>> WorldObjects => _worldObjects;

		/// <summary>
		/// Gets or sets whether the GUID system is initialized.
		/// </summary>
		public bool IsInitialized
		{
			get { return _initialized; }
			set { _initialized = value; }
		}

		/// <summary>
		/// Initializes the WorldObjects dictionary with the provided dictionary.
		/// </summary>
		/// <param name="worldObjects">The dictionary to set as WorldObjects.</param>
		public void InitializeWorldObjects(Dictionary<string, Dictionary<uint, PoolableObject>> worldObjects)
		{
			_worldObjects = worldObjects;
		}

		/// <summary>
		/// Initializes the GUID runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
