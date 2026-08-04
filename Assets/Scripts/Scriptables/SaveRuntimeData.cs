using System;
using ScriptablesProcessorInfrastructure;

namespace Processors
{
	public enum SaveOperationState
	{
		Idle,
		Saving,
		Loading,
		Succeeded,
		Failed
	}

	/// <summary>
	/// Mutable runtime state for SaveProcessor. It contains no persistence or
	/// world-mutation logic.
	/// </summary>
	public sealed class SaveRuntimeData : IRuntimeDataScriptable
	{
		public bool Autosave { get; set; }
		public float AutosaveTime { get; set; }
		public float TimeElapsed { get; set; }
		public SaveOperationState OperationState { get; private set; }
		public float Progress { get; private set; }
		public string Status { get; private set; }
		public string LastError { get; private set; }
		public bool IsBusy => OperationState == SaveOperationState.Saving || OperationState == SaveOperationState.Loading;

		public event Action<SaveOperationState, float, string> OperationChanged;

		public SaveRuntimeData()
		{
			OperationState = SaveOperationState.Idle;
			Status = "Ready";
		}

		public void Begin(SaveOperationState state, string status)
		{
			OperationState = state;
			Progress = 0f;
			Status = status;
			LastError = null;
			OperationChanged?.Invoke(OperationState, Progress, Status);
		}

		public void Report(float progress, string status)
		{
			Progress = Math.Max(0f, Math.Min(1f, progress));
			Status = status;
			OperationChanged?.Invoke(OperationState, Progress, Status);
		}

		public void Complete(string status)
		{
			OperationState = SaveOperationState.Succeeded;
			Progress = 1f;
			Status = status;
			OperationChanged?.Invoke(OperationState, Progress, Status);
		}

		public void Fail(string error)
		{
			OperationState = SaveOperationState.Failed;
			LastError = error;
			Status = error;
			OperationChanged?.Invoke(OperationState, Progress, Status);
		}
	}
}
