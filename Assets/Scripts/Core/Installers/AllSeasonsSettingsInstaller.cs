using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for AllSeasonsSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class AllSeasonsSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private AllSeasonsSettings _allSeasonsData;

		public AllSeasonsSettings AllSeasonsData => _allSeasonsData;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
