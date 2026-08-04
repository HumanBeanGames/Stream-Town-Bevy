using System.IO;

namespace SavingAndLoading
{
	internal static class SaveFileValidation
	{
		public static void Validate(SaveFileData data)
		{
			if (data == null)
				throw new InvalidDataException("Save data is empty.");

			if (data.SchemaVersion <= 0 || data.SchemaVersion > SaveFileData.CurrentSchemaVersion)
				throw new InvalidDataException($"Unsupported save schema version {data.SchemaVersion}.");

			if (data.Game == null)
				throw new InvalidDataException("Save data does not contain a game snapshot.");

			if (data.Players == null)
				throw new InvalidDataException("Save data does not contain a player snapshot.");
		}
	}
}
