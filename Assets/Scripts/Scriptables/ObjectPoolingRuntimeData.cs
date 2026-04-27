using System;

using ScriptablesProcessorInfrastructure;
using System.Collections.Generic;
using Utils.Pooling;
using Utils;
using UnityEngine;

namespace Processors
{
	/// <summary>
	/// Runtime data for ObjectPoolingProcessor.
	/// Manages pooled object queues, all object lists, and pool parent GameObjects.
	/// </summary>
	public class ObjectPoolingRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Dictionary mapping object types to their pooled object queues.
		/// Each queue contains inactive objects ready for reuse.
		/// </summary>
		private Dictionary<string, Queue<PoolableObject>> _pooledObjects;

		/// <summary>
		/// Dictionary mapping object types to lists of all objects of that type.
		/// Includes both active and inactive objects.
		/// </summary>
		private Dictionary<string, List<PoolableObject>> _allObjects;

		/// <summary>
		/// Dictionary mapping object types to their parent GameObjects.
		/// Used to organize pooled objects in the scene hierarchy.
		/// </summary>
		private Dictionary<string, GameObject> _poolParents;

		/// <summary>
		/// Time taken to complete the object pooling initialization.
		/// Used for performance monitoring.
		/// </summary>
		private TimeSpan _poolingDuration;

		/// <summary>
		/// Time budget per frame for object pooling operations.
		/// Ensures pooling doesn't cause frame rate drops by limiting work per frame.
		/// </summary>
		private const float _frameBudgetSeconds = 0.01f;

		/// <summary>
		/// Gets the time taken to complete object pooling initialization.
		/// </summary>
		public TimeSpan PoolingDuration => _poolingDuration;

		/// <summary>
		/// Gets the dictionary of all objects by type.
		/// </summary>
		public Dictionary<string, List<PoolableObject>> AllObjects => _allObjects;

		/// <summary>
		/// Gets the dictionary of pooled object queues by type.
		/// </summary>
		public Dictionary<string, Queue<PoolableObject>> PooledObjects => _pooledObjects;

		/// <summary>
		/// Gets the dictionary of pool parent GameObjects by type.
		/// </summary>
		public Dictionary<string, GameObject> PoolParents => _poolParents;

		/// <summary>
		/// Initializes the object pooling runtime data with default values.
		/// </summary>
		public ObjectPoolingRuntimeData()
		{
			_pooledObjects = new Dictionary<string, Queue<PoolableObject>>();
			_allObjects = new Dictionary<string, List<PoolableObject>>();
			_poolParents = new Dictionary<string, GameObject>();
			_poolingDuration = TimeSpan.Zero;
		}

		/// <summary>
		/// Initializes the pooling state with the provided values.
		/// </summary>
		public void InitializePooling(Dictionary<string, Queue<PoolableObject>> pooledObjects, Dictionary<string, List<PoolableObject>> allObjects, Dictionary<string, GameObject> poolParents, TimeSpan poolingDuration)
		{
			_pooledObjects = pooledObjects;
			_allObjects = allObjects;
			_poolParents = poolParents;
			_poolingDuration = poolingDuration;
		}
	}
}
