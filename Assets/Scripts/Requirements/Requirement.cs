namespace Requirements 
{
	/// <summary>
	/// Holds data for any type of requirement for buildings or technology.
	/// </summary>
	[System.Serializable]
    public class Requirement 
	{
        /// <summary>
        /// The type of requirement.
        /// </summary>
		public RequirementType RequirementType;

        /// <summary>
        /// The requirement data.
        /// </summary>
		[System.NonSerialized] public object Data;
    }
}
