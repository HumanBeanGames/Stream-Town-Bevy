namespace SavingAndLoading.Structs
{
	[System.Serializable]
	public struct ObjectiveSaveData
	{
		public string ObjectiveType;
		public string ResourceType;
		public string BuildingType;
		public string EnemyType;
		public int RequiredAmount;
		public int Amount;

		public ObjectiveSaveData(int amount, int requiredAmount)
		{
			ObjectiveType = null;
			ResourceType = null;
			BuildingType = null;
			EnemyType = null;
			Amount = amount;
			RequiredAmount = requiredAmount;
		}
	}
}
