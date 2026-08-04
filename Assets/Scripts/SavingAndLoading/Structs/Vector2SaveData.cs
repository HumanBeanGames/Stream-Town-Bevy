namespace SavingAndLoading.Structs 
{
    /// <summary>
    /// Raw two-component value used by save DTOs.
    /// </summary>
    [System.Serializable]
    public struct Vector2SaveData 
	{
        public float X;
        public float Y;

        /// <summary>
        /// Overloaded constructor,
        /// Creates a Vector2SaveData from 2 floats,
        /// </summary>
        /// <param name="x">The x component</param>
        /// <param name="y">The y component</param>
        public Vector2SaveData(float x, float y)
        {
            X = x;
            Y = y;
        }

    }
}
