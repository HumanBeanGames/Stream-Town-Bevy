using Buildings;
using Character;
using Processors;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;
using Utils;
using Reflex.Attributes;

namespace Sensors
{
	/// <summary>
	/// A sensor that finds appropriate stations for units.
	/// </summary>
	public class StationSensor : SensorBase
	{
        /// <summary>
        /// The station mask.
        /// </summary>
		[SerializeField]
		private StationMask _stationMask;

        /// <summary>
        /// The current station.
        /// </summary>
		[SerializeField]
		private Station _currentStation;

        /// <summary>
        /// The previous station.
        /// </summary>
		private Station _previousStation;

        /// <summary>
        /// The player.
        /// </summary>
		private Player _player;

        /// <summary>
        /// Event invoked when the station changes.
        /// </summary>
		[SerializeField]
		private UnityEvent _onStationChange;

        /// <summary>
        /// The station processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private StationProcessor _stationProcessor;

        /// <summary>
        /// The debug processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private Processors.DebugProcessor _debugProcessor;

        /// <summary>
        /// Gets the current station.
        /// </summary>
		public Station CurrentStation => _currentStation;

        /// <summary>
        /// Gets whether the sensor has a station.
        /// </summary>
		public bool HasStation => _currentStation == null ? false : true;

        /// <summary>
        /// Gets or sets whether to update the station.
        /// </summary>
		public bool UpdateStation { get; set; }

        /// <summary>
        /// Gets the distance to the current station.
        /// </summary>
		public float DistanceToStation => _currentStation == null ? float.MaxValue : Vector3.Distance(transform.position, _currentStation.transform.position);

        /// <summary>
        /// Gets or sets the station mask.
        /// </summary>
		public StationMask StationMask
		{
			get { return _stationMask; }
			set { OnSetStationMask(value); }
		}

        /// <summary>
        /// Gets or sets the player.
        /// </summary>
		public Player Player
		{
			get { return _player; }
			set { _player = value; }
		}

        /// <summary>
        /// Called every frame by the sensor processor.
        /// </summary>
		public override void STUpdate()
		{
			base.STUpdate();

			if (UpdateStation || _currentStation == null)
				GetNearestStation();
		}

		/// <summary>
		/// Attemmpts to set the current station.
		/// </summary>
		/// <param name="station">The station to set.</param>
		/// <returns>True if successful.</returns>
		public bool TrySetStation(Station station)
		{
			_currentStation = station;
			return true;
		}

        /// <summary>
        /// Attempts to set the current station for a player.
        /// </summary>
        /// <param name="station">The station to set.</param>
        /// <param name="player">The player.</param>
        /// <returns>True if successful.</returns>
		public bool TrySetStation(Station station, Player player)
		{
			if (station.Flags.HasFlag(player.StationSensor.StationMask))
			{
				_currentStation = station;
				_debugProcessor.Log(DebugLogCategory.Targetable, $"Set {player.RoleHandler.CurrentRole}'s station to {station.gameObject.name}");
				_onStationChange.Invoke();
				return true;
			}

			_debugProcessor.Log(DebugLogCategory.Targetable, $"Can't set {player.RoleHandler.CurrentRole}'s station to {station.gameObject.name}");
			return false;
		}

		/// <summary>
		/// Forces the current station to update to the nearest available station.
		/// </summary>
		public void ForceUpdateStation()
		{
			GetNearestStation();
		}

		/// <summary>
		/// Sets current station to the nearest station.
		/// </summary>
		private void GetNearestStation()
		{
			if (!_stationProcessor)
				return;

			//TODO: Implement BSP, also doesnt need to be called so often
			List<Station> stations = _stationProcessor.GetStationsByFlag(_stationMask);

			if (stations == null || stations.Count == 0)
			{
				_currentStation = null;
			}
			else
			{
				Station closest = null;
				float closestDistanceSqrd = float.MaxValue;

				for (int i = 0; i < stations.Count; i++)
				{
					float distanceSqrd = Vector3.SqrMagnitude(stations[i].transform.position - transform.position);

					if (distanceSqrd < closestDistanceSqrd)
					{
						closest = stations[i];
						closestDistanceSqrd = distanceSqrd;
					}
				}

				_currentStation = closest;
			}

			if (_currentStation != _previousStation)
			{
				_onStationChange.Invoke();
				_previousStation = _currentStation;
			}
		}

		/// <summary>
		/// Called when station mask has been set.
		/// </summary>
		/// <param name="flags">The station mask flags.</param>
		private void OnSetStationMask(StationMask flags)
		{
			_stationMask = flags;

			_currentStation = null;
		}

		// Unity Functions.
        /// <summary>
        /// Enables station updates on start.
        /// </summary>
		private void Start()
		{
			UpdateStation = true;
		}

        /// <summary>
        /// Clears the current station on disable.
        /// </summary>
		private void OnDisable()
		{
			_currentStation = null;
		}
	}
}
