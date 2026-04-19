using Buildings;
using Character;
using GameEventSystem.Events.Voting;
using Processors;
using System;
using System.Collections.Generic;
using System.Collections;
using TechTree;
using TechTree.ScriptableObjects;
using TechTree.Data;
using TownGoal.Data;
using TownGoal;
using Twitch;
using UnityEngine;
using World;
using Reflex.Attributes;
using Reflex.Core;
using SavingAndLoading;
using ScriptablesProcessorInfrastructure;
using Data.Containers;
using GameEventSystem;
using GameResources;
using UserInterface;
using Utils;

namespace Processors
{
	public class TechTreeProcessor : MonoBehaviour, IInstaller, IProcessor
	{
		[Inject] private MetaData.MetaData _metaData;
		[Inject] private TechTreeSettings _techTreeSettings;
		[Inject] private TechTreeRuntimeData _techTreeRuntimeData;
		[Inject] private GameEventProcessor _gameEventProcessor;
		[Inject] private TownGoalProcessor _townGoalProcessor;
		[Inject] private BuildingSettings _buildingSettings;
		[Inject] private BuildingProcessor _buildingProcessor;
		[Inject] private UIProcessor _uiProcessor;
		[Inject] private TownResourceProcessor _townResourceProcessor;
		[Inject] private PlayerProcessor _playerProcessor;

		public Action<Resource> OnStorageBoostUnlocked;
		public Action<PlayerRole, StatType> OnStatBoostUnlocked;
		public Action<BuildingType> OnBuildingUnlocked;
		public Action<BuildingType> OnBuildingLevelIncreased;
		public Action<BuildingType> OnBuildingCostReduction;
		public Action<BuildingType> OnBuildingAgedUp;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			// Instantiate and register TechTreeRuntimeData ScriptableObject
			TechTreeRuntimeData techTreeRuntimeData = ScriptableObject.CreateInstance<TechTreeRuntimeData>();
			containerBuilder.AddSingleton(techTreeRuntimeData);
		}

		public void InitializeTree()
		{
			_techTreeRuntimeData.InitializeTechTree(new TechnologyTree(_techTreeSettings.TechTreeSO, this, _metaData));
			_techTreeRuntimeData.InitializeGoalsFollowed(new Dictionary<Goal, Node_SO>());
			//PrintAvailableNodes();
		}

		public void IncrementTimeSinceLastUnlock()
		{
			_techTreeRuntimeData.TimeSinceLastUnlock++;
		}

		/// <summary>
		/// Gets the time since last tech unlock.
		/// </summary>
		public int TimeSinceLastUnlock => _techTreeRuntimeData.TimeSinceLastUnlock;

		/// <summary>
		/// Gets the number of techs unlocked.
		/// </summary>
		public int TechsUnlocked => _techTreeRuntimeData.TechsUnlocked;

		public void ResetTimeSinceLastUnlock()
		{
			_techTreeRuntimeData.TimeSinceLastUnlock = 0;
		}

		public void IncrementTechsUnlocked()
		{
			_techTreeRuntimeData.TechsUnlocked++;
		}

		public TechNodeData CurrentTechData => TechNodeData.FromNodeSO(_techTreeRuntimeData.CurrentTech);

		// Internal property for Node_SO access (kept for internal use)
		public Node_SO CurrentTech
		{
			get => _techTreeRuntimeData.CurrentTech;
			set => _techTreeRuntimeData.CurrentTech = value;
		}

		public float RequestedTechVoteDelay
		{
			get => _techTreeRuntimeData.RequestedTechVoteDelay;
			set => _techTreeRuntimeData.RequestedTechVoteDelay = value;
		}

		public bool RequestStartTechVote
		{
			get => _techTreeRuntimeData.RequestStartTechVote;
			set => _techTreeRuntimeData.RequestStartTechVote = value;
		}

		public List<bool> GetUnlockedTechStates()
		{
			return _techTreeRuntimeData.TechTree.GetUnlockedNodes();
		}

		public void SetUnlockedTechStates(List<bool> unlockedNodes)
		{
			_techTreeRuntimeData.TechTree.SetUnlockedNodes(unlockedNodes);
		}

		public Goal StartGoalFromNodeName(string techName)
		{
			return StartGoalFromNode(_techTreeRuntimeData.TechTree.GetNodeFromName(techName));
		}

		public void RequestDelayedSetup()
		{
			_techTreeRuntimeData.RequestDelayedSetup = true;
		}

		public void AddGoalFollowed(Goal goal, Node_SO node)
		{
			_techTreeRuntimeData.GoalsFollowed.Add(goal, node);
		}

		public void RemoveGoalFollowed(Goal goal)
		{
			_techTreeRuntimeData.GoalsFollowed.Remove(goal);
		}

		public bool HasGoalFollowed(Goal goal)
		{
			return _techTreeRuntimeData.GoalsFollowed.ContainsKey(goal);
		}

		public TechNodeData GetGoalNodeData(Goal goal)
		{
			return TechNodeData.FromNodeSO(_techTreeRuntimeData.GoalsFollowed[goal]);
		}

		// Internal method for Node_SO access (kept for internal use)
		public Node_SO GetGoalNode(Goal goal)
		{
			return _techTreeRuntimeData.GoalsFollowed[goal];
		}

		public void PrintAvailableNodes()
		{
			var availableNodes = _techTreeRuntimeData.TechTree.AvailableNodes;

			for (int i = 0; i < availableNodes.Count; i++)
				Debug.Log(availableNodes[i].TechName);
		}

		public TechNodeData[] GetRandomAvailableTechsData(int count = 3)
		{
			Node_SO[] nodes = GetRandomAvailableTechs(count);
			TechNodeData[] techData = new TechNodeData[nodes.Length];
			for (int i = 0; i < nodes.Length; i++)
			{
				techData[i] = TechNodeData.FromNodeSO(nodes[i]);
			}
			return techData;
		}

		// Internal method for Node_SO access (kept for internal use)
		public Node_SO[] GetRandomAvailableTechs(int count = 3)
		{
			List<Node_SO> nodes = new List<Node_SO>(_techTreeRuntimeData.TechTree.AvailableNodes.Count);

			for (int i = 0; i < _techTreeRuntimeData.TechTree.AvailableNodes.Count; i++)
			{
				if (_techTreeRuntimeData.TechTree.AvailableNodes[i].Unavailable)
					continue;

				bool canAdd = false;
				if (_buildingSettings.BuildingAges[BuildingType.Townhall] != Age.Age2)
					if (_techTreeRuntimeData.TechTree.AvailableNodes[i].Age == Age.Age2)
						for (int j = 0; j < _techTreeRuntimeData.TechTree.AvailableNodes[i].Unlocks.Count; j++)
						{
							if (_techTreeRuntimeData.TechTree.AvailableNodes[i].Unlocks[j].BuildingType == BuildingType.Townhall && _techTreeRuntimeData.TechTree.AvailableNodes[i].Unlocks[j].TechType == TechType.AgeUpBuilding && _techTreeRuntimeData.TechsUnlocked >= _techTreeSettings.TechCountReqAge2)
								canAdd = true;
						}
					else
						canAdd = true;
				else
					canAdd = true;
				if (canAdd)
					nodes.Add(_techTreeRuntimeData.TechTree.AvailableNodes[i]);
			}

			Node_SO[] randomNodes = new Node_SO[count];

			for (int i = 0; i < count; i++)
			{
				randomNodes[i] = GetRandomTechFromList(nodes);
			}

			return randomNodes;
		}

		private void GoalCompleted(Goal goal)
		{

			if (_techTreeRuntimeData.HasGoalFollowed(goal))
			{
				_techTreeRuntimeData.TechTree.UnlockNode(_techTreeRuntimeData.GetGoalNode(goal));
				_techTreeRuntimeData.RemoveGoalFollowed(goal);
				goal.OnGoalCompleted -= GoalCompleted;
			}

			_techTreeRuntimeData.ClearCurrentTech();
			StartNewTechVote();
		}

		public void StartNewRandomTech()
		{
			var nodes = GetRandomAvailableTechs();

			if (nodes[0] == null)
				return;

			string text = "Options: ";

			for (int i = 0; i < nodes.Length; i++)
			{
				if (nodes[i] != null)
					text += $"{nodes[i].TechName} |";
			}

			this.StartGoalFromNode(nodes[0]);
		}

		public void StartNewTechVote(float delay = 0)
		{
			var nodeDataArray = GetRandomAvailableTechsData();

			if (nodeDataArray.Length == 0)
				return;

			if (nodeDataArray[0] == null)
				return;

			TechVote voteEvent = new TechVote(delay, 60, nodeDataArray);
			voteEvent.EventEnded += OnTechVoteEnded;
			if (!EventTypeExistsInQueue(voteEvent.Event))
				_gameEventProcessor.EventQueue.Add(voteEvent);
		}

		private bool EventTypeExistsInQueue(GameEvent.EventType type)
		{
			if (_gameEventProcessor.CurrentEvent != null && _gameEventProcessor.CurrentEvent.Event == type)
				return true;

			foreach (GameEvent gameEvent in _gameEventProcessor.EventQueue)
			{
				if (gameEvent.Event == type)
					return true;
			}

			return false;
		}

		private void OnTechVoteEnded(bool success, GameEvent.EventType type, object data)
		{
			if (data == null)
				return;

			TechNodeData nodeData = ((VoteOption)data).OptionData as TechNodeData;
			if (nodeData == null || nodeData.Objectives == null || nodeData.Objectives.Count == 0)
				return;

			// Find the actual Node_SO from TechTree by name
			Node_SO node = _techTreeRuntimeData.TechTree.GetNodeFromName(nodeData.TechName);
			if (node == null)
				return;

			Goal goal = new Goal(node.Objectives);
			_townGoalProcessor.StartNewGoal(goal);
			goal.OnGoalCompleted += GoalCompleted;
			_techTreeRuntimeData.AddGoalFollowed(goal, node);
			_uiProcessor.TownGoalInterface.AddGoal(goal, nodeData);

			_techTreeRuntimeData.CurrentTech = node;
		}

		private Node_SO GetRandomTechFromList(List<Node_SO> values)
		{
			if (values.Count == 0)
				return null;

			int rand = UnityEngine.Random.Range(0, values.Count);
			Node_SO node = values[rand];

			values.Remove(node);

			return node;
		}

		public TechNodeData GetCurrentTechData()
		{
			return TechNodeData.FromNodeSO(_techTreeRuntimeData.CurrentTech);
		}

		// Internal method for Node_SO access (kept for internal use)
		public Node_SO GetCurrentTech()
		{
			return _techTreeRuntimeData.CurrentTech;
		}

		public Goal StartGoalFromNode(Node_SO node)
		{
			if (node == null || node.Objectives == null || node.Objectives.Count == 0)
				return null;

			TechNodeData nodeData = TechNodeData.FromNodeSO(node);
			Goal goal = new Goal(node.Objectives);
			_townGoalProcessor.StartNewGoal(goal);
			goal.OnGoalCompleted += GoalCompleted;
			_techTreeRuntimeData.AddGoalFollowed(goal, node);
			_uiProcessor.TownGoalInterface.AddGoal(goal, nodeData);

			_techTreeRuntimeData.CurrentTech = node;
			return goal;
		}

		public void UnlockAllTech()
		{
			var availableTechs = GetRandomAvailableTechs();
			do
			{
				_techTreeRuntimeData.TechTree.UnlockNode(availableTechs[0]);
				availableTechs = GetRandomAvailableTechs();
			} while (availableTechs.Length > 0 && availableTechs[0] != null);
		}

		public void UnlockToAge2Tech()
		{
			var availableTechs = GetRandomAvailableTechs();
			do
			{
				_techTreeRuntimeData.TechTree.UnlockNode(availableTechs[0]);
				availableTechs = GetRandomAvailableTechs();
			} while (availableTechs.Length > 0 && availableTechs[0] != null && availableTechs[0].Age == Age.Age1);
		}

		public void UnlockTech(Node_SO techNode)
		{
			OnTechUnlocked(techNode);
		}

		public void OnTechUnlocked(Node_SO techNode)
		{
			_techTreeRuntimeData.IncrementTechsUnlocked();
			for (int i = 0; i < techNode.Unlocks.Count; i++)
			{
				var data = techNode.Unlocks[i];
				switch (data.TechType)
				{
					case TechType.StatBoost:
						StatBoostUnlocked(data);
						break;
					case TechType.UnlockBuilding:
						UnlockBuilding(data);
						break;
					case TechType.BuildingCostReduction:
						BuildingCostReduction(data);
						break;
					case TechType.StorageBoost:
						StorageBoostUpgrade(data);
						break;
					case TechType.UpgradeBuilding:
						BuildingMaxLevelIncreased(data);
						break;
					case TechType.AgeUpBuilding:
						AgeBuilding(data);
						break;
				}
			}
		}

		/// <summary>
		/// Called when the node unlock data boosts a stat.
		/// </summary>
		/// <param name="data"></param>
		private void StatBoostUnlocked(NodeUnlockData data)
		{
			StatType statType = data.StatType;

			PlayerRole playerRole = data.PlayerRole;

			if (playerRole == PlayerRole.Count)
			{
				_playerProcessor.GlobalStatModifiers.AddToModifier(statType, data.IntValue);
			}
			else
				_playerProcessor.GetStatModifiers(playerRole).AddToModifier(statType, data.IntValue);

			OnStatBoostUnlocked?.Invoke(playerRole, statType);
		}

		/// <summary>
		/// Called when a node unlock unlocks a building.
		/// </summary>
		/// <param name="data"></param>
		private void UnlockBuilding(NodeUnlockData data)
		{
			_buildingSettings.UnlockBuilding(data.BuildingType);

			OnBuildingUnlocked?.Invoke(data.BuildingType);
		}

		private void BuildingCostReduction(NodeUnlockData data)
		{
			BuildingType buildingType = data.BuildingType;

			if (buildingType == BuildingType.Count)
				_buildingSettings.GlobalBuildCostModifier += data.IntValue;
			else
				_buildingSettings.BuildingCostModifiers[buildingType] += data.IntValue;

			OnBuildingCostReduction?.Invoke(buildingType);
		}

		/// <summary>
		/// Triggered when a node Unlock boosts the storage.
		/// </summary>
		/// <param name="data"></param>
		private void StorageBoostUpgrade(NodeUnlockData data)
		{
			Resource resourceType = data.ResourceType;

			if (_townResourceProcessor.ResourceBoostValues.ContainsKey(resourceType))
				_townResourceProcessor.ResourceBoostValues[resourceType] += data.IntValue;

			OnStorageBoostUnlocked?.Invoke(resourceType);
		}

		/// <summary>
		/// Called when a node unlock upgrades a building.
		/// </summary>
		/// <param name="data"></param>
		private void BuildingMaxLevelIncreased(NodeUnlockData data)
		{
			_buildingSettings.BuildingsMaxLevel[data.BuildingType] = data.IntValue;

			OnBuildingLevelIncreased?.Invoke(data.BuildingType);
		}

		/// <summary>
		/// Called when a node unlock Ages up a building.
		/// </summary>
		/// <param name="data"></param>
		private void AgeBuilding(NodeUnlockData data)
		{
			if ((int)_buildingSettings.BuildingAges[data.BuildingType] < 1)
				_buildingSettings.BuildingAges[data.BuildingType]++;

			Age age = (Age)data.IntValue;
			OnBuildingAgedUp?.Invoke(data.BuildingType);
		}

		public void Initialize()
		{
			if (_metaData != null && _metaData.LoadType == MetaData.LoadType.Generate)
				StartNewTechVote(20);
		}

		/// <summary>
		/// Processes tech tree logic every frame.
		/// Called every frame by the Coordinator.
		/// TechTreeProcessor does not require per-frame updates.
		/// </summary>
		public void Process()
		{
			if (_techTreeRuntimeData.RequestDelayedSetup)
			{
				InitializeTree();
				_techTreeRuntimeData.RequestDelayedSetup = false;
			}

			if (_techTreeRuntimeData.RequestStartTechVote)
			{
				StartNewTechVote(_techTreeRuntimeData.RequestedTechVoteDelay);
				_techTreeRuntimeData.RequestStartTechVote = false;
				_techTreeRuntimeData.RequestedTechVoteDelay = 0f;
			}
		}
	}
}
