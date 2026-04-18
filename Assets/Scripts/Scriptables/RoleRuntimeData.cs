using System;
using System.Collections.Generic;
using Character;
using UnityEngine;
using UnityEngine.Events;
using Utils;
using Data.Containers;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime role state for the game.
	/// Manages role slot availability and role data container.
	/// </summary>
	public class RoleRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Dictionary mapping player roles to their slot availability.
		/// Tracks how many slots are available for each role.
		/// </summary>
		private Dictionary<PlayerRole, RoleSlot> _roleSlotsDictionary;

		/// <summary>
		/// Event fired when role slots change.
		/// Passes the role whose slots have changed.
		/// </summary>
		private UnityEvent<PlayerRole> _onRoleSlotsChangedEvent = new UnityEvent<PlayerRole>();

		/// <summary>
		/// Event fired when a role slot is removed.
		/// Passes the role whose slot was removed.
		/// </summary>
		private UnityEvent<PlayerRole> _onSlotRemoved = new UnityEvent<PlayerRole>();

		/// <summary>
		/// Container for role data and configuration.
		/// Provides access to role settings and runtime data.
		/// </summary>
		private Data.Containers.RoleDataContainer _roleDataContainer;

		/// <summary>
		/// Whether player role limits are enforced.
		/// If true, limits the number of players per role; if false, unlimited.
		/// </summary>
		public bool PlayerRoleLimits = true;

		/// <summary>
		/// Gets the event fired when role slots change.
		/// </summary>
		public UnityEvent<PlayerRole> OnRoleSlotsChangedEvent => _onRoleSlotsChangedEvent;

		/// <summary>
		/// Gets the dictionary of role slot availability.
		/// </summary>
		public Dictionary<PlayerRole, RoleSlot> RoleSlotsDictionary => _roleSlotsDictionary;

		/// <summary>
		/// Gets the role data container.
		/// </summary>
		public Data.Containers.RoleDataContainer RoleDataContainer => _roleDataContainer;

		/// <summary>
		/// Gets the event fired when a role slot is removed.
		/// </summary>
		public UnityEvent<PlayerRole> OnSlotRemoved => _onSlotRemoved;

		/// <summary>
		/// Initializes the role slots dictionary.
		/// </summary>
		/// <param name="roleSlotsDictionary">The dictionary to set.</param>
		public void InitializeRoleSlots(Dictionary<PlayerRole, RoleSlot> roleSlotsDictionary)
		{
			_roleSlotsDictionary = roleSlotsDictionary;
		}

		/// <summary>
		/// Initializes the role data container.
		/// </summary>
		/// <param name="roleDataContainer">The container to set.</param>
		public void InitializeRoleData(Data.Containers.RoleDataContainer roleDataContainer)
		{
			_roleDataContainer = roleDataContainer;
		}
	}
}
