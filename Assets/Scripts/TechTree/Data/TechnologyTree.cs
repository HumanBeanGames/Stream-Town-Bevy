using Processors;
using System;
using System.Collections.Generic;
using TechTree.ScriptableObjects;
using UnityEngine;
using UnityEngine.Profiling;

namespace TechTree.Data
{
    /// <summary>
    /// Manages the technology tree.
    /// </summary>
	public class TechnologyTree
	{
        /// <summary>
        /// The tech tree ScriptableObject.
        /// </summary>
		private TechTree_SO _tree;

        /// <summary>
        /// The root node.
        /// </summary>
		private Node_SO _rootNode;

        /// <summary>
        /// The available nodes.
        /// </summary>
		private List<Node_SO> _availableNodes;

        /// <summary>
        /// Gets the root node.
        /// </summary>
		public Node_SO RootNode => _rootNode;

        /// <summary>
        /// Gets the available nodes.
        /// </summary>
		public List<Node_SO> AvailableNodes => _availableNodes;

        /// <summary>
        /// Event invoked when a technology is unlocked.
        /// </summary>
		public Action<Node_SO> TechUnlocked;

        /// <summary>
        /// The dictionary of unlocked nodes.
        /// </summary>
		public Dictionary<Node_SO, bool> _unlockedNodes;

        /// <summary>
        /// Whether to debug unlocks.
        /// </summary>
		private static bool _debugUnlocks = false;

        /// <summary>
        /// The tech tree processor.
        /// </summary>
		private TechTreeProcessor _techTreeProcessor;

        /// <summary>
        /// The metadata.
        /// </summary>
		private MetaData.MetaData _metaData;

        /// <summary>
        /// Initializes a new instance of the TechnologyTree class.
        /// </summary>
        /// <param name="tree">The tech tree ScriptableObject.</param>
        /// <param name="processor">The tech tree processor.</param>
        /// <param name="metaData">The metadata.</param>
		public TechnologyTree(TechTree_SO tree, TechTreeProcessor processor, MetaData.MetaData metaData)
		{
			Profiler.BeginSample("Initialize Tech Tree");
			_tree = tree;
			_availableNodes = new List<Node_SO>();
			_unlockedNodes = new Dictionary<Node_SO, bool>();
			_techTreeProcessor = processor;
			_metaData = metaData;
			Profiler.EndSample();
			TechUnlocked += processor.OnTechUnlocked;
			InitializeData();
			Debug.Log($"Root Node Result: {_rootNode.TechName}");
		}

        /// <summary>
        /// Unlocks a node.
        /// </summary>
        /// <param name="node">The node to unlock.</param>
		public void UnlockNode(Node_SO node)
		{
			if (AvailableNodes.Contains(node))
			{
				_unlockedNodes[node] = true;
				AvailableNodes.Remove(node);

				RecursivelyAddAvailableNodes(node);
				TechUnlocked?.Invoke(node);
			}
		}

        /// <summary>
        /// Forces a node to unlock.
        /// </summary>
        /// <param name="node">The node to unlock.</param>
		public void ForceUnlockNode(Node_SO node)
		{
			if (AvailableNodes.Contains(node))
				AvailableNodes.Remove(node);

			if (_debugUnlocks)
				Debug.Log($"{node} unlocked.");
			_unlockedNodes[node] = true;

			RecursivelyAddAvailableNodes(node);
			TechUnlocked?.Invoke(node);
		}

        /// <summary>
        /// Checks if a node is unlocked.
        /// </summary>
        /// <param name="node">The node to check.</param>
        /// <returns>True if unlocked, false otherwise.</returns>
		public bool IsUnlocked(Node_SO node)
		{
			return _unlockedNodes[node];
		}

        /// <summary>
        /// Gets the unlocked nodes.
        /// </summary>
        /// <returns>The list of unlocked node states.</returns>
		public List<bool> GetUnlockedNodes()
		{
			List<bool> result = new List<bool>();

			foreach (Node_SO node in _unlockedNodes.Keys)
			{
				result.Add(_unlockedNodes[node]);
			}

			return result;
		}

		/// <summary>
		/// Returns stable, data-authored identifiers for unlocked technologies.
		/// TechName is the persistence contract; adding or reordering nodes does not
		/// change existing saves.
		/// </summary>
		public List<string> GetUnlockedNodeIds()
		{
			List<string> result = new List<string>();
			foreach (KeyValuePair<Node_SO, bool> node in _unlockedNodes)
			{
				if (node.Value && node.Key != null && !string.IsNullOrWhiteSpace(node.Key.TechName))
					result.Add(node.Key.TechName);
			}

			return result;
		}

		/// <summary>
		/// Rebuilds the tree from stable identifiers and invokes unlock effects once,
		/// from roots toward leaves.
		/// </summary>
		public void RestoreUnlockedNodeIds(IEnumerable<string> unlockedNodeIds)
		{
			HashSet<string> ids = new HashSet<string>(unlockedNodeIds ?? Array.Empty<string>(), StringComparer.Ordinal);
			// Authored baseline unlocks are part of the game definition, not mutable
			// runtime state. Merge them so older/broken saves cannot relock Hamlet.
			foreach (Node_SO node in _unlockedNodes.Keys)
			{
				if (node != null && node.IsUnlocked && !string.IsNullOrWhiteSpace(node.TechName))
					ids.Add(node.TechName);
			}
			// Generate-mode initialization always unlocks the root. It is therefore
			// baseline game state even when an older save omitted it.
			if (_rootNode != null && !string.IsNullOrWhiteSpace(_rootNode.TechName))
				ids.Add(_rootNode.TechName);

			_availableNodes.Clear();
			foreach (Node_SO node in new List<Node_SO>(_unlockedNodes.Keys))
				_unlockedNodes[node] = false;

			List<Node_SO> nodes = new List<Node_SO>();
			foreach (Node_SO node in _unlockedNodes.Keys)
			{
				if (node != null && ids.Contains(node.TechName))
					nodes.Add(node);
			}

			nodes.Sort((left, right) => GetDepth(left).CompareTo(GetDepth(right)));
			for (int i = 0; i < nodes.Count; i++)
				ForceUnlockNode(nodes[i]);

			if (_rootNode != null && !_unlockedNodes[_rootNode])
				_availableNodes.Add(_rootNode);
		}

		private static int GetDepth(Node_SO node)
		{
			int depth = 0;
			Node_SO current = node;
			while (current != null && current.Parent != null)
			{
				depth++;
				current = current.Parent;
			}

			return depth;
		}

        /// <summary>
        /// Gets the current node's index.
        /// </summary>
        /// <returns>The current node index.</returns>
		public int GetCurrentNodesIndex()
		{
			int i = 0;
			Node_SO currentTech = _techTreeProcessor.GetCurrentTech();

			foreach (Node_SO node in _unlockedNodes.Keys)
			{
				if (node == currentTech)
					break;
				i++;
			}

			return i;
		}

        /// <summary>
        /// Gets a node from its name.
        /// </summary>
        /// <param name="techName">The technology name.</param>
        /// <returns>The node, or null if not found.</returns>
		public Node_SO GetNodeFromName(string techName)
		{
			foreach (Node_SO node in _unlockedNodes.Keys)
			{
				if (techName == node.TechName)
					return node;
			}

			return null;
		}

        /// <summary>
        /// Sets the unlocked nodes.
        /// </summary>
        /// <param name="unlockedNodes">The list of unlocked node states.</param>
		public void SetUnlockedNodes(List<bool> unlockedNodes)
		{
			List<string> unlockedNodeIds = new List<string>();
			int i = 0;
			foreach (Node_SO node in _unlockedNodes.Keys)
			{
				if (unlockedNodes != null && i < unlockedNodes.Count && unlockedNodes[i])
					unlockedNodeIds.Add(node.TechName);
				i++;
			}

			RestoreUnlockedNodeIds(unlockedNodeIds);
		}

        /// <summary>
        /// Initializes the tech tree data.
        /// </summary>
		private void InitializeData()
		{
			Debug.Log("TechTree InitializeData: Starting initialization");
			Debug.Log($"TechTree InitializeData: _metaData is null: {_metaData == null}");
			if (_metaData != null)
				Debug.Log($"TechTree InitializeData: LoadType: {_metaData.LoadType}");

			List<Node_SO> allNodes = new List<Node_SO>(_tree.UngroupedNodes);

			foreach (List<Node_SO> group in _tree.NodeGroups.Values)
				allNodes.AddRange(group);

			Debug.Log($"TechTree InitializeData: Total nodes: {allNodes.Count}");

			foreach (Node_SO node in allNodes)
			{
				_unlockedNodes.Add(node, node.IsUnlocked);
				if (node.IsUnlocked)
					Debug.Log($"TechTree InitializeData: Node marked as unlocked in asset: {node.TechName}");
			}

			//TODO: This should probably be done in the editor tool...
			ConnectParents(ref allNodes);
			SetRootNode(ref allNodes);
			Debug.Log($"TechTree InitializeData: Root node: {_rootNode?.TechName ?? "NULL"}");
			Debug.Log($"TechTree InitializeData: Root node IsUnlocked: {_rootNode?.IsUnlocked}");

			if (_metaData != null && _metaData.LoadType == MetaData.LoadType.Generate)
			{
				Debug.Log("TechTree InitializeData: LoadType is Generate, force-unlocking root node");
				ForceUnlockNode(_rootNode);
				RecursivelyAddAvailableNodes(_rootNode);
			}
			else if (_metaData == null)
			{
				Debug.Log("TechTree InitializeData: _metaData is null, defaulting to Generate behavior");
				// Default to generating if no metadata is present
				ForceUnlockNode(_rootNode);
				RecursivelyAddAvailableNodes(_rootNode);
			}
			else
			{
				Debug.Log($"TechTree InitializeData: LoadType is {_metaData.LoadType}, skipping root node force-unlock");
			}

			// Always apply unlock effects for nodes marked as unlocked in the asset
			// These represent baseline unlocks that should be available regardless of load type
			int preUnlockedCount = 0;
			foreach (Node_SO node in allNodes)
			{
				if (node.IsUnlocked)
				{
					Debug.Log($"Force-unlocking pre-unlocked node: {node.TechName}");
					ForceUnlockNode(node);
					preUnlockedCount++;
				}
			}
			Debug.Log($"TechTree InitializeData: Force-unlocked {preUnlockedCount} pre-unlocked nodes");

			Debug.Log("TechTree InitializeData: Initialization complete");
		}

        /// <summary>
        /// Connects parent nodes to their children.
        /// </summary>
        /// <param name="allNodes">The list of all nodes.</param>
		private void ConnectParents(ref List<Node_SO> allNodes)
		{
			for (int i = 0; i < allNodes.Count; i++)
			{
				if (allNodes[i].Children != null)
					for (int j = 0; j < allNodes[i].Children.Count; j++)
					{
						if (allNodes[i].Children[j].NextTech != null)
							allNodes[i].Children[j].NextTech.Parent = allNodes[i];
					}
			}
		}

		/// <summary>
		/// Attempts to find the root node.
		/// </summary>
		/// <param name="allNodes">The list of all nodes.</param>
		private void SetRootNode(ref List<Node_SO> allNodes)
		{
			for (int i = 0; i < allNodes.Count; i++)
			{
				if (allNodes[i].Parent == null && allNodes[i].Children.Count > 0 && !allNodes[i].Unavailable)
				{
					_rootNode = allNodes[i];
					return;
				}
			}
		}

        /// <summary>
        /// Recursively adds available nodes.
        /// </summary>
        /// <param name="node">The node to process.</param>
		private void RecursivelyAddAvailableNodes(Node_SO node)
		{
			if (!IsUnlocked(node) && !_availableNodes.Contains(node) && !node.Unavailable)
			{
				Debug.Log($"{node} added to available list.");
				_availableNodes.Add(node);
			}

			for (int i = 0; i < node.Children.Count; i++)
			{
				if (node.Children[i].NextTech != null)
				{
					var child = node.Children[i].NextTech;

					if (IsUnlocked(child))
					{
						ForceUnlockNode(child);
					}
					else if (!_availableNodes.Contains(child))
						_availableNodes.Add(child);
				}
			}
		}
	}
}
