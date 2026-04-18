using MetaData;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores runtime main menu state for the game.
	/// Manages loading state, load type, and Twitch channel name.
	/// </summary>
	public class MainMenuRuntimeData : ScriptableObject, IRuntimeDataScriptable
	{
		/// <summary>
		/// Whether the game is currently loading.
		/// Set to true when loading begins, false when loading completes.
		/// </summary>
		[SerializeField]
		private bool _loading;

		/// <summary>
		/// The type of load operation to perform.
		/// Determines whether to generate a new world or load an existing save.
		/// </summary>
		[SerializeField]
		private LoadType _loadType;

		/// <summary>
		/// The Twitch channel name to connect to.
		/// Used for Twitch integration to connect to the correct stream.
		/// </summary>
		[SerializeField]
		private string _channelName;

		/// <summary>
		/// Action to display code on the UI.
		/// </summary>
		public System.Action<string> CodeDisplay;

		/// <summary>
		/// The connect panel GameObject for the main menu UI.
		/// </summary>
		[SerializeField]
		private GameObject _connectPanel;

		/// <summary>
		/// Gets or sets the connect panel GameObject.
		/// </summary>
		public GameObject ConnectPanel
		{
			get => _connectPanel;
			set => _connectPanel = value;
		}

		/// <summary>
		/// Gets or sets whether the game is loading.
		/// </summary>
		public bool Loading
		{
			get => _loading;
			set => _loading = value;
		}

		/// <summary>
		/// Gets or sets the load type.
		/// </summary>
		public LoadType LoadType
		{
			get => _loadType;
			set => _loadType = value;
		}

		/// <summary>
		/// Gets or sets the Twitch channel name.
		/// </summary>
		public string ChannelName
		{
			get => _channelName;
			set => _channelName = value;
		}

		/// <summary>
		/// Initializes the main menu runtime data with default values.
		/// </summary>
		public void Initialize()
		{
			// Initialize with default values if needed
		}
	}
}
