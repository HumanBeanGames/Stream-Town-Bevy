using System;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "UISettings", menuName = "Scriptables/UI Settings")]
	public class UISettings : ScriptableObject, IDataScriptable
	{
		[SerializeField]
		private TextMeshProUGUI _woodDisplayText;
		[SerializeField]
		private TextMeshProUGUI _foodDisplayText;
		[SerializeField]
		private TextMeshProUGUI _oreDisplayText;
		[SerializeField]
		private TextMeshProUGUI _goldDisplayText;

		[SerializeField]
		private TextMeshProUGUI _woodRateOfChangeText;
		[SerializeField]
		private TextMeshProUGUI _foodRateOfChangeText;
		[SerializeField]
		private TextMeshProUGUI _oreRateOfChangeText;
		[SerializeField]
		private TextMeshProUGUI _goldRateOfChangeText;

		[SerializeField]
		private Slider _seasonalSlider;
		[SerializeField]
		private float _seaonSliderStartOffset;

		[SerializeField]
		private TextMeshProUGUI _playerCountText;

		[SerializeField]
		private TextMeshProUGUI _buildingCountText;

		[SerializeField]
		private TextMeshProUGUI _timeDisplayText;

		public TextMeshProUGUI WoodDisplayText => _woodDisplayText;
		public TextMeshProUGUI FoodDisplayText => _foodDisplayText;
		public TextMeshProUGUI OreDisplayText => _oreDisplayText;
		public TextMeshProUGUI GoldDisplayText => _goldDisplayText;

		public TextMeshProUGUI WoodRateOfChangeText => _woodRateOfChangeText;
		public TextMeshProUGUI FoodRateOfChangeText => _foodRateOfChangeText;
		public TextMeshProUGUI OreRateOfChangeText => _oreRateOfChangeText;
		public TextMeshProUGUI GoldRateOfChangeText => _goldRateOfChangeText;

		public Slider SeasonalSlider => _seasonalSlider;
		public float SeaonSliderStartOffset => _seaonSliderStartOffset;

		public TextMeshProUGUI PlayerCountText => _playerCountText;
		public TextMeshProUGUI BuildingCountText => _buildingCountText;
		public TextMeshProUGUI TimeDisplayText => _timeDisplayText;
	}
}
