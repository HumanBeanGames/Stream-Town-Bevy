using UnityEngine;
using Utils;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores all role data configurations for the game.
	/// Acts as a central registry for player role data.
	/// </summary>
	[CreateAssetMenu(fileName = "AllRoleDataSettings", menuName = "ScriptableObjects/AllRoleDataSettings", order = 2)]
	public class AllRoleDataSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Array of all role data configurations indexed by player role type.
		/// </summary>
		public RoleDataSettings[] RoleData;
	}
}
