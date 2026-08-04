using System.Collections.Generic;

namespace SavingAndLoading.Structs 
{
    [System.Serializable]
    public class SavePlayersData 
	{
        public List<PlayerSaveData> PlayerSaveDatas;

		public SavePlayersData()
		{
		}

        public SavePlayersData(List<PlayerSaveData> playerSaveDatas)
		{
            PlayerSaveDatas = playerSaveDatas;
		}
    }
}
