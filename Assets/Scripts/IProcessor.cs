using Reflex.Core;
using System;
using System.Threading;
using System.Threading.Tasks;

namespace Processors
{
	public interface IProcessor
	{
		void Initialize();
		void Process();
		void RefreshSceneData(Container sceneContainer);
	}

	public interface IAsyncInitializableProcessor
	{
		Task InitializeAsync(ProcessorStartupContext startupContext, CancellationToken cancellationToken);
	}

	public interface IMainThreadInitializableProcessor
	{
	}

	public interface IPostInitializeProcessor
	{
		void Activate();
	}

	public enum ProcessorStartupStage
	{
		NotStarted,
		Initializing,
		Initialized,
		Activating,
		Activated,
		Failed
	}

	public sealed class ProcessorStartupReport
	{
		public ProcessorStartupReport(string processorName)
		{
			ProcessorName = processorName;
			Stage = ProcessorStartupStage.NotStarted;
			Progress = 0f;
			Status = "Waiting...";
		}

		public string ProcessorName { get; }

		public ProcessorStartupStage Stage { get; set; }

		public float Progress { get; set; }

		public string Status { get; set; }

		public Exception Exception { get; set; }
	}

	public sealed class ProcessorStartupContext
	{
		private readonly Action<float, string> _reportProgress;

		public ProcessorStartupContext(string processorName, Action<float, string> reportProgress)
		{
			ProcessorName = processorName;
			_reportProgress = reportProgress;
		}

		public string ProcessorName { get; }

		public void Report(float progress, string status)
		{
			_reportProgress?.Invoke(progress, status);
		}
	}

	public interface IProcessor<TData> : IProcessor
	{
		TData Data { get; }
	}
}
