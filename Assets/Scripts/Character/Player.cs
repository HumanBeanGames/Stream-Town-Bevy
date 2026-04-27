using GUIDSystem;
using Pets;
using Pets.Enumerations;
using Sensors;
using System;
using System.Collections.Generic;
using Target;
using Twitch;
using Units;
using UnityEngine;
using UserInterface;
using Utils.Pooling;

namespace Character
{
	/// <summary>
	/// Holds all player data for a Twitch user.
	/// </summary>
	[System.Serializable]
	public class Player
	{
        /// <summary>
        /// The Twitch user associated with this player.
        /// </summary>
		private TwitchUser _user;

        /// <summary>
        /// The last position where the player placed a building.
        /// </summary>
		private Vector3 _lastBuildingPlacement = -Vector3.right * 5;

        /// <summary>
        /// Gets or sets the last building placement position.
        /// </summary>
		public Vector3 LastBuildingPlacement
		{
			get { return _lastBuildingPlacement; }
			set { _lastBuildingPlacement = value; }
		}

        /// <summary>
        /// Gets or sets the character GameObject.
        /// </summary>
		public GameObject Character { get; set; }

        /// <summary>
        /// Gets or sets the role handler.
        /// </summary>
		public RoleHandler RoleHandler { get; set; }

        /// <summary>
        /// Gets or sets the health handler.
        /// </summary>
		public HealthHandler HealthHandler { get; set; }

        /// <summary>
        /// Gets or sets the station sensor.
        /// </summary>
		public StationSensor StationSensor { get; set; }

        /// <summary>
        /// Gets or sets the target sensor.
        /// </summary>
		public TargetSensor TargetSensor { get; set; }

        /// <summary>
        /// Gets or sets the character model handler.
        /// </summary>
		public CharacterModelHandler EquipmentHandler { get; set; }

        /// <summary>
        /// Gets or sets the GUID component.
        /// </summary>
		public GUIDComponent GUIDComponent { get; set; }

        /// <summary>
        /// Gets or sets the targetable player component.
        /// </summary>
		public TargetablePlayer PlayerTarget { get; set; }

        /// <summary>
        /// Gets the Twitch user.
        /// </summary>
		public TwitchUser TwitchUser => _user;

        /// <summary>
        /// Gets or sets the dictionary of unlocked pet types.
        /// </summary>
		public Dictionary<PetType, bool> PetsUnlocked;

        /// <summary>
        /// Gets or sets the active pet.
        /// </summary>
		public Pet Pet { get; set; }

        /// <summary>
        /// Gets or sets the poolable object.
        /// </summary>
		public PoolableObject PoolableObject { get; set; }

        /// <summary>
        /// Gets or sets whether this player is an NPC.
        /// </summary>
		public bool IsNPC { get; set; }

        /// <summary>
        /// Gets or sets the total building rotation.
        /// </summary>
		public int TotalBuildingRotation { get; set; }

        /// <summary>
        /// Gets or sets the unit text display.
        /// </summary>
		public UnitTextDisplay UnitTextDisplay { get; set; }

		// Constructor.
        /// <summary>
        /// Initializes a new player with the specified Twitch user.
        /// </summary>
        /// <param name="user">The Twitch user.</param>
        /// <param name="IsNPC">Whether this player is an NPC.</param>
		public Player(TwitchUser user, bool IsNPC = false)
		{
			this.IsNPC = IsNPC;

			_user = user;

			PetsUnlocked = new Dictionary<PetType, bool>();

			for (int i = 0; i < (int)PetType.Count; i++)
				PetsUnlocked.Add((PetType)i, false);

			if (TwitchUser.GameUserType == Twitch.Utils.GameUserType.Subscriber || TwitchUser.GameUserType == Twitch.Utils.GameUserType.GameMaster)
				PetsUnlocked[PetType.RedPanda] = true;

			if (IsNPC)
				TwitchUser.ActivityStatus = Enumerations.ActivityStatus.Inactive;

			PetsUnlocked[PetType.None] = true;
		}

        /// <summary>
        /// Gets the list of unlocked pet types.
        /// </summary>
        /// <returns>List of unlocked pet types.</returns>
		public List<PetType> GetUnlockedPetTypes()
		{
			List<PetType> type = new List<PetType>();

			for (int i = 0; i < (int)PetType.Count; i++)
			{
				if (PetsUnlocked.TryGetValue((PetType)i, out bool yes))
					if (yes)
						type.Add((PetType)i);
			}
			return type;
		}

        /// <summary>
        /// Called when the character dies.
        /// </summary>
        /// <param name="attacked">Whether the character was attacked.</param>
        /// <param name="twitchChatProcessor">The Twitch chat processor to send notifications.</param>
		public void OnCharacterDied(bool attacked, Processors.TwitchChatProcessor twitchChatProcessor)
		{
			if (IsNPC || !attacked)
				return;

			twitchChatProcessor.SendPlayerMessage(this, "You died!");
		}

        /// <summary>
        /// Called when the character respawns.
        /// </summary>
        /// <param name="twitchChatProcessor">The Twitch chat processor to send notifications.</param>
		public void OnCharacterRespawned(Processors.TwitchChatProcessor twitchChatProcessor)
		{
			if (IsNPC)
				return;

			twitchChatProcessor.SendPlayerMessage(this, "You have revived!");
		}
	}
}
