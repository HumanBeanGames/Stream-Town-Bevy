using Character;
using Processors;
using UnityEngine;
using Reflex.Attributes;

namespace VFX
{
	public class VFXArrowPointer : MonoBehaviour
	{
		[Inject] private UtilDisplayProcessor _utilDisplayProcessor;
		private Player _attachedPlayer;
		private ParticleSystem _particleSystem;
		private float _vfxLifetime = 0;
		private float _remainingLife = 0;

		public void SetPlayer(Player player)
		{
			_attachedPlayer = player;
		}

		private void Update()
		{
			_remainingLife -= Time.deltaTime;

			if (_remainingLife <= 0)
			{
				_remainingLife = _vfxLifetime;
				if (_attachedPlayer != null)
					_utilDisplayProcessor.RemovePingObject(_attachedPlayer);
				gameObject.SetActive(false);
			}
		}

		private void OnEnable()
		{
			if (_particleSystem == null)
			{
				_particleSystem = GetComponent<ParticleSystem>();
				_vfxLifetime = _particleSystem.main.duration;
			}

			_remainingLife = _vfxLifetime;
		}

		private void OnDisable()
		{
			if (_attachedPlayer != null)
				_utilDisplayProcessor.RemovePingObject(_attachedPlayer);
		}
	}
}
