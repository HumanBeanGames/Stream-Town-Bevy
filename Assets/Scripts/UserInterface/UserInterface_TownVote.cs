using GameEventSystem;
using GameEventSystem.Events.Voting;
using Processors;
using System;
using System.Collections.Generic;
using TechTree.Data;
using Character;
using TMPro;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.UI;
using Reflex.Attributes;

namespace UserInterface
{
	public class UserInterface_TownVote : MonoBehaviour
	{
		public GameObject TownVoteContainer;
		public Transform TownVoteOptionsContainer;
		public GameObject TechOptionPrefab;
		public Slider TimerSlider;
		public TextMeshProUGUI TitleTMP;
		public TextMeshProUGUI TimerTMP;
		public  UnityAction<int> OnBroadcasterVote;

		[SerializeField]
		private Button _bottomBarButton;

		private List<UI_TechOption> _techOptions;
		[Inject] private GameEventProcessor _gameEventProcessor;
		[Inject] private PlayerProcessor _playerProcessor;

		/// <summary>
		/// The debug processor. Injected via Reflex dependency injection.
		/// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

		private bool _canOpenVoteContainer = false;

		public void ActivateVoteContainer()
		{
			TownVoteContainer.SetActive(true);

			_bottomBarButton.interactable = true;
			_canOpenVoteContainer = true;
		}

		public void SetupButtons()
		{

			for (int i = 0; i < _techOptions.Count; i++)
			{
				int num = i;
				_techOptions[i].TechButton.gameObject.SetActive(true);
				_techOptions[i].TechButton.onClick.AddListener(() => BroadcasterVote(num + 1));
			}
		}

		public void BroadcasterVote(int voteId)
		{
			((VoteEvent)_gameEventProcessor.CurrentEvent).Action(new PlayerVote(_playerProcessor.UserPlayer, new VoteOption(voteId.ToString(), null)));
			for (int i = 0; i < _techOptions.Count; i ++)
			{
				_techOptions[i].TechButton.onClick.RemoveAllListeners();
				_techOptions[i].TechButton.interactable = false;
			}
		}

		public void DeactivateVoteContainer()
		{
			_bottomBarButton.interactable = false;
			_canOpenVoteContainer = false;

			if (_techOptions != null)
			{
				for (int i = _techOptions.Count - 1; i >= 0; i--)
				{
					Destroy(_techOptions[i].gameObject);
					_techOptions.RemoveAt(i);
				}
			}

			_techOptions = new List<UI_TechOption>();

			TownVoteContainer.SetActive(false);
		}

		public void ToggleVotingMenu()
		{
			if (_canOpenVoteContainer)
			{
				TownVoteContainer.SetActive(!TownVoteContainer.activeSelf);
			}
		}

		public UI_TechOption AddOption(TechNodeData nodeData, int index)
		{
			GameObject go = Instantiate(TechOptionPrefab, TownVoteOptionsContainer);

			UI_TechOption uiTech = go.GetComponent<UI_TechOption>();
			if (uiTech == null)
				throw new InvalidOperationException("UserInterface_TownVote: TechOptionPrefab is missing UI_TechOption.");
			if (uiTech.ObjectivesContainer == null)
				throw new InvalidOperationException("UserInterface_TownVote: UI_TechOption is missing ObjectivesContainer.");
			if (uiTech.ObjectiveRowPrefab == null)
				throw new InvalidOperationException("UserInterface_TownVote: UI_TechOption is missing ObjectiveRowPrefab.");

			uiTech.TitleTMP.text = nodeData.NodeTitle;
			uiTech.DescriptionTMP.text = nodeData.Description;
			for (int i = uiTech.ObjectivesContainer.childCount - 1; i >= 0; i--)
				Destroy(uiTech.ObjectivesContainer.GetChild(i).gameObject);

			if (nodeData.Objectives != null)
			{
				for (int i = 0; i < nodeData.Objectives.Count; i++)
				{
					if (nodeData.Objectives[i] == null)
						continue;

					GameObject objectiveRow = Instantiate(uiTech.ObjectiveRowPrefab, uiTech.ObjectivesContainer);
					UI_VoteObjectiveRow uiObjectiveRow = objectiveRow.GetComponent<UI_VoteObjectiveRow>();
					if (uiObjectiveRow == null)
						throw new InvalidOperationException("UserInterface_TownVote: ObjectiveRowPrefab is missing UI_VoteObjectiveRow.");
					if (uiObjectiveRow.ObjectiveText == null)
						throw new InvalidOperationException("UserInterface_TownVote: UI_VoteObjectiveRow is missing ObjectiveText.");

					uiObjectiveRow.ObjectiveText.text = nodeData.Objectives[i].GetRequirementText();
				}
			}
			uiTech.VoteCommandTMP.text = $"!vote {index}";
			uiTech.TechButton.gameObject.SetActive(true);
			string modPath = nodeData.IconPath.Remove(0, 17);
			modPath = modPath.Remove(modPath.Length - 4, 4);
			_debugProcessor.Log(DebugLogCategory.UserInterface_TownVote, modPath);
			uiTech.TechIcon.sprite = Resources.Load<Sprite>(modPath) as Sprite;
			_techOptions.Add(uiTech);
			return uiTech;
		}

		private void Start()
		{
			DeactivateVoteContainer();
		}
	}
}
