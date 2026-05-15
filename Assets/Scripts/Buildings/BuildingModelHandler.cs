using System.Collections.Generic;
using UnityEngine;

namespace Buildings
{
	/// <summary>
	/// Handles the Buildings Models and Construction Staging
	/// </summary>
	[System.Serializable]
	public class BuildingModelHandler : MonoBehaviour
	{
        /// <summary>
        /// The full building model.
        /// </summary>
		public GameObject FullModel;

        /// <summary>
        /// The first stage construction model.
        /// </summary>
		public GameObject Stage1;

        /// <summary>
        /// The second stage construction model.
        /// </summary>
		public GameObject Stage2;

        /// <summary>
        /// The third stage construction model.
        /// </summary>
		public GameObject Stage3;

        /// <summary>
        /// List of upgrade models.
        /// </summary>
		public List<GameObject> Upgrades;

        /// <summary>
        /// List of other models.
        /// </summary>
		public List<GameObject> OtherModels;

		/// <summary>
		/// Used for the start of Construction.
		/// </summary>
		public void OnConstructionStart()
		{
			if (OtherModels != null)
			{
				foreach (GameObject gameObject in OtherModels)
					gameObject.SetActive(false);
			}

			if (Upgrades != null)
			{
				for(int i = 0; i < Upgrades.Count; i++)
					Upgrades[i].SetActive(false);
			}

			if (Stage1 != null)
				Stage1.gameObject.SetActive(true);
			else
				Debug.LogWarning($"Stage1 is null on {name}, cannot show construction stage 1");

			if (FullModel != null)
				FullModel.gameObject.SetActive(false);

			if (Stage3 != null)
				Stage3.gameObject.SetActive(false);

			if (Stage2 != null)
				Stage2.gameObject.SetActive(false);
		}

		/// <summary>
		/// Used to show first stage of construction.
		/// </summary>
		public void OnStage2()
		{
			if (Stage1 != null)
				Stage1.SetActive(false);
			else
				Debug.LogWarning($"Stage1 is null on {name}, cannot hide construction stage 1");

			if (Stage2 != null)
				Stage2.SetActive(true);
			else
				Debug.LogWarning($"Stage2 is null on {name}, cannot show construction stage 2");
		}

		/// <summary>
		/// Used to show the second stage of construction.
		/// </summary>
		public void OnStage3()
		{
			if (Stage2 != null)
				Stage2.SetActive(false);
			else
				Debug.LogWarning($"Stage2 is null on {name}, cannot hide construction stage 2");

			if (Stage3 != null)
				Stage3.SetActive(true);
			else
				Debug.LogWarning($"Stage3 is null on {name}, cannot show construction stage 3");
		}

		/// <summary>
		/// Used to show the full building model.
		/// </summary>
		public void OnFinishedConstruction()
		{
			if (Stage1 != null)
				Stage1.SetActive(false);
			else
				Debug.LogWarning($"Stage1 is null on {name}, cannot hide construction stage 1");

			if (Stage2 != null)
				Stage2.SetActive(false);
			else
				Debug.LogWarning($"Stage2 is null on {name}, cannot hide construction stage 2");

			if (Stage3 != null)
				Stage3.SetActive(false);
			else
				Debug.LogWarning($"Stage3 is null on {name}, cannot hide construction stage 3");

			if (FullModel != null)
				FullModel.SetActive(true);
			else
				Debug.LogWarning($"FullModel is null, cannot activate finished building model on {name}");

			if (Upgrades != null)
			{
				for(int i = 0;i < Upgrades.Count;i++)
					Upgrades[i].SetActive(true);
			}
		}
	}
}
