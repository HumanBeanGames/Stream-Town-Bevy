using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "ResourceData", menuName = "Scriptables/ResourceData")]
	public class ResourceData : ScriptableObject, IDataScriptable
	{
		[Header("Food")]
		public int FoodStartingAmount = 5000;
		public int FoodMaxAmount = 15000;
		public bool FoodInfinite = false;

		[Header("Ore")]
		public int OreStartingAmount = 5000;
		public int OreMaxAmount = 15000;
		public bool OreInfinite = false;

		[Header("Wood")]
		public int WoodStartingAmount = 5000;
		public int WoodMaxAmount = 15000;
		public bool WoodInfinite = false;

		[Header("Gold")]
		public int GoldStartingAmount = 5000;
		public int GoldMaxAmount = 0;
		public bool GoldInfinite = true;

		[Header("Recruit")]
		public int RecruitStartingAmount = 0;
		public int RecruitMaxAmount = 5;
		public bool RecruitInfinite = false;

		[Header("Resource Bounds")]
		public Vector3 ResourceBounds = new Vector3(1, 5, 1);
	}
}
