namespace SavingAndLoading.Structs
{
	[System.Serializable]
	public struct EnemySaveData
	{
		public TransformSaveData Transform;
		public string EnemyType;
		public int Health;
		public uint GUID;
		public uint TargetGUID;
		public string TargetPoolType;
		public uint CampGUID;
		public string CampPoolType;

	}
}
