using System.Collections.Generic;
using TechTree.Data;
using TMPro;
using TownGoal.Data;
using UnityEngine;
using UnityEngine.UI;

namespace UserInterface
{
	public class UserInterface_TownGoal : MonoBehaviour
	{
		public Transform TownGoalContainer;
		public Image Icon;
		public TextMeshProUGUI TechTitle;
		public RectTransform ObjectivesContainer;
		public GameObject ObjectivePrefab;

		private Dictionary<Objective, UI_Objective> _followedObjectives;

		public void AddGoal(Goal goal, TechNodeData nodeData)
		{
			List<Objective> objectives = new List<Objective>();

			foreach (var kvp in goal.ObjectivesStatuses)
			{
				objectives.Add(kvp.Key);
			}

			for (int i = 0; i < objectives.Count; i++)
			{
				CreateNewObjective(objectives[i]);
			}

			TechTitle.text = nodeData.NodeTitle;
			ActivateTownGoalContainer();
			goal.OnGoalCompleted += OnGoalFinished;
			string modPath = nodeData.IconPath.Remove(0, 17);
			modPath = modPath.Remove(modPath.Length - 4, 4);
			Icon.sprite = Resources.Load<Sprite>(modPath) as Sprite;
			Debug.Log(modPath);
			//Icon = node Icon;
		}

		private void CreateNewObjective(Objective objective)
		{
			GameObject go = Instantiate(ObjectivePrefab, ObjectivesContainer);

			UI_Objective uiObj = go.GetComponent<UI_Objective>();

			uiObj.AmountTMP.text = $"{objective.Amount} / {objective.RequiredAmount}";
			uiObj.ObjectiveText.text = objective.GetRequirementText();

			uiObj.ObjectiveSlider.value = 0;

			objective.AmountChanged += AmountChanged;
			_followedObjectives.Add(objective, uiObj);
		}

		private void AmountChanged(Objective objective, int amount)
		{
			_followedObjectives[objective].AmountTMP.text = $"{amount} / {objective.RequiredAmount}";
			_followedObjectives[objective].ObjectiveSlider.value = (amount / (float)objective.RequiredAmount);
		}

		public void ActivateTownGoalContainer()
		{
			TownGoalContainer.gameObject.SetActive(true);
		}

		private void OnGoalFinished(Goal goal)
		{
			goal.OnGoalCompleted -= OnGoalFinished;
			DisableTownGoalContainer();
		}

		public void DisableTownGoalContainer()
		{
			if (_followedObjectives != null)
			{
				List<Objective> objectivesToRemove = new List<Objective>();

				foreach (var v in _followedObjectives)
					objectivesToRemove.Add(v.Key);

				for (int i = 0; i < objectivesToRemove.Count; i++)
				{
					Destroy(_followedObjectives[objectivesToRemove[i]].gameObject);
					objectivesToRemove[i].AmountChanged -= AmountChanged;
					_followedObjectives.Remove(objectivesToRemove[i]);
				}
			}

			_followedObjectives = new Dictionary<Objective, UI_Objective>();
			TownGoalContainer.gameObject.SetActive(false);
		}

		private void Start()
		{
			DisableTownGoalContainer();
		}
	}
}
