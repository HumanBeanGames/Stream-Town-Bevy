using System;
using System.Collections.Generic;
using Character;
using Pets.Enumerations;
using Twitch.Commands;
using Twitch.Utils;
using TwitchLib.Client.Enums;
using Utils;

namespace SavingAndLoading.Structs
{
	/// <summary>
	/// Raw player snapshot. Runtime Player, GameObject and component construction
	/// belongs to SaveProcessor, not this data type.
	/// </summary>
	[Serializable]
	public struct PlayerSaveData
	{
		public string TwitchID;
		public string TwitchName;
		public UserType TwitchUserType;
		public GameUserType GameUserType;
		public bool IsBroadcaster;
		public bool IsUserPlayer;
		public uint GUID;
		public uint TargetGUID;
		public string TargetPoolType;
		public uint StationGUID;
		public string StationPoolType;

		public bool PetActive;
		public PetType CurrentPet;
		public List<PetType> UnlockedPets;

		public TransformSaveData Transform;
		public PlayerRole CurrentRole;
		public PlayerRole PreviousRole;
		public List<PlayerRoleSaveData> Roles;
		public InventorySaveData Inventory;
		public PlayerCustomizationSaveData Customization;
		public int Health;
		public bool RegenRequiresFood;
	}
}
