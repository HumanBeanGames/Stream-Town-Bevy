using UserInterface;

using ScriptablesProcessorInfrastructure;
using TownGoal.Data;
using TMPro;
using UnityEngine.UI;

namespace Processors
{
	/// <summary>
	/// Runtime data class that stores UI state for the game.
	/// Manages UI interface references.
	/// </summary>
	public class UIRuntimeData : IRuntimeDataScriptable
	{
		private UserInterface_TownGoal _townGoalInterface;
		private UserInterface_RulerVote _rulerVoteInterface;
		private UserInterface_TownVote _townVoteInterface;
		private UserInterface_Event _eventInterface;
		private Goal _renderedTownGoal;
		private TextMeshProUGUI _woodDisplayText;
		private TextMeshProUGUI _foodDisplayText;
		private TextMeshProUGUI _oreDisplayText;
		private TextMeshProUGUI _goldDisplayText;
		private TextMeshProUGUI _woodRateOfChangeText;
		private TextMeshProUGUI _foodRateOfChangeText;
		private TextMeshProUGUI _oreRateOfChangeText;
		private TextMeshProUGUI _goldRateOfChangeText;
		private TextMeshProUGUI _recruitDisplayText;
		private TextMeshProUGUI _recruitRateOfChangeText;
		private Slider _seasonalSlider;
		private TextMeshProUGUI _playerCountText;
		private TextMeshProUGUI _buildingCountText;
		private TextMeshProUGUI _timeDisplayText;
		private TextMeshProUGUI _recruitCountText;
		private TextMeshProUGUI _activeObjectiveText;

		public UserInterface_TownGoal TownGoalInterface
		{
			get => _townGoalInterface;
			set => _townGoalInterface = value;
		}
		public UserInterface_RulerVote RulerVoteInterface
		{
			get => _rulerVoteInterface;
			set => _rulerVoteInterface = value;
		}
		public UserInterface_TownVote TownVoteInterface
		{
			get => _townVoteInterface;
			set => _townVoteInterface = value;
		}
		public UserInterface_Event EventInterface
		{
			get => _eventInterface;
			set => _eventInterface = value;
		}
		public Goal RenderedTownGoal
		{
			get => _renderedTownGoal;
			set => _renderedTownGoal = value;
		}
		public TextMeshProUGUI WoodDisplayText
		{
			get => _woodDisplayText;
			set => _woodDisplayText = value;
		}
		public TextMeshProUGUI FoodDisplayText
		{
			get => _foodDisplayText;
			set => _foodDisplayText = value;
		}
		public TextMeshProUGUI OreDisplayText
		{
			get => _oreDisplayText;
			set => _oreDisplayText = value;
		}
		public TextMeshProUGUI GoldDisplayText
		{
			get => _goldDisplayText;
			set => _goldDisplayText = value;
		}
		public TextMeshProUGUI WoodRateOfChangeText
		{
			get => _woodRateOfChangeText;
			set => _woodRateOfChangeText = value;
		}
		public TextMeshProUGUI FoodRateOfChangeText
		{
			get => _foodRateOfChangeText;
			set => _foodRateOfChangeText = value;
		}
		public TextMeshProUGUI OreRateOfChangeText
		{
			get => _oreRateOfChangeText;
			set => _oreRateOfChangeText = value;
		}
		public TextMeshProUGUI GoldRateOfChangeText
		{
			get => _goldRateOfChangeText;
			set => _goldRateOfChangeText = value;
		}
		public TextMeshProUGUI RecruitDisplayText
		{
			get => _recruitDisplayText;
			set => _recruitDisplayText = value;
		}
		public TextMeshProUGUI RecruitRateOfChangeText
		{
			get => _recruitRateOfChangeText;
			set => _recruitRateOfChangeText = value;
		}
		public Slider SeasonalSlider
		{
			get => _seasonalSlider;
			set => _seasonalSlider = value;
		}
		public TextMeshProUGUI PlayerCountText
		{
			get => _playerCountText;
			set => _playerCountText = value;
		}
		public TextMeshProUGUI BuildingCountText
		{
			get => _buildingCountText;
			set => _buildingCountText = value;
		}
		public TextMeshProUGUI TimeDisplayText
		{
			get => _timeDisplayText;
			set => _timeDisplayText = value;
		}
		public TextMeshProUGUI RecruitCountText
		{
			get => _recruitCountText;
			set => _recruitCountText = value;
		}
		public TextMeshProUGUI ActiveObjectiveText
		{
			get => _activeObjectiveText;
			set => _activeObjectiveText = value;
		}

		/// <summary>
		/// Initializes the UI runtime data with default values.
		/// </summary>
		public UIRuntimeData()
		{
			_townGoalInterface = null;
			_rulerVoteInterface = null;
			_townVoteInterface = null;
			_eventInterface = null;
			_renderedTownGoal = null;
			_woodDisplayText = null;
			_foodDisplayText = null;
			_oreDisplayText = null;
			_goldDisplayText = null;
			_woodRateOfChangeText = null;
			_foodRateOfChangeText = null;
			_oreRateOfChangeText = null;
			_goldRateOfChangeText = null;
			_recruitDisplayText = null;
			_recruitRateOfChangeText = null;
			_seasonalSlider = null;
			_playerCountText = null;
			_buildingCountText = null;
			_timeDisplayText = null;
			_recruitCountText = null;
			_activeObjectiveText = null;
		}
	}
}
