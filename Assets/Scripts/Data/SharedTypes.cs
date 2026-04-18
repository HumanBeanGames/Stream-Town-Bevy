namespace Data
{
    /// <summary>
    /// Contains shared type definitions used across the application.
    /// </summary>
	public static class SharedTypes
	{
        /// <summary>
        /// Represents input button types.
        /// </summary>
		public enum InputButton
		{
            /// <summary>No input button.</summary>
			None,
            /// <summary>Left mouse button.</summary>
			LeftMouse,
            /// <summary>Right mouse button.</summary>
			RightMouse,
            /// <summary>Middle mouse button.</summary>
			MiddleMouse,
            /// <summary>Total count of input buttons.</summary>
			Count
		}
	}
}
