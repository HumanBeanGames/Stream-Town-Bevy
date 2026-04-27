using UnityEngine;
using Processors;
using TMPro;
using Utils;
using Reflex.Attributes;

namespace UserInterface
{
	/// <summary>
	/// Handles the User Interface for the Town Resources.
	/// </summary>
	public class UserInterface_Resources : MonoBehaviour
	{
		[SerializeField]
		private GameObject _resourcePanel;
		[SerializeField]
		private TextMeshProUGUI[] _resourceTMPs;
		[SerializeField]
		private Color _positiveColor = Color.green;
		[SerializeField]
		private Color _negativeColor = Color.red;
		[Inject] private TownResourceProcessor _resourceProcessor;
		[Inject] private UIProcessor _uiProcessor;

		/// <summary>
		/// Called when a resource amount has changed and updates the text accordingly.
		/// </summary>
		/// <param name="resource"></param>
		/// <param name="amount"></param>
		private void OnResourceChange(Resource resource, int amount, bool purchase)
		{
			return;

			//int rateOfChange = _resourceProcessor.RateOfChangeForResource(resource);
			//bool positiveROC = rateOfChange >= 0;

			//Color toUse = positiveROC ? _positiveColor : _negativeColor;
			//string rateOfChangeString = $"<color=#{ColorUtility.ToHtmlStringRGBA(toUse)}>" + (positiveROC ? " + " : "-") + $"{StringUtils.GetShortenedNumberAsString(rateOfChange * 60)}/h</color>";
			//if ((int)resource - 1 >= _resourceTMPs.Length)
			//	return;
			//_resourceTMPs[(int)resource - 1].text = $"{_resourceProcessor.ResourcePrint(resource)} {rateOfChangeString}";
		}

		private void Start()
		{
			RegisterHud();
			_resourceProcessor.OnAnyResourceChangeEvent += OnResourceChange;

			for (int i = 1; i < (int)Resource.Count-1; i++)
			{
				OnResourceChange((Utils.Resource)i, 0, false);
			}
		}

		private void RegisterHud()
		{
			if (_uiProcessor == null || _resourceTMPs == null || _resourceTMPs.Length < 4)
				return;

			_uiProcessor.RegisterResourceDisplay(_resourceTMPs[0], _resourceTMPs[1], _resourceTMPs[2], _resourceTMPs[3]);

			if (_resourceTMPs.Length >= 8)
			{
				_uiProcessor.RegisterResourceRateOfChangeDisplay(_resourceTMPs[4], _resourceTMPs[5], _resourceTMPs[6], _resourceTMPs[7]);
			}
		}
	}
}
