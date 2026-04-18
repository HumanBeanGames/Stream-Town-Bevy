using UnityEngine;
using Utils;

namespace Character
{
	/// <summary>
	/// Used to keep track of each player role's active equipment.
	/// </summary>
	[System.Serializable]
	public class RoleEquipment
	{
        /// <summary>
        /// The name of the role.
        /// </summary>
		public string RoleName;

        /// <summary>
        /// The player role.
        /// </summary>
		public PlayerRole PlayerRole;

        /// <summary>
        /// The slim body GameObject.
        /// </summary>
		public GameObject BodieSlim;

        /// <summary>
        /// The bulk body GameObject.
        /// </summary>
		public GameObject BodieBulk;

        /// <summary>
        /// The feminine body GameObject.
        /// </summary>
		public GameObject BodieFeminine;

        /// <summary>
        /// The left hand GameObject.
        /// </summary>
		public GameObject LeftHand;

        /// <summary>
        /// The right hand GameObject.
        /// </summary>
		public GameObject RightHand;

        /// <summary>
        /// The helmet GameObject.
        /// </summary>
		public GameObject Helmet;

        /// <summary>
        /// Whether the role has a carry animation.
        /// </summary>
		public bool HasCarryAnimation;

        /// <summary>
        /// The carry animation name.
        /// </summary>
		public AnimationName CarryAnimation;

        /// <summary>
        /// Whether the left hand is permanent.
        /// </summary>
		public bool LeftHandPermanent;
	}
}
