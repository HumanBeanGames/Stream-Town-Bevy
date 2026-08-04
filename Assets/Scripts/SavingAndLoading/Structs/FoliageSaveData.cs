namespace SavingAndLoading.Structs
{
    /// <summary>
    /// Struct Holding information needed to load foliage from a save file
    /// </summary>
    
    [System.Serializable]
    public struct FoliageSaveData 
	{
        public TransformSaveData FoliageTransform;
        public string FoliageType;
    }
}
