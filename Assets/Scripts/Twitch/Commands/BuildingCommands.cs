using Buildings;
using Character;
using Processors;
using Twitch.Utils;
using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using Utils;
using Reflex.Attributes;

namespace Twitch.Commands
{
	/// <summary>
	/// Handles all Twitch Chat Commands related to Building.
	/// </summary>
	public class BuildingCommands
	{
        private BuildingProcessor _buildingProcessor;
        private Processors.TwitchChatProcessor _twitchChatProcessor;

        public BuildingCommands(BuildingProcessor buildingProcessor, Processors.TwitchChatProcessor twitchChatProcessor)
        {
            _buildingProcessor = buildingProcessor;
            _twitchChatProcessor = twitchChatProcessor;
        }

		/// <summary>
		/// Returns the type of Building from the given argument.
		/// </summary>
		/// <param name="arg">The argument string.</param>
		/// <returns>The building type string.</returns>
		public string GetBuildingTypeFromArg(string arg)
		{
			return char.ToUpper(arg[0]) + arg.Substring(1);
		}

		/// <summary>
		/// Attempts to start a building with type given in the argument.
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="command">The command.</param>
		/// <param name="args">The arguments.</param>
		public void StartBuild(Player player, string command, params string[] args)
		{
			string buildType = GetBuildingTypeFromArg(args[0]);

			if (Enum.TryParse(buildType, out BuildingType type))
			{
				if (!_buildingProcessor.TryStartNewBuildingPlacer(player, type, out string errorMessage))
					_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} : Failed - {errorMessage}");
			}
		}

		/// <summary>
		/// Moves and Rotates the Building Placer by the specified amount.
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="command">The command.</param>
		/// <param name="args">The arguments.</param>
		public void AdjustBuildingPlacer(Player player, string command, params string[] args)
		{
			Vector3 moveVector = Vector3.zero;
			int rotationAmount = 0;

			// Append Command as an argument
			string[] argsAppended = new string[args.Length + 1];
			argsAppended[0] = command;
			args.CopyTo(argsAppended, 1);


			for (int i = 0; i < argsAppended.Length; i += 2)
			{
				int value = 0;

				if (!((i + 1) < argsAppended.Length && int.TryParse(argsAppended[i + 1], out value)))
					value = 1;

				switch (argsAppended[i])
				{
					case "up":
						moveVector += Vector3.right * value;
						break;
					case "down":
						moveVector += Vector3.left * value;
						break;
					case "left":
						moveVector += Vector3.forward * value;
						break;
					case "right":
						moveVector += Vector3.back * value;
						break;
					case "rotate":
						rotationAmount += value;
						break;
				}

			}

			_buildingProcessor.TryMoveBuilding(player, moveVector);
			_buildingProcessor.TryRotateBuilding(player, rotationAmount);
		}

		/// <summary>
		/// Confirms the placement of the building and returns a message if it failed.
		/// </summary>
		/// <param name="player"></param>
		public void ConfirmBuildingPlacement(Player player)
		{
			if (!_buildingProcessor.TryPlaceBuilding(player, out string errorMessage))
			{
				_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} Failed - {errorMessage}");
			}
		}

		/// <summary>
		/// Cancels the placement of a building.
		/// </summary>
		/// <param name="player">The player.</param>
		public void CancelBuildingPlacement(Player player)
		{
			_buildingProcessor.TryCancelBuilding(player);
		}

		/// <summary>
		/// Attempts to level up all buildings of a given type by the given amount;
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="command">The command.</param>
		/// <param name="args">The arguments.</param>
		public void LevelBuilding(Player player, BuildingType type, int id, int iterations)
		{

			string errorMessage = "";
			int successfulLevels = 0;
			for (int i = 0; i < iterations; i++)
				if (!_buildingProcessor.TryLevelBuilding(type, id - 1, out errorMessage))
					break;
				else
					successfulLevels++;

			if (successfulLevels > 0)
				_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} Successfully Leveled Building {successfulLevels} {(successfulLevels > 1 ? "Times" : "Time")}");
			else
				_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} Failed - {errorMessage}");
		}


		/// <summary>
		/// Levels all buildings of a specified type to a target level.
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="command">The command.</param>
		/// <param name="args">The arguments.</param>
		public void LevelAllOfType(Player player, string command, params string[] args)
		{
			if (args.Length < 2)
				return;

			string buildType = GetBuildingTypeFromArg(args[0]);
			int successfulAttempts = 0;
			if (Enum.TryParse(buildType, out BuildingType type) && int.TryParse(args[1], out int levelTo))
			{
				var buildingsOfType = _buildingProcessor.GetBuildingsByType(type);

				if (buildingsOfType.Count <= 0)
					return;

				buildingsOfType = buildingsOfType.OrderByDescending(x => x.LevelHandler.Level).ToList();

				for (int i = 0; i < levelTo; i++)
				{
					bool successfulLevel = false;
					for (int j = buildingsOfType.Count - 1; j >= 0; j--)
					{
						if (buildingsOfType[j].LevelHandler.Level >= levelTo)
							continue;
						if (!_buildingProcessor.TryLevelBuilding(buildingsOfType[j], out string errorMessage))
							continue;
						successfulAttempts++;
						successfulLevel = true;
					}

					if (!successfulLevel)
						break;

				}
			}

			if (successfulAttempts > 0)
				_twitchChatProcessor.SendPlayerMessage(player, $"Successfully leveled {successfulAttempts} times!");
			else
				_twitchChatProcessor.SendPlayerMessage(player, $"Failed to level buildings");
		}

        /// <summary>
        /// Gets building information including cost and max level.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments.</param>
		public void GetBuildingInformation(Player player, string command, params string[] args)
		{
			string buildingTypeString = GetBuildingTypeFromArg(args[0]);

			if (!Enum.TryParse(buildingTypeString, out BuildingType type))
				return;

			BuildingProcessor processor = _buildingProcessor;
			var costSummary = processor.GetBuildingCostSummary(type);

			string message = $"Building Cost for '{type}': Wood: {costSummary.woodCost} | Ore: {costSummary.oreCost} | Food: {costSummary.foodCost} | Gold: {costSummary.goldCost} | Max Level: {costSummary.maxLevel}";
			_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username}: {message}");
		}

		/// <summary>
		/// Attempts to remove a building based on the given arguments.
		/// </summary>
		/// <param name="player"></param>
		/// <param name="command"></param>
		/// <param name="args"></param>
		public void RemoveBuilding(Player player, string command, params string[] args)
		{
			if (args.Length < 2)
				return;

			string buildType = GetBuildingTypeFromArg(args[0]);
			if (Enum.TryParse(buildType, out BuildingType type) && int.TryParse(args[1], out int index))
			{
				if (_buildingProcessor.TryRemoveBuilding(type, index - 1, out string errorMessage))
				{
					_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} Successfully Removed Building");
				}
			}
		}

		/// <summary>
		/// Displays the ID of all buildings of the given type.
		/// </summary>
		/// <param name="player">The player.</param>
		/// <param name="command">The command.</param>
		/// <param name="args">The arguments.</param>
		public void ShowBuildingIDsByType(Player player, string command, params string[] args)
		{
			string buildType = GetBuildingTypeFromArg(args[0]);

			if (Enum.TryParse(buildType, out BuildingType type))
			{
				_buildingProcessor.DisplayBuildingIdsOfType(type);
			}
		}

		/// <summary>
		/// Displays the currently unlocked buildings.
		/// </summary>
		/// <param name="player"></param>
		public void PrintUnlockedBuildings(Player player)
		{
			string buildingList = "Unlocked Buildings: ";
			bool hasBuildings = false;

			for (int i = 0; i < (int)BuildingType.Count; i++)
			{
				BuildingType type = (BuildingType)i;

				if (_buildingProcessor.BuildingsUnlocked.ContainsKey(type) && _buildingProcessor.BuildingsUnlocked[type] && type != BuildingType.Townhall)
				{
					hasBuildings = true;
					buildingList += $"{type}, ";
				}
			}

			if (hasBuildings)
			{
				buildingList = buildingList.Remove(buildingList.Length - 2, 2);
			}
			else
				buildingList += "None";

			_twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} {buildingList}");
		}
	}
}
