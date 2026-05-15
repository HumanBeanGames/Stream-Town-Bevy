using UnityEngine;

namespace TownGoal.Data
{
	using Enumerations;
	using Utils;

    /// <summary>
    /// Data class for an objective.
    /// </summary>
	[System.Serializable]
	public class ObjectiveData
	{
        /// <summary>
        /// The objective type.
        /// </summary>
		[field: SerializeField]
		public ObjectiveType ObjectiveType { get; set; }

        /// <summary>
        /// The integer value.
        /// </summary>
		[field: SerializeField]
		public int IntValue { get; set; }

        /// <summary>
        /// The float value.
        /// </summary>
		[field: SerializeField]
		public float FloatValue { get; set; }

        /// <summary>
        /// The building type.
        /// </summary>
		[field: SerializeField]
		public BuildingType BuildingType { get; set; }

        /// <summary>
        /// The resource type.
        /// </summary>
		[field: SerializeField]
		public Resource ResourceType { get; set; }

        /// <summary>
        /// The enemy type.
        /// </summary>
		[field: SerializeField]
		public EnemyType EnemyType { get; set; }

		public string GetDisplayLabel()
		{
			switch (ObjectiveType)
			{
				case ObjectiveType.Build:
					return $"Build {BuildingType}";
				case ObjectiveType.BuildAny:
					return "Build Buildings";
				case ObjectiveType.Collect:
					return $"Gather {ResourceType}";
				case ObjectiveType.Kill:
					return $"Kill {EnemyType}";
				case ObjectiveType.KillAny:
					return "Kill Enemies";
				case ObjectiveType.EarnPerHour:
					return $"Earn {ResourceType}/Hour";
				case ObjectiveType.Buy:
					return $"Buy {ResourceType}";
				case ObjectiveType.BuyAny:
					return "Buy Resources";
				case ObjectiveType.Sell:
					return $"Sell {ResourceType}";
				case ObjectiveType.SellAny:
					return "Sell Resources";
				default:
					return ObjectiveType.ToString();
			}
		}

		public string GetRequirementText()
		{
			return $"{GetDisplayLabel()}: {IntValue}";
		}
	}
}
