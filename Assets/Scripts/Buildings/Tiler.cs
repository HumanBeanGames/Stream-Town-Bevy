
using UnityEngine;

namespace Buildings
{
	/// <summary>
	/// A Generic Tiler class that is used for any objects that need to be tiled.
	/// </summary>
	public class Tiler : MonoBehaviour
	{
		/// <summary>
		/// The unit size of the object in the world.
		/// </summary>
		[SerializeField]
		protected int _size = 2;

		/// <summary>
		/// Comparison tag used to tile the same objects.
		/// </summary>
		[SerializeField]
		protected string _tag;

        /// <summary>
        /// The current tile value.
        /// </summary>
		protected int _tileValue = -1;

        /// <summary>
        /// Gets the current tile value.
        /// </summary>
		public int TileValue => _tileValue;

		/// <summary>
		/// Updates the value of this tile based on it's neighbours.
		/// </summary>
		/// <param name="currentValue">The current tile value (unused in base implementation).</param>
		/// <param name="enqueueNeighbours">Whether to enqueue neighbouring tiles for update.</param>
		public virtual void UpdateTileValue(int currentValue, bool enqueueNeighbours = false)
		{
			_tileValue = TileHelper.CalculateTileValue(transform.position, _tag, _size, enqueueNeighbours);

			OnTileValueChanged();
		}

        // Initializes the tiler.
		protected virtual void Init() { }

        // Called when the tile value changes.
		protected virtual void OnTileValueChanged() { }

		// Unity Events.
        // Initializes the tiler on Awake.
		private void Awake()
		{
			// Call the Init method to initialize the tiler.
			Init();
		}

        // Updates the tile value when the object is enabled.
		private void OnEnable()
		{
			// Reset the tile value to -1.
			_tileValue = -1;
			// Update the tile value and enqueue neighbours.
			UpdateTileValue(_tileValue, true);
		}
	}
}
