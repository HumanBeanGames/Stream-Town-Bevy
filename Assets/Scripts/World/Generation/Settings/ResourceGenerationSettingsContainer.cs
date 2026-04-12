using System.Collections.Generic;
using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for resource generation settings list.
	/// </summary>
	public class ResourceGenerationSettingsContainer : MonoBehaviour
	{
		[SerializeField]
		private List<ResourceGenerationSettings> _resourceGenerationSettings;

		public List<ResourceGenerationSettings> ResourceGenerationSettings => _resourceGenerationSettings;
	}
}
