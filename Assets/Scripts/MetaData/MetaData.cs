using UnityEngine;

namespace MetaData
{
    /// <summary>
    /// Enum representing the type of load operation.
    /// </summary>
	public enum LoadType
	{
        /// <summary>
        /// Generate a new world.
        /// </summary>
		Generate,

        /// <summary>
        /// Load an existing world.
        /// </summary>
		Load,
	}

    /// <summary>
    /// Holds metadata about the game state and load type.
    /// </summary>
	public class MetaData : MonoBehaviour
	{
        /// <summary>
        /// The type of load operation.
        /// </summary>
		public LoadType LoadType;

        /// <summary>
        /// Initializes the metadata and marks it to not be destroyed on load.
        /// </summary>
		private void Awake()
		{
			DontDestroyOnLoad(this);
		}
	}
}
