using Processors;
using System.Collections;
using UnityEngine;
using Utils;
using Reflex.Attributes;
using Environment;
using Data.Containers;

namespace Audio
{
	public class GlobalAudioController : MonoBehaviour
	{
		[SerializeField]
		private AudioSource _musicSource;
		[SerializeField]
		private AudioSource _ambienceSource;
		[SerializeField]
		private float _maxMusicVolume;
		[SerializeField]
		private float _maxAmbienceVolume;
		[SerializeField]
		private float _volumeChangeRate = 0.2f;

		[SerializeField]
		private SeasonAudioData[] _audioData;

		[SerializeField]
		private float _minTimeBetweenMusic = 600;
		[SerializeField]
		private float _maxTimeBetweenMusic = 900;
		[SerializeField]
		private float _missingClipRetryDelay = 30;
		[SerializeField]
		private float _fadeOutTime = 10;
		[SerializeField]
		private bool _playAmbienceDuringMusic = true;

		[Inject] private SeasonProcessor _seasonProcessor;
		[Inject] private DayAndNightProcessor _dayNightProcessor;

		private float _timeUntilMusicPlays = 30;

		private void OnSeasonChange(Season season)
		{
			StartNextTrack(season, true);
		}

		private void OnDayStarted() => StartCoroutine(DayTimeChangeRoutine(true));

		private void OnNightStarted() => StartCoroutine(DayTimeChangeRoutine(false));


		private IEnumerator DayTimeChangeRoutine(bool day)
		{
			yield return StopMusic(_seasonProcessor.GetCurrentSeason());
			StartNextTrack(_seasonProcessor.GetCurrentSeason(), day);
		}

		private void StartNextTrack(Season season, bool day)
		{
			if (_musicSource == null || _ambienceSource == null)
			{
				Debug.LogWarning("GlobalAudioController: AudioSources not assigned in Inspector!", this);
				return;
			}

			StopCoroutine("StartMusic");
			StartCoroutine(StartMusic(season, day));

			if (_playAmbienceDuringMusic)
			{
				SeasonAudioData data = GetDataBySeason(season);
				if (data != null && data.GetRandomDayAmbienceTrack(out AudioClip clip))
				{
					_ambienceSource.clip = clip;
					_ambienceSource.Play();
				}
				else
				{
					Debug.LogWarning($"GlobalAudioController: Could not find ambience clip for season {season}", this);
				}
			}
		}

		private IEnumerator StartMusic(Season season, bool day)
		{
			if (!_playAmbienceDuringMusic)
				StartCoroutine(VolumeToZero(_ambienceSource));
			SeasonAudioData data = GetDataBySeason(season);

			if (data == null)
			{
				Debug.LogWarning($"GlobalAudioController: SeasonAudioData is null for season {season}", this);
				yield break;
			}

			AudioClip musicClip = null;
			if (day)
			{
				if (!data.GetRandomDayMusicTrack(out musicClip))
					Debug.LogWarning($"Couldnt find day music clip for season '{data.Season}'", this);
			}
			else
			{
				if (!data.GetRandomNightMusicTrack(out musicClip))
					Debug.LogWarning($"Couldnt find night music clip for season '{data.Season}'", this);
			}

			if (musicClip == null)
			{
				Debug.LogWarning($"GlobalAudioController: No music clip available for {season}, day={day}", this);
				_timeUntilMusicPlays = Mathf.Max(5f, _missingClipRetryDelay);
				yield break;
			}

			_musicSource.clip = musicClip;
			_musicSource.Play();

			UpdateTimeUntilMusicPlays();

			StartCoroutine(VolumeToFull(_musicSource, true));

			if (_musicSource.clip != null)
				yield return new WaitForSeconds(_musicSource.clip.length - _fadeOutTime);
			else
				yield return new WaitForSeconds(5f); // Fallback duration

			StartCoroutine(VolumeToZero(_musicSource));

			if (!_playAmbienceDuringMusic)
			{
				AudioClip ambienceClip = null;

				if (day ? data.GetRandomDayAmbienceTrack(out ambienceClip) : data.GetRandomNightAmbienceTrack(out ambienceClip))
				{
					_ambienceSource.clip = ambienceClip;
					_ambienceSource.Play();
					StartCoroutine(VolumeToFull(_ambienceSource, false));
				}
				else
					Debug.LogWarning($"Couldn't find ambience clip for season '{data.Season}'", this);
			}
		}

		private IEnumerator StopMusic(Season season)
		{
			StopCoroutine("StartMusic");
			if (_musicSource == null)
			{
				Debug.LogWarning("GlobalAudioController: _musicSource is null! Assign AudioSource in Inspector.", this);
				yield break;
			}
			yield return StartCoroutine(VolumeToZero(_musicSource));
		}

		private void UpdateTimeUntilMusicPlays()
		{
			if (_musicSource == null || _musicSource.clip == null)
			{
				Debug.LogWarning("GlobalAudioController: _musicSource or its clip is null! Assign AudioSource in Inspector.", this);
				return;
			}
			_timeUntilMusicPlays += Random.Range(_minTimeBetweenMusic, _maxTimeBetweenMusic) + _musicSource.clip.length;
		}

		private IEnumerator VolumeToZero(AudioSource audioSource)
		{
			if (audioSource == null)
			{
				Debug.LogWarning("GlobalAudioController: AudioSource is null! Assign AudioSource in Inspector.", this);
				yield break;
			}
			while (audioSource.volume > 0)
			{
				audioSource.volume -= Time.deltaTime * _volumeChangeRate;
				if (audioSource.volume < 0)
					audioSource.volume = 0;

				yield return new WaitForEndOfFrame();
			}
		}

		private IEnumerator VolumeToFull(AudioSource audioSource, bool music)
		{
			while (audioSource.volume < (music ? _maxMusicVolume : _maxAmbienceVolume))
			{
				audioSource.volume += Time.deltaTime * _volumeChangeRate;

				if (audioSource.volume > (music ? _maxMusicVolume : _maxAmbienceVolume))
					audioSource.volume = (music ? _maxMusicVolume : _maxAmbienceVolume);

				yield return new WaitForEndOfFrame();
			}
		}

		private SeasonAudioData GetDataBySeason(Season season)
		{
			for (int i = 0; i < _audioData.Length; i++)
			{
				if (_audioData[i].Season == season)
					return _audioData[i];
			}

			Debug.LogError($"No audio data found for the season '{season}'");
			return null;
		}

		private void Start()
		{
			Debug.Log($"GlobalAudioController.Start() called on GameObject: {gameObject.name}, Scene: {gameObject.scene.name}", this);
			Debug.Log($"_musicSource is null: {_musicSource == null}, _ambienceSource is null: {_ambienceSource == null}", this);

			// Fallback: Try to find AudioSources on the GameObject if serialized references are null
			if (_musicSource == null)
			{
				_musicSource = GetComponent<AudioSource>();
				if (_musicSource != null)
					Debug.Log("GlobalAudioController: Found _musicSource via GetComponent fallback", this);
			}

			if (_ambienceSource == null)
			{
				AudioSource[] sources = GetComponents<AudioSource>();
				if (sources.Length > 1)
				{
					_ambienceSource = sources[1];
					Debug.Log("GlobalAudioController: Found _ambienceSource via GetComponent fallback (second AudioSource)", this);
				}
				else if (sources.Length > 0 && _musicSource != sources[0])
				{
					_ambienceSource = sources[0];
					Debug.Log("GlobalAudioController: Found _ambienceSource via GetComponent fallback", this);
				}
			}

			if (_musicSource == null || _ambienceSource == null)
			{
				Debug.LogError($"GlobalAudioController: AudioSources not assigned in Inspector! GameObject: {gameObject.name}, Scene: {gameObject.scene.name}. Audio will not play.", this);
				return;
			}

			_seasonProcessor.OnSeasonChanging += OnSeasonChange;
			_dayNightProcessor.OnNightStarting += OnNightStarted;
			_dayNightProcessor.OnDayStarting += OnDayStarted;
			StartNextTrack(_seasonProcessor.GetCurrentSeason(), true);
			_ambienceSource.volume = _maxAmbienceVolume;
		}

		private void Update()
		{
			if (_musicSource == null || _ambienceSource == null)
				return;

			if (_timeUntilMusicPlays > 0)
			{
				_timeUntilMusicPlays -= Time.deltaTime;
			}
			else
			{
				StartNextTrack(_seasonProcessor.GetCurrentSeason(), _dayNightProcessor.IsDayTime);
				_ambienceSource.Play();
				_musicSource.Play();
			}
		}

	}
}
