using Utils;

namespace Target
{
	/// <summary>
	/// Holds Targetable Object data.
	/// </summary>
	[System.Serializable]
	public class TargetableData
	{
        /// <summary>
        /// The target mask type.
        /// </summary>
		public TargetMask TargetType;

        /// <summary>
        /// The station update type.
        /// </summary>
		public StationUpdate UpdateType;
	}
}
