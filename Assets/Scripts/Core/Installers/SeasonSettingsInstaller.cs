using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for SeasonSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class SeasonSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private SeasonSettings _seasonSettings;

		public SeasonSettings SeasonSettings => _seasonSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
