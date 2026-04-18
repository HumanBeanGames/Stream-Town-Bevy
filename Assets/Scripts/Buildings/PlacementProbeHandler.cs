using UnityEngine;

namespace Buildings
{
	/// <summary>
	/// Used for checking if a building or structure can be placed at it's current location.
	/// </summary>
	public class PlacementProbeHandler : MonoBehaviour
	{
        /// <summary>
        /// Array of placement probes.
        /// </summary>
		private PlacementProbe[] _probes;

		/// <summary>
		/// Returns true if all probes are on their desired surface.
		/// </summary>
		/// <returns>True if all probes passed the check.</returns>
		public bool AllProbesPassedCheck()
		{
			// Loop through all probes and check if they are on their desired surfaces.
			for (int i = 0; i < _probes.Length; i++)
			{
				// If any probe is not on it's desired surface, early out and return false.
				if (!_probes[i].OnDesiredSurface)
					return false;
			}

			return true;
		}

		/// <summary>
		/// Populates the probes array.
		/// </summary>
		// This method is used to get all child placement probes and store them in the _probes array.
		private void GetAllProbes()
		{
			_probes = GetComponentsInChildren<PlacementProbe>();

			if (_probes == null || _probes.Length == 0)
				Debug.LogWarning($"No placement probes found.", this);
		}

		// Unity Functions.
        // Initializes the probe handler by getting all child placement probes.
		private void Awake()
		{
			GetAllProbes();
		}
	}
}
