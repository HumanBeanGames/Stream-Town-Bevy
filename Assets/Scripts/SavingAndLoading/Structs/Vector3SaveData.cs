namespace SavingAndLoading.Structs 
{
    /// <summary>
    /// Raw three-component value used by save DTOs.
    /// </summary>
    [System.Serializable]
    public struct Vector3SaveData
    {
        public float X;
        public float Y;
        public float Z;

        /// <summary>
        /// Overloaded constructor,
        /// Creates a Vector3SaveData from 3 floats,
        /// </summary>
        /// <param name="x"></param>
        /// <param name="y"></param>
        /// <param name="z"></param>
        public Vector3SaveData(float x, float y, float z)
        {
            X = x;
            Y = y;
            Z = z;
        }

    }
}
