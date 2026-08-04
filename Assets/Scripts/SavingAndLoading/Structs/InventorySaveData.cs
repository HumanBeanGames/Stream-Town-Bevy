using System;
using System.Collections.Generic;

namespace SavingAndLoading.Structs
{
	[Serializable]
	public struct InventoryEntrySaveData
	{
		public string ResourceType;
		public int Amount;
		public int MaxAmount;
		public bool IsUnlimited;
	}

	/// <summary>
	/// Raw inventory snapshot. Runtime ResourceInventory objects are reconstructed
	/// by SaveProcessor and are never serialized.
	/// </summary>
	[Serializable]
	public struct InventorySaveData
	{
		public List<InventoryEntrySaveData> Entries;
	}
}
