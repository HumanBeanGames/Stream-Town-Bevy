using ScriptablesProcessorInfrastructure;
using Character;
using System;
using System.Collections.Generic;
using UnityEngine;

namespace Processors
{
	/// <summary>
	/// Runtime data for Twitch chat integration.
	/// Contains runtime state for message sending, Twitch client, command handlers, and game state data.
	/// </summary>
	public class TwitchChatRuntimeData : IRuntimeDataScriptable
	{
		/// <summary>
		/// Determines if messages are allowed to be sent or not.
		/// </summary>
		public bool MessagesAllowed = false;
		public Queue<TwitchCameraRequest> CameraRequests;

		/// <summary>
		/// Game state data that TwitchChatProcessor needs.
		/// </summary>
		public bool PlayerReady = false;
		public float CurrentWorldTime = 0f;
		public string BroadcasterConnectCode = string.Empty;
		public bool ConnectionPauseActive = false;
		public float TimeScaleBeforeConnectionPause = 1f;
		public string LastCommand = string.Empty;
		public string LastCommandUser = string.Empty;
		public string LastCommandResult = string.Empty;

		/// <summary>
		/// Player data access methods.
		/// </summary>
		public PlayerExistsByIDDelegate PlayerExistsByID;
		public PlayerExistsByNameDelegate PlayerExistsByName;
		public Func<int, Player> GetPlayer;

		public TwitchChatRuntimeData()
		{
			MessagesAllowed = false;
			CameraRequests = new Queue<TwitchCameraRequest>();
			PlayerReady = false;
			CurrentWorldTime = 0f;
			BroadcasterConnectCode = string.Empty;
			ConnectionPauseActive = false;
			TimeScaleBeforeConnectionPause = 1f;
			LastCommand = string.Empty;
			LastCommandUser = string.Empty;
			LastCommandResult = string.Empty;
		}
	}

	public struct TwitchCameraRequest
	{
		public Vector3 MoveVector;
		public int ZoomFactor;
		public bool Reset;

		public TwitchCameraRequest(Vector3 moveVector, int zoomFactor, bool reset)
		{
			MoveVector = moveVector;
			ZoomFactor = zoomFactor;
			Reset = reset;
		}
	}

	/// <summary>
	/// Delegate for checking if a player exists by ID with an output parameter for the index.
	/// </summary>
	public delegate bool PlayerExistsByIDDelegate(string userID, out int index);

	public delegate bool PlayerExistsByNameDelegate(string username, out int index);
}
