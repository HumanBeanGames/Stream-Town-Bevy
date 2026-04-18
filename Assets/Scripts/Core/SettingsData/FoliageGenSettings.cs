using System.Collections.Generic;
using UnityEngine;
using World.Generation;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores foliage generation settings for world generation.
	/// Contains a list of foliage generation configurations used during world creation.
	/// </summary>
	[CreateAssetMenu(fileName = "FoliageGenSettings", menuName = "Scriptables/World Generation/Foliage Generation Settings")]
	public class FoliageGenSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// List of foliage generation settings for different foliage types.
		/// Each setting defines how a specific type of foliage should be generated.
		/// </summary>
		[SerializeField]
		private List<FoliageGenerationSettings> _foliageGenerationSettings;

		/// <summary>
		/// Gets the list of foliage generation settings.
		/// </summary>
		public List<FoliageGenerationSettings> FoliageGenerationSettings => _foliageGenerationSettings;
	}
}
