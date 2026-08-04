using System.Threading;
using System.Threading.Tasks;

namespace SavingAndLoading
{
	/// <summary>
	/// Persistence boundary for save files. Implementations only read and write
	/// raw save data; they never inspect or mutate the game world.
	/// </summary>
	public interface ISaveStorage
	{
		string SavePath { get; }
		bool SaveExists { get; }
		Task WriteAsync(SaveFileData data, CancellationToken cancellationToken = default);
		Task<SaveFileData> ReadAsync(CancellationToken cancellationToken = default);
	}
}
