using System.Collections.Generic;
using Utils;
using ScriptablesProcessorInfrastructure;

namespace Data.Containers
{
	/// <summary>
	/// Container for static role data including role definitions and experience table.
	/// Registered in ProjectScope - available immediately on scene load.
	/// </summary>
	public class RoleDataContainer
	{
        /// <summary>
        /// Dictionary mapping player roles to their scriptable data.
        /// </summary>
		private Dictionary<PlayerRole, RoleDataSettings> _roleDataDictionary;

        /// <summary>
        /// Array of experience values for each level.
        /// </summary>
		private int[] _expTableLookup;

        /// <summary>
        /// The all role data scriptable object.
        /// </summary>
		private AllRoleDataSettings _allRoleData;

        /// <summary>
        /// Gets the role data dictionary.
        /// </summary>
		public Dictionary<PlayerRole, RoleDataSettings> RoleDataDictionary => _roleDataDictionary;

        /// <summary>
        /// Gets the experience table lookup array.
        /// </summary>
		public int[] ExpTableLookup => _expTableLookup;

        /// <summary>
        /// Gets the all role data scriptable object.
        /// </summary>
		public AllRoleDataSettings AllRoleData => _allRoleData;

        /// <summary>
        /// Initializes a new role data container.
        /// </summary>
        /// <param name="allRoleData">The all role data scriptable object.</param>
		public RoleDataContainer(AllRoleDataSettings allRoleData)
		{
			_allRoleData = allRoleData;
			InitializeRoleData(allRoleData);
			CalculateEXPTable();
		}

        /// <summary>
        /// Initializes the role data dictionary from the all role data scriptable.
        /// </summary>
        /// <param name="allRoleData">The all role data scriptable object.</param>
		private void InitializeRoleData(AllRoleDataSettings allRoleData)
		{
			_roleDataDictionary = new Dictionary<PlayerRole, RoleDataSettings>();

			for (int i = 0; i < allRoleData.RoleData.Length; i++)
			{
				var role = (PlayerRole)i;
				if (_roleDataDictionary.ContainsKey(role))
				{
					UnityEngine.Debug.LogError($"Attempted to add the same role multiple times {role}.");
					continue;
				}
				_roleDataDictionary.Add(role, allRoleData.RoleData[i]);
			}
		}

        /// <summary>
        /// Calculates the experience table for each level.
        /// </summary>
		private void CalculateEXPTable()
		{
			const int MAX_ROLE_LEVEL = 99;
			const int MAX_LEVEL_EXP = 100000;
			_expTableLookup = new int[MAX_ROLE_LEVEL];

			for (int i = 0; i < MAX_ROLE_LEVEL; i++)
			{
				float t = ((float)i + 2) / 100;
				float pow = (t * t);
				float sqrt = 1 - UnityEngine.Mathf.Sqrt(1 - pow);
				_expTableLookup[i] = (int)(sqrt * MAX_LEVEL_EXP);
			}
		}

        /// <summary>
        /// Gets the required experience for a given level.
        /// </summary>
        /// <param name="level">The level to get required experience for.</param>
        /// <returns>The required experience for the level.</returns>
		public int GetRequiredExperience(int level)
		{
			if (level < 1 || level >= _expTableLookup.Length)
				return _expTableLookup[_expTableLookup.Length - 1];
			return _expTableLookup[level - 1];
		}
	}
}
