using System;
using System.Collections.Generic;
using System.Linq;

namespace UserInterface.MainMenu
{
	/// <summary>
	/// Reports progress for parallel tasks with multiple concurrent progress tracks.
	/// Supports weighted overall progress calculation and individual track reporting.
	/// </summary>
	public static class ParallelProgressReporter
	{
		public static event Action<float, string> OnOverallProgressUpdated;
		public static event Action<Dictionary<string, (float progress, string status)>> OnTrackProgressUpdated;

		private static Dictionary<string, (float progress, string status)> _tracks = new Dictionary<string, (float, string)>();
		private static Dictionary<string, float> _trackWeights = new Dictionary<string, float>();

		/// <summary>
		/// Registers a progress track with a weight for overall progress calculation.
		/// </summary>
		/// <param name="trackName">Unique identifier for this track</param>
		/// <param name="weight">Weight contribution to overall progress (sum of all weights = 1.0 recommended)</param>
		public static void RegisterTrack(string trackName, float weight)
		{
			_trackWeights[trackName] = weight;
			_tracks[trackName] = (0f, "Waiting...");
		}

		/// <summary>
		/// Updates the progress and status of a specific track.
		/// </summary>
		public static void UpdateTrack(string trackName, float progress, string status)
		{
			if (_tracks.ContainsKey(trackName))
			{
				_tracks[trackName] = (progress, status);
				OnTrackProgressUpdated?.Invoke(new Dictionary<string, (float, string)>(_tracks));
				OnOverallProgressUpdated?.Invoke(CalculateOverallProgress(), GetCombinedStatus());
			}
		}

		/// <summary>
		/// Removes a track from tracking (e.g., when a task completes and is no longer relevant).
		/// </summary>
		public static void UnregisterTrack(string trackName)
		{
			_tracks.Remove(trackName);
			_trackWeights.Remove(trackName);
			OnTrackProgressUpdated?.Invoke(new Dictionary<string, (float, string)>(_tracks));
			OnOverallProgressUpdated?.Invoke(CalculateOverallProgress(), GetCombinedStatus());
		}

		/// <summary>
		/// Resets all tracks (call at start of loading sequence).
		/// </summary>
		public static void Reset()
		{
			_tracks.Clear();
			_trackWeights.Clear();
			OnTrackProgressUpdated?.Invoke(_tracks);
			OnOverallProgressUpdated?.Invoke(0f, "Ready");
		}

		/// <summary>
		/// Calculates weighted overall progress from all active tracks.
		/// </summary>
		private static float CalculateOverallProgress()
		{
			float totalWeight = _trackWeights.Values.Sum();
			if (totalWeight == 0) return 0f;
			return _tracks.Sum(kvp => kvp.Value.progress * _trackWeights[kvp.Key]) / totalWeight;
		}

		/// <summary>
		/// Gets combined status string from all active tracks.
		/// </summary>
		private static string GetCombinedStatus()
		{
			var activeTracks = _tracks.Where(kvp => kvp.Value.progress < 1f && kvp.Value.progress > 0f);
			if (!activeTracks.Any()) return "Complete";
			return string.Join(" | ", activeTracks.Select(kvp => $"{kvp.Key}: {kvp.Value.status}"));
		}
	}
}
