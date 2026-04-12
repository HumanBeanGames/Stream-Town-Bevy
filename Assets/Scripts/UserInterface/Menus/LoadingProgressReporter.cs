using System;
using UnityEngine;

namespace UserInterface.MainMenu
{
	public static class LoadingProgressReporter
	{
		public static event Action<float, string> OnProgressUpdated;

		private static float _progress;
		private static string _status = string.Empty;
		private static bool _active;

		public static float Progress => _progress;
		public static string Status => _status;
		public static bool IsActive => _active;

		public static void Begin(string initialStatus)
		{
			_active = true;
			Report(0f, initialStatus);
		}

		public static void Report(float progress01, string status)
		{
			_progress = Mathf.Clamp01(progress01);
			_status = status ?? string.Empty;
			OnProgressUpdated?.Invoke(_progress, _status);
		}

		public static void End(string finalStatus = "Done")
		{
			Report(1f, finalStatus);
			_active = false;
		}

		public static void Reset()
		{
			_active = false;
			_progress = 0f;
			_status = string.Empty;
			OnProgressUpdated?.Invoke(_progress, _status);
		}
	}
}
