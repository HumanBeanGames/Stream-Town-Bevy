namespace SavingAndLoading.Structs 
{
    /// <summary>
    /// A struct holding information on the Enemy Camps
    /// </summary>
    [System.Serializable]
    public struct EnemyCampSaveData 
	{
        public TransformSaveData Transform;
        public int Health;
        public uint GUID;
    }
}
