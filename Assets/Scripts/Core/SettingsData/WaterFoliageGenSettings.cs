using System.Collections.Generic;
using UnityEngine;
using World.Generation;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject for water foliage generation settings list.
	/// 
	/// Existing values to copy:
	/// - WaterFoliageGenerationSettings: (List - copy from existing prefab)
	/// </summary>
	[CreateAssetMenu(fileName = "WaterFoliageGenSettings", menuName = "Scriptables/World Generation/Water Foliage Generation Settings")]
	public class WaterFoliageGenSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private List<FoliageGenerationSettings> _waterFoliageGenerationSettings;

		public List<FoliageGenerationSettings> WaterFoliageGenerationSettings => _waterFoliageGenerationSettings;
	}
}
