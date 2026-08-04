using System.Collections.Generic;

namespace SavingAndLoading.Structs 
{
	[System.Serializable]
	public struct TechVotePlayerSaveData
	{
		public string TwitchId;
		public string OptionName;
	}

	[System.Serializable]
	public struct TechVoteSaveData
	{
		public bool Exists;
		public float SecondsUntilStart;
		public float RemainingDuration;
		public List<string> TechNames;
		public List<TechVotePlayerSaveData> PlayerVotes;
	}

    [System.Serializable]
    public struct TechTreeSaveData 
	{
        public bool TechAvailable;
        public List<string> UnlockedTechIds;
        // Legacy positional data. New saves use UnlockedTechIds.
        public List<bool> UnlockedTechs;
        public string CurrentTechName;

        public List<ObjectiveSaveData> CurrentTechData;
		public TechVoteSaveData TechVote;

        public TechTreeSaveData(List<bool> unlockedTechs, string curreentTechName, List<ObjectiveSaveData> currentTechData, bool techAvailable = true)
		{
            TechAvailable = techAvailable;
            UnlockedTechIds = new List<string>();
            UnlockedTechs = unlockedTechs;
            CurrentTechName = curreentTechName;
            CurrentTechData = currentTechData;
			TechVote = default;
		}
    }
}
