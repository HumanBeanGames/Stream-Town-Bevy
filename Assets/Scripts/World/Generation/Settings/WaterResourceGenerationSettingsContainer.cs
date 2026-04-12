using System.Collections.Generic;
using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for water resource generation settings list.
	/// </summary>
	public class WaterResourceGenerationSettingsContainer : MonoBehaviour
	{
		[SerializeField]
		private List<ResourceGenerationSettings> _waterResourceGenerationSettings;

		public List<ResourceGenerationSettings> WaterResourceGenerationSettings => _waterResourceGenerationSettings;
	}
}
