using System.Collections.Generic;
using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for foliage generation settings list.
	/// </summary>
	public class FoliageGenerationSettingsContainer : MonoBehaviour
	{
		[SerializeField]
		private List<FoliageGenerationSettings> _foliageGenerationSettings;

		public List<FoliageGenerationSettings> FoliageGenerationSettings => _foliageGenerationSettings;
	}
}
