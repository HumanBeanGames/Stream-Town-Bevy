using System.Collections.Generic;
using UnityEngine;
using World.Generation;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject for resource generation settings list.
	/// 
	/// Existing values to copy:
	/// - ResourceGenerationSettings: (List - copy from existing prefab)
	/// </summary>
	[CreateAssetMenu(fileName = "ResourceGenSettings", menuName = "Scriptables/World Generation/Resource Generation Settings")]
	public class ResourceGenSettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private List<ResourceGenerationSettings> _resourceGenerationSettings;

		public List<ResourceGenerationSettings> ResourceGenerationSettings => _resourceGenerationSettings;
	}
}
