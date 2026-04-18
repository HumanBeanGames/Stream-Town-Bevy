using Buildings;
using System.Collections.Generic;
using UnityEngine;
using Utils;
using GameResources;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores global building system settings for the game.
	/// Contains cost modifiers, max levels, building ages, and cost resources.
	/// </summary>
	[CreateAssetMenu(fileName = "BuildingSettings", menuName = "Scriptables/Building Settings")]
	public class BuildingSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Dictionary mapping building types to their individual cost modifiers.
		/// Positive values increase cost, negative values decrease cost.
		/// </summary>
		[SerializeField]
		private Dictionary<BuildingType, int> _buildingCostModifiers = new Dictionary<BuildingType, int>();

		public Dictionary<BuildingType, int> BuildingCostModifiers
		{
			get => _buildingCostModifiers;
			set => _buildingCostModifiers = value;
		}

		/// <summary>
		/// Global modifier applied to all building costs.
		/// Positive values increase all building costs, negative values decrease them.
		/// </summary>
		[SerializeField]
		private int _globalBuildCostModifier;

		public int GlobalBuildCostModifier
		{
			get => _globalBuildCostModifier;
			set => _globalBuildCostModifier = value;
		}

		/// <summary>
		/// Dictionary mapping building types to their maximum upgrade levels.
		/// Determines how many times each building type can be upgraded.
		/// </summary>
		[SerializeField]
		private Dictionary<BuildingType, int> _buildingsMaxLevel = new Dictionary<BuildingType, int>();

		public Dictionary<BuildingType, int> BuildingsMaxLevel
		{
			get => _buildingsMaxLevel;
			set => _buildingsMaxLevel = value;
		}

		/// <summary>
		/// Dictionary mapping building types to their required building age.
		/// Determines which age a building belongs to for progression purposes.
		/// </summary>
		[SerializeField]
		private Dictionary<BuildingType, Age> _buildingAges = new Dictionary<BuildingType, Age>();

		public Dictionary<BuildingType, Age> BuildingAges
		{
			get => _buildingAges;
			set => _buildingAges = value;
		}

		/// <summary>
		/// Dictionary mapping building types to their resource cost requirements.
		/// Nested dictionary maps each building type to a dictionary of resource types and amounts.
		/// </summary>
		[SerializeField]
		private Dictionary<BuildingType, Dictionary<Resource, int>> _buildingsCostResources = new Dictionary<BuildingType, Dictionary<Resource, int>>();

		public Dictionary<BuildingType, Dictionary<Resource, int>> BuildingsCostResources => _buildingsCostResources;

		/// <summary>
		/// Whether buildings cost resources to construct.
		/// If true, resources are consumed when building; if false, buildings are free.
		/// </summary>
		[SerializeField]
		private bool _buildingsCostResourcesEnabled = true;

		public bool BuildingsCostResourcesEnabled => _buildingsCostResourcesEnabled;

		/// <summary>
		/// Whether tech tree unlocks are ignored.
		/// If true, all buildings/features are available regardless of tech progress.
		/// Used for testing or debug purposes.
		/// </summary>
		[SerializeField]
		private bool _ignoreTechUnlocks = false;

		public bool IgnoreTechUnlocks => _ignoreTechUnlocks;

		public void UnlockBuilding(BuildingType buildingType)
		{
			if (_buildingsMaxLevel.ContainsKey(buildingType))
				_buildingsMaxLevel[buildingType] = 1;
			else
				_buildingsMaxLevel.Add(buildingType, 1);
		}
	}
}
