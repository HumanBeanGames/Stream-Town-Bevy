using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

using AudioSettings = ScriptablesProcessorInfrastructure.AudioSettings;
namespace Data.Containers
{
	public class AudioSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private AudioSettings _audioSettings;

		public AudioSettings AudioSettings => _audioSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_audioSettings);
		}
	}
}
