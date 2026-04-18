using Reflex.Core;

namespace Processors
{
	public interface IProcessor
	{
		void Initialize();
		void Process();
	}

	public interface IProcessor<TData> : IProcessor
	{
		TData Data { get; }
	}
}
