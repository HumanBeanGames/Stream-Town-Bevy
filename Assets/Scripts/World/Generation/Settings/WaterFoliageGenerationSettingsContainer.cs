using System.Collections.Generic;
using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for water foliage generation settings list.
	/// </summary>
	public class WaterFoliageGenerationSettingsContainer : MonoBehaviour
	{
		[SerializeField]
		private List<FoliageGenerationSettings> _waterFoliageGenerationSettings;

		public List<FoliageGenerationSettings> WaterFoliageGenerationSettings => _waterFoliageGenerationSettings;
	}
}
