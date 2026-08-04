using TMPro;
using UnityEngine;

namespace UserInterface
{
	/// <summary>
	/// Scene view for the broadcaster connection challenge.
	/// </summary>
	public sealed class TwitchConnectionPanelView : MonoBehaviour
	{
		[SerializeField] private TMP_Text _commandText;

		public bool IsVisible => gameObject.activeInHierarchy;
		public string DisplayedCommand => _commandText != null ? _commandText.text : string.Empty;

		public void Show(string connectionCode)
		{
			if (_commandText == null)
			{
				Debug.LogError("Twitch connection panel is missing its command text reference.", this);
				return;
			}

			_commandText.text = $"!connect {connectionCode}";
			gameObject.SetActive(true);
		}

		public void Hide()
		{
			gameObject.SetActive(false);
		}
	}
}
