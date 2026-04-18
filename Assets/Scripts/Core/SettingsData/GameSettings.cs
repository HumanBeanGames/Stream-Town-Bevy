using System.Collections.Generic;
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores game-wide settings.
	/// Contains configuration for game masters and other game-wide settings.
	/// </summary>
	[CreateAssetMenu(fileName = "GameSettings", menuName = "Scriptables/Game Settings")]
	public class GameSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// List of game master user IDs.
		/// Users in this list have GameMaster privileges.
		/// </summary>
		[SerializeField]
		private List<string> _gmIDs = new List<string>();

		/// <summary>
		/// Gets the list of game master user IDs.
		/// </summary>
		public List<string> GM_IDs => _gmIDs;
	}
}
