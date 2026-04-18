using System.Collections.Generic;
using UnityEngine;
using Utils.Pooling;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores object pooling system settings for the game.
	/// Contains the list of objects to pool and debug configuration.
	/// </summary>
	[CreateAssetMenu(fileName = "ObjectPoolingSettings", menuName = "Scriptables/Object Pooling Settings")]
	public class ObjectPoolingSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// List of objects to be pooled on game initialization.
		/// Each entry defines the prefab, initial pool size, and expansion settings.
		/// </summary>
		[SerializeField]
		private List<PooledObjectData> _objectsToPool;

		/// <summary>
		/// Whether to enable debug logging for object pooling operations.
		/// If true, pooling operations are logged to console for debugging.
		/// </summary>
		[SerializeField]
		private bool _debugPooling = false;

		/// <summary>
		/// Gets the list of objects to pool.
		/// </summary>
		public List<PooledObjectData> ObjectsToPool => _objectsToPool;

		/// <summary>
		/// Gets whether debug logging is enabled for pooling.
		/// </summary>
		public bool DebugPooling => _debugPooling;
	}
}
