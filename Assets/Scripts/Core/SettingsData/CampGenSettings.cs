using System.Collections.Generic;
using UnityEngine;
using World.Generation;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores camp generation settings for world generation.
	/// Contains a list of camp generation configurations used during world creation.
	/// </summary>
	[CreateAssetMenu(fileName = "CampGenSettings", menuName = "Scriptables/World Generation/Camp Generation Settings")]
	public class CampGenSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// List of camp generation settings for different camp types.
		/// Each setting defines how a specific type of camp should be generated.
		/// </summary>
		[SerializeField]
		private List<CampGenerationSettings> _campGenerationSettings;

		/// <summary>
		/// Gets the list of camp generation settings.
		/// </summary>
		public List<CampGenerationSettings> CampGenerationSettings => _campGenerationSettings;
	}
}
