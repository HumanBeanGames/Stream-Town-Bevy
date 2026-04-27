using UnityEngine;
using UnityEngine.UI;
using TMPro;

namespace UserInterface.Menus
{
	public class StatusBar : MonoBehaviour
	{
		[SerializeField]
		private Image _progressFillImage;

		[SerializeField]
		private TextMeshProUGUI _progressPercentText;

		[SerializeField]
		private TextMeshProUGUI _statusText;

		public void SetProgress(float progress01, string status)
		{
			if (_progressFillImage != null)
				_progressFillImage.fillAmount = Mathf.Clamp01(progress01);

			if (_progressPercentText != null)
				_progressPercentText.text = $"{Mathf.RoundToInt(Mathf.Clamp01(progress01) * 100f)}%";

			if (_statusText != null)
				_statusText.text = status;
		}
	}
}
