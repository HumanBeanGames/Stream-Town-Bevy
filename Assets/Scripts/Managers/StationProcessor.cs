using Buildings;
using System;
using System.Collections.Generic;
using UnityEngine;
using UserInterface;
using Utils;
using Reflex.Core;
using Reflex.Attributes;
using ScriptablesProcessorInfrastructure;
using Data.Containers;

namespace Processors
{
	/// <summary>
	/// Processor that manages station system for the game.
	/// Handles station registration, updates, and disabled station clearing.
	/// </summary>
	public partial class StationProcessor : MonoBehaviour, IInstaller, IProcessor
	{
        /// <summary>
        /// Runtime data for station data.
        /// Assigned in InjectRuntimeData.
        /// </summary>
        private StationRuntimeData _stationRuntimeData;

        /// <summary>
        /// Object pooling processor for accessing pooled objects.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingProcessor _poolingProcessor;
        /// <summary>
        /// ScriptableObject containing object pooling settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private ObjectPoolingSettings _poolingSettings;

		/// <summary>
		/// Adds the station to the update queue so that it's targets will be updated.
		/// </summary>
		/// <param name="station">The station to update.</param>
		public void UpdateStation(Station station)
		{
			if (station == null)
				return;

			if (_stationRuntimeData.StationUpdateQueue.Contains(station))
				return;

			_stationRuntimeData.StationUpdateQueue.Enqueue(station);
		}

		/// <summary>
		/// Adds a station to the station dictionary based on its flags.
		/// </summary>
		/// <param name="station">The station to add.</param>
		public void AddStation(Station station)
		{
			foreach (int i in Enum.GetValues(typeof(StationMask)))
			{
				StationMask t = (StationMask)i;

				if (t == StationMask.Nothing)
					continue;

				if (station.Flags.HasFlag(t))
				{
					AddStation(t, station);
				}
			}
		}

		/// <summary>
		/// Removes station from station dictionary based on its flags.
		/// </summary>
		/// <param name="station">The station to remove.</param>
		public void RemoveStation(Station station)
		{
			foreach (int i in Enum.GetValues(typeof(StationMask)))
			{
				StationMask t = (StationMask)i;

				if (t == StationMask.Nothing)
					continue;

				if (station.Flags.HasFlag(t))
				{
					RemoveStation(t, station);
				}
			}
		}

		/// <summary>
		/// Gets a list of stations matching the specified flags.
		/// </summary>
		/// <param name="flag">The station flags to filter by.</param>
		/// <returns>List of stations matching the flags.</returns>
		public List<Station> GetStationsByFlag(StationMask flag)
		{
			List<Station> stations = new List<Station>();

			foreach (int i in Enum.GetValues(typeof(StationMask)))
			{
				StationMask t = (StationMask)i;

				if (t == StationMask.Nothing)
					continue;

				if (!flag.HasFlag(t) || !_stationRuntimeData.StationsDictionary.ContainsKey(t))
					continue;

				stations.AddRange(_stationRuntimeData.StationsDictionary[t]);
			}

			return stations;
		}

		/// <summary>
		/// Displays numbered IDs above stations matching the specified flags.
		/// </summary>
		/// <param name="flags">The station flags to display IDs for.</param>
		/// <returns>True if stations were found and IDs displayed.</returns>
		public bool DisplayStationIdByType(StationMask flags)
		{
			List<Station> _validStations = GetStationsByFlag(flags);

			if (_validStations.Count == 0)
			{
				return false;
			}

			for (int i = 0; i < _validStations.Count; i++)
			{
				var textDisplay = _poolingProcessor.GetPooledObject("TextDisplay");
				textDisplay.gameObject.SetActive(true);
				var rectTransform = textDisplay.GetComponent<RectTransform>();
				rectTransform.SetParent(_validStations[i].Targetable.TextDisplayTransform, false);
				rectTransform.localPosition = _validStations[i].Targetable.TextDisplayTransform.localPosition;

				var display = textDisplay.GetComponent<UnitTextDisplay>();
				display.Targetable = _validStations[i].Targetable;
				display.SetDisplayText($"{i + 1}");
				display.SetDisplayTextAfterTime("", 15.0f);
			}

			return true;
		}

		/// <summary>
		/// Gets a station by flags and index.
		/// </summary>
		/// <param name="flags">The station flags to filter by.</param>
		/// <param name="index">The index of the station in the filtered list.</param>
		/// <returns>The station at the specified index, or null if not found.</returns>
		public Station GetStationByFlaggedIndex(StationMask flags, int index)
		{
			List<Station> _validStations = GetStationsByFlag(flags);

			if (_validStations.Count <= index)
				return null;

			return _validStations[index];
		}

		/// <summary>
		/// Updates station dictionaries and checks disabled targets.
		/// Called every frame by the Coordinator.
		/// </summary>
		public void Process()
		{
			if (_stationRuntimeData.StationUpdateQueue.Count > 0)
			{
				Station station = _stationRuntimeData.StationUpdateQueue.Dequeue();
				if (station != null && station.isActiveAndEnabled)
					station.PopulateDictionary();
			}

			if (_stationRuntimeData.ClearDisabledQueue.Count > 0)
			{
				var station = _stationRuntimeData.ClearDisabledQueue.Dequeue();

				if (station != null && station.gameObject.activeInHierarchy)
				{
					_stationRuntimeData.ClearDisabledQueue.Enqueue(station);
					station.CheckDisabledTargets();
				}
			}
		}

		/// <summary>
		/// Refreshes scene-specific data when a new scene loads.
		/// Called by the Coordinator after scene container is available.
		/// </summary>
		public void RefreshSceneData(Container sceneContainer)
		{
			// StationProcessor does not have scene-specific settings to refresh
		}

		/// <summary>
		/// Initializes the station processor.
		/// No initialization logic required.
		/// </summary>
		public void Initialize()
		{
			if (_stationRuntimeData == null)
				throw new InvalidOperationException("StationProcessor: StationRuntimeData has not been installed.");
		}

		/// <summary>
		/// Registers this processor as a singleton in the dependency injection container.
		/// Called by Reflex during container initialization.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
			InjectRuntimeData(containerBuilder);
		}

		/// <summary>
		/// Injects the StationRuntimeData ScriptableObject into the DI container.
		/// </summary>
		/// <param name="containerBuilder">The container builder to register bindings with.</param>
		public void InjectRuntimeData(ContainerBuilder containerBuilder)
		{
			if (_stationRuntimeData != null)
				throw new InvalidOperationException("StationProcessor: StationRuntimeData has already been installed.");

			_stationRuntimeData = new StationRuntimeData();
			containerBuilder.AddSingleton(_stationRuntimeData);
		}

		/// <summary>
		/// Adds a station to the dictionary under a specific mask.
		/// </summary>
		/// <param name="mask">The mask to add the station under.</param>
		/// <param name="station">The station to add.</param>
		private void AddStation(StationMask mask, Station station)
		{
			if (!_stationRuntimeData.StationsDictionary.ContainsKey(mask))
				_stationRuntimeData.StationsDictionary[mask] = new List<Station>();

			if (_stationRuntimeData.StationsDictionary[mask].Contains(station))
				return;
			_stationRuntimeData.StationsDictionary[mask].Add(station);
			_stationRuntimeData.ClearDisabledQueue.Enqueue(station);
		}

		/// <summary>
		/// Removes a station from the dictionary under a specific mask.
		/// </summary>
		/// <param name="mask">The mask to remove the station from.</param>
		/// <param name="station">The station to remove.</param>
		private void RemoveStation(StationMask mask, Station station)
		{
			if (!_stationRuntimeData.StationsDictionary.ContainsKey(mask))
				return;

			if (!_stationRuntimeData.StationsDictionary[mask].Contains(station))
				return;

			_stationRuntimeData.StationsDictionary[mask].Remove(station);
		}
	}
}
