using System.Collections.Generic;
using UnityEngine;
using World.Generation;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject for water resource generation settings list.
	/// 
	/// Existing values to copy:
	/// - WaterResourceGenerationSettings: (List - copy from existing prefab)
	/// </summary>
	[CreateAssetMenu(fileName = "WaterResourceGenSettings", menuName = "Scriptables/World Generation/Water Resource Generation Settings")]
	public class WaterResourceGenSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private List<ResourceGenerationSettings> _waterResourceGenerationSettings;

		public List<ResourceGenerationSettings> WaterResourceGenerationSettings => _waterResourceGenerationSettings;
	}
}
