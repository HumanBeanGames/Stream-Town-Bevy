using TechTree.ScriptableObjects;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "TechTreeSettings", menuName = "Scriptables/Tech Tree Settings")]
	public class TechTreeSettings : ScriptableObject, IDataScriptable
	{
		public int MinTimeBetweenVotes;
		public int TechCountReqAge2 = 50;
		public TechTree_SO TechTreeSO;
	}
}
