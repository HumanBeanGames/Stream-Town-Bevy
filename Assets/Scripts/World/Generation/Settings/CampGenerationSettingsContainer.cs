using System.Collections.Generic;
using UnityEngine;

namespace World.Generation.Settings
{
	/// <summary>
	/// MonoBehaviour wrapper for camp generation settings list.
	/// </summary>
	public class CampGenerationSettingsContainer : MonoBehaviour
	{
		[SerializeField]
		private List<CampGenerationSettings> _campGenerationSettings;

		public List<CampGenerationSettings> CampGenerationSettings => _campGenerationSettings;
	}
}
