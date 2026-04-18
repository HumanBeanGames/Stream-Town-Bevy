using DataStructures;
using Processors;
using System.Collections.Generic;
using UnityEngine;
using Utils;

namespace GameResources
{
	/// <summary>
	/// Used for calculating the rate of change of a resource.
	/// </summary>
	public class ResourceRateOfChange
	{
        /// <summary>
        /// Queue of change timestamps.
        /// </summary>
		private Queue<ChangeTimeStamp> _timestampData = new Queue<ChangeTimeStamp>();

        /// <summary>
        /// The average over time.
        /// </summary>
		private int _averageOverTime;

        /// <summary>
        /// The time period for averaging.
        /// </summary>
		private float _timePeriod;

        /// <summary>
        /// The update rate.
        /// </summary>
		private float _updateRate;

        /// <summary>
        /// The update timer.
        /// </summary>
		private float _updateTimer;

        /// <summary>
        /// The resource type.
        /// </summary>
		private Utils.Resource _resourceType;

        /// <summary>
        /// The change during the period.
        /// </summary>
		private int _changeDuringPeriod;

        /// <summary>
        /// Gets the average over time.
        /// </summary>
		public int AverageOverTime => _averageOverTime;

		// Constructor.
        /// <summary>
        /// Initializes a new resource rate of change instance.
        /// </summary>
        /// <param name="resourceType">The resource type.</param>
        /// <param name="timePeriod">The time period for averaging.</param>
        /// <param name="updateRate">The update rate.</param>
        /// <param name="townResourceProcessor">The town resource processor.</param>
		public ResourceRateOfChange(Utils.Resource resourceType, float timePeriod, float updateRate, TownResourceProcessor townResourceProcessor)
		{
			_timePeriod = timePeriod;
			_updateRate = updateRate;
			_resourceType = resourceType;
			// Subscribe to resource change event.
			townResourceProcessor.OnAnyResourceChangeEvent += OnResourceChange;
		}

        /// <summary>
        /// Processes a queue and calculates rate of change for town resources.
        /// </summary>
		public void ProcessQueue()
		{
			_updateTimer += Time.deltaTime;

			// Return if not enough time has elapsed.
			if (_updateTimer < _updateRate)
				return;

			_updateTimer -= _updateRate;

			// Add change to queue and reset changeDuringPeriod.
			// Get the current time.
			System.DateTime now = System.DateTime.UtcNow;
			_timestampData.Enqueue(new ChangeTimeStamp(now, _changeDuringPeriod));
			_changeDuringPeriod = 0;

			List<float> medianAmounts = new List<float>();

			// Loop through all timestamps and calculate the rate of change.
			for (int i = _timestampData.Count - 1; i >= 0; i--)
			{
				ChangeTimeStamp cts = _timestampData.Dequeue();
				double timeDifference = (now - cts.TimeStamp).TotalSeconds;

				if (timeDifference < _timePeriod)
				{
					medianAmounts.Add(cts.Change);
					_timestampData.Enqueue(cts);
				}
			}

			if (medianAmounts.Count < 10)
				return;

			List<float> plottedPoints = new List<float>();
			for (int i = 2; i < medianAmounts.Count - 3; i++)
			{
				plottedPoints.Add((medianAmounts[i - 2] + medianAmounts[i - 1] + medianAmounts[i] + medianAmounts[i + 1] + medianAmounts[i + 2]) / 5.0f);
			}

			float movingMean = 0;

			for (int i = 0; i < plottedPoints.Count; i++)
			{
				movingMean += plottedPoints[i];
			}

			movingMean /= plottedPoints.Count;

			_averageOverTime = (int)(movingMean * 60 * 60);
		}

        /// <summary>
        /// Called when a town resource value changes and enqueues the rate of change to be calculated.
        /// </summary>
        /// <param name="resource">The resource type.</param>
        /// <param name="amount">The amount changed.</param>
        /// <param name="purchase">Whether the change was due to a purchase.</param>
		private void OnResourceChange(Utils.Resource resource, int amount, bool purchase)
		{
			// If it was a purchase, we don't want to calculate the rate of change as it messes up the data.
			if (resource != _resourceType || purchase)
				return;

			_changeDuringPeriod += amount;
		}
	}
}
