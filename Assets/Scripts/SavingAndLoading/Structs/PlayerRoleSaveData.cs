using System;
using Utils;

namespace SavingAndLoading.Structs
{
	/// <summary>
	/// Raw progression snapshot for one role.
	/// </summary>
	[Serializable]
	public struct PlayerRoleSaveData
	{
		public PlayerRole Role;
		public int Level;
		public int Experience;
	}
}
