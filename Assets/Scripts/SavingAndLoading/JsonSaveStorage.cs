using System;
using System.IO;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace SavingAndLoading
{
	/// <summary>
	/// Stateless JSON implementation of the save persistence boundary.
	/// Writes through a temporary file and retains the previous valid save as a
	/// backup so a failed write cannot leave a half-written save.
	/// </summary>
	public sealed class JsonSaveStorage : ISaveStorage
	{
		public const string SaveDirectoryName = "Panda Belly/Stream Town/Saves";
		public const string SaveFileName = "StreamTownSave.json";
		public const string BackupFileName = "StreamTownSave.backup.json";
		private const string TemporaryFileName = "StreamTownSave.tmp";

		private static readonly JsonSerializerSettings SerializerSettings = new JsonSerializerSettings
		{
			Formatting = Formatting.Indented,
			MissingMemberHandling = MissingMemberHandling.Ignore,
			NullValueHandling = NullValueHandling.Include,
			TypeNameHandling = TypeNameHandling.None,
			Converters = { new StringEnumConverter() }
		};

		private readonly string _saveDirectory;

		public JsonSaveStorage()
			: this(Path.Combine(
				System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments),
				SaveDirectoryName))
		{
		}

		public JsonSaveStorage(string saveDirectory)
		{
			if (string.IsNullOrWhiteSpace(saveDirectory))
				throw new ArgumentException("A save directory is required.", nameof(saveDirectory));

			_saveDirectory = Path.GetFullPath(saveDirectory);
		}

		public string SavePath => Path.Combine(_saveDirectory, SaveFileName);
		private string BackupPath => Path.Combine(_saveDirectory, BackupFileName);
		private string TemporaryPath => Path.Combine(_saveDirectory, TemporaryFileName);

		public bool SaveExists => File.Exists(SavePath) || File.Exists(BackupPath);

		public Task WriteAsync(SaveFileData data, CancellationToken cancellationToken = default)
		{
			SaveFileValidation.Validate(data);
			return Task.Run(() => Write(data, cancellationToken), cancellationToken);
		}

		public Task<SaveFileData> ReadAsync(CancellationToken cancellationToken = default)
		{
			return Task.Run(() => Read(cancellationToken), cancellationToken);
		}

		private void Write(SaveFileData data, CancellationToken cancellationToken)
		{
			cancellationToken.ThrowIfCancellationRequested();
			Directory.CreateDirectory(_saveDirectory);

			string json = JsonConvert.SerializeObject(data, SerializerSettings);
			byte[] bytes = new UTF8Encoding(false).GetBytes(json);

			using (FileStream stream = new FileStream(
				TemporaryPath,
				FileMode.Create,
				FileAccess.Write,
				FileShare.None,
				4096,
				FileOptions.WriteThrough))
			{
				stream.Write(bytes, 0, bytes.Length);
				stream.Flush(true);
			}

			cancellationToken.ThrowIfCancellationRequested();
			if (File.Exists(SavePath))
			{
				File.Replace(TemporaryPath, SavePath, BackupPath, true);
			}
			else
			{
				File.Move(TemporaryPath, SavePath);
			}
		}

		private SaveFileData Read(CancellationToken cancellationToken)
		{
			cancellationToken.ThrowIfCancellationRequested();

			Exception primaryException = null;
			if (File.Exists(SavePath))
			{
				try
				{
					return ReadAndValidate(SavePath);
				}
				catch (Exception exception) when (exception is IOException || exception is JsonException || exception is InvalidDataException)
				{
					primaryException = exception;
				}
			}

			if (File.Exists(BackupPath))
				return ReadAndValidate(BackupPath);

			if (primaryException != null)
				throw new InvalidDataException("The save file is invalid and no valid backup exists.", primaryException);

			throw new FileNotFoundException("No Stream Town save file exists.", SavePath);
		}

		private static SaveFileData ReadAndValidate(string path)
		{
			string json = File.ReadAllText(path, Encoding.UTF8);
			SaveFileData data = JsonConvert.DeserializeObject<SaveFileData>(json, SerializerSettings);
			SaveFileValidation.Validate(data);
			return data;
		}
	}
}
