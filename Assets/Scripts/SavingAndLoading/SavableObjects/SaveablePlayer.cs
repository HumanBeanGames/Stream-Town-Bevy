using Character;
using GUIDSystem;
using Processors;
using Pets.Enumerations;
using Reflex.Attributes;
using SavingAndLoading.Structs;
using System.Collections.Generic;
using Target;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects
{
    /// <summary>
    /// Handles saving and loading for players.
    /// </summary>
	public class SaveablePlayer : SaveableObject
	{
        /// <summary>
        /// The role handler.
        /// </summary>
		public RoleHandler RoleHandler;

        /// <summary>
        /// The GUID processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GUIDProcessor _guidProcessor;

        /// <summary>
        /// Loads the player data.
        /// </summary>
        /// <param name="data">The player save data.</param>
		public override void LoadData(object data)
		{
			// Done in SaveProcessor
		}

        /// <summary>
        /// Saves the player data.
        /// </summary>
        /// <returns>The player save data.</returns>
		public override object SaveData()
		{
			List<PlayerRoleSaveData> roleData = PlayerRoleSaveData.ToPlayerRoleSaveDatas(RoleHandler.Player.RoleHandler.PlayerRolesData);
			PetType currentPet = RoleHandler.Player.Pet.ActivePetType;
			List<PetType> unlockedPets = RoleHandler.Player.GetUnlockedPetTypes();
			return (object)new PlayerSaveData(RoleHandler.Player.TwitchUser.UserID
				, RoleHandler.Player.TwitchUser.Username
				, RoleHandler.Player.TwitchUser.TwitchUserType
				, RoleHandler.Player.TwitchUser.GameUserType
				, RoleHandler.Player.TwitchUser.IsBroadcaster
				, _guidProcessor.CreateGUIDandAddToDictionary(PoolableObject)
				, RoleHandler.Player.Pet.IsActive
				, currentPet
				, unlockedPets
				, new TransformSaveData(RoleHandler.Player.Character.transform)
				, RoleHandler.Player.RoleHandler.CurrentRole
				, RoleHandler.Player.RoleHandler.PreviousRole
				, roleData, new InventorySaveData(RoleHandler.Player.RoleHandler.Inventory.Resources)
				, RoleHandler.Player.EquipmentHandler.ToSaveData()
				, RoleHandler.Player.HealthHandler.Health
				, RoleHandler.Player.HealthHandler.RegenRequiresFood) ;
		}

        /// <summary>
        /// Sets the player variables.
        /// </summary>
        /// <param name="target">The targetable object.</param>
        /// <param name="component">The GUID component.</param>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
        /// <param name="roleHandler">The role handler.</param>
		public void SetVariables(Targetable target, GUIDComponent component, string poolName, PoolableObject poolableObject, RoleHandler roleHandler )
		{
			RoleHandler = roleHandler;
			base.SetVariables(target, component, poolName, poolableObject);
		}
	}
}
