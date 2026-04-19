using System.Collections.Generic;
using UnityEngine;
using Utils.Pooling;

namespace ScriptablesProcessorInfrastructure
{
	[CreateAssetMenu(fileName = "PoolingConfigSettings", menuName = "Scriptables/PoolingConfigSettings")]
	public class PoolingConfigSettings : ScriptableObject, IDataScriptable
	{
		[Header("Objects to Pool")]
		public List<PooledObjectData> ObjectsToPool = new List<PooledObjectData>();
		
		[Header("Debug Settings")]
		public bool DebugPooling = false;
	}
}
