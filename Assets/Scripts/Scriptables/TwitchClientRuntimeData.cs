using TwitchLib.Unity;

namespace Processors
{
	/// <summary>
	/// Runtime data for Twitch client connection.
	/// Manages the Twitch client instance, connection state, and channel information.
	/// </summary>
	public class TwitchClientRuntimeData
	{
		/// <summary>
		/// The Twitch client instance.
		/// </summary>
		public Client Client { get; set; }

		/// <summary>
		/// The channel name.
		/// </summary>
		public string ChannelName { get; set; } = "";

		/// <summary>
		/// Whether the client is connecting.
		/// </summary>
		public bool IsConnecting { get; set; } = false;

		/// <summary>
		/// Whether the send ping loop is running.
		/// </summary>
		public bool SendPingRunning { get; set; } = false;

		public TwitchClientRuntimeData()
		{
			Client = null;
			ChannelName = "";
			IsConnecting = false;
			SendPingRunning = false;
		}
	}
}
