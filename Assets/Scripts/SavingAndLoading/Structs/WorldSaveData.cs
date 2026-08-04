using System.Collections.Generic;
using Utils;

namespace SavingAndLoading.Structs
{
	[System.Serializable]
	public struct ResourceAmountSaveData
	{
		public string ResourceType;
		public int Amount;
	}

	/// <summary>
	/// Struct holding information for the world state  (time, last event, time since last event, )
	/// </summary>
	[System.Serializable]
	public struct WorldSaveData
	{
		public float WorldAgeInSeconds;

		public GameEventType LastEvent;
		public int TimeSinceLastEvent;

		public TechTreeSaveData TechTree;
		public List<ResourceAmountSaveData> TownResources;

		//public Weather Weather;
		//public float WeatherTimeLeft;

		public int WoodResourceAmount;
		public int OreResourceAmount;
		public int FoodResourceAmount;
		public int GoldResourceAmount;

		public bool IsCurrentRuler;
		public float TimeUntillNextRulerVote;
		public string RulerName;

		public WorldSaveData(int wood, int ore, int food, int gold, float worldAge, GameEventType lastEvent, int timeSinceLastEvent, TechTreeSaveData techtree, Weather weather, float weatherTimeLeft, bool isCurrentRuler, float timeUntillNextRulerVote, string rulerName)
		{
			WoodResourceAmount = wood;
			OreResourceAmount = ore;
			FoodResourceAmount = food;
			GoldResourceAmount = gold;

			TechTree = techtree;
			TownResources = null;

			WorldAgeInSeconds = worldAge;
			LastEvent = lastEvent;
			TimeSinceLastEvent = timeSinceLastEvent;

			IsCurrentRuler = isCurrentRuler;
			TimeUntillNextRulerVote = timeUntillNextRulerVote;
			RulerName = rulerName;
			//Weather = weather;
			//         WeatherTimeLeft = weatherTimeLeft;
		}
	}
}
