using Character;
using Processors;
using UnityEngine;
using UnityEngine.Events;
using Reflex.Attributes;

namespace Utils
{
	public class SimpleCancelBuildingPlacer : MonoBehaviour
	{
		[SerializeField]
		private float _timeInSeconds = 300;

		private float _timer = 0;

		private Player _player;
		[Inject] private BuildingProcessor _buildingProcessor;

		public void SetPlayer(Player player)
		{
			_player = player;
		}

		public void ResetTimer()
		{
			_timer = _timeInSeconds;
		}


		private void Update()
		{
			_timer -= Time.deltaTime;

			if (_timer <= 0)
			{
				_buildingProcessor.TryCancelBuilding(_player);
				ResetTimer();
			}
		}

		private void OnEnable()
		{
			ResetTimer();
		}
	}
}
