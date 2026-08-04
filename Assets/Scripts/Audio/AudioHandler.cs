using Processors;
using Reflex.Attributes;
using UnityEngine;

namespace Audio
{
	public class AudioHandler : MonoBehaviour
	{
		//[SerializeField]
		//private bool _disableWhenInvisible = true;
		[SerializeField]
		private bool _disableOnDistance = true;
		[SerializeField]
		private float _disableDistanceFromCamera = 20.0f;
		private float _disableDistanceSqr;
		[SerializeField]
		private bool _3DSound = true;

		[SerializeField]
		private AudioSource _audioSource;

		[Inject] private AudioSourcesProcessor _audioSourcesProcessor;

		private Camera _camera;

		public bool Tracked { get; set; }
		//private bool _isVisible = false;

		public void PlayClip(AudioClip clip) => _audioSource.PlayOneShot(clip);

		public void UpdateLogic()
		{
			_audioSource.enabled = !DisableByDistanceToCameraCheck();
		}

		private bool DisableByDistanceToCameraCheck()
		{
			if (!_disableOnDistance)
				return false;

			return Vector3.SqrMagnitude(transform.position - _camera.transform.position) >= _disableDistanceSqr ? true : false;
		}

		private void Awake()
		{
			if (_audioSource == null)
			{
				if (!TryGetComponent(out _audioSource))
					_audioSource = gameObject.AddComponent<AudioSource>();
			}

			_camera = Camera.main;
			_disableDistanceSqr = _disableDistanceFromCamera * _disableDistanceFromCamera;

			_audioSource.spatialBlend = _3DSound ? 1 : 0;
			Initialize();
		}

		protected virtual void Initialize() { }

#if UNITY_EDITOR
		private void OnValidate()
		{
			_disableDistanceSqr = _disableDistanceFromCamera * _disableDistanceFromCamera;
		}
#endif
		private void OnEnable()
		{
			if (!Tracked && _audioSourcesProcessor != null)
				_audioSourcesProcessor.AddSourceToQueue(this);
		}

		private void OnDisable()
		{
			_audioSourcesProcessor?.RemoveSourceFromQueue(this);
		}
	}
}
