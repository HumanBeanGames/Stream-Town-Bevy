using DataStructures;
using System.Collections.Generic;

namespace TechTree.Utilities
{
    /// <summary>
    /// Utility class for collection operations.
    /// </summary>
	public static class CollectionUtility
	{
        /// <summary>
        /// Adds an item to a list in a serializable dictionary.
        /// </summary>
        /// <typeparam name="K">The key type.</typeparam>
        /// <typeparam name="V">The value type.</typeparam>
        /// <param name="serializableDictionary">The serializable dictionary.</param>
        /// <param name="key">The key.</param>
        /// <param name="Value">The value to add.</param>
		public static void AddItem<K, V>(this SerializableDictionary<K, List<V>> serializableDictionary, K key, V Value)
		{
			if (serializableDictionary.ContainsKey(key))
			{
				serializableDictionary[key].Add(Value);

				return;
			}

			serializableDictionary.Add(key, new List<V>() { Value });
		}
	}
}
