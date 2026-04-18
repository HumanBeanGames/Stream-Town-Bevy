using System.Collections.Generic;
using UnityEngine;

namespace Requirements 
{
    /// <summary>
    /// ScriptableObject container for a list of requirements.
    /// </summary>
    [CreateAssetMenu(fileName = "RequirementsDataData", menuName = "ScriptableObjects/RequirementsData", order = 1)]
    public class RequirementsData : ScriptableObject 
	{
        /// <summary>
        /// The list of requirements.
        /// </summary>
		public List<Requirement> Requirements;
    }
}
