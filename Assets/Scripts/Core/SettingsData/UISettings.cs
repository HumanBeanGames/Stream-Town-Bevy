using System;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "UISettings", menuName = "Scriptables/UI Settings")]
	public class UISettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private float _seaonSliderStartOffset;
		public float SeaonSliderStartOffset => _seaonSliderStartOffset;
	}
}
