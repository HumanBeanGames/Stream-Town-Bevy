namespace SavingAndLoading.Structs 
{
    /// <summary>
    /// A struct to hold transform information
    /// </summary>
    [System.Serializable]
    public struct TransformSaveData 
	{
        public Vector3SaveData Position;
        public Vector3SaveData Rotation;
        public Vector3SaveData LossyScale;

    }
}
