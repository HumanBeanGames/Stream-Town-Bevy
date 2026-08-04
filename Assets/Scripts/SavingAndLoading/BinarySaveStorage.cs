using System;
using System.IO;
using System.IO.Compression;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace SavingAndLoading
{
	/// <summary>
	/// Versioned, compressed binary persistence for aggregate game snapshots.
	/// The storage remains stateless: it only validates and persists raw DTOs.
	/// Existing JSON saves remain readable and are migrated on their first load.
	/// </summary>
	public sealed class BinarySaveStorage : ISaveStorage
	{
		public const string SaveDirectoryName = JsonSaveStorage.SaveDirectoryName;
		public const string SaveFileName = "StreamTownSave.stsave";
		public const string BackupFileName = "StreamTownSave.backup.stsave";
		private const string TemporaryFileName = "StreamTownSave.tmp.stsave";
		private const int CurrentContainerVersion = 1;

		private static readonly byte[] Magic = { (byte)'S', (byte)'T', (byte)'S', (byte)'V' };
		private static readonly UTF8Encoding Utf8 = new UTF8Encoding(false, true);

		private readonly string _saveDirectory;
		private readonly JsonSaveStorage _legacyStorage;

		public BinarySaveStorage()
			: this(Path.Combine(
				System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments),
				SaveDirectoryName))
		{
		}

		public BinarySaveStorage(string saveDirectory)
		{
			if (string.IsNullOrWhiteSpace(saveDirectory))
				throw new ArgumentException("A save directory is required.", nameof(saveDirectory));

			_saveDirectory = Path.GetFullPath(saveDirectory);
			_legacyStorage = new JsonSaveStorage(_saveDirectory);
		}

		public string SavePath => Path.Combine(_saveDirectory, SaveFileName);
		private string BackupPath => Path.Combine(_saveDirectory, BackupFileName);
		private string TemporaryPath => Path.Combine(_saveDirectory, TemporaryFileName);

		public bool SaveExists =>
			File.Exists(SavePath) ||
			File.Exists(BackupPath) ||
			_legacyStorage.SaveExists;

		public Task WriteAsync(SaveFileData data, CancellationToken cancellationToken = default)
		{
			SaveFileValidation.Validate(data);
			return Task.Run(() => Write(data, cancellationToken), cancellationToken);
		}

		public async Task<SaveFileData> ReadAsync(CancellationToken cancellationToken = default)
		{
			Exception binaryException = null;
			if (File.Exists(SavePath) || File.Exists(BackupPath))
			{
				try
				{
					return await Task.Run(() => ReadBinary(cancellationToken), cancellationToken);
				}
				catch (Exception exception) when (
					exception is IOException ||
					exception is InvalidDataException ||
					exception is EndOfStreamException)
				{
					binaryException = exception;
				}
			}

			if (_legacyStorage.SaveExists)
			{
				SaveFileData legacyData = await _legacyStorage.ReadAsync(cancellationToken);
				// Keep the JSON files as an additional recovery path. The atomic binary
				// write makes migration safe even if the application closes midway.
				await WriteAsync(legacyData, cancellationToken);
				return legacyData;
			}

			if (binaryException != null)
				throw new InvalidDataException("The binary save and its backup are invalid.", binaryException);

			throw new FileNotFoundException("No Stream Town save file exists.", SavePath);
		}

		private void Write(SaveFileData data, CancellationToken cancellationToken)
		{
			cancellationToken.ThrowIfCancellationRequested();
			Directory.CreateDirectory(_saveDirectory);

			try
			{
				using (FileStream stream = new FileStream(
					TemporaryPath,
					FileMode.Create,
					FileAccess.Write,
					FileShare.None,
					64 * 1024,
					FileOptions.WriteThrough))
				{
					using (BinaryWriter header = new BinaryWriter(stream, Utf8, true))
					{
						header.Write(Magic);
						header.Write(CurrentContainerVersion);
						header.Flush();
					}

					using (GZipStream compressed = new GZipStream(
						stream,
						CompressionLevel.Fastest,
						true))
					using (BinaryWriter writer = new BinaryWriter(compressed, Utf8, true))
					{
						BinarySaveCodec.Write(writer, data, cancellationToken);
						writer.Flush();
					}

					stream.Flush(true);
				}

				cancellationToken.ThrowIfCancellationRequested();
				if (File.Exists(SavePath))
					File.Replace(TemporaryPath, SavePath, BackupPath, true);
				else
					File.Move(TemporaryPath, SavePath);
			}
			finally
			{
				if (File.Exists(TemporaryPath))
					File.Delete(TemporaryPath);
			}
		}

		private SaveFileData ReadBinary(CancellationToken cancellationToken)
		{
			cancellationToken.ThrowIfCancellationRequested();

			Exception primaryException = null;
			if (File.Exists(SavePath))
			{
				try
				{
					return ReadAndValidate(SavePath, cancellationToken);
				}
				catch (Exception exception) when (
					exception is IOException ||
					exception is InvalidDataException ||
					exception is EndOfStreamException)
				{
					primaryException = exception;
				}
			}

			if (File.Exists(BackupPath))
				return ReadAndValidate(BackupPath, cancellationToken);

			if (primaryException != null)
				throw new InvalidDataException("The binary save is invalid and no valid backup exists.", primaryException);

			throw new FileNotFoundException("No binary Stream Town save file exists.", SavePath);
		}

		private static SaveFileData ReadAndValidate(string path, CancellationToken cancellationToken)
		{
			using (FileStream stream = new FileStream(
				path,
				FileMode.Open,
				FileAccess.Read,
				FileShare.Read,
				64 * 1024,
				FileOptions.SequentialScan))
			using (BinaryReader header = new BinaryReader(stream, Utf8, true))
			{
				byte[] magic = header.ReadBytes(Magic.Length);
				if (magic.Length != Magic.Length)
					throw new InvalidDataException("The save header is incomplete.");

				for (int i = 0; i < Magic.Length; i++)
				{
					if (magic[i] != Magic[i])
						throw new InvalidDataException("The file is not a Stream Town binary save.");
				}

				int containerVersion = header.ReadInt32();
				if (containerVersion <= 0 || containerVersion > CurrentContainerVersion)
					throw new InvalidDataException($"Unsupported save container version {containerVersion}.");

				using (GZipStream compressed = new GZipStream(stream, CompressionMode.Decompress, true))
				using (BinaryReader reader = new BinaryReader(compressed, Utf8, true))
				{
					SaveFileData data = BinarySaveCodec.Read(reader, cancellationToken);
					if (compressed.ReadByte() != -1)
						throw new InvalidDataException("The save contains unexpected trailing payload data.");

					SaveFileValidation.Validate(data);
					return data;
				}
			}
		}
	}
}
